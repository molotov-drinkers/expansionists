// TODO: Remove this line once the file is implemented
#![allow(dead_code)]

use godot::{classes::RectangleShape2D, prelude::*};


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SelectionSystem {
  selectable_area: Gd<RectangleShape2D>,
  dragging: bool,
}

#[godot_api]
impl INode3D for SelectionSystem {
  fn init(_base: Base<Node3D>) -> Self {
    SelectionSystem {
      selectable_area: RectangleShape2D::new_gd(),
      dragging: false,
    }
  }
}