use godot::classes::{INode3D, StandardMaterial3D, Node3D};
use godot::{classes::MeshInstance3D, prelude::*};

use super::territory::types::{Territory, Territories, Continent, SubContinent};

use crate::player;
use player::troop::Troop;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GlobeScene {
  base: Base<Node3D>,
  territories: Territories,
}

#[godot_api]
impl INode3D for GlobeScene {
  fn init(base: Base<Node3D>) -> GlobeScene {

    GlobeScene {
      base: base,
      territories: Territory::get_map(),
    }
  }

  fn ready(&mut self) {
    let territories_node = self.base()
      .find_child("territories")
      .expect("'territories' to exist");

    let territories = territories_node.get_children();
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


      let scene: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
      let mut new_troop = scene.instantiate_as::<Troop>();
      new_troop.set_name(&"troop".to_godot());
      self.base_mut().add_child(&new_troop);

      new_troop.set_position(Vector3::new(1.2, 0., 0.));

      let troop_node = new_troop.find_child("MeshInstance3D").expect("MeshInstance3D to exist");
      let mut troop_mesh = troop_node.cast::<MeshInstance3D>();
      troop_mesh.set_surface_override_material(0, &material);

    }
  }

  fn physics_process(&mut self, _delta: f64) {

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