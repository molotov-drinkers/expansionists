use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc, sync::{Arc, Mutex}};
use godot::{classes::World3D, prelude::*};
use std::collections::VecDeque;

use crate::globe::territories::territory::TerritoryId;
use super::{surface_point::{Coordinates, SurfacePoint}, virtual_planet::VirtualPlanet};

#[derive(Debug)]
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

    // Check if base geodesic trajectory could be used
    // let world = Rc::clone(&base_rc_world);
    // let passes_by_other_territories = Self::passes_by_other_territories(&base_geodesic_trajectory, world, within_the_territory_id);
    // if passes_by_other_territories {
      // return base_geodesic_trajectory.to_vec();
    // }

    let mapper: Arc<Mutex<HashMap<Coordinates, i32>>> = Arc::new(Mutex::new(HashMap::new()));

    let world = Rc::clone(&base_rc_world);
    let mut world = world.borrow_mut();

    let origin_lat_long = SurfacePoint::get_lat_long_from_vec3(origin, &mut world)
      .expect("Expected origin_lat_long to exist");
    let dest_lat_long = SurfacePoint::get_lat_long_from_vec3(destination, &mut world)
      .expect("Expected dest_lat_long to exist");

    godot_print!("origin_lat_long: {:?}.... dest_lat_long: {:?}", origin_lat_long, dest_lat_long);

    let distance_level_from_origin = 0;
    mapper.lock().unwrap().insert(origin_lat_long, distance_level_from_origin);

    let mut visited_set: HashSet<Coordinates> = HashSet::new();
    visited_set.insert(origin_lat_long);
    let mut b: HashMap<Coordinates, i32> = HashMap::new();
    b.insert(origin_lat_long, distance_level_from_origin);

    let g = Self::populate_heat_map(
      // &Arc::clone(&mapper),
      origin_lat_long,
      dest_lat_long,
      // distance_level_from_origin,
      within_the_territory_id,
      virtual_planet,
      &mut visited_set,
      &mut b,
    );
    godot_print!("mapper: {:?}", b);

    // let mapper = Arc::clone(&mapper);
    // let mapper = mapper.lock().expect("Expected mapper to exist");
    // let mut in_the_frontiers_coordinates: VecDeque<Coordinates> = VecDeque::from(vec![]);
    // Self::trace_back_dest_to_origin(&mapper, origin_lat_long, dest_lat_long, &mut in_the_frontiers_coordinates);
    // godot_print!("in_the_frontiers_coordinates: {:?}", in_the_frontiers_coordinates);

    // todo: implement the conversion from coordinates to Vector3
    
    return base_geodesic_trajectory.to_vec();
  }

  fn populate_heat_map(
    // mapper: &Arc<Mutex<HashMap<Coordinates, i32>>>,
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    // mut distance_level_from_origin: i32,
    within_the_territory_id: &TerritoryId,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
    visited_set: &mut HashSet<Coordinates>,
    b: &mut HashMap<Coordinates, i32>,
  ) -> HashMap<Coordinates, i32> {

    let neighbors = Self::get_neighbors(origin_lat_long);

    for neighbor in neighbors.iter() {
      if b.contains_key(neighbor) {
        continue;
      }

      if b.contains_key(&dest_lat_long) {
        godot_print!("stop condition! b.contains_key(&dest_lat_long)");
        break;
      }

      let Some(neighbor_metadata) = virtual_planet.coordinate_map.get(neighbor)
        else { continue; };

      let in_other_territory = neighbor_metadata
        .territory_id
        .as_ref()
        .is_some_and(|neighbor_territory_id| { neighbor_territory_id != within_the_territory_id });

      let on_the_water = neighbor_metadata.territory_id.is_none();

      if in_other_territory || on_the_water {
        b.insert(*neighbor, i32::MAX);
        continue;
      }

      let distance_level_from_origin =  Self::get_lowest_distance_from_neighbors(&neighbors, &b);
      godot_print!("{origin_lat_long:?} :: distance_level_from_origin: {distance_level_from_origin:?}");

      let neighbor_distance  = distance_level_from_origin + 1;
      b.insert(*neighbor, neighbor_distance);
      godot_print!("b.insert({neighbor:?}, {neighbor_distance});");

      let a = Self::populate_heat_map(
        // &Arc::clone(&mapper),
        *neighbor,
        dest_lat_long,
        // distance_level_from_origin + 1,
        within_the_territory_id,
        virtual_planet,
        visited_set,
        b,
      );
      b.extend(a);
    }

    b.clone()
  }


  fn get_lowest_distance_from_neighbors(neighbors: &[Coordinates; 8], b: &HashMap<Coordinates, i32>,) -> i32 {
    let a = neighbors.iter().fold(i32::MAX, |acc, neighbor| {
      let neighbor_distance = b.get(neighbor);

      if neighbor_distance.is_some_and(|&neighbor_distance| neighbor_distance < acc)  {
        *neighbor_distance.unwrap()
      } else {
        acc
      }
    });

    if a == i32::MAX { 1 } else { a }
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
      godot_print!("!!! longitude == VirtualPlanet::get_num_of_longitudes()");
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


  fn trace_back_dest_to_origin(
    mapper: &HashMap<Coordinates, i32>,
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    in_the_frontiers_coordinates: &mut VecDeque<Coordinates>,
  ) {
    let dest_distance = mapper.get(&dest_lat_long)
      .expect("Expected dest_lat_long to exist");

    let dest_neighbors = Self::get_neighbors(dest_lat_long);

    for neighbor in dest_neighbors.iter() {
      let neighbor_distance = mapper.get(neighbor)
        .expect("Expected neighbor_distance to exist");

      if neighbor_distance < dest_distance {
        // in_the_frontiers_coordinates.insert(0, *neighbor);
        in_the_frontiers_coordinates.push_front(*neighbor);
        
        if neighbor == &origin_lat_long {
          break;
        }
        Self::trace_back_dest_to_origin(mapper, origin_lat_long, *neighbor, in_the_frontiers_coordinates);
        break;
      }
    }
  }

  fn radius_scale(trajectory_point: Vector3, radius: f32) -> Vector3 {
    Vector3 {
      x: trajectory_point.x * radius,
      y: trajectory_point.y * radius,
      z: trajectory_point.z * radius,
    }
  }
}