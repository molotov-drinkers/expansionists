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
  pub range: f32,
  // pub firing: bool,
  pub cooling_down_counter: f64,
}

pub struct CombatStats {
  pub in_after_combat: bool,
  pub cannon: Cannon,
  pub moving_while_fighting: bool,

  pub hp: i32,
  pub alive: bool,

  _fighting_behavior: FighthingBehavior,
  pub opening_fire_on_troop: Option<TroopId>,
}

impl CombatStats {
  pub const COOL_DOWN_TIMER_IN_SECS: f64 = 2.;
  pub const CANNON_RANGE: f32 = 0.4;

  pub fn new() -> CombatStats {
    CombatStats {
      in_after_combat: false,
      hp: 100,
      alive: true,
      _fighting_behavior: FighthingBehavior::Beligerent,
      cannon: Cannon {
        // firing: false,
        range: Self::CANNON_RANGE,
        cooling_down_counter: Self::COOL_DOWN_TIMER_IN_SECS,
      },
      opening_fire_on_troop: None,
      moving_while_fighting: false,
    }
  }

  pub fn reset_cannon_cool_down(&mut self) {
    self.cannon.cooling_down_counter = Self::COOL_DOWN_TIMER_IN_SECS;
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