use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

use crate::player::troop;

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

    // TODO: Set race condition better to avoid trying to spawn troops before the planet is ready
    troop::troop_spawner(self);
  }
}
