use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::player::troop;
use rand::Rng;


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RootScene {
  base: Base<Node3D>,
}

#[godot_api]
impl INode3D for RootScene {
  fn init(base: Base<Node3D>) -> RootScene {

    RootScene {
      base: base,
    }
  }

  fn physics_process(&mut self, _delta: f64) {
    Self::troop_spawner(self);
  }
}

impl RootScene {
  pub fn troop_spawner(&mut self) {
    
    let virtual_planet = self.base_mut()
      .get_node_as::<VirtualPlanet>("virtual_planet");
    let virtual_planet = &virtual_planet.bind();
    let territories = &virtual_planet.territories;

    let caatinga = territories.get("caatinga").unwrap();

    let caatinga_territory_number = caatinga.coordinates.len();

    // get a random coordinate from the territory
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..caatinga_territory_number);

    let coordinate = caatinga.coordinates[random_index];

    let coordinate_metadata = &virtual_planet.coordinate_map
      .get(&coordinate)
      .expect("Coordinate expected to exist");

    let cartesian = coordinate_metadata.cartesian;

    godot_print!("Coordinate: {:?}", coordinate);
    godot_print!("Cartesian: {:?}", cartesian);


    // TODO: get troop from Troop Scene
  }
}