use std::collections::HashMap;

use godot::builtin::Color;

use crate::{globe::coordinates_system::surface_point::Coordinates, troops::troop::Troop};

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

pub enum Size {
  Tiny,
  Small,
  Medium,
  Large,
  Huge,
  Humongous,
}

pub type TerritoryId = String;
pub type Territories = HashMap<TerritoryId, Territory>;

pub struct Territory {
  pub territory_id: TerritoryId,
  pub location: Location,

  pub coordinates: Vec<Coordinates>,

  /// (TODO:) backs up the population of organic_max_troops and troops_growth_velocity
  pub size: Size,
  pub organic_max_troops: i32,
  pub troops_growth_velocity: f32,

  // TICKET: #40 Implement territory size, used for organic_max_troops, etc.

  pub current_landlord: Option<String>, // That will be player_id
  /// (TODO:) uses all the surface points of the territory to calculate which troops are inside it
  pub current_troops: Vec<Troop>,
}

impl Territory {
  fn continent_to_color(continent: &Continent) -> Color {
    match continent {
      Continent::Africa => Color::LIGHT_SLATE_GRAY,
      Continent::Asia => Color::GREEN_YELLOW,
      Continent::Europe => Color::SKY_BLUE,
      Continent::NorthAmerica => Color::INDIAN_RED,
      Continent::Oceania => Color::BURLYWOOD,
      Continent::SouthAmerica => Color::TOMATO,
      Continent::Antarctica => Color::LAVENDER_BLUSH,
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