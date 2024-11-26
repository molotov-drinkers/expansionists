use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Territory {
  base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Territory {
  fn init(base: Base<Node3D>) -> Territory {

    Territory {
      base: base,
    }
  }
}