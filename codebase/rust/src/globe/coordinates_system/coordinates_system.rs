use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::DerefMut, rc::Rc, sync::{Arc, Mutex}};
use godot::{classes::World3D, prelude::*};

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

  /// STILL WIP
  /// 
  /// TODO: Create a doc
  pub fn get_in_the_frontiers_trajectory(
    origin: Vector3,
    destination: Vector3,
    radius: f32,
    world: &mut Gd<World3D>,
    within_the_territory_id: &TerritoryId,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
  ) -> Vec<Vector3> {
    // Needs to be a Arc because it's going to be passed to the recursive function
    let base_arc_world = Arc::new(Mutex::new(world));
    let geodesic_trajectory = Self::get_geodesic_trajectory(origin, destination, radius);

    let mut in_the_frontiers_trajectory: Vec<Vector3> = Vec::new();

    for (index, trajectory_point) in geodesic_trajectory.iter().enumerate() {

      let world = Arc::clone(&base_arc_world);
      let Ok(mut world) = world.lock() else {
        godot_error!("6548464 Error getting world");
        continue;
      };

      let surface_point = SurfacePoint::get_surface_point(
        *trajectory_point,
        &mut world,
        Some(1.3)
      );
      
      let Some(surface_point) = surface_point else {
        continue;
      };

      let surface_point = surface_point.bind();
      let Some(ref trajectory_territory_id) = surface_point.surface_point_metadata.territory_id else {
        continue;
      };

      if within_the_territory_id != trajectory_territory_id {

        let (latitude, longitude) = surface_point.surface_point_metadata.lat_long;
        const BUFFER: i16 = 2;

        // TODO: Check if it's more natural move  Norteast, Northwest, Southeast, Southwest
        let possible_waypoints = [
          // Trajectory passing by North
          (latitude + BUFFER, longitude),
          // Trajectory passing by South
          (latitude - BUFFER, longitude),
          // Trajectory passing by East
          (latitude, longitude + BUFFER),
          // Trajectory passing by West
          (latitude, longitude - BUFFER),
        ];

        for possible_waypoint in possible_waypoints.iter() {
          // Waypoint doesn't exist in the coordinate_map
          let Some(possible_waypoint_metadata) = virtual_planet.coordinate_map.get(possible_waypoint) else {
            // coordinate does not exist
            continue;
          };

          // Waypoint is not in the same territory
          if possible_waypoint_metadata.territory_id.as_ref().is_some_and(
            |possible_waypoint_territory_id| possible_waypoint_territory_id != within_the_territory_id
          ) {
            break;
          }
          
          let world = Arc::clone(&base_arc_world);
          let Ok(mut world) = world.lock() else {
            godot_error!("8794618 Error getting world");
            continue;
          };

          // Recursively checking if the geodesic waypoint->destination is in the frontiers
          let remaining_points = Self::get_in_the_frontiers_trajectory(
            possible_waypoint_metadata.cartesian,
            destination,
            radius,
            &mut world,
            within_the_territory_id,
            virtual_planet,
          );


          for initial_points_in_the_frontiers in &geodesic_trajectory[0..index] {
            in_the_frontiers_trajectory.push(*initial_points_in_the_frontiers);
          }

          in_the_frontiers_trajectory.extend(remaining_points);
          return in_the_frontiers_trajectory;
        }
      }
    }

    // TODO: If recursion doesnt find a path, should do basic geodesic trajectory to no crash the program
    return geodesic_trajectory.to_vec();

  }
  
  fn radius_scale(trajectory_point: Vector3, radius: f32) -> Vector3 {
    Vector3 {
      x: trajectory_point.x * radius,
      y: trajectory_point.y * radius,
      z: trajectory_point.z * radius,
    }
  }
}