use godot::prelude::*;

// enum TypesOfTarget {
//   Troop,
// }

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Projectile {
  base: Base<Node3D>,
  showing: bool,
  // up_to_date_target_position: Vector3,
  // current_position: Vector3,
}

#[godot_api]
impl INode3D for Projectile {
  fn init(base: Base<Node3D>) -> Projectile {

    Projectile {
      base: base,
      showing: true,
      // up_to_date_target_position: Vector3::ZERO,
      // current_position: Vector3::ZERO,
    }
  }

  fn ready(&mut self) {
    let showing = self.showing;
    self.base_mut().set_visible(showing);
  }
}