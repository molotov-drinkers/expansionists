use godot::{classes::{ICharacterBody3D, CharacterBody3D}, prelude::*};

use crate::globe::coordinates_system::{surface_point::Coordinates, virtual_planet::{self, VirtualPlanet}};

pub enum LocationSituation {
  SelfLand,
  AllyLand,
  NeutralLand,
  EnemyLand,
}

pub enum Surface {
  Land,
  Water,

  // future_version:
  // Air,
}

pub enum FighthingBehavior {
  /// will fight any non-ally troop who crosses by it doesn't matter the territory
  Beligerent,

  /// will only fight if attacked or if it's territory is attacked
  Pacifist,
}

pub struct CombatStats {
  pub in_combat: bool,
  pub in_after_combat: bool,

  pub damage: i32,
  pub hp: i32,
  pub speed: i32,
  pub alive: bool,

  pub fighting_behavior: FighthingBehavior,
}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Troop {
  base: Base<CharacterBody3D>,

  pub located_at: Coordinates,
  pub location_situation: LocationSituation,
  pub surface_type: Surface,

  pub owner: String,

  pub combat_stats: CombatStats,

  // used for animation inside of the territory
  pub is_moving: bool,
  pub randomly_walking_to: Coordinates,
}

#[godot_api]
impl ICharacterBody3D for Troop {
  fn init(base: Base<CharacterBody3D>) -> Troop {

    Troop {
      base: base,
      
      located_at: (0, 0),
      location_situation: LocationSituation::NeutralLand,
      surface_type: Surface::Land,

      owner: "".to_string(),

      combat_stats: CombatStats {
        in_combat: false,
        in_after_combat: false,
        damage: 0,
        hp: 0,
        speed: 0,
        alive: false,
        fighting_behavior: FighthingBehavior::Beligerent,
      },

      is_moving: false,
      randomly_walking_to: (0, 0),
    }
  }

  fn ready(&mut self) {
    godot_print!("Troop ready");

  }

  fn physics_process(&mut self, _delta: f64) {

  }
}

impl Troop {
  fn is_on_self_land(&self) -> bool {
    // TODO: implement
    self.located_at;
    true
  }

  fn is_on_ally_land(&self) -> bool {
    // TODO: implement
    false
  }


  fn start_random_walk_within_territory(&mut self) {
    if
      (Self::is_on_self_land(self) || Self::is_on_ally_land(self)) &&
      self.combat_stats.in_combat == false {

      // TODO: implement
      self.is_moving = true;
      self.randomly_walking_to = (0, 0);

      // let randomly_walking_to = VirtualPlanet::get_another_territory_coordinate(
      //   &virtual_planet,
      //   self.located_at
      // );

    }
  }
}