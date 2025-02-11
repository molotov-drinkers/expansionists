use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};
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
    let distance_level_from_origin = 0;

    let world = Rc::clone(&base_rc_world);
    let mut world = world.borrow_mut();

    let origin_lat_long = SurfacePoint::get_lat_long_from_vec3(origin, &mut world)
      .expect("Expected origin_lat_long to exist");
    let dest_lat_long = SurfacePoint::get_lat_long_from_vec3(destination, &mut world)
      .expect("Expected dest_lat_long to exist");

    godot_print!("origin_lat_long: {:?}.... dest_lat_long: {:?}", origin_lat_long, dest_lat_long);

    Self::populate_heat_map(
      &Arc::clone(&mapper),
      origin_lat_long,
      dest_lat_long,
      distance_level_from_origin,
      within_the_territory_id,
      virtual_planet
    );
    godot_print!("mapper: {:?}", mapper);

    let mapper = Arc::clone(&mapper);
    let mapper = mapper.lock().expect("Expected mapper to exist");
    let mut in_the_frontiers_coordinates: VecDeque<Coordinates> = VecDeque::from(vec![]);
    Self::trace_back_dest_to_origin(&mapper, origin_lat_long, dest_lat_long, &mut in_the_frontiers_coordinates);
    godot_print!("in_the_frontiers_coordinates: {:?}", in_the_frontiers_coordinates);

    // todo: implement the conversion from coordinates to Vector3
    
    return base_geodesic_trajectory.to_vec();
  }

  fn populate_heat_map(
    mapper: &Arc<Mutex<HashMap<Coordinates, i32>>>,
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    distance_level_from_origin: i32,
    within_the_territory_id: &TerritoryId,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
  ) {
    let neighbors = Self::get_neighbors(origin_lat_long);
    
    if mapper.lock().unwrap().contains_key(&dest_lat_long) {
      return;
    }

    for neighbor in neighbors.iter() {
      let Some(neighbor_metadata) = virtual_planet.coordinate_map.get(neighbor)
        else { continue; };

      let in_other_territory = neighbor_metadata
        .territory_id
        .as_ref()
        .is_some_and(|neighbor_territory_id| {
          neighbor_territory_id != within_the_territory_id
        });

      // assuming none is water
      let on_the_water = neighbor_metadata.territory_id.is_none();
      let mut mapper_mut = mapper.lock().unwrap();

      if mapper_mut.contains_key(neighbor) {
        continue;
      };

      if in_other_territory || on_the_water {
        mapper_mut.insert(*neighbor, i32::MAX);
        continue;
      }

      mapper_mut.insert(*neighbor, distance_level_from_origin);

      Self::populate_heat_map(
        &Arc::clone(&mapper),
        *neighbor,
        dest_lat_long,
        distance_level_from_origin + 1,
        within_the_territory_id,
        virtual_planet
      );
    }
  }

  fn get_neighbors(
    current_coordinate: Coordinates,
  ) -> [Coordinates; 8] {
    const BUFFER: i16 = 1;
    [
      // Trajectory passing by North
      (current_coordinate.0 + BUFFER, current_coordinate.1),
      // Trajectory passing by South
      (current_coordinate.0 - BUFFER, current_coordinate.1),
      // Trajectory passing by East
      (current_coordinate.0, current_coordinate.1 + BUFFER),
      // Trajectory passing by West
      (current_coordinate.0, current_coordinate.1 - BUFFER),

      // Trajectory passing by Northeast
      (current_coordinate.0 + BUFFER, current_coordinate.1 + BUFFER),
      // Trajectory passing by Northwest
      (current_coordinate.0 + BUFFER, current_coordinate.1 - BUFFER),
      // Trajectory passing by Southeast
      (current_coordinate.0 - BUFFER, current_coordinate.1 + BUFFER),
      // Trajectory passing by Southwest
      (current_coordinate.0 - BUFFER, current_coordinate.1 - BUFFER),
    ]
  }


  fn trace_back_dest_to_origin(mapper: &HashMap<Coordinates, i32>, origin_lat_long: Coordinates, dest_lat_long: Coordinates, in_the_frontiers_coordinates: &mut VecDeque<Coordinates>) {
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