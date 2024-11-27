use godot::{classes::{IRigidBody3D, RigidBody3D}, prelude::*};

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct Troop {
  base: Base<RigidBody3D>,
  pub territory: String,
  pub owner: String,
  pub count: i32,

  pub damage: i32,
  pub hp: i32,
  pub speed: i32,
  pub alive: bool,
}

#[godot_api]
impl IRigidBody3D for Troop {
  fn init(base: Base<RigidBody3D>) -> Troop {
    Troop {
      base: base,
      territory: "".to_string(),
      owner: "".to_string(),
      count: 0,

      hp: 100,
      damage: 5,
      speed: 20,
      alive: true,
    }
  }

  fn ready(&mut self) {
    godot_print!("Troop ready");
    // TODO: how to attach a troop to a territory?
  }
}