pub enum SpeedType {
  Patrolling,
  FightOrFlight,
}

impl SpeedType {
  pub fn get_speed(&self) -> f32 {
    match self {
      SpeedType::Patrolling => 0.05,
      SpeedType::FightOrFlight => 0.15,
    }
  }
}

