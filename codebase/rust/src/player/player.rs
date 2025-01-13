use std::collections::{HashMap, HashSet};

use godot::{classes::INode3D, prelude::*};

use crate::{globe::{coordinates_system::virtual_planet::VirtualPlanet, territories::territory::TerritoryId}, root::root::RootScene, troops::mesh_map::MeshId};
use super::color::PlayerColor;

/// Defines
/// troop colors,
/// allyship,
/// spawn engine,
/// troops counter,
/// territory counter
/// 
/// Should also have players actions, such as
/// move troops
/// atck
/// run away

struct EnemyStats {
  /// The number of troops that were injured
  casualties_caused_by_player: f32,
  /// The number of troops that were killed
  fatalities_caused_by_player: i32,
  /// The number of territories that were taken
  territories_taken_by_player: i32,
}

#[derive(Debug, Clone)]
pub struct TroopMeshes {
  pub land: MeshId,
  pub sea: MeshId,
}

pub type PlayerId = i32;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Player {
  base: Base<Node3D>,
  pub static_info: PlayerStaticInfo,
  pub troops_counter: i32,
  territory_counter: i32,

  /// it's the sum of every territory's organic_max_troops being ruled by the player
  pub max_troop_allowed: i32,

  alive: bool,
  
  in_combat_with: HashSet<Player>,
  allied_with: HashSet<Player>,
  enemies_stats: HashMap<PlayerId, EnemyStats>,
}

#[derive(PartialEq, Debug, Clone, GodotConvert)]
#[godot(via = i64)]
pub enum PlayerType {
  MainPlayer,
  OtherPlayers,
  Bot,
}

#[derive(Debug, Clone)]
pub struct PlayerStaticInfo {
  pub player_id: PlayerId,
  pub user_name: String,
  pub color: PlayerColor,
  pub initial_territory: TerritoryId,
  pub player_type: PlayerType,
  pub troop_meshes: TroopMeshes,
}

#[godot_api]
impl INode3D for Player {
  fn init(base: Base<Node3D>) -> Player {

    Player {
      base: base,
      static_info: Self::get_blank_static_info(),
      troops_counter: 0,
      territory_counter: 0,
      max_troop_allowed: 0,
      alive: true, 
      in_combat_with: HashSet::new(),
      allied_with: HashSet::new(),
      enemies_stats: HashMap::new(),
    }
  }

}

#[godot_api]
impl Player {
  pub fn set_player(
    &mut self,
    player_id: PlayerId,
    user_name: String,
    color: PlayerColor,
    initial_territory: TerritoryId,
    player_type: PlayerType,
    troop_meshes: TroopMeshes,
  ) {
    self.static_info = PlayerStaticInfo {
      player_id,
      user_name,
      color,
      initial_territory,
      player_type,
      troop_meshes,
    };

    let player_group_id = &Self::get_player_godot_identifier(player_id);
    self.base_mut().add_to_group(player_group_id);
    self.base_mut().set_name(player_group_id);
  }

  /// Returns the player id used in Godot set on the nodes' name and as group
  /// it couldn't use the PlayerId beucase it's just a i32 and
  /// it could clash with other nodes and group names
  fn get_player_godot_identifier(player_id: PlayerId) -> String {
    format!("player_{player_id}")
  }

  pub fn get_blank_static_info() -> PlayerStaticInfo {
    PlayerStaticInfo {
      player_id: 0,
      user_name: "to_be_set".to_owned(),
      color: PlayerColor::Black,
      initial_territory: "to_be_set".to_owned(),
      player_type: PlayerType::Bot,
      troop_meshes: TroopMeshes {
        land: MeshId::Tank1,
        sea: MeshId::Boat1,
      }
    }
  }

  pub fn register_troop_spawning(&mut self) {
    self.troops_counter += 1;
  }

  pub fn register_territory_occupation(&mut self, _territory_id: TerritoryId) {
    self.territory_counter += 1;
  }

  fn register_territory_loss(&mut self) {
    self.territory_counter -= 1;

    if self.territory_counter <= 0 {
      self.territory_counter = 0;
    }
  }

  /// expects the following hierarchy:
  /// ```
  /// root_scene
  /// |-players
  /// ||-player
  /// ```
  pub fn get_root_from_player(&mut self) -> Gd<Node> {
    self
      .base()
      .get_parent().expect("Expected player to have players as parent")
      .get_parent().expect("Expected players to have root as parent")
  }

  fn get_virtual_planet_from_player(&mut self) -> Gd<VirtualPlanet> {
    let virtual_planet = self
      .get_root_from_player()
      .try_get_node_as::<VirtualPlanet>("virtual_planet")
      .expect("Expected to find VirtualPlanet from RootScene");

    virtual_planet
  }

  /// Assumes Player node has PlayerId as name
  pub fn get_player_by_id(root_scene: Gd<RootScene>, player_id: PlayerId) -> Gd<Player> {
    let player = root_scene.get_node_as::<Player>(&format!("players/{player_id}"));
    player
  }

}
