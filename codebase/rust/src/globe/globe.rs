use godot::classes::{CollisionShape3D, INode3D, Node3D, StandardMaterial3D};
use godot::{classes::MeshInstance3D, prelude::*};

use super::territories::territory::{Territory, Territories};

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

      let mut color = Territory::get_territory_color(
        &territory_data.unwrap().location.sub_continent,
        &territory_data.unwrap().location.continent
      );

      // TODO: Remove all this commented code
      // if territory_name == "atlantic_forest".into() || territory_name == "latin_variations".into() {
      //   color = Color::WHITE_SMOKE;
      // }

      // if territory_name == "pampas".into() || territory_name == "the_islands".into() {
      //   color = Color::LIGHT_PINK;
      // }

      // if territory_name == "tropical_highlands".into() {
      //   color = Color::LIGHT_YELLOW;
      // }

      // if territory_name == "caatinga".into() || territory_name == "west_slavs".into() {
      //   color = Color::LIGHT_SALMON;
      // }
      
      // if territory_name == "amazon".into() || territory_name == "rhine_region".into() {
      //   color = Color::LIGHT_CYAN;
      // }

      // if territory_name == "andes".into() || territory_name == "nordics".into() {
      //   color = Color::LIGHT_SLATE_GRAY;
      // }

      // if territory_name == "incas".into() || territory_name == "balkan_peninsula".into() {
      //   color = Color::LIGHT_CYAN;
      // }

      // if territory_name == "patagonia".into() || territory_name == "north_black_sea".into()  {
      //   color = Color::LIGHT_STEEL_BLUE;
      // }

      if territory_name == "patagonia".into() || territory_name == "antartica_peninsula".into() {
        color = Color::LIGHT_CYAN;
      }

      let mut material = StandardMaterial3D::new_gd();
      material.set_albedo(color);
      territory.set_material_override(&material);


      let collision_shape_territory = territory
        .find_child("StaticBody3D")
        .expect("StaticBody3D to exist")
        .find_child("CollisionShape3D")
        .expect("CollisionShape3D to exist");

      let collision_shape_territory = collision_shape_territory.cast::<CollisionShape3D>();

      let _territory_position = collision_shape_territory.get_global_transform();
      // godot_print!("Territory: {:?}, position: {:?}", territory_name, territory_position);


    }
  }

  fn physics_process(&mut self, _delta: f64) {

  }
}
