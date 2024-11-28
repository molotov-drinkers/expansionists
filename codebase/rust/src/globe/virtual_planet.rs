
use std::f64::consts::PI;
use godot::{classes::{Area3D, BoxMesh, BoxShape3D, CollisionShape3D, MeshInstance3D, StandardMaterial3D}, obj::NewAlloc, prelude::*};

use super::territory::types::{Territories, Territory, TerritoryId};

type Latitude = f64;
type Longitude = f64;
#[derive(Debug, Clone)]
pub struct PlanetSurfacePoint {
  cartesian: Vector3,
  lat_long: (Latitude, Longitude),
  territory_id: Option<TerritoryId>
}


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VirtualPlanet {
  base: Base<Node3D>,
  planet_surface_points: Vec<PlanetSurfacePoint>,
  is_ready_for_physics: bool,

  // TEMP?
  pub territories: Territories,
}

#[godot_api]
impl INode3D for VirtualPlanet {
  fn init(base: Base<Node3D>) -> VirtualPlanet {

    VirtualPlanet {
      base: base,
      planet_surface_points: vec![],
      is_ready_for_physics: false,
      territories: Territory::get_map()
    }
  }

  fn ready (&mut self) {
    godot_print!("VirtualPlanet ready - init");
    Self::create_planet_surface_points(self);

    for planet_surface_point in self.planet_surface_points.clone() {
      let point_area = VirtualPlanet::create_mesh_point(
        planet_surface_point
      );

      // godot_print!("{:?}",point_area.get_tree_string_pretty());

      self.base_mut().add_child(&point_area);
    }

    self.is_ready_for_physics = true;

  }

  fn physics_process(&mut self, _delta: f64) {
    if self.is_ready_for_physics == true {
      godot_print!("READY FOR PHYSICS NOW");

      for node in self.base().get_children().iter_shared() {
        let point_area = node.cast::<Area3D>();
  
        let overlaping_bodies = point_area.get_overlapping_bodies();
        
        // HACK ATTEMPT: to avoid multiple calls to physics_process =(
        if overlaping_bodies.len() > 0 {
          self.is_ready_for_physics = false;

          for colliding_body in overlaping_bodies.iter_shared() {
            let colliding_body = colliding_body.cast::<Node3D>();
            let parent = colliding_body.get_parent().unwrap();
            let parent_name = parent.get_name();

            if parent_name != "ocean".into() {
              godot_print!("parent: {:?}", parent_name);
              // let territory_data = self.territories.get(&parent_name.to_string());
            }
          }
        }
      }

    }
  }
}

impl VirtualPlanet {
  #[inline] pub fn get_planet_radius() -> f64 { 1. }
  #[inline] pub fn get_num_of_latitudes() -> i8 { 90 }
  #[inline] pub fn get_num_of_longitudes() -> i16 { 180 }

  pub fn create_planet_surface_points(&mut self) {
    let planet_radius = Self::get_planet_radius();
    let num_latitudes = Self::get_num_of_latitudes();
    let num_longitudes = Self::get_num_of_longitudes();

    for i in 0..num_latitudes {
      let theta = (i as f64) * PI / (num_latitudes as f64); // Latitude angle (0 to pi)

      for j in 0..num_longitudes {
        let phi = (j as f64) * 2.0 * PI / (num_longitudes as f64);
        let x = (planet_radius * theta.sin() * phi.cos()) as f32;
        let y = (planet_radius * theta.sin() * phi.sin()) as f32;
        let z = (planet_radius * theta.cos()) as f32;

        let cartesian = Vector3::new(x, y, z);
        let lat_long = (i as f64, j as f64);
        let territory_id: Option<TerritoryId> = None;

        self.planet_surface_points.push(PlanetSurfacePoint {
          cartesian,
          lat_long,
          territory_id
        });
      }
    }
  }

  pub fn create_mesh_point(planet_surface_point: PlanetSurfacePoint) -> Gd<Area3D> {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(Color::BLUE_VIOLET);

    let mesh_and_collider_size = Vector3::new(0.1, 0.1, 0.1);

    let mut mesh = BoxMesh::new_gd();
    mesh.set_size(mesh_and_collider_size.clone());
    mesh.set_material(&material);

    let coordinates_name = format!("coordinate({:},{:})",
      planet_surface_point.lat_long.0.to_string(),
      planet_surface_point.lat_long.1.to_string(),
    );

    let mut virtual_point = MeshInstance3D::new_alloc();
    virtual_point.set_name(&coordinates_name);
    virtual_point.set_mesh(&mesh);
    virtual_point.set_position(planet_surface_point.cartesian);
    // virtual_point.create_trimesh_collision();

    // TODO: set colliders ----------------
    // virtual_point.find_child()

    let mut point_area = Area3D::new_alloc();
    let mut collision_shape = CollisionShape3D::new_alloc();
    let mut shape = BoxShape3D::new_gd();
    shape.set_size(mesh_and_collider_size);
    collision_shape.set_shape(&shape);
    collision_shape.set_position(planet_surface_point.cartesian);

    point_area.add_child(&collision_shape);
    point_area.add_child(&virtual_point);

    // TODO: set colliders ----------------

    // let tree = point_area.get_tree_string_pretty();
    // godot_print!("{:?}", tree);

    
    point_area
    // virtual_point
  }
}