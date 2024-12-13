
use godot::{classes::{Area3D, IArea3D, PhysicsRayQueryParameters3D}, prelude::*};
use crate::{globe::territory::types::TerritoryId, player::troop::Troop};

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

  /// Traces a ray from the origin of the world to the position of the troop
  /// Returns the SurfacePoint where the ray collides with the virtual planet
  pub fn get_troop_surface_point(troop: &Troop) -> Gd<SurfacePoint> {
    let world_origin = Vector3::new(0.0, 0.0, 0.0);
    let troop_position = troop.base().get_global_position();

    let mut world = troop
      .base()
      .get_world_3d()
      .expect("World to exist");

    let mut space_state = world
      .get_direct_space_state()
      .expect("Expected to get direct space state");

    let mut query = PhysicsRayQueryParameters3D::create(
      world_origin,
      troop_position,
    ).expect("Expected to create ray query");

    query.set_collide_with_areas(true);
    query.set_collide_with_bodies(false);

    let collision_dict = space_state.intersect_ray(&query);
    let collider = collision_dict
      .get("collider")
      .expect(&format!("'collider' key to exist in collision dictionary: {:?}", collision_dict));

    // The collided area has to be a SurfacePoint
    let surface_point = collider
      .try_to::<Gd<SurfacePoint>>()
      .expect("Expected to get surface point as collided area");

    surface_point
  }
}

fn get_blank_surface_point_metadata() -> SurfacePointMetadata {
  SurfacePointMetadata {
    cartesian: Vector3::new(0.0, 0.0, 0.0),
    lat_long: (0, 0),
    territory_id: None,
  }
}