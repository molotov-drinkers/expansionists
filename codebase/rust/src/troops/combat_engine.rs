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
  _in_after_combat: bool,

  _damage: i32,
  _hp: i32,
  _fight_or_flight_speed: f32,
  _alive: bool,

  _fighting_behavior: FighthingBehavior,
}

impl CombatStats {
  pub fn new() -> CombatStats {
    CombatStats {
      _in_after_combat: false,
      _damage: 0,
      _hp: 0,
      _fight_or_flight_speed: 1.5,
      _alive: false,
      _fighting_behavior: FighthingBehavior::Beligerent,
    }
  }
}