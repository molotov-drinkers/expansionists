
use godot::{
  classes::StandardMaterial3D, prelude::*
};
use crate::{
  globe::{coordinates_system::{
    surface_point::Coordinates,
    virtual_planet::VirtualPlanet,
  }, territories::territory::TerritoryId},
  root::root::RootScene,
};

use super::troop::Troop;

/// Called from root.rs
pub fn troop_spawner(root_scene: &mut RootScene, virtual_planet: &VirtualPlanet, troops_spawn: i32, territory_id: TerritoryId) {
  let coordinates: Coordinates = VirtualPlanet
    ::get_spawner_territory_coordinate(&virtual_planet, &territory_id);

  let cartesian = virtual_planet
    .coordinate_map
    .get(&coordinates)
    .expect("Coordinate expected to exist")
    .cartesian;

  let player_color = Color::FLORAL_WHITE;
  let mut material = StandardMaterial3D::new_gd();
  material.set_albedo(player_color);
  let new_troop: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
  let mut new_troop = new_troop.instantiate_as::<Troop>();

  // TICKET: #39 generate a troop ID base on: territory_id + player_id + timestamp
  let troop_id = format!("troop ... {:}-{:}", troops_spawn, territory_id);
  new_troop.set_name(&troop_id.to_godot());
  // new_troop.bind_mut().located_at = coordinate;

  // For organization matter, new_troops are spawn under /root_scene/troops
  root_scene.base()
    .find_child("troops") 
    .expect("troops to exist")
    .add_child(&new_troop);

  new_troop.set_position(cartesian);
  new_troop.bind_mut().deployed_to_territory = territory_id.to_string();

  // TICKET: #46, Get player color and set it for sea and land troops
  // let troop_node = new_troop.find_child("default_mesh").expect("MeshInstance3D to exist");
  // let mut troop_mesh = troop_node.cast::<MeshInstance3D>();
  // troop_mesh.set_surface_override_material(0, &material);

}