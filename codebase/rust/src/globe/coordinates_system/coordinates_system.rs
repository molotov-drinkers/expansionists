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
  
  //TODO: Implement Geodesic coordinate system
  /// Receives the origin and destination coordinates and 
  /// returns a list of coordinates represented by the 
  /// trajectory where a moving point would pass by.
  fn get_geodesic_trajectory(
    origin: Coordinates,
    destination: Coordinates,
    coordinate_map: &CoordinateMap,
  ) -> Vec<Coordinates> {
    
    [].to_vec()
  }
}