use std::collections::{HashMap, HashSet};

use godot::{classes::INode3D, prelude::*};

use crate::{globe::{coordinates_system::virtual_planet::VirtualPlanet, territories::territory::TerritoryId}, troops::{mesh_map::MeshId, troop::Troop}};
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
      alive: true, 
      in_combat_with: HashSet::new(),
      allied_with: HashSet::new(),
      enemies_stats: HashMap::new(),
    }
  }

  fn ready(&mut self) {
    self.set_virtual_planet_event_receptions();
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
    self.static_info.player_id = player_id;
    self.static_info.user_name = user_name;
    self.static_info.color = color;
    self.static_info.initial_territory = initial_territory;
    self.static_info.player_type = player_type;
    self.static_info.troop_meshes = troop_meshes;

    self.base_mut().add_to_group(&player_id.to_string());
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


  fn set_virtual_planet_event_receptions(&mut self) {
    let mut virtual_planet = self.get_virtual_planet_from_player();
    let callable = self.base_mut().callable("register_territory_conquest");
    virtual_planet.connect(VirtualPlanet::EVENT_TERRITORY_CONQUEST, &callable);

    let callable = self.base_mut().callable("register_troop_fatality");
    virtual_planet.connect(VirtualPlanet::EVENT_TERRITORY_LOST, &callable);
  }

  pub fn set_troop_spawn_event_receptions(&mut self, new_troop: &mut Gd<Troop>) {
    let callable = self.base_mut().callable("register_troop_spawning");
    new_troop.connect(Troop::EVENT_TROOP_SPAWNED, &callable);
  }

  #[func]
  fn register_troop_spawning(&mut self, _player_id: PlayerId, player_type: PlayerType) {
    if player_type == PlayerType::MainPlayer {
      self.troops_counter += 1;
    }
  }

  #[func]
  fn register_troop_fatality(&mut self) {
    self.troops_counter -= 1;

    if self.troops_counter <= 0 {
      self.troops_counter = 0;
    }
  }

  #[func]
  fn register_territory_conquest(&mut self, _player_id: PlayerId, player_type: PlayerType) {
    if player_type == PlayerType::MainPlayer {
      self.territory_counter += 1;
    }
  }

  #[func]
  fn register_territory_loss(&mut self, _player_id: PlayerId, player_type: PlayerType) {
    if player_type == PlayerType::MainPlayer {
      self.territory_counter -= 1;

      if self.territory_counter <= 0 {
        self.territory_counter = 0;
      }
    }
  }

  /// expects the following hierarchy:
  /// ```
  /// root_scene
  /// |-players
  /// ||-player
  /// ```
  fn get_root_from_player(&mut self) -> Gd<Node> {
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
}
