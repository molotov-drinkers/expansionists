use godot::prelude::*;

pub enum TypesOfTarget {
  Troop,
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Projectile {
  base: Base<Node3D>,
  showing: bool,
  
  pub trajectory: Vec<Vector3>,
  pub trajectory_is_set: bool,

  pub target: Option<TypesOfTarget>,

  pub up_to_date_target_position: Vector3,
  current_position: Vector3,
}

#[godot_api]
impl INode3D for Projectile {
  fn init(base: Base<Node3D>) -> Projectile {

    Projectile {
      base: base,
      showing: true,

      trajectory: Vec::new(),
      trajectory_is_set: false,

      target: None,

      up_to_date_target_position: Vector3::ZERO,
      current_position: Vector3::ZERO,
    }
  }

  fn ready(&mut self) {
    let showing = self.showing;
    self.base_mut().set_visible(showing);
  }

  fn process(&mut self, delta: f64) {
    self.move_towards_target(delta);
  }
}

impl Projectile {
  fn move_towards_target(&mut self, _delta: f64) {

  }
}