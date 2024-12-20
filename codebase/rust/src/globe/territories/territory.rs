use std::collections::HashMap;

use godot::{builtin::Color, classes::{MeshInstance3D, StandardMaterial3D}, prelude::*};

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
      Continent::Africa => /* Color::LIGHT_SLATE_GRAY */ Color::WHITE_SMOKE,
      Continent::Asia => /* Color::GREEN_YELLOW */ Color::WHITE_SMOKE,
      Continent::Europe => /* Color::SKY_BLUE */ Color::WHITE_SMOKE,
      Continent::NorthAmerica => /* Color::INDIAN_RED */ Color::WHITE_SMOKE,
      Continent::Oceania => /* Color::BURLYWOOD */ Color::WHITE_SMOKE,
      Continent::SouthAmerica => /* Color::TOMATO */ Color::WHITE_SMOKE,
      Continent::Antarctica => /* Color::LAVENDER_BLUSH */ Color::WHITE_SMOKE,
      Continent::Special => /* Color::GOLD */ Color::WHITE_SMOKE,
    }
  }

  pub fn get_territory_color(sub_continent: &Option<SubContinent>, continent: &Continent) -> Color {
    match sub_continent {
      Some(SubContinent::MiddleEast) => /* Color::from_rgba(0., 0.3, 0., 1.)*/ Color::WHITE_SMOKE,
      Some(SubContinent::InteriorAsia) => /* Color::from_rgba(0., 0.4, 0., 1.)*/ Color::WHITE_SMOKE,
      Some(SubContinent::IndianSubcontinent) => /* Color::from_rgba(0., 0.5, 0., 1.)*/ Color::WHITE_SMOKE,
      Some(SubContinent::PacificAndSoutheastAsia) => /* Color::from_rgba(0., 0.6, 0., 1.)*/ Color::WHITE_SMOKE,
      Some(SubContinent::EuropeRelatedAsia) => /* Color::from_rgba(0., 0.7, 0., 1.)*/ Color::WHITE_SMOKE,
      None => Self::continent_to_color(&continent)
    }
  }

  pub fn set_color_to_territory(mut territory: Gd<MeshInstance3D>, color: Color) {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(color);
    territory.set_material_override(&material);
  }

  pub fn checking_territory(territory: Gd<MeshInstance3D>) {
    Self::set_color_to_territory(territory, Color::LIGHT_PINK);
  }

  pub fn unchecking_territory(territory: Gd<MeshInstance3D>) {
    Self::set_color_to_territory(territory, Color::WHITE_SMOKE);
  }

  pub fn clicking_territory(territory: Gd<MeshInstance3D>) {
    Self::set_color_to_territory(territory, Color::LIGHT_GREEN);
  }
}