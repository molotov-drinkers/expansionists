use std::collections::HashMap;

use godot::builtin::Color;

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
  // base_name is also used as the territory id
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

impl Territory {
  fn continent_to_color(continent: &Continent) -> Color {
    match continent {
      Continent::Africa => Color::DARK_ORANGE,
      Continent::Asia => Color::GREEN_YELLOW,
      Continent::Europe => Color::SKY_BLUE,
      Continent::NorthAmerica => Color::DARK_RED,
      Continent::Oceania => Color::BURLYWOOD,
      Continent::SouthAmerica => Color::TOMATO,
      Continent::Antarctica => Color::DARK_CYAN,
      Continent::Special => Color::GOLD,
    }
  }

  pub fn get_territory_color(sub_continent: &Option<SubContinent>, continent: &Continent) -> Color {
    match sub_continent {
      Some(SubContinent::MiddleEast) => Color::from_rgba(0., 0.3, 0., 1.),
      Some(SubContinent::InteriorAsia) => Color::from_rgba(0., 0.4, 0., 1.),
      Some(SubContinent::IndianSubcontinent) => Color::from_rgba(0., 0.5, 0., 1.),
      Some(SubContinent::PacificAndSoutheastAsia) => Color::from_rgba(0., 0.6, 0., 1.),
      Some(SubContinent::EuropeRelatedAsia) => Color::from_rgba(0., 0.7, 0., 1.),
      None => Self::continent_to_color(&continent)
    }
  }
}