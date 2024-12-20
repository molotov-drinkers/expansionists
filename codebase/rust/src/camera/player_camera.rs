use godot::classes::{Camera3D, ICamera3D, InputEvent, InputEventMouseButton};
use godot::global::MouseButton;
use godot::prelude::*;

const MAX_HEIGHT_ANGLE: f32 = 89.9;
const MIN_HEIGHT_ANGLE: f32 = -89.9;

const MIN_DISTANCE_TO_ORIGIN: f32 = 3.5;

/// That's the distance from the origin to the farthest point in the globe
/// If greater, we will have problems to catch the mouse_enter on land.rs
const MAX_DISTANCE_TO_ORIGIN: f32 = 5.95;

#[derive(GodotClass)]
#[class(base=Camera3D)]
pub struct PlayerCamera {
  base: Base<Camera3D>,
  pub camera_speed: f64,
  pub zoom_speed: f64,
  pub theta: f32,
  pub phi: f32,
}

#[godot_api]
impl ICamera3D for PlayerCamera {
  fn init(base: Base<Camera3D>) -> PlayerCamera {
    PlayerCamera {
      base,
      // TICKET: #44
      camera_speed: 4.,
      zoom_speed: 0.1,
      theta: 0.0,
      phi: 0.0,
    }
  }

  fn process(&mut self, delta: f64) {
    let input = Input::singleton();

    let (radius, vector_to_origin) = self.get_data_to_move_camera();

    if input.is_action_pressed("camera_up") {
      self.phi = (self.phi + self.camera_speed as f32 * delta as f32).clamp(
        MIN_HEIGHT_ANGLE.to_radians(), 
        MAX_HEIGHT_ANGLE.to_radians()
      );
    }
    if input.is_action_pressed("camera_down") {
      self.phi = (self.phi - self.camera_speed as f32 * delta as f32).clamp(
        MIN_HEIGHT_ANGLE.to_radians(), 
        MAX_HEIGHT_ANGLE.to_radians()
      );
    }
    if input.is_action_pressed("camera_left") {
      self.theta += (self.camera_speed * delta) as f32;
    }
    if input.is_action_pressed("camera_right") {
      self.theta -= (self.camera_speed * delta) as f32;
    }

    self.set_new_position(radius, vector_to_origin);
  }

  fn input(&mut self, event: Gd<InputEvent>) {
    if let Ok(mouse_click) = event.try_cast::<InputEventMouseButton>() {
      let mouse_button = mouse_click.get_button_index();

      let (mut radius, vector_to_origin) = self.get_data_to_move_camera();

      match mouse_button {
        MouseButton::WHEEL_DOWN => {
          radius += self.zoom_speed as f32
        },
        MouseButton::WHEEL_UP => {
          radius -= self.zoom_speed as f32
        },
        _ => {}
      }

      self.set_new_position(radius, vector_to_origin);
    }
  }
}

impl PlayerCamera {
  fn get_data_to_move_camera(&mut self) -> (f32, Vector3) {
    let cam_location = self.base().get_global_position();
    let world_origin = Vector3::new(0.0, 0.0, 0.0);
    let vector_to_origin = (world_origin - cam_location).normalized();
    let radius = cam_location.distance_to(world_origin);

    (radius, vector_to_origin)
  }

  fn set_new_position(&mut self, mut radius: f32, vector_to_origin: Vector3) {
    let mut transform = self.base().get_global_transform();

    radius = radius.clamp(MIN_DISTANCE_TO_ORIGIN, MAX_DISTANCE_TO_ORIGIN);
    transform.origin = Vector3::new(
      radius * self.phi.cos() * self.theta.cos(),
      radius * self.phi.sin(),
      radius * self.phi.cos() * self.theta.sin(),
    );

    self.base_mut().set_global_transform(transform);
    self.base_mut().look_at(vector_to_origin);
  }
}