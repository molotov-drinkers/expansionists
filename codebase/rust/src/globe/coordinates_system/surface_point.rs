
use godot::{classes::{Area3D, IArea3D, MeshInstance3D, PhysicsRayQueryParameters3D, World3D}, prelude::*};
use crate::{globe::territories::territory::TerritoryId, troops::troop::Troop};

type Latitude = i16;
type Longitude = i16;
pub type Coordinates = (Latitude, Longitude);

#[derive(Debug, Clone)]
pub struct SurfacePointMetadata {
  pub cartesian: Vector3,
  pub lat_long: Coordinates,
  pub territory_id: Option<TerritoryId>,
}

/// Represents a point on the surface of the virtual planet
/// It's used, for instance, to trace rays from the origin of the world to the position of the troops
#[derive(Debug, GodotClass)]
#[class(base=Area3D)]
pub struct SurfacePoint {
  base: Base<Area3D>,
  pub surface_point_metadata: SurfacePointMetadata,
}

#[godot_api]
impl IArea3D for SurfacePoint {
  fn init(base: Base<Area3D>) -> SurfacePoint {
    SurfacePoint {
      base: base,
      surface_point_metadata: Self::get_blank_surface_point_metadata(),
    }
  }
}

impl SurfacePoint {
  pub fn get_blank_coordinates() -> Coordinates {
    (0, 0)
  }

  pub fn get_blank_surface_point_metadata() -> SurfacePointMetadata {
    SurfacePointMetadata {
      cartesian: Vector3::new(0.0, 0.0, 0.0),
      lat_long: Self::get_blank_coordinates(),
      territory_id: None,
    }
  }

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
  pub fn get_troop_surface_point(troop: &Troop) -> Option<Gd<SurfacePoint>> {
    // default_mesh is the child of the troop node that lives on top of the troop itself
    // it's used to be sure that the ray passed by a surface point
    let troop_position = troop.base()
      .find_child("default_mesh")
      .expect("Expected to find default mesh")
      .cast::<MeshInstance3D>()
      .get_global_position();

    let world = troop
      .base()
      .get_world_3d()
      .expect("World to exist");

    Self::get_surface_point(
      troop_position,
      world,
      None,
    )
  }

  /// Traces a ray from the origin of the world to the target position
  /// 
  /// # Arguments
  /// * `scale_factor` - To decrease the odds of the ray not collinding with any surface_point
  /// we're pushing the target_position a bit further from the actual surface
  /// That problem was happening when the target_position was too close to the actual surface.
  /// 
  /// # Returns
  /// * `Option<Gd<SurfacePoint>>` - The SurfacePoint where the ray collides with the virtual planet,
  /// to avoid panicking it returns None if it doesn't find any
  pub fn get_surface_point(target_position: Vector3, mut world: Gd<World3D>, scale_factor: Option<f32>) -> Option<Gd<SurfacePoint>> {
    let world_origin = Vector3::new(0.0, 0.0, 0.0);
    let scale_factor = scale_factor.unwrap_or(1.);

    let mut space_state = world
      .get_direct_space_state()
      .expect("Expected to get direct space state");

    let direction = target_position.normalized();
    let target_position = direction * (target_position.length() * scale_factor);

    let mut query = PhysicsRayQueryParameters3D::create(
      world_origin,
      target_position,
    ).expect("Expected to create ray query");

    query.set_collide_with_areas(true);
    query.set_collide_with_bodies(false);

    let collision_dict = space_state.intersect_ray(&query);
    let collider = collision_dict
      .get("collider");

    if collider.is_none() {
      godot_error!("{}", format!("Expected 'collider' key to exist in the ray from origin to {:?} collision dictionary: {:?}",
        target_position,
        collision_dict
      ));
      return None;
    }

    let collider = collider.unwrap();
    // The collided area has to be a SurfacePoint
    let surface_point: Result<Gd<SurfacePoint>, ConvertError> = collider
      .try_to::<Gd<SurfacePoint>>();

    match surface_point {
      Ok(surface_point) => Some(surface_point),
      Err(err) => {
        godot_error!("surface_point ConvertError: {:?}", err);
        return None
      },
    }
    // surface_point
  }
}
