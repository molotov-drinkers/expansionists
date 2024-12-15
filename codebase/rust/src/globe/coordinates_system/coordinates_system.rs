use std::collections::HashMap;
use godot::prelude::*;

use crate::globe::territory::types::TerritoryId;
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
  
  /// Receives the origin and destination coordinates and 
  /// returns a list of coordinates represented by the 
  /// trajectory where a moving point would pass by.
  pub fn get_geodesic_trajectory(
    origin: Vector3,
    destination: Vector3,
    radius: f32
  ) -> Vec<Vector3> {
    let origin = origin.normalized();
    let destination = destination.normalized();

    let mut trajectory = vec![];

    // TODO: make this dynamic
    let hard_coded_num_of_points = 100;

    for i in 0..hard_coded_num_of_points{
      let t = i as f64 / (hard_coded_num_of_points - 1) as f64;

      let trajectory_point = origin.slerp(destination, t as f32);
      let trajectory_point = Self::radius_scale(trajectory_point, radius);
      trajectory.push(trajectory_point);
    }
    // godot_print!("Trajectory size is {:?}", trajectory.len());

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