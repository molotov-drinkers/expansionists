
use godot::{
  classes::StandardMaterial3D, prelude::*
};
use crate::{
  globe::{coordinates_system::{
    surface_point::Coordinates,
    virtual_planet::VirtualPlanet,
  }, territories::territory::TerritoryId}, player::player::{PlayerStaticInfo, TroopMeshes}, root::root::RootScene
};

use super::{mesh_map::TroopMesh, troop::Troop};

// TODO maybe push this to territory or land?
pub fn troop_spawner_1() {

}

/// Called from root.rs
pub fn troop_spawner(root_scene: &mut RootScene,
  virtual_planet: &VirtualPlanet,
  troops_spawn: i32,
  territory_id: &TerritoryId,
  player: &PlayerStaticInfo
) {
  let coordinates: Coordinates = VirtualPlanet
    ::get_spawner_territory_coordinate(&virtual_planet, territory_id);

  let cartesian = virtual_planet
    .coordinate_map
    .get(&coordinates)
    .expect("Coordinate expected to exist")
    .cartesian;

  let new_troop: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
  let mut new_troop = new_troop.instantiate_as::<Troop>();
  new_troop.bind_mut().set_ownership(player);

  let (land_troop, sea_troop) = get_troop_scenes(&player.troop_meshes);

  // TODO: Add troops to main troop_scene and paint the selected sprite3d too

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

fn get_troop_scenes(troop_meshes: &TroopMeshes, ) -> (Gd<Node3D>, Gd<Node3D>) {
  let lands = TroopMesh::get_land_meshes();
  let seas = TroopMesh::get_sea_meshes();
  
  let land = lands.get(&troop_meshes.land)
    .expect("Expected land mesh to exist");
  let sea = seas.get(&troop_meshes.sea)
    .expect("Expected sea mesh to exist");

  let land_scene_name = &land.scene_name;
  let sea_scene_name = &sea.scene_name;

  let land_mesh: Gd<PackedScene> = load(&format!("res://scenes/troops/land/{land_scene_name}.tscn"));
  let sea_mesh: Gd<PackedScene> = load(&format!("res://scenes/troops/sea/{sea_scene_name}.tscn"));
  let land_mesh = land_mesh.instantiate_as::<Node3D>();
  let sea_mesh = sea_mesh.instantiate_as::<Node3D>();

  // TODO: set colors

  (land_mesh, sea_mesh)
}