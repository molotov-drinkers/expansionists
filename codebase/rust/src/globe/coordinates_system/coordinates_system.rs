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
  
  /// Receives the origin and destination coordinates and 
  /// returns a list of coordinates represented by the 
  /// trajectory where a moving point would pass by.
  pub fn get_geodesic_trajectory(
    origin: Coordinates,
    destination: Coordinates,
    coordinate_map: &CoordinateMap,
    radius: f32
  ) -> Vec<Vector3> {
    let origin = coordinate_map.get(&origin).unwrap().cartesian;
    let destination = coordinate_map.get(&destination).unwrap().cartesian;
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

  pub fn get_dynamic_geodesic_trajectory(
    origin: Vector3,
    direction_vector: Vector3,
    pseudo_destination: Vector3,
    radius: f32,
  ) -> Vector3 {
    // Get the origin from the coordinate map
    let origin = origin.normalized();
    let pseudo_destination = pseudo_destination.normalized();

    godot_print!("origin: {:?}", origin);
    godot_print_rich!("pseudo_destination: {:?}", pseudo_destination);
    godot_print_rich!("direction_vector: {:?}", direction_vector);

    // let t = 1. as f64 / (steps - 1) as f64;

    // Calculate the point along the geodesic using slerp
    let next_position = origin.slerp(direction_vector, 0.01);
    // let next_position = origin.slerp(pseudo_destination, 0.02);
    let next_position = Self::radius_scale(next_position, radius);
    next_position
  }

  pub fn get_antipode(currrent_position: Vector3) -> Vector3 {
    let antipode   = Vector3 {
      x: currrent_position.x * -1.,
      y: currrent_position.y * -1.,
      z: currrent_position.z * -1.,
    };

    antipode
  }


  pub fn get_geodesic_trajectory_b(
    origin: Vector3,
    destination: Vector3,
    direction_vector: Vector3,
    radius: f32
  ) -> Vector3 {
   
    // Calculate the point along the geodesic using slerp
    let next_position = Self::a_slerp(origin, destination, 0.05, direction_vector);
    // let next_position = origin.slerp(pseudo_destination, 0.02);
    let next_position = Self::radius_scale(next_position, radius);
    next_position
  }
  

    // Function to perform Slerp between the origin and destination
    pub fn a_slerp(
      origin: Vector3, 
      destination: Vector3, 
      t: f32, 
      direction: Vector3
  ) -> Vector3 {
      // Normalize both origin and destination to unit vectors
      let origin = origin.normalized();
      let destination = destination.normalized();

      // Calculate the dot product between the two vectors
      let dot = origin.dot(destination);

      // // Handle case when the vectors are very close or exactly opposite (antipodal)
      // if dot > 0.9995 {
      //     // If the vectors are too close, do a linear interpolation (avoiding numerical errors)
      //     return origin.linear_interpolate(destination, t);
      // }

      // Clamp dot product between -1 and 1 to avoid out of range errors due to floating point precision
      let dot = dot.clamp(-1.0, 1.0);

      // Compute the angle between the two vectors
      let theta = dot.acos();
      let sin_theta = theta.sin();

      // Compute the scale factors for origin and destination vectors based on t
      let scale_origin = (1.0 - t) * theta.sin() / sin_theta;
      let scale_destination = t * theta.sin() / sin_theta;

      // Slerp between the origin and destination (weighted average of both vectors)
      let interpolated = origin * scale_origin + destination * scale_destination;

      // Ensure the resulting vector stays normalized (it should already be normalized)
      let result = interpolated.normalized();

      // If direction is provided, adjust the result to ensure proper path alignment
      // Normalize the direction vector and adjust the result to stay on the geodesic path
      let direction = direction.normalized();
      return result * direction.length(); // Scale the result according to the radius/direction
  }
}