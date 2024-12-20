use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::troops::spawner_engine;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RootScene {
  base: Base<Node3D>,
  troops_spawn: i8
}

#[godot_api]
impl INode3D for RootScene {
  fn init(base: Base<Node3D>) -> RootScene {

    RootScene {
      base: base,
      troops_spawn: 0
    }
  }

  fn process(&mut self, _delta: f64) {
    let virtual_planet = self.base()
      .find_child("virtual_planet")
      .expect("Expected to find virtual_planet")
      .cast::<VirtualPlanet>();
    let virtual_planet = virtual_planet.bind();

    if virtual_planet.are_surface_points_matched && self.base().is_node_ready() {
      self.startup_troops_spawn(&virtual_planet);
    }
  }
}

const ORIGIN: &str = "parallel_49th";
const ORIGIN_B: &str = "diomede_islands";

impl RootScene {
  pub fn startup_troops_spawn(&mut self, virtual_planet: &VirtualPlanet) {
    let max_troops = 10;
    while self.troops_spawn < max_troops {
      spawner_engine::troop_spawner(
        self,
        &virtual_planet,
        self.troops_spawn,
        ORIGIN.to_owned(),
      );

      spawner_engine::troop_spawner(
        self,
        &virtual_planet,
        self.troops_spawn,
        ORIGIN_B.to_owned(),
      );
      self.troops_spawn+=1;
    }
  }
}
