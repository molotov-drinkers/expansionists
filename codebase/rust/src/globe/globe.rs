use godot::classes::{CollisionShape3D, INode3D, Node3D, StandardMaterial3D};
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

      let color = Territory::get_territory_color(
        &territory_data.unwrap().location.sub_continent,
        &territory_data.unwrap().location.continent
      );

      let mut material = StandardMaterial3D::new_gd();
      material.set_albedo(color);
      territory.set_material_override(&material);


      let collision_shape_territory = territory
        .find_child("StaticBody3D")
        .expect("StaticBody3D to exist")
        .find_child("CollisionShape3D")
        .expect("CollisionShape3D to exist");

      let collision_shape_territory = collision_shape_territory.cast::<CollisionShape3D>();

      let territory_position = collision_shape_territory.get_global_transform();
      // godot_print!("Territory: {:?}, position: {:?}", territory_name, territory_position);


      // Troop Spawner Engine:
      // let scene: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
      // let mut new_troop = scene.instantiate_as::<Troop>();
      // new_troop.set_name(&"troop".to_godot());
      // self.base_mut().add_child(&new_troop);

      // new_troop.set_position(Vector3::new(1.2, 0., 0.));

      // let troop_node = new_troop.find_child("MeshInstance3D").expect("MeshInstance3D to exist");
      // let mut troop_mesh = troop_node.cast::<MeshInstance3D>();
      // troop_mesh.set_surface_override_material(0, &material);

    }
  }

  fn physics_process(&mut self, _delta: f64) {

  }
}
