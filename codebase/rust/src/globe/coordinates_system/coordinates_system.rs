use std::collections::HashMap;
use godot::prelude::*;

use crate::globe::territories::territory::TerritoryId;
use super::surface_point::Coordinates;

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
  pub const NUM_OF_WAYPOINTS: usize = 10;
  
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
  
  fn radius_scale(trajectory_point: Vector3, radius: f32) -> Vector3 {
    Vector3 {
      x: trajectory_point.x * radius,
      y: trajectory_point.y * radius,
      z: trajectory_point.z * radius,
    }
  }
}