
use godot::{classes::{Area3D, IArea3D}, prelude::*};
use crate::globe::territory::types::TerritoryId;

type Latitude = i16;
type Longitude = i16;
pub type Coordinates = (Latitude, Longitude);
#[derive(Debug, Clone)]
pub struct SurfacePointMetadata {
  pub cartesian: Vector3,
  pub lat_long: Coordinates,
  pub territory_id: Option<TerritoryId>,
}

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct SurfacePoint {
  base: Base<Area3D>,
  surface_point_metadata: SurfacePointMetadata,
}

#[godot_api]
impl IArea3D for SurfacePoint {
  fn init(base: Base<Area3D>) -> SurfacePoint {
    SurfacePoint {
      base: base,
      surface_point_metadata: get_blank_surface_point_metadata(),
    }
  }
}

impl SurfacePoint {
  pub fn set_surface_point_metadata(&mut self, surface_point_metadata: SurfacePointMetadata) {
    self.surface_point_metadata = surface_point_metadata;
  }

  pub fn get_surface_point_metadata(&self) -> &SurfacePointMetadata {
    &self.surface_point_metadata
  }
  pub fn get_surface_point_metadata_mut(&mut self) -> &mut SurfacePointMetadata {
    &mut self.surface_point_metadata
  }
}

fn get_blank_surface_point_metadata() -> SurfacePointMetadata {
  SurfacePointMetadata {
    cartesian: Vector3::new(0.0, 0.0, 0.0),
    lat_long: (0, 0),
    territory_id: None,
  }
}