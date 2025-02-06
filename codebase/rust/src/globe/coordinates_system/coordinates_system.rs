use std::{cell::RefCell, collections::HashMap, rc::Rc};
use godot::{classes::World3D, prelude::*};

use crate::globe::territories::territory::TerritoryId;
use super::surface_point::{Coordinates, SurfacePoint};

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

  /// TODO: Create a doc
  pub fn get_in_the_frontiers_trajectory(
    origin: Vector3,
    destination: Vector3,
    radius: f32,
    world: Gd<World3D>,
    jurisdiction_territory_id: TerritoryId,
  ) {
    let world = Rc::new(RefCell::new(world));
    let geodesic_trajectory = Self::get_geodesic_trajectory(origin, destination, radius);
    
    for trajectory_point in geodesic_trajectory.iter() {
      let world = Rc::clone(&world);
      let mut world = world.borrow_mut();

      let surface_point = SurfacePoint::get_surface_point(
        *trajectory_point,
        &mut world,
        Some(1.3)
      );
      
      let Some(surface_point) = surface_point else {
        return;
      };

      let surface_point = surface_point.bind();
      let Some(ref trajectory_territory_id) = surface_point.surface_point_metadata.territory_id else {
        return;
      };

      if jurisdiction_territory_id != *trajectory_territory_id {
        godot_print!("should recreate trajectory");

        // TODO: Pathfinding draft:
        // Create a recursion using the surface_point_metadata.lat_long
        // then, increase/decrease +N -N on the lat/long
        // until it finds a territory_id that matches the jurisdiction_territory_id
        // N should be 2? 3? 10?
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