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
  pub opening_fire_on_troop: Option<TroopId>,
}

impl CombatStats {
  pub const COOL_DOWN_TIMER_IN_SECS: f64 = 2.;
  pub const CANNON_RANGE: f32 = 0.4;

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
      opening_fire_on_troop: None,
      moving_while_fighting: false,
    }
  }
}

/// Combat types the troop can engage
/// Needs to populate CombatTypes::iter() method
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CombatTypes {
  Attacking,
  Defending,
  FightingOverUnoccupiedTerritory,
}

impl CombatTypes {
  pub fn iter() -> impl Iterator<Item = CombatTypes> {
    [
      Self::Attacking,
      Self::Defending,
      Self::FightingOverUnoccupiedTerritory,
    ]
    .iter()
    .cloned()
  }
}