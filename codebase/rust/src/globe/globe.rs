use godot::classes::{IStaticBody3D, StandardMaterial3D, StaticBody3D};
use godot::{classes::MeshInstance3D, prelude::*};

use super::territory::types::{Territory, Territories, Continent, SubContinent};

#[derive(GodotClass)]
#[class(base=StaticBody3D)]
pub struct GlobeScene {
  base: Base<StaticBody3D>,
  territories: Territories,
}

#[godot_api]
impl IStaticBody3D for GlobeScene {
  fn init(base: Base<StaticBody3D>) -> GlobeScene {

    GlobeScene {
      base: base,
      territories: Territory::get_map(),
    }
  }

  fn ready(&mut self) {
    let globe = self.base()
      .find_child("globe")
      .expect("'globe' to exist");

    let territories = globe.get_children();
    for node_territory in territories.iter_shared() {
      let mut territory = node_territory.cast::<MeshInstance3D>();
      let territory_name = territory.get_name();

      let territory_data = self.territories.get(&territory_name.to_string());
      if territory_data.is_none() {
        godot_print!("No data for territory: {:?}", territory_name);
        continue;
      }

      let color = Self::get_territory_color(
        &territory_data.unwrap().location.sub_continent,
        &territory_data.unwrap().location.continent
      );

      let mut material = StandardMaterial3D::new_gd();
      material.set_albedo(color);
      territory.set_material_override(&material);
    }
  }
}

impl GlobeScene {
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

  fn get_territory_color(sub_continent: &Option<SubContinent>, continent: &Continent) -> Color {
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