use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Projectile {
  base: Base<Node3D>,
  showing: bool,
  target: Vector3,
}

#[godot_api]
impl INode3D for Projectile {
  fn init(base: Base<Node3D>) -> Projectile {

    Projectile {
      base: base,
      showing: true,
      target: Vector3::ZERO,
    }
  }

  fn ready(&mut self) {
    let showing = self.showing;
    self.base_mut().set_visible(showing);
  }
}