use godot::classes::{INode3D, Node3D, StandardMaterial3D};
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

      let color = Territory::get_territory_color(
        &territory_data.unwrap().location.sub_continent,
        &territory_data.unwrap().location.continent
      );

      let mut material = StandardMaterial3D::new_gd();
      material.set_albedo(color);
      territory.set_material_override(&material);
    }
  }
}
