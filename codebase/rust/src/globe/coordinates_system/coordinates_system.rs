use std::{cell::RefCell, collections::HashMap, rc::Rc};
use godot::{classes::World3D, obj::BaseRef, prelude::*};
use std::collections::VecDeque;

use crate::{globe::territories::territory::TerritoryId, troops::troop::Troop};
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

  fn passes_by_other_territories(
    base_geodesic_trajectory: &[Vector3; Self::NUM_OF_WAYPOINTS],
    world: Rc<RefCell<Gd<World3D>>>,
    within_the_territory_id: &TerritoryId,
  ) -> bool {
    base_geodesic_trajectory.iter().find(|trajectory_point| {
      let Some(surface_point) = SurfacePoint::get_surface_point(
      **trajectory_point,
        &mut world.borrow_mut(),
        None
      ) else {
        godot_error!("passes_by_other_territories() -> Error getting surface point");
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
    troop: BaseRef<'_, Troop>,
  ) -> Vec<Vector3> {
    let base_rc_world: Rc<RefCell<Gd<World3D>>> = Rc::new(RefCell::new(world));
    let base_geodesic_trajectory = Self::get_geodesic_trajectory(origin, destination, radius);

    if !Self::passes_by_other_territories(
      &base_geodesic_trajectory,
      Rc::clone(&base_rc_world),
      within_the_territory_id
    ) {
      return base_geodesic_trajectory.to_vec();
    }

    let world = Rc::clone(&base_rc_world);
    let mut world = world.borrow_mut();

    let origin_lat_long = SurfacePoint::get_lat_long_from_vec3(
      origin,
      &mut world
      ).expect("Expected origin_lat_long to exist");
    let dest_lat_long = SurfacePoint::get_lat_long_from_vec3(
      destination,
      &mut world
      ).expect("Expected dest_lat_long to exist");

    let heat_map_dictionary = troop.get_meta("heat_map_for_within_territory_trajectory");
    let mut heat_map_dictionary = heat_map_dictionary.to::<Dictionary>();
    heat_map_dictionary.clear();
    let _ = heat_map_dictionary.insert(format!("{:?}", origin_lat_long), 0);

    let Some(populated_heat_map) = Self::populate_heat_map(
        origin_lat_long,
        dest_lat_long,
        within_the_territory_id,
        &virtual_planet,
        &troop
      ) else {
      return base_geodesic_trajectory.to_vec();
    };  

    let mut in_the_frontiers_coordinates: VecDeque<Coordinates> = VecDeque::from(vec![]);
    Self::back_trace_dest_to_origin(
      &populated_heat_map,
      origin_lat_long,
      dest_lat_long,
      &mut in_the_frontiers_coordinates,
    );

    let in_the_frontiers_trajectory = in_the_frontiers_coordinates.iter().map(|coordinate| {
      let cartesian: Vector3 = virtual_planet.get_cartesian_from_coordinates(coordinate);
      cartesian
    }).collect::<Vec<Vector3>>();

    if in_the_frontiers_coordinates.is_empty() {
      return base_geodesic_trajectory.to_vec();
    }

    return in_the_frontiers_trajectory;
  }

  fn populate_heat_map(
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    within_the_territory_id: &TerritoryId,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
    troop: &BaseRef<'_, Troop>,
  ) -> Option<Dictionary> {
    let heat_map_dictionary = troop
      .get_meta("heat_map_for_within_territory_trajectory")
      .to::<Dictionary>();

    if heat_map_dictionary.contains_key(format!("{:?}", dest_lat_long)) {
      return Some(heat_map_dictionary);
    }

    let neighbors = Self::get_neighbors(origin_lat_long);
    for neighbor in neighbors.iter() {
      let heat_map_dictionary_key = format!("{:?}", neighbor);
      let heat_map_dictionary_key = heat_map_dictionary_key.as_str();

      let heat_map_dictionary = troop
        .get_meta("heat_map_for_within_territory_trajectory")
        .to::<Dictionary>();
      if heat_map_dictionary.contains_key(heat_map_dictionary_key) {
        continue;
      }

      let dic_coordinates_map = virtual_planet.base().get_meta("coordinates_map");
      let dic_coordinates_map = dic_coordinates_map.to::<Dictionary>();
      let neighbor_metadata = dic_coordinates_map.get(heat_map_dictionary_key);

      if neighbor_metadata.is_none() {
        continue;
      }

      let in_other_territory = neighbor_metadata.unwrap()
        .to::<Dictionary>()
        .get("territory_id")
        .is_some_and(
          |neighbor_territory_id| neighbor_territory_id.to::<String>() != *within_the_territory_id
        );

      if in_other_territory {
        let mut heat_map_dictionary = troop
          .get_meta("heat_map_for_within_territory_trajectory")
          .to::<Dictionary>();
        let _ = heat_map_dictionary.set(heat_map_dictionary_key, i32::MAX);
        continue;
      }

      let distance_level_from_origin = Self::get_lowest_distance_from_neighbors(
        &Self::get_neighbors(*neighbor).to_vec(),
        &troop
      );

      let neighbor_distance = distance_level_from_origin + 1;
      let mut heat_map_dictionary = troop
        .get_meta("heat_map_for_within_territory_trajectory")
        .to::<Dictionary>();
      let _ = heat_map_dictionary.set(heat_map_dictionary_key, neighbor_distance);

      if let Some(result_map) = Self::populate_heat_map(
        *neighbor,
        dest_lat_long,
        within_the_territory_id,
        virtual_planet,
        troop,
      ) {
        return Some(result_map);
      }
    }
    
    None
  }

  // Helper function with improved lock handling
  fn get_lowest_distance_from_neighbors(
    neighbors: &Vec<Coordinates>,
    troop: &BaseRef<'_, Troop>,
  ) -> i32 {
    let heat_map_dictionary = troop
      .get_meta("heat_map_for_within_territory_trajectory")
      .to::<Dictionary>();

    let min_distance = neighbors.iter()
      .filter_map(|neighbor| heat_map_dictionary.get(format!("{:?}", neighbor)))
      .fold(i32::MAX, |acc, distance| {
        let distance = distance.to::<i32>();
        acc.min(distance)
      });
    min_distance
  }
  
  fn get_neighbors(
    current_coordinate: Coordinates,
  ) -> [Coordinates; 8] {
    const BUFFER: i16 = 1;

    let (latitude, longitude) = current_coordinate;

    let mut latitude_east = latitude + BUFFER;
    let mut latitude_west = latitude - BUFFER;
    let mut longitude_north = longitude + BUFFER;
    let mut longitude_south = longitude - BUFFER;

    if latitude == VirtualPlanet::get_num_of_latitudes() -1 {
      latitude_east = 0;
    }

    if latitude == 0 {
      latitude_west = VirtualPlanet::get_num_of_latitudes() -1;
    }

    if longitude == VirtualPlanet::get_num_of_longitudes() -1 {
      longitude_north = 0;
    }

    if longitude == 0 {
      longitude_south = VirtualPlanet::get_num_of_longitudes() -1;
    }

    let neighbors = [
      // Trajectory passing by North
      (latitude, longitude_north),
      // Trajectory passing by South
      (latitude, longitude_south),
      // Trajectory passing by East
      (latitude_east, longitude),
      // Trajectory passing by West
      (latitude_west, longitude),

      // Trajectory passing by Northeast
      (latitude_east, longitude_north),
      // Trajectory passing by Northwest
      (latitude_west, longitude_north),
      // Trajectory passing by Southeast
      (latitude_east, longitude_south),
      // Trajectory passing by Southwest
      (latitude_west, longitude_south),
    ];

    neighbors
  }

  fn back_trace_dest_to_origin(
    // heat_map: &HashMap<Coordinates, i32>,
    heat_map: &Dictionary,
    origin_lat_long: Coordinates,
    dest_lat_long: Coordinates,
    in_the_frontiers_coordinates: &mut VecDeque<Coordinates>,
  ) {
    
    if let Some(dest_distance) = heat_map.get(format!("{:?}", dest_lat_long)){
      let dest_neighbors = Self::get_neighbors(dest_lat_long);

      for neighbor in dest_neighbors.iter() {
        if let Some(neighbor_distance) = heat_map.get(format!("{:?}", neighbor)) {

          let dest_distance = dest_distance.to::<i32>();
          let neighbor_distance = neighbor_distance.to::<i32>();

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