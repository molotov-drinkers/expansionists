
use godot::{
  classes::{MeshInstance3D, Sprite3D, StandardMaterial3D}, prelude::*
};
use crate::{
  globe::{coordinates_system::{
    surface_point::Coordinates,
    virtual_planet::VirtualPlanet,
  }, territories::territory::TerritoryId}, player::{color::PlayerColor, player::{PlayerStaticInfo, PlayerType}}, root::root::RootScene
};

use super::{mesh_map::TroopMesh, troop::Troop};


/// Called from root.rs
pub fn troop_spawner(
  root_scene: &mut RootScene,
  virtual_planet: &VirtualPlanet,
  territory_id: &TerritoryId,
  player_static_info: &PlayerStaticInfo,
  // player: &mut Gd<Player>,
) {
  let coordinates: Coordinates = virtual_planet.get_spawner_territory_coordinate(territory_id);

  let cartesian = virtual_planet
    .coordinate_map
    .get(&coordinates)
    .expect("Coordinate expected to exist")
    .cartesian;

  let new_troop: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
  let mut new_troop = new_troop.instantiate_as::<Troop>();
  new_troop.bind_mut().set_ownership(&player_static_info);

  let mut land_node = new_troop
    .find_child("land")
    .expect("Expected land to exist");
    
  let mut sea_node = new_troop
    .find_child("sea")
    .expect("Expected sea to exist");

  let (land_troop, sea_troop) = get_colored_troop_scenes(&player_static_info);
  land_node.add_child(&land_troop);
  sea_node.add_child(&sea_troop);

  land_node
    .get_node_as::<Sprite3D>("selected")
    .set_modulate(PlayerColor::get_troop_selected_color(&player_static_info.color));
  sea_node
    .get_node_as::<Sprite3D>("selected")
    .set_modulate(PlayerColor::get_troop_selected_color(&player_static_info.color));


  // TICKET: #39 generate a troop ID base on: territory_id + player_id + timestamp
  let troop_id = format!("troop ... {:}-{:}", &player_static_info.user_name, territory_id);
  new_troop.set_name(&troop_id.to_godot());
  new_troop.add_to_group(&player_static_info.player_id.to_string());

  match player_static_info.player_type {
    PlayerType::MainPlayer => new_troop.add_to_group(Troop::MAIN_PLAYER_TROOPS),
    PlayerType::Bot => new_troop.add_to_group(Troop::BOT_TROOPS),
    _ => (),
  }

  let player_type = &player_static_info.player_type;
  let player_id = &player_static_info.player_id;
  new_troop.emit_signal(
    Troop::EVENT_TROOP_SPAWNED,
    &[
      player_id.to_variant(),
      player_type.to_variant(),
    ]
  );

  // player.bind_mut().set_troop_spawn_event_receptions(&mut new_troop);

  // For organization matter, new_troops are spawn under /root_scene/troops
  root_scene.base()
    .find_child("troops") 
    .expect("troops to exist")
    .add_child(&new_troop);

  new_troop.set_position(cartesian);
  new_troop.bind_mut().deployed_to_territory = territory_id.to_string();

}

fn get_colored_troop_scenes(player: &PlayerStaticInfo) -> (Gd<Node3D>, Gd<Node3D>) {
  let troop_meshes = &player.troop_meshes;
  let lands = TroopMesh::get_land_meshes();
  let seas = TroopMesh::get_sea_meshes();
  
  let land_troop = lands.get(&troop_meshes.land)
    .expect(&format!("Expected {:?} land mesh to exist", &troop_meshes.land));
  let sea_troop = seas.get(&troop_meshes.sea)
    .expect(&format!("Expected {:?} sea mesh to exist", &troop_meshes.sea));

  let land_scene_name = &land_troop.scene_name;
  let sea_scene_name = &sea_troop.scene_name;

  let land_mesh: Gd<PackedScene> = load(&format!("res://scenes/troops/land/{land_scene_name}.tscn"));
  let sea_mesh: Gd<PackedScene> = load(&format!("res://scenes/troops/sea/{sea_scene_name}.tscn"));
  let land_mesh = land_mesh.instantiate_as::<Node3D>();
  let sea_mesh = sea_mesh.instantiate_as::<Node3D>();

  let mut material = StandardMaterial3D::new_gd();
  material.set_albedo(PlayerColor::get_troop_player_color(&player.color));
  
  land_mesh
    .get_child(0)
    .expect("Expected land mesh to exist")
    .try_cast::<MeshInstance3D>()
    .expect("Expected land child to be a MeshInstance3D")
    .set_surface_override_material(land_troop.surface_to_be_colored, &material);

  sea_mesh
    .get_child(0)
    .expect("Expected sea mesh to exist")
    .try_cast::<MeshInstance3D>()
    .expect("Expected sea child to be a MeshInstance3D")
    .set_surface_override_material(sea_troop.surface_to_be_colored, &material);

  (land_mesh, sea_mesh)
}