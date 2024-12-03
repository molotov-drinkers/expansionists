use godot::classes::{INode3D, MeshInstance3D, Node3D, StandardMaterial3D};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::player::troop::Troop;
use rand::Rng;


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RootScene {
  base: Base<Node3D>,
}

#[godot_api]
impl INode3D for RootScene {
  fn init(base: Base<Node3D>) -> RootScene {

    RootScene {
      base: base,
    }
  }

  fn physics_process(&mut self, _delta: f64) {

    // TODO: Set race condition better to avoid trying to spawn troops before the planet is ready
    Self::troop_spawner(self);
  }
}

impl RootScene {
  pub fn troop_spawner(&mut self) {
    
    // STEP 1: GETTING TERRITORIES
    let virtual_planet = self.base_mut()
      .get_node_as::<VirtualPlanet>("virtual_planet");
    let virtual_planet = &virtual_planet.bind();
    let territories = &virtual_planet.territories;

    // STEP 2: GETTING TERRITORY LAND LOCATION TO SPAWN TROOP
    let spawning_land = territories.get("atlantic_forest").unwrap();
    let spawning_land_coordinates_list_len = spawning_land.coordinates.len();

    // get a random coordinate from the territory
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(1..spawning_land_coordinates_list_len);

    let coordinate = spawning_land.coordinates[random_index];

    let coordinate_metadata = &virtual_planet.coordinate_map
      .get(&coordinate)
      .expect("Coordinate expected to exist");
    let cartesian = coordinate_metadata.cartesian;

    // STEP 3: SPAWNING TROOP
    let player_color = Color::DARK_CYAN;
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(player_color);
    let scene: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
    let mut new_troop = scene.instantiate_as::<Troop>();
    //TODO: generate a troop ID base on: territory_id + player_id + timestamp
    //TODO: use troop_id to acknologe the troop location along the planet
    let troop_id = "troop";
    new_troop.set_name(&troop_id.to_godot());

    // For organization matter, new_troops are spawn under /root_scene/troops
    self.base()
      .find_child("troops") 
      .expect("troops to exist")
      .add_child(&new_troop);

    new_troop.set_position(cartesian);

    let troop_node = new_troop.find_child("default_mesh").expect("MeshInstance3D to exist");
    let mut troop_mesh = troop_node.cast::<MeshInstance3D>();
    troop_mesh.set_surface_override_material(0, &material);



  }
}