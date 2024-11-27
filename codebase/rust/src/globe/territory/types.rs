use std::collections::HashMap;

pub enum Continent {
  Africa,
  Asia,
  Europe,
  NorthAmerica,
  Oceania,
  SouthAmerica,
  Antarctica,
  Special,
}

pub enum SubContinent {
  MiddleEast,
  InteriorAsia,
  IndianSubcontinent,
  PacificAndSoutheastAsia,
  EuropeRelatedAsia,
}

pub struct Location {
  pub continent: Continent,
  pub sub_continent: Option<SubContinent>,
}

pub struct Territory {
  pub base_name: String,
  pub location: Location,

  // pub neighbors: Vec<TerritoryId>,

  // pub organic_max_troops: i32,
  // pub troops_growth_velocity: i32,

  // pub current_owner: String,
  // pub current_troops: i32,
}

pub type TerritoryId = String;
pub type Territories = HashMap<TerritoryId, Territory>;