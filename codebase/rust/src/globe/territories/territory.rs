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
      Continent::Africa => /* Color::LIGHT_SLATE_GRAY */ Color::LIGHT_SLATE_GRAY,
      Continent::Asia => /* Color::GREEN_YELLOW */ Color::LIGHT_SLATE_GRAY,
      Continent::Europe => /* Color::SKY_BLUE */ Color::LIGHT_SLATE_GRAY,
      Continent::NorthAmerica => /* Color::INDIAN_RED */ Color::LIGHT_SLATE_GRAY,
      Continent::Oceania => /* Color::BURLYWOOD */ Color::LIGHT_SLATE_GRAY,
      Continent::SouthAmerica => /* Color::TOMATO */ Color::LIGHT_SLATE_GRAY,
      Continent::Antarctica => /* Color::LAVENDER_BLUSH */ Color::LIGHT_SLATE_GRAY,
      Continent::Special => /* Color::GOLD */ Color::LIGHT_SLATE_GRAY,
    }
  }

  pub fn get_territory_color(sub_continent: &Option<SubContinent>, continent: &Continent) -> Color {
    match sub_continent {
      Some(SubContinent::MiddleEast) => /* Color::from_rgba(0., 0.3, 0., 1.)*/ Color::LIGHT_SLATE_GRAY,
      Some(SubContinent::InteriorAsia) => /* Color::from_rgba(0., 0.4, 0., 1.)*/ Color::LIGHT_SLATE_GRAY,
      Some(SubContinent::IndianSubcontinent) => /* Color::from_rgba(0., 0.5, 0., 1.)*/ Color::LIGHT_SLATE_GRAY,
      Some(SubContinent::PacificAndSoutheastAsia) => /* Color::from_rgba(0., 0.6, 0., 1.)*/ Color::LIGHT_SLATE_GRAY,
      Some(SubContinent::EuropeRelatedAsia) => /* Color::from_rgba(0., 0.7, 0., 1.)*/ Color::LIGHT_SLATE_GRAY,
      None => Self::continent_to_color(&continent)
    }
  }

  pub fn checking_territory(mut territory: Gd<MeshInstance3D>) {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(Color::RED);
    territory.set_material_override(&material);
  }

  pub fn unchecking_territory(mut territory: Gd<MeshInstance3D>) {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(Color::LIGHT_SLATE_GRAY);
    territory.set_material_override(&material);
  }

  pub fn clicking_territory(mut territory: Gd<MeshInstance3D>) {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(Color::GREEN);
    territory.set_material_override(&material);
  }
}