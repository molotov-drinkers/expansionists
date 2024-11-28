use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct CoordinatesSystem {
  base: Base<Node3D>,
}

#[godot_api]
impl INode3D for CoordinatesSystem {
  fn init(base: Base<Node3D>) -> CoordinatesSystem {

    CoordinatesSystem {
      base: base,
    }
  }
}