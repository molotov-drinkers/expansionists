use crate::{globe::territories::territory::TroopId, troops::troop::Troop};

pub enum LocationSituation {
  SelfLand,
  AllyLand,
  NeutralLand,
  EnemyLand,
}

pub enum FighthingBehavior {
  /// will fight any non-ally troop who crosses by it doesn't matter the territory
  Beligerent,

  /// will only fight if attacked or if it's territory is attacked
  Pacifist,
}

pub struct CombatStats {
  pub in_after_combat: bool,
  pub cannon: Cannon,

  _hp: i32,
  _alive: bool,

  _fighting_behavior: FighthingBehavior,
  pub troop_being_attacked: Option<TroopId>,
}

const COOL_DOWN_TIMER_IN_SECS: f64 = 2.;
const CANNON_RANGE: f32 = 10.;

pub struct Cannon {
  _damage: i32,
  pub range: f32,
  pub cooling_down: bool,
  cooling_down_counter: f64,
}

impl CombatStats {
  pub fn new() -> CombatStats {
    CombatStats {
      in_after_combat: false,
      _hp: 100,
      _alive: false,
      _fighting_behavior: FighthingBehavior::Beligerent,
      cannon: Cannon {
        _damage: 11,
        range: CANNON_RANGE,
        cooling_down: false,
        cooling_down_counter: 0.,
      },
      troop_being_attacked: None,
    }
  }
}