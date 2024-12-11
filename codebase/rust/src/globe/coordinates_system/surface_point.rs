
use godot::{classes::{Area3D, IArea3D}, meta::GodotType, prelude::*};
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

#[derive(Debug, GodotClass)]
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

impl GodotConvert for SurfacePoint {
  type Via = Variant;
}

impl FromGodot for SurfacePoint {
  fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
    godot_print_rich!("SurfacePoint::try_from_godot: {:?}", via);
    
    // let mut surface_point = SurfacePoint::init(Area3D::new());
    // let surface_point_metadata = SurfacePointMetadata {
    //   cartesian: via.get("cartesian").try_to()?,
    //   lat_long: via.get("lat_long").try_to()?,
    //   territory_id: via.get("territory_id").try_to()?,
    // };
    todo!()
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