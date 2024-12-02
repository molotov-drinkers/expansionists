use godot::classes::{INode3D, Node3D};
use godot::prelude::*;


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
}