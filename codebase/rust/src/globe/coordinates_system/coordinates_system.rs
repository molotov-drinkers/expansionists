use std::collections::HashMap;
use godot::prelude::*;

use crate::globe::territory::types::TerritoryId;
use super::surface_point::Coordinates;

pub struct CoordinateMetadata {
  pub territory_id: Option<TerritoryId>,
  pub cartesian: Vector3,
}
pub type CoordinateMap = HashMap<Coordinates, CoordinateMetadata>;

pub struct CoordinatesSystem {}

impl CoordinatesSystem {
  //TODO: Implement Geodesic coordinate system
}