use godot::classes::{Camera3D, ICamera3D};
use godot::prelude::*;

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
      camera_speed: 5.,
      zoom_speed: 5.,
      theta: 0.0,
      phi: 0.0,
    }
  }

  fn process(&mut self, delta: f64) {
    let mut transform = self.base().get_global_transform();
    let input = Input::singleton();

    let cam_location = self.base().get_global_position();
    let world_origin = Vector3::new(0.0, 0.0, 0.0);
    let mut radius = cam_location.distance_to(world_origin);
    let vector_to_origin = (world_origin - cam_location).normalized();


    let max_height_angle = 89.9_f32.to_radians();
    let min_height_angle = -89.9_f32.to_radians();

    if input.is_action_pressed("camera_up") {
      self.phi = (self.phi + self.camera_speed as f32 * delta as f32).clamp(min_height_angle, max_height_angle); // Clamp between -90° and 90°
    }
    if input.is_action_pressed("camera_down") {
      self.phi = (self.phi - self.camera_speed as f32 * delta as f32).clamp(min_height_angle, max_height_angle);
    }
    if input.is_action_pressed("camera_left") {
      self.theta += self.camera_speed as f32 * delta as f32;
    }
    if input.is_action_pressed("camera_right") {
      self.theta -= self.camera_speed as f32 * delta as f32;
    }

    // Adjust radius (Zoom In/Out)
    if input.is_action_pressed("zoom_in") {
      radius += self.zoom_speed as f32 * delta as f32;
    }

    if input.is_action_pressed("zoom_out") {
      radius = (radius - self.zoom_speed as f32 * delta as f32).max(1.5); // Minimum radius is 1.5
    }

    //TODO: add reset camera position, like pressing spacebar and it goes back to the original position

    let new_x = radius * self.phi.cos() * self.theta.cos();
    let new_y = radius * self.phi.sin();
    let new_z = radius * self.phi.cos() * self.theta.sin();
    transform.origin = Vector3::new(new_x, new_y, new_z);

    self.base_mut().set_global_transform(transform);
    self.base_mut().look_at(vector_to_origin);
  }
}