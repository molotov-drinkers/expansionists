use std::collections::{HashMap, HashSet};

use godot::{classes::INode3D, prelude::*};

use crate::{globe::territories::territory::TerritoryId, troops::mesh_map::MeshId};
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
  troops_counter: i32,
  territory_counter: i32,

  alive: bool,
  
  in_combat_with: HashSet<Player>,
  allied_with: HashSet<Player>,
  enemies_stats: HashMap<PlayerId, EnemyStats>,
}

#[derive(Debug, Clone)]
pub struct PlayerStaticInfo {
  pub player_id: PlayerId,
  pub user_name: String,
  pub color: PlayerColor,
  pub initial_territory: TerritoryId,
  pub actual_player: bool,
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
      alive: true, 
      in_combat_with: HashSet::new(),
      allied_with: HashSet::new(),
      enemies_stats: HashMap::new(),
    }
  }
}

impl Player {
  pub fn set_player(
    &mut self,
    player_id: PlayerId,
    user_name: String,
    color: PlayerColor,
    initial_territory: TerritoryId,
    actual_player: bool,
    troop_meshes: TroopMeshes,
  ) {
    self.static_info.player_id = player_id;
    self.static_info.user_name = user_name;
    self.static_info.color = color;
    self.static_info.initial_territory = initial_territory;
    self.static_info.actual_player = actual_player;
    self.static_info.troop_meshes = troop_meshes;
  }

  pub fn get_blank_static_info() -> PlayerStaticInfo {
    PlayerStaticInfo {
      player_id: 0,
      user_name: "to_be_set".to_owned(),
      color: PlayerColor::Black,
      initial_territory: "to_be_set".to_owned(),
      actual_player: false,
      troop_meshes: TroopMeshes {
        land: MeshId::Tank1,
        sea: MeshId::Boat1,
      }
    }
  }
}