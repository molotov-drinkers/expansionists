
use std::{collections::HashMap, f64::consts::PI};
use godot::{classes::{BoxMesh, BoxShape3D, CollisionShape3D, MeshInstance3D, StandardMaterial3D}, obj::NewAlloc, prelude::*};
use fastrand;

use crate::{
  globe::territories::{land::Land, territory::{Territories, Territory, TerritoryId}},
  troops::surface::Surface
};
use super::{
  coordinates_system::{CoordinateMap, CoordinateMetadata},
  surface_point::{Coordinates, SurfacePoint, SurfacePointMetadata}
};

/// VirtualPlanert is used to create a virtual sphere that will be used for physics and collision calculations
/// It's not visible in the game
/// Also used to calculate navigation paths along the surface of the planet
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VirtualPlanet {
  base: Base<Node3D>,
  pub is_ready_for_physics: bool,
  /// turns true when all surface points are matched with territories
  pub are_surface_points_matched: bool,
  pub territories: Territories,
  pub surface_points_metadata: Vec<SurfacePointMetadata>,
  pub coordinate_map: CoordinateMap,
}

#[godot_api]
impl INode3D for VirtualPlanet {
  fn init(base: Base<Node3D>) -> VirtualPlanet {

    VirtualPlanet {
      base: base,
      is_ready_for_physics: false,
      are_surface_points_matched: false,
      territories: Territory::get_map(),
      surface_points_metadata: vec![],
      coordinate_map: HashMap::new(),
    }
  }

  fn ready (&mut self) {
    // By default, the VirtualPlanet is not visible. It's only used for physics and collision calculations
    self.base_mut().set_visible(false);

    self.populate_surface_points_and_coordinate_map();
    self.create_virtual_sphere();
    self.is_ready_for_physics = true;
  }

  fn physics_process(&mut self, _delta: f64) {
    if self.is_ready_for_physics == true {
      self.match_surface_points_and_territories();
    }
  }
}

impl VirtualPlanet {
  /// Following inline functions have pseudo-arbitrary numbers defined after checking the globe mesh size
  /// that's the reason they all seem to be magic numbers
  #[inline] pub fn get_planet_radius() -> f64 { 1.078 * 3.0 }
  #[inline] pub fn get_num_of_latitudes() -> i16 { (90. * 2.5) as i16 }
  #[inline] pub fn get_num_of_longitudes() -> i16 { (180. * 2.5) as i16 }
  #[inline] pub fn get_surface_mesh_and_collider_size() -> Vector3 { Vector3::new(0.06, 0.06, 0.06) }

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

        let lat_long = (lat, long);
        let cartesian = Vector3::new(x, y, z);
        let blank_territory_id: Option<TerritoryId> = None;

        self.coordinate_map.insert(lat_long, CoordinateMetadata {
          territory_id: blank_territory_id.clone(),
          cartesian,
        });

        self.surface_points_metadata.push(SurfacePointMetadata {
          cartesian,
          lat_long,
          territory_id: blank_territory_id,
        });
      }
    }
  }

  pub fn create_virtual_sphere(&mut self) {
    for surface_point_metadata in self.surface_points_metadata.clone() {
      let surface_point = VirtualPlanet::create_surface_point_area(
        surface_point_metadata
      );
      self.base_mut().add_child(&surface_point);
    }
  }

  pub fn create_surface_point_area(surface_point_metadata: SurfacePointMetadata) -> Gd<SurfacePoint> {
    let surface_mesh_and_collider_size = Self::get_surface_mesh_and_collider_size();
    let mesh_instance = Self::create_surface_mesh(
      surface_mesh_and_collider_size,
      surface_point_metadata.cartesian
    );

    let collision_shape = Self::create_surface_collider(
      surface_mesh_and_collider_size,
      surface_point_metadata.cartesian
    );

    let mut surface_point = SurfacePoint::new_alloc();
    surface_point.add_child(&collision_shape);
    surface_point.add_child(&mesh_instance);
    surface_point.bind_mut().set_surface_point_metadata(surface_point_metadata);
    surface_point
  }

  pub fn create_surface_material() -> Gd<StandardMaterial3D> {
    let ocean_color = Color::from_rgba(0.093, 0.139, 0.614, 1.);
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(ocean_color);
    material
  }

  pub fn create_surface_mesh(mesh_size: Vector3, cartesian: Vector3) -> Gd<MeshInstance3D> {
    let material = Self::create_surface_material();
    let mut mesh = BoxMesh::new_gd();
    mesh.set_size(mesh_size);
    mesh.set_material(&material);

    let mut mesh_instance = MeshInstance3D::new_alloc();
    mesh_instance.set_mesh(&mesh);
    mesh_instance.set_position(cartesian);
    mesh_instance.look_at_from_position(cartesian, Vector3::ZERO);
    mesh_instance
  }

  pub fn create_surface_collider(collider_size: Vector3, cartesian: Vector3) -> Gd<CollisionShape3D> {
    let mut collision_shape = CollisionShape3D::new_alloc();
    let mut shape = BoxShape3D::new_gd();
    shape.set_size(collider_size);
    collision_shape.set_shape(&shape);
    collision_shape.set_position(cartesian);
    collision_shape.look_at_from_position(cartesian, Vector3::ZERO);
    collision_shape
  }

  /// Matches surface points with territories and
  /// sets the territory_id into SurfacePointMetadata, CoordinateMetadata, and Territory.coordinates
  pub fn match_surface_points_and_territories(&mut self) {
    for surface_point_node in self.base().get_children().iter_shared() {
      let mut surface_point = surface_point_node.cast::<SurfacePoint>();
      let bodies_overlapping_with_surface_point = &surface_point.get_overlapping_bodies();
      // HACK: to avoid multiple calls to physics_process =(
      if bodies_overlapping_with_surface_point.len() > 0 {
        self.is_ready_for_physics = false;

        for body_overlapping_with_surface_point in bodies_overlapping_with_surface_point.iter_shared() {
          if let Ok(collided_land) = body_overlapping_with_surface_point.try_cast::<Land>() {

            let territory_id = collided_land
              .get_parent()
              .expect("Expected 'Land' to have a parent")
              .get_name()
              .to_string();

            let possible_territory_colission = self.territories.get_mut(&territory_id);
            if possible_territory_colission.is_some() {
              let overlapped_territory = possible_territory_colission.unwrap();
              // Self::paint_surface_point(&surface_point, overlapped_territory);

              surface_point.add_to_group(&territory_id);
              surface_point.add_to_group(&Surface::Land.to_string());
              let mut surface_point_bind = surface_point.bind_mut();
              let surface_point_metadata = surface_point_bind.get_surface_point_metadata_mut();

              overlapped_territory.coordinates.push(surface_point_metadata.lat_long);

              self.coordinate_map.insert(
                surface_point_metadata.lat_long,
                CoordinateMetadata {
                  territory_id: Some(territory_id.clone()),
                  cartesian: surface_point_metadata.cartesian,
                }
              );

              surface_point_metadata.territory_id = Some(territory_id);
            }
          }
        }

        self.territories.iter_mut().for_each(|(_, territory)| {
          territory.set_territory_size();
          territory.set_troops_growth_velocity();
          territory.set_organic_max_troops();
        });

      }
    }
    self.are_surface_points_matched = true;
  }

  #[allow(dead_code)]
  /// Paints the surface point with the continent/territory color
  /// useful for debugging
  pub fn paint_surface_point(surface_point: &Gd<SurfacePoint>, territory: &Territory) {
    let color = Territory::get_territory_color(
      &territory.location.sub_continent,
      &territory.location.continent
    );

    for child in surface_point.get_children().iter_shared() {
      let child = child.try_cast::<MeshInstance3D>();

      // If it's not a MeshInstance3D, skip it
      if child.is_err() { continue; }
      
      let mut material = StandardMaterial3D::new_gd();
      material.set_albedo(color);
      child.unwrap().set_material_override(&material);
    }
  }

  /// Receives a territory coordinate and returns a random coordinate from the same territory
  /// It's used for keeping a troop walking inside of a territory
  fn _get_another_territory_coordinate(&self, given_coordinates: &Coordinates) -> Coordinates {
    let coordinate_metadata = self
      .coordinate_map
      .get(&given_coordinates)
      .expect("Expected coordinates to exist");

    let territory_id = coordinate_metadata
      .territory_id
      .clone()
      .expect(&format!("expect territory_id to exist, {:?}", coordinate_metadata));

    self.get_an_random_territory_coordinate(territory_id.as_str())
  }

  /// Receives a territory_id and returns a random coordinate from the territory
  pub fn get_an_random_territory_coordinate(&self, territory_id: &str) -> Coordinates {
    let territory = self.territories.get(territory_id).expect("Expected territory to exist");
    let territory_coordinates = &territory.coordinates;
    if territory_coordinates.len() == 0 {
      panic!("Expected territory_coordinates to have at least one element");
    }
    let random_index = fastrand::usize(0..(territory_coordinates.len()-1));
    territory_coordinates[random_index]
  }

  /// Receives a territory_id and returns a random coordinate from the territory
  pub fn get_spawner_territory_coordinate(&self, territory_id: &str) -> Coordinates {
      let territory = self.territories.get(territory_id).expect("Expected territory to exist");
      let territory_coordinates = &territory.coordinates;
      if territory_coordinates.len() == 0 {
        panic!("Expected territory_coordinates to have at least one element");
      }
  
      // TICKET: #50 this divided by 4 is a hack to get a coordinate in the territory not close to the border
      // Sometime it does not work, but it's good enough for now
      let territory_point = territory_coordinates.len() / 1/4;
      territory_coordinates[territory_point]
    }

  /// Receives a latitude and longitude and returns the cartesian coordinates
  pub fn get_cartesian_from_coordinates(&self, given_coordinates: &Coordinates) -> Vector3 {
    let coordinate_metadata = self.coordinate_map.get(&given_coordinates).expect("Expected coordinates to exist");
    coordinate_metadata.cartesian
  }

}
