use std::collections::HashMap;
use std::fmt;

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

impl fmt::Display for Continent {
  /// allows to use `&Continent::Africa.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Continent::Africa => write!(f, "africa"),
      Continent::Asia => write!(f, "asia"),
      Continent::Europe => write!(f, "europe"),
      Continent::NorthAmerica => write!(f, "north_america"),
      Continent::Oceania => write!(f, "oceania"),
      Continent::SouthAmerica => write!(f, "south_america"),
      Continent::Antarctica => write!(f, "antarctica"),
      Continent::Special => write!(f, "special"),
    }
  }
}

pub enum SubContinent {
  MiddleEast,
  InteriorAsia,
  IndianSubcontinent,
  SoutheastAsia,
  EastAsia,
  EuropeRelatedAsia,
}

impl fmt::Display for SubContinent {
  /// allows to use `&SubContinent::MiddleEast.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SubContinent::MiddleEast => write!(f, "middle_east"),
      SubContinent::InteriorAsia => write!(f, "interior_asia"),
      SubContinent::IndianSubcontinent => write!(f, "indian_subcontinent"),
      SubContinent::SoutheastAsia => write!(f, "southeast_asia"),
      SubContinent::EastAsia => write!(f, "east_asia"),
      SubContinent::EuropeRelatedAsia => write!(f, "europe_related_asia"),
    }
  }
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
  None,
}

impl fmt::Display for Size {
  /// allows to use `&Size::Tiny.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Size::Tiny => write!(f, "tiny"),
      Size::Small => write!(f, "small"),
      Size::Medium => write!(f, "medium"),
      Size::Large => write!(f, "large"),
      Size::Huge => write!(f, "huge"),
      Size::Humongous => write!(f, "humongous"),
      Size::None => write!(f, "none"),
    }
  }
}

pub type TerritoryId = String;
pub type Territories = HashMap<TerritoryId, Territory>;

/// Not a Godot class, look at `land.rs`, `surface_point.rs` and
/// `virtual_planet.rs` for the Godot classes related to territories
pub struct Territory {
  pub territory_id: TerritoryId,
  pub location: Location,

  pub coordinates: Vec<Coordinates>,
  pub size: Size,

  pub organic_max_troops: i32,
  pub troops_growth_velocity: f32,

  /// (TODO:) uses all the surface points of the territory to calculate which troops are inside it
  pub current_troops: Vec<Troop>,

  pub current_ruler: Option<String>, // That will be player_id
}

pub enum ColorChange {
  Lighten,
  Darken,
  Exact,
}

impl Territory {
  fn continent_to_color(continent: &Continent) -> Color {
    match continent {
      Continent::Africa => Color::LIGHT_SLATE_GRAY  /* Color::WHITE_SMOKE */,
      Continent::Asia => Color::GREEN_YELLOW  /* Color::WHITE_SMOKE */,
      Continent::Europe => Color::SKY_BLUE  /* Color::WHITE_SMOKE */,
      Continent::NorthAmerica => Color::INDIAN_RED  /* Color::WHITE_SMOKE */,
      Continent::Oceania => Color::BURLYWOOD  /* Color::WHITE_SMOKE */,
      Continent::SouthAmerica => Color::TOMATO  /* Color::WHITE_SMOKE */,
      Continent::Antarctica => Color::GRAY  /* Color::WHITE_SMOKE */,
      Continent::Special => Color::GOLD  /* Color::WHITE_SMOKE */,
    }
  }

  pub fn get_territory_color(sub_continent: &Option<SubContinent>, continent: &Continent) -> Color {
    match sub_continent {
      Some(SubContinent::MiddleEast) => Color::GREEN /* Color::WHITE_SMOKE */,
      Some(SubContinent::InteriorAsia) => Color::WEB_GREEN /* Color::WHITE_SMOKE */,
      Some(SubContinent::IndianSubcontinent) => Color::LAWN_GREEN /* Color::WHITE_SMOKE */,
      Some(SubContinent::SoutheastAsia) => Color::LIME_GREEN /* Color::WHITE_SMOKE */,
      Some(SubContinent::EastAsia) => Color::GREEN_YELLOW /* Color::WHITE_SMOKE */,
      Some(SubContinent::EuropeRelatedAsia) => Color::LIGHT_GREEN /* Color::WHITE_SMOKE */,
      None => Self::continent_to_color(&continent)
    }
  }

  pub fn set_color_to_territory(mut territory_mesh: Gd<MeshInstance3D>, color_change: ColorChange) {
    // (TODO:) Calling get_map from here is temporary, 
    // should be removed after setting colors on land dinamically on game
    let binding = Self::get_map();
    let territory_id = territory_mesh.get_name().to_string();
    let territory_metadata = binding.get(&territory_id).unwrap();

    let color = Territory::get_territory_color(
      &territory_metadata.location.sub_continent,
      &territory_metadata.location.continent
    );

    let color = match color_change {
      ColorChange::Lighten => color.lightened(0.5),
      ColorChange::Darken => color.darkened(0.5),
      ColorChange::Exact => color,
    };

    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(color);
    territory_mesh.set_material_override(&material);
  }

  pub fn checking_territory(territory_mesh: Gd<MeshInstance3D>) {
    Self::set_color_to_territory(territory_mesh, ColorChange::Lighten);
  }

  pub fn unchecking_territory(territory_mesh: Gd<MeshInstance3D>) {
    Self::set_color_to_territory(territory_mesh, ColorChange::Exact);
  }

  pub fn clicking_territory(territory_mesh: Gd<MeshInstance3D>) {
    Self::set_color_to_territory(territory_mesh, ColorChange::Darken);
  }

  /// Should be called when the coordinates of the territory are set
  pub fn set_territory_size(&mut self) {
    // TODO: Instead of using size, use coordinates.len to calculate organic_max_troops and troops_growth_velocity
    let num_of_coordinates = self.coordinates.len();

    self.size = match num_of_coordinates {
      1..=40 => Size::Tiny,
      41..=150 => Size::Small,
      151..=450 => Size::Medium,
      451..=750 => Size::Large,
      751..=1000 => Size::Huge,
      1001..=9999 => Size::Humongous,
      _ => Size::None,
    };
  }
  
  /// Should be called when the coordinates of the territory are set
  pub fn set_troops_growth_velocity(&mut self) {
    let base_factor: f32 = 0.001;
    let num_of_coordinates = self.coordinates.len();

    self.troops_growth_velocity = (base_factor * num_of_coordinates as f32)
      .clamp(0.01, 3.);
  }

  /// Should be called when the coordinates of the territory are set
  pub fn set_organic_max_troops(&mut self) {
    let base_factor: f32 = 0.05;
    let num_of_coordinates = self.coordinates.len();

    self.organic_max_troops = ((base_factor * num_of_coordinates as f32) as i32)
      .clamp(1, 40);
  }
}