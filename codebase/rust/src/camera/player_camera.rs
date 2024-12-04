use godot::classes::{Camera3D, ICamera3D, InputEvent};
use godot::prelude::*;

use crate::globe::coordinates_system::coordinates_system::CoordinatesSystem;
use crate::globe::coordinates_system::surface_point::Coordinates;

pub enum CameraDirection {
  Up, // W
  Down, // S
  Left, // A
  Right, // D
  UpLeft, // W+A
  UpRight, // W+D
  DownLeft, // S+A
  DownRight, // S+D
  None, // A+D or W+S
}

#[derive(GodotClass)]
#[class(base=Camera3D)]
pub struct PlayerCamera {
  base: Base<Camera3D>,
  pub camera_speed: f64,
  pub looking_at: Coordinates,
  pub distance_from_globe_surface: f64,
}

#[godot_api]
impl ICamera3D for PlayerCamera {
  fn init(base: Base<Camera3D>) -> PlayerCamera {
    PlayerCamera {
      base,
      camera_speed: 0.1,
      looking_at: (0, 0),
      distance_from_globe_surface: 0.0,
    }
  }

  fn ready(&mut self) {
    godot_print!("PlayerCamera ready");
  }

  // todo: should the process content be here in input function?
  fn input(&mut self, _event: Gd<InputEvent>) {
    godot_print!("PlayerCamera input");
  }

  fn process(&mut self, _delta: f64) {
    let mut transform = self.base().get_global_transform();
    let translation = transform.origin;
    let mut direction = CameraDirection::None;
    let input = Input::singleton();

    // todo: implement zoom system
    // input.is_action_pressed("BUTTON_WHEEL_UP");
    // input.is_action_pressed("BUTTON_WHEEL_DOWN");

    if input.is_action_pressed("ui_up") {
      // todo: translation.z -= self.camera_speed * delta;
      direction = CameraDirection::Up;
    }
    if input.is_action_pressed("ui_down") {
      // todo: translation.z += self.camera_speed * delta;
      direction = CameraDirection::Down;
    }
    if input.is_action_pressed("ui_left") {
      // todo: translation.x -= self.camera_speed * delta;
      direction = CameraDirection::Left;
    }
    if input.is_action_pressed("ui_right") {
      // todo: translation.x += self.camera_speed * delta;
      direction = CameraDirection::Right;
    }

    CoordinatesSystem::get_geodesic_neighbour_position(
      self.looking_at,
      translation,
      direction,
    );

    transform.origin = translation;
    self.base_mut().set_global_transform(transform);
  }
}