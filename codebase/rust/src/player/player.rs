// TODO: Remove this line once the file is implemented
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::globe::territories::territory::TerritoryId;

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

enum PlayerColor {
  Red,
  Blue,
  Green,
  Yellow,
  Purple,
  Orange,
  Black,
  White,
}

struct EnemyStats {
  /// The number of troops that were injured
  casualties_caused_by_player: f32,
  /// The number of troops that were killed
  fatalities_caused_by_player: i32,
  /// The number of territories that were taken
  territories_taken_by_player: i32,
}

type PlayerId = i32;

struct Player {
  player_id: PlayerId,
  color: PlayerColor,
  initial_territory: TerritoryId,
  troops_counter: i32,
  territory_counter: i32,

  alive: bool,
  
  in_combat_with: HashSet<Player>,
  allied_with: HashSet<Player>,
  enemies_stats: HashMap<PlayerId, EnemyStats>,
}

impl Player {
  fn new(
    player_id: PlayerId,
    color: PlayerColor,
    initial_territory: TerritoryId
  ) -> Self {
    Player {
      player_id,
      color,
      initial_territory,
      troops_counter: 0,
      territory_counter: 0,
      alive: true,
      in_combat_with: HashSet::new(),
      allied_with: HashSet::new(),
      enemies_stats: HashMap::new(),
    }
  }
}