use std::{cell::RefCell, cmp::Ordering, collections::{HashMap, HashSet}, rc::Rc, sync::{Arc, Mutex}};
use godot::{classes::World3D, prelude::*};

use crate::globe::{coordinates_system::surface_point::SurfacePointMetadata, territories::territory::TerritoryId};
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

  /// 
  /// 
  /// 
  pub fn get_in_the_frontiers_trajectory(
    origin: Vector3,
    destination: Vector3,
    radius: f32,
    world: Gd<World3D>,
    within_the_territory_id: &TerritoryId,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
  ) -> Vec<Vector3> {
    let base_rc_world = Rc::new(RefCell::new(world));
    let base_geodesic_trajectory = Self::get_geodesic_trajectory(origin, destination, radius);

    // Check if base geodesic trajectory could be used
    let passes_by_other_territories = base_geodesic_trajectory.iter().find(|trajectory_point| {
      let world = Rc::clone(&base_rc_world);
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
    });
    if passes_by_other_territories.is_none() {
      return base_geodesic_trajectory.to_vec();
    }

    // Creates near optimal path
    let mut near_optimal_path: Vec<Vector3> = Vec::new();
    near_optimal_path.push(origin);

    loop {
      let current = near_optimal_path.last().expect("Expected last to exist");
      let world = Rc::clone(&base_rc_world);
      let mut world = world.borrow_mut();

      let Some(surface_point) = SurfacePoint::get_surface_point(
        *current,
        &mut world,
        None
      ) else {
        godot_error!("======> Error getting surface point");
        return base_geodesic_trajectory.to_vec();
      };
      let surface_point = surface_point.bind();
      let surface_point_metadata = surface_point.get_surface_point_metadata();
      let current_coordinate = surface_point_metadata.lat_long;

      const BUFFER: i16 = 1;
      let neighbors: [Coordinates; 8] = [
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
      ];

      let Some(closest_neighbor) = neighbors
        .iter()
        .filter(|neighbor| {
          let Some(neighbor_metadata) = virtual_planet.coordinate_map.get(neighbor)
            else { return false; };

          let within_the_territory = neighbor_metadata
            .territory_id
            .as_ref()
            .is_some_and(|neighbor_territory_id| {
              neighbor_territory_id == within_the_territory_id
            });

          // assuming none is water
          let on_the_water = neighbor_metadata.territory_id.is_none();

          within_the_territory || on_the_water
        })
        .min_by(|neighbor_a, neighbor_b| {
          let Some(neighbor_a_metadata) = virtual_planet.coordinate_map.get(neighbor_a) else { return Ordering::Equal; };
          let Some(neighbor_b_metadata) = virtual_planet.coordinate_map.get(neighbor_b) else { return Ordering::Equal; };
  
          let neighbor_a_distance = neighbor_a_metadata.cartesian.distance_to(destination);
          let neighbor_b_distance = neighbor_b_metadata.cartesian.distance_to(destination);
  
          neighbor_a_distance.partial_cmp(&neighbor_b_distance).unwrap()
        })
        else {
          godot_print!("======> ERROR getting closest_neighbor");
          break;
        };

      let near_optimal_next_point = virtual_planet.coordinate_map.get(closest_neighbor);

      if near_optimal_next_point.is_some() {
        let near_optimal_next_point = near_optimal_next_point.unwrap();
        near_optimal_path.push(near_optimal_next_point.cartesian);
        if near_optimal_path.len() == Self::NUM_OF_WAYPOINTS {
          break;
        }
      } else {
        godot_print!("======> ERROR getting near_optimal_next_point");
        break;
      }
    }
    
    near_optimal_path
  }

  fn radius_scale(trajectory_point: Vector3, radius: f32) -> Vector3 {
    Vector3 {
      x: trajectory_point.x * radius,
      y: trajectory_point.y * radius,
      z: trajectory_point.z * radius,
    }
  }
}