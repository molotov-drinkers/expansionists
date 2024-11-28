
use std::{collections::HashMap, f64::consts::PI};
use godot::{classes::{Area3D, BoxMesh, BoxShape3D, CollisionShape3D, IArea3D, MeshInstance3D, StandardMaterial3D}, obj::NewAlloc, prelude::*};

use super::territory::types::{Territories, Territory, TerritoryId};

type Latitude = i16;
type Longitude = i16;
type Coordinates = (Latitude, Longitude);
#[derive(Debug, Clone)]
pub struct PlanetSurfacePoint {
  cartesian: Vector3,
  lat_long: Coordinates,
  territory_id: Option<TerritoryId>,
}


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VirtualPlanet {
  base: Base<Node3D>,
  planet_surface_points: Vec<PlanetSurfacePoint>,
  is_ready_for_physics: bool,


  virtual_coodinates_map: HashMap<Coordinates, Option<TerritoryId>>,
  // TEMP?
  territories: Territories,
}

#[godot_api]
impl INode3D for VirtualPlanet {
  fn init(base: Base<Node3D>) -> VirtualPlanet {

    VirtualPlanet {
      base: base,
      planet_surface_points: vec![],
      is_ready_for_physics: false,
      territories: Territory::get_map(),
      virtual_coodinates_map: HashMap::new(),
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

      for node in self.base().get_children().iter_shared() {
        let mut surface_point = node.cast::<SurfacePoint>();

        surface_point.set_visible(false);


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
              let planet_surface_point = surface_point_ref.get_planet_surface_point_mut();

              // TODO: create a matrix of coordinates // territories and owners
              // TODO: Do we really need territpri_id at both virtual_coordinates and planet_surface_point?
              self.virtual_coodinates_map.insert(planet_surface_point.lat_long, Some(territory_data.base_name.clone()));
              planet_surface_point.territory_id = Some(territory_data.base_name.clone());


              // let color = Territory::get_territory_color(
              //   &territory_data.location.sub_continent,
              //   &territory_data.location.continent
              // );


              // for child in surface_point.get_children().iter_shared() {
              //   let child = child.try_cast::<MeshInstance3D>();
              //   if child.is_err() {
              //     continue;
              //   }
                
              //   let mut material = StandardMaterial3D::new_gd();
              //   material.set_albedo(color);
              //   child.unwrap().set_material_override(&material);
              // }
            } else {
              
              // surface_point.set_visible(false);

              // for child in point_area.get_children().iter_shared() {
              //   let child = child.try_cast::<MeshInstance3D>();
              //   if child.is_err() {
              //     continue;
              //   }
                
              //   let mut material = StandardMaterial3D::new_gd();
              //   material.set_albedo(Color::BLUE);
              //   child.unwrap().set_material_override(&material);
              // }
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

  pub fn create_planet_surface_points(&mut self) {
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
        let initial_territory_id: Option<TerritoryId> = None;

        self.virtual_coodinates_map.insert(lat_long, initial_territory_id.clone());

        self.planet_surface_points.push(PlanetSurfacePoint {
          cartesian,
          lat_long,
          territory_id: initial_territory_id,
        });
      }
    }
  }

  pub fn create_mesh_point(planet_surface_point: PlanetSurfacePoint) -> Gd<SurfacePoint> {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(Color::BLUE_VIOLET);

    let mesh_and_collider_size = Vector3::new(0.05, 0.05, 0.05);

    let mut mesh = BoxMesh::new_gd();
    mesh.set_size(mesh_and_collider_size.clone());
    mesh.set_material(&material);

    let mut virtual_point = MeshInstance3D::new_alloc();
    virtual_point.set_name("point_mesh");
    virtual_point.set_mesh(&mesh);
    virtual_point.set_position(planet_surface_point.cartesian);

    // set colliders ----------------
    let mut surface_point = SurfacePoint::new_alloc();
    let mut collision_shape = CollisionShape3D::new_alloc();
    let mut shape = BoxShape3D::new_gd();
    shape.set_size(mesh_and_collider_size);
    collision_shape.set_shape(&shape);
    collision_shape.set_position(planet_surface_point.cartesian);

    surface_point.add_child(&collision_shape);
    surface_point.add_child(&virtual_point);
    surface_point.bind_mut().set_planet_surface_point(planet_surface_point);

    // set colliders ----------------

    surface_point
  }
}



#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct SurfacePoint {
  base: Base<Area3D>,
  planet_surface_point: PlanetSurfacePoint,
}

#[godot_api]
impl IArea3D for SurfacePoint {
  fn init(base: Base<Area3D>) -> SurfacePoint {
    SurfacePoint {
      base: base,
      planet_surface_point: PlanetSurfacePoint {
        cartesian: Vector3::new(0.0, 0.0, 0.0),
        lat_long: (0, 0),
        territory_id: None,
      }
    }
  }
}

impl SurfacePoint {
  pub fn set_planet_surface_point(&mut self, planet_surface_point: PlanetSurfacePoint) {
    self.planet_surface_point = planet_surface_point;
  }

  pub fn get_planet_surface_point(&self) -> &PlanetSurfacePoint {
    &self.planet_surface_point
  }
  pub fn get_planet_surface_point_mut(&mut self) -> &mut PlanetSurfacePoint {
    &mut self.planet_surface_point
  }
}