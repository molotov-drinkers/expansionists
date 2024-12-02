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
  ) -> Vec<Coordinates> {

    [].to_vec()
  }

  /// Receives the current position of the camera,
  /// which is the central globe coordinate the camera is pointing to
  /// and the the direction the camera is going to move to
  /// returns the new position of the camera
  pub fn get_geodesic_neighbour_position(
    looking_at: Coordinates,
    current_position: Vector3,
    direction: CameraDirection,
    // coordinate_map: &CoordinateMap,
  ) -> Vector3 {
    let mut neighbour_position = Vector3::new(0.0, 0.0, 0.0);

    // TODO it's not just translate the trasform position,
    // the looking_at point also matters, so probably needs to use it or use "rotation"

    neighbour_position
  }
}