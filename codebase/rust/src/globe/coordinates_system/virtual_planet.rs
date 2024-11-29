
use std::{collections::HashMap, f64::consts::PI};
use godot::{classes::{BoxMesh, BoxShape3D, CollisionShape3D, MeshInstance3D, StandardMaterial3D}, obj::NewAlloc, prelude::*};

use crate::globe::territory::types::{Territories, Territory, TerritoryId};
use super::{
  coordinates_system::{CoordinateMap, CoordinateMetadata},
  surface_point::{SurfacePoint, SurfacePointMetadata}
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VirtualPlanet {
  base: Base<Node3D>,
  is_ready_for_physics: bool,
  territories: Territories,
  surface_point_metadata: Vec<SurfacePointMetadata>,
  coordinate_map: CoordinateMap,
}

#[godot_api]
impl INode3D for VirtualPlanet {
  fn init(base: Base<Node3D>) -> VirtualPlanet {

    VirtualPlanet {
      base: base,
      is_ready_for_physics: false,
      territories: Territory::get_map(),
      surface_point_metadata: vec![],
      coordinate_map: HashMap::new(),
    }
  }

  fn ready (&mut self) {
    Self::populate_surface_points_and_coordinate_map(self);
    Self::create_virtual_sphere(self);
    self.is_ready_for_physics = true;
  }

  fn physics_process(&mut self, _delta: f64) {
    if self.is_ready_for_physics == true {

      for node in self.base().get_children().iter_shared() {
        let surface_point = node.cast::<SurfacePoint>();

        // surface_point.set_visible(false);

        let overlaping_bodies = surface_point.get_overlapping_bodies();
        // HACK ATTEMPT: to avoid multiple calls to physics_process =(
        if overlaping_bodies.len() > 0 {
          self.is_ready_for_physics = false;

          for colliding_body in overlaping_bodies.iter_shared() {
            let parent_name = colliding_body.get_parent().unwrap().get_name();

            let possible_territory_colission = self.territories.get(&parent_name.to_string());
            if possible_territory_colission.is_some() {
              let territory_data = possible_territory_colission.unwrap();

              let mut clone = surface_point.clone();
              let mut surface_point_ref = clone.bind_mut();
              let planet_surface_point = surface_point_ref.get_surface_point_metadata_mut();

              // TODO: create a matrix of coordinates // territories and owners
              // TODO: Do we really need territpri_id at both virtual_coordinates and planet_surface_point?
              self.coordinate_map.insert(planet_surface_point.lat_long, CoordinateMetadata {
                territory_id: Some(territory_data.base_name.clone()),
                cartesian: planet_surface_point.cartesian,
              });

              planet_surface_point.territory_id = Some(territory_data.base_name.clone());


              let color = Territory::get_territory_color(
                &territory_data.location.sub_continent,
                &territory_data.location.continent
              );


              for child in surface_point.get_children().iter_shared() {
                let child = child.try_cast::<MeshInstance3D>();
                if child.is_err() {
                  continue;
                }
                
                let mut material = StandardMaterial3D::new_gd();
                material.set_albedo(color);
                child.unwrap().set_material_override(&material);
              }
            }
          }
        }
      }

    }
  }
}

impl VirtualPlanet {
  #[inline] pub fn get_planet_radius() -> f64 { 1.08 }
  #[inline] pub fn get_num_of_latitudes() -> i16 { 90 + 45 }
  #[inline] pub fn get_num_of_longitudes() -> i16 { 180 + 90 }

  pub fn populate_surface_points_and_coordinate_map(&mut self) {
    let planet_radius = Self::get_planet_radius();
    let num_latitudes = Self::get_num_of_latitudes();
    let num_longitudes = Self::get_num_of_longitudes();

    for lat in 0..num_latitudes {
      let theta = (lat as f64) * PI / (num_latitudes as f64); // Latitude angle (0 to pi)

      for long in 0..num_longitudes {
        let phi = (long as f64) * 2.0 * PI / (num_longitudes as f64);
        let x = (planet_radius * theta.sin() * phi.cos()) as f32;
        let y = (planet_radius * theta.sin() * phi.sin()) as f32;
        let z = (planet_radius * theta.cos()) as f32;

        let cartesian = Vector3::new(x, y, z);
        let lat_long = (lat, long);
        let blank_territory_id: Option<TerritoryId> = None;

        self.coordinate_map.insert(lat_long, CoordinateMetadata {
          territory_id: blank_territory_id.clone(),
          cartesian,
        });

        self.surface_point_metadata.push(SurfacePointMetadata {
          cartesian,
          lat_long,
          territory_id: blank_territory_id,
        });
      }
    }
  }

  pub fn create_virtual_sphere(&mut self) {
    for planet_surface_point in self.surface_point_metadata.clone() {
      let surface_point = VirtualPlanet::create_surface_point_area(
        planet_surface_point
      );
      self.base_mut().add_child(&surface_point);
    }
  }

  pub fn create_surface_point_area(planet_surface_point: SurfacePointMetadata) -> Gd<SurfacePoint> {
    let surface_mesh_and_collider_size = Vector3::new(0.05, 0.05, 0.05);
    let mesh_instance = Self::create_surface_mesh(
      surface_mesh_and_collider_size,
      planet_surface_point.cartesian
    );

    let collision_shape = Self::create_surface_collider(
      surface_mesh_and_collider_size,
      planet_surface_point.cartesian
    );

    let mut surface_point = SurfacePoint::new_alloc();
    surface_point.add_child(&collision_shape);
    surface_point.add_child(&mesh_instance);
    surface_point.bind_mut().set_surface_point_metadata(planet_surface_point);
    surface_point
  }

  pub fn create_material() -> Gd<StandardMaterial3D> {
    let ocean_color = Color::from_rgba(0.093, 0.139, 0.614, 1.);
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(ocean_color);
    material
  }

  pub fn create_surface_mesh(mesh_size: Vector3, cartesian: Vector3) -> Gd<MeshInstance3D> {
    let material = Self::create_material();
    let mut mesh = BoxMesh::new_gd();
    mesh.set_size(mesh_size.clone());
    mesh.set_material(&material);

    let mut mesh_instance = MeshInstance3D::new_alloc();
    mesh_instance.set_mesh(&mesh);
    mesh_instance.set_position(cartesian);

    mesh_instance
  }

  pub fn create_surface_collider(collider_size: Vector3, cartesian: Vector3) -> Gd<CollisionShape3D> {
    let mut collision_shape = CollisionShape3D::new_alloc();
    let mut shape = BoxShape3D::new_gd();
    shape.set_size(collider_size);
    collision_shape.set_shape(&shape);
    collision_shape.set_position(cartesian);

    collision_shape
  }
}

