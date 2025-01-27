use crate::troops::troop::TroopId;

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

pub struct Cannon {
  _damage: i32,
  pub range: f32,
  // pub firing: bool,
  pub cooling_down_counter: f64,
}

pub struct CombatStats {
  pub in_after_combat: bool,
  pub cannon: Cannon,
  pub moving_while_fighting: bool,

  _hp: i32,
  _alive: bool,

  _fighting_behavior: FighthingBehavior,
  pub troop_being_attacked: Option<TroopId>,
}

impl CombatStats {
  pub const COOL_DOWN_TIMER_IN_SECS: f64 = 2.;
  pub const CANNON_RANGE: f32 = 1.;

  pub fn new() -> CombatStats {
    CombatStats {
      in_after_combat: false,
      _hp: 100,
      _alive: false,
      _fighting_behavior: FighthingBehavior::Beligerent,
      cannon: Cannon {
        _damage: 11,
        // firing: false,
        range: Self::CANNON_RANGE,
        cooling_down_counter: 0.,
      },
      troop_being_attacked: None,
      moving_while_fighting: false,
    }
  }
}