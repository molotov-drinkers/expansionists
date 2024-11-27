use godot::classes::StandardMaterial3D;
use godot::{classes::MeshInstance3D, prelude::*};

use super::territory::types;
use types::Territory;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GlobeScene {
  base: Base<Node3D>,
}

#[godot_api]
impl INode3D for GlobeScene {
  fn init(base: Base<Node3D>) -> GlobeScene {

    GlobeScene {
      base: base,
    }
  }

  fn ready(&mut self) {
    let globe = self.base()
      .find_child("globe")
      .expect("'globe' to exist");

    let territories_base = Territory::get_map();

    let territories = globe.get_children();
    for node_territory in territories.iter_shared() {
      let mut territory = node_territory.cast::<MeshInstance3D>();
      let territory_name = territory.get_name();

      let territory_data = territories_base
        .get(&territory_name.to_string());

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
  fn continent_to_color(continent: &types::Continent) -> Color {
    match continent {
      types::Continent::Africa => Color::DARK_ORANGE,
      types::Continent::Asia => Color::GREEN_YELLOW,
      types::Continent::Europe => Color::SKY_BLUE,
      types::Continent::NorthAmerica => Color::DARK_RED,
      types::Continent::Oceania => Color::BURLYWOOD,
      types::Continent::SouthAmerica => Color::TOMATO,
      types::Continent::Antarctica => Color::DARK_CYAN,
      types::Continent::Special => Color::GOLD,
    }
  }

  fn get_territory_color(sub_continent: &Option<types::SubContinent>, continent: &types::Continent) -> Color {
    match sub_continent {
      Some(types::SubContinent::MiddleEast) => Color::from_rgba(0., 0.3, 0., 1.),
      Some(types::SubContinent::InteriorAsia) => Color::from_rgba(0., 0.4, 0., 1.),
      Some(types::SubContinent::IndianSubcontinent) => Color::from_rgba(0., 0.5, 0., 1.),
      Some(types::SubContinent::PacificAndSoutheastAsia) => Color::from_rgba(0., 0.6, 0., 1.),
      Some(types::SubContinent::EuropeRelatedAsia) => Color::from_rgba(0., 0.7, 0., 1.),
      None => Self::continent_to_color(&continent)
    }
  }
}