use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex, RwLock}};
use godot::{classes::World3D, prelude::*};
use std::collections::VecDeque;

use crate::globe::territories::territory::TerritoryId;
use super::{surface_point::{Coordinates, SurfacePoint}, virtual_planet::VirtualPlanet};

#[derive(Debug, Clone)]
pub struct CoordinateMetadata {
  pub territory_id: Option<TerritoryId>,
  pub cartesian: Vector3,
}

/// It maps the coordinates of the planet to the metadata of the coordinates
/// Populated by the `VirtualPlanet::populate_surface_points_and_coordinate_map` method
pub type CoordinateMap = HashMap<Coordinates, CoordinateMetadata>;

pub struct CoordinatesSystem {}

impl CoordinatesSystem {

  // (TODO:)
  // Create a dynamic way to calculate the number of waypoints based on linear distance between origin and destination
  // possibly using (.distance_to)
  // Then clamp it between 10 and 50, should be enough for most cases
  pub const NUM_OF_WAYPOINTS: usize = 30;
  
  /// Receives the origin and destination coordinates and 
  /// returns a list of coordinates represented by the 
  /// trajectory where a moving point would pass by.
  /// 
  /// It returns an array of NUM_OF_WAYPOINTS size, this way the compiler 
  /// puts the array on the stack instead of the heap. Providing a better performance.
  pub fn get_geodesic_trajectory(
    origin: Vector3,
    destination: Vector3,
    radius: f32
  ) -> [Vector3; Self::NUM_OF_WAYPOINTS] {
    let origin = origin.normalized();
    let destination = destination.normalized();

    let mut trajectory = [Vector3::ZERO; Self::NUM_OF_WAYPOINTS];

    for i in 0..Self::NUM_OF_WAYPOINTS{
      let t = i as f64 / (Self::NUM_OF_WAYPOINTS - 1) as f64;

      let trajectory_point = origin.slerp(destination, t as f32);
      let trajectory_point = Self::radius_scale(trajectory_point, radius);
      trajectory[i] = trajectory_point;
    }

    trajectory
  }


  fn _passes_by_other_territories(
    base_geodesic_trajectory: &[Vector3; Self::NUM_OF_WAYPOINTS],
    world: Rc<RefCell<Gd<World3D>>>,
    within_the_territory_id: &TerritoryId,
  ) -> bool {
    base_geodesic_trajectory.iter().find(|trajectory_point| {
      let mut world = world.borrow_mut();

      let Some(surface_point) = SurfacePoint::get_surface_point(
      **trajectory_point,
        &mut world,
        None
      ) else {
        godot_error!(" 486483 Error getting surface point");
        return false;
      };

      let surface_point = surface_point.bind();
      let passes_by_other_territories = surface_point.surface_point_metadata.territory_id.as_ref().is_some_and(|territory_id| {
        territory_id != within_the_territory_id
      });

      passes_by_other_territories
    }).is_some()
  }

  /// It implements Flow Field Pathfinding
  /// Applying tchebychev distance to calculate the distance between two points
  /// Also, considering the planet as a grid, where each cell is a coordinate
  /// And mainly, respecting the territory boundaries
  /// https://www.youtube.com/watch?v=ZJZu3zLMYAc
  /// 
  pub fn get_in_the_frontiers_trajectory(
    origin: Vector3,
    destination: Vector3,
    radius: f32,
    world: Gd<World3D>,
    within_the_territory_id: &TerritoryId,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
  ) -> Vec<Vector3> {
    let base_rc_world: Rc<RefCell<Gd<World3D>>> = Rc::new(RefCell::new(world));
    let base_geodesic_trajectory = Self::get_geodesic_trajectory(origin, destination, radius);

    let world = Rc::clone(&base_rc_world);
    let mut world = world.borrow_mut();

    let origin_lat_long = SurfacePoint::get_lat_long_from_vec3(origin, &mut world)
      .expect("Expected origin_lat_long to exist");
    let dest_lat_long = SurfacePoint::get_lat_long_from_vec3(destination, &mut world)
      .expect("Expected dest_lat_long to exist");

    let distance_level_from_origin = 0;
    
    godot_print!("origin_lat_long: {:?}.... dest_lat_long: {:?}", origin_lat_long, dest_lat_long);
    let heat_map: Arc<Mutex<HashMap<Coordinates, i32>>> = Arc::new(Mutex::new(HashMap::new()));

    {
      let mut heat_map_lock = heat_map.lock().unwrap();
      heat_map_lock.insert(origin_lat_long, distance_level_from_origin);
    }

    // Concurrent read access to the coordinate_map made us create a Arc RwLock
    let coordinate_map_arc = Arc::new(RwLock::new(virtual_planet.coordinate_map.clone()));

    let Some(populated_heat_map) = Self::populate_heat_map(
        origin_lat_long,
        dest_lat_long,
        within_the_territory_id,
        &coordinate_map_arc,
        &heat_map,
      ) else {
      return base_geodesic_trajectory.to_vec();
    };  
    godot_print!("heat_map: {:?}", populated_heat_map);

    let mut in_the_frontiers_coordinates: VecDeque<Coordinates> = VecDeque::from(vec![]);
    Self::back_trace_dest_to_origin(
      &populated_heat_map,
      origin_lat_long,
      dest_lat_long,
      &mut in_the_frontiers_coordinates,
    );
    // godot_print!("in_the_frontiers_coordinates: {:?}", in_the_frontiers_coordinates);

    let in_the_frontiers_trajectory = in_the_frontiers_coordinates.iter().map(|coordinate| {
      let cartesian: Vector3 = virtual_planet.get_cartesian_from_coordinates(coordinate);
      cartesian
    }).collect::<Vec<Vector3>>();

    // godot_print!("in_the_frontiers_trajectory: {:?}", in_the_frontiers_trajectory);
    
    if in_the_frontiers_coordinates.is_empty() {
      // godot_print!("in_the_frontiers_coordinates.is_empty()");
      return base_geodesic_trajectory.to_vec();
    }

    return in_the_frontiers_trajectory;
  }

  fn populate_heat_map(
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    within_the_territory_id: &TerritoryId,
    arc_coordinate_map: &Arc<RwLock<HashMap<Coordinates, CoordinateMetadata>>>,
    arc_heat_map: &Arc<Mutex<HashMap<Coordinates, i32>>>,
  ) -> Option<HashMap<Coordinates, i32>> {
    // Check destination first with a single lock acquisition
    {
      let heat_map_lock = arc_heat_map.lock().unwrap();
      if heat_map_lock.contains_key(&dest_lat_long) {
        return Some(heat_map_lock.clone());
      }
    }

    let neighbors = Self::get_neighbors(origin_lat_long);
    
    for neighbor in neighbors.iter() {
      // Check heat map with minimal lock duration
      {
        let heat_map_lock = arc_heat_map.lock().unwrap();
        if heat_map_lock.contains_key(neighbor) {
          continue;
        }
      }

      // Scope the coordinate map lock to this iteration
      let neighbor_metadata = {
        let coordinate_map_lock = arc_coordinate_map.read().unwrap();
        match coordinate_map_lock.get(neighbor) {
          Some(metadata) => metadata.clone(),
          None => continue,
        }
      };

      let in_other_territory = neighbor_metadata
        .territory_id
        .as_ref()
        .is_some_and(|neighbor_territory_id| neighbor_territory_id != within_the_territory_id);

      if in_other_territory {
        let mut heat_map_lock = arc_heat_map.lock().unwrap();
        heat_map_lock.insert(*neighbor, i32::MAX);
        continue;
      }

      let distance_level_from_origin = Self::get_lowest_distance_from_neighbors(
        &Self::get_neighbors(*neighbor).to_vec(),
        arc_heat_map,
      );

      let neighbor_distance = distance_level_from_origin + 1;

      // Minimize lock duration for insert
      {
        let mut heat_map_lock = arc_heat_map.lock().unwrap();
        heat_map_lock.insert(*neighbor, neighbor_distance);
      }

      if let Some(result_map) = Self::populate_heat_map(
        *neighbor,
        dest_lat_long,
        within_the_territory_id,
        arc_coordinate_map,
        arc_heat_map,
      ) {
        return Some(result_map);
      }
    }
    
    None
  }
  // Helper function with improved lock handling
  fn get_lowest_distance_from_neighbors(
    neighbors: &Vec<Coordinates>,
    arc_heat_map: &Arc<Mutex<HashMap<Coordinates, i32>>>,
  ) -> i32 {
    let heat_map_lock = arc_heat_map.lock().unwrap();
    let min_distance = neighbors.iter()
      .filter_map(|neighbor| heat_map_lock.get(neighbor))
      .fold(i32::MAX, |acc, &distance| acc.min(distance));
    min_distance
  }
  
  fn get_neighbors(
    current_coordinate: Coordinates,
  ) -> [Coordinates; 8] {
    const BUFFER: i16 = 1;

    let (latitude, longitude) = current_coordinate;

    let mut latitude_north = latitude + BUFFER;
    let mut latitude_south = latitude - BUFFER;
    let mut longitude_east = longitude + BUFFER;
    let mut longitude_west = longitude - BUFFER;

    if latitude == VirtualPlanet::get_num_of_latitudes() {
      latitude_north = 0;
    }

    if latitude == 0 {
      latitude_south = VirtualPlanet::get_num_of_latitudes();
    }

    if longitude == VirtualPlanet::get_num_of_longitudes() {
      longitude_east = 0;
    }

    if longitude == 0 {
      longitude_west = VirtualPlanet::get_num_of_longitudes();
    }

    [
      // Trajectory passing by North
      (latitude_north, longitude),
      // Trajectory passing by South
      (latitude_south, longitude),
      // Trajectory passing by East
      (latitude, longitude_east),
      // Trajectory passing by West
      (latitude, longitude_west),

      // Trajectory passing by Northeast
      (latitude_north, longitude_east),
      // Trajectory passing by Northwest
      (latitude_north, longitude_west),
      // Trajectory passing by Southeast
      (latitude_south, longitude_east),
      // Trajectory passing by Southwest
      (latitude_south, longitude_west),
    ]
  }

  fn back_trace_dest_to_origin(
    heat_map: &HashMap<Coordinates, i32>,
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    in_the_frontiers_coordinates: &mut VecDeque<Coordinates>,
  ) {
    if let Some(dest_distance) = heat_map.get(&dest_lat_long){
      let dest_neighbors = Self::get_neighbors(dest_lat_long);

      for neighbor in dest_neighbors.iter() {
        if let Some(neighbor_distance) = heat_map.get(neighbor) {
          if neighbor_distance < dest_distance {
          // if (neighbor_distance -1) == *dest_distance {
            // in_the_frontiers_coordinates.insert(0, *neighbor);
            in_the_frontiers_coordinates.push_front(*neighbor);
            
            if neighbor == &origin_lat_long {
              break;
            }
            Self::back_trace_dest_to_origin(heat_map, origin_lat_long, *neighbor, in_the_frontiers_coordinates);
            break;
          }
        }
      }
      return;
    };

    godot_print!("dest_lat_long {dest_lat_long:?} not found in heat_map");
  }

  fn radius_scale(trajectory_point: Vector3, radius: f32) -> Vector3 {
    Vector3 {
      x: trajectory_point.x * radius,
      y: trajectory_point.y * radius,
      z: trajectory_point.z * radius,
    }
  }
}