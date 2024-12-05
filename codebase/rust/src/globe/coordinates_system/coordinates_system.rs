use std::collections::HashMap;
use godot::prelude::*;

use crate::{camera::player_camera::CameraDirection, globe::territory::types::TerritoryId};
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
  
  //TODO: Implement Geodesic coordinate system
  /// Receives the origin and destination coordinates and 
  /// returns a list of coordinates represented by the 
  /// trajectory where a moving point would pass by.
  pub fn get_geodesic_trajectory(
    origin: Coordinates,
    destination: Coordinates,
    coordinate_map: &CoordinateMap,
  ) -> Vec<Vector3> {
    let origin = coordinate_map.get(&origin).unwrap().cartesian;
    let destination = coordinate_map.get(&destination).unwrap().cartesian;
    let origin = origin.normalized();
    let destination = destination.normalized();

    let mut trajectory = vec![];

    // TODO: make this dynamic
    let hard_coded_num_of_points = 100;

    // TODO: use dynamic radius
    // let radius = 1.09;
    let radius = 1.08;

    for i in 0..hard_coded_num_of_points{
      let t = i as f64 / (hard_coded_num_of_points - 1) as f64;

      let trajectory_point = origin.slerp(destination, t as f32);
      let trajectory_point = Self::radius_scale(trajectory_point, radius);
      trajectory.push(trajectory_point);
    }
    godot_print!("Trajectory size is {:?}", trajectory.len());

    trajectory
  }
  
  fn radius_scale(trajectory_point: Vector3, radius: f32) -> Vector3 {
    Vector3 {
      x: trajectory_point.x * radius,
      y: trajectory_point.y * radius,
      z: trajectory_point.z * radius,
    }
  }

  /// Receives the current position of the camera,
  /// which is the central globe coordinate the camera is pointing to
  /// and the the direction the camera is going to move to
  /// returns the new position of the camera
  pub fn get_geodesic_neighbour_position(
    _looking_at: Coordinates,
    _current_position: Vector3,
    _direction: CameraDirection,
    // coordinate_map: &CoordinateMap,
  ) -> Vector3 {
    let neighbour_position = Vector3::new(0.0, 0.0, 0.0);

    // TODO it's not just translate the trasform position,
    // the looking_at point also matters, so probably needs to use it or use "rotation"

    neighbour_position
  }
}