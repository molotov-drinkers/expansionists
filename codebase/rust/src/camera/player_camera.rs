use godot::classes::{Camera3D, ICamera3D, InputEvent};
use godot::prelude::*;

use crate::globe::coordinates_system::coordinates_system::{CoordinateMap, CoordinatesSystem};
use crate::globe::coordinates_system::surface_point::Coordinates;

#[derive(PartialEq, Clone, Copy)]
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

impl CameraDirection {
  /// Returns the corresponding movement vector for the given direction.
  pub fn to_vector_x(&self) -> Option<Vector3> {
    match *self {      
      CameraDirection::Up => Some(Vector3::new(0.0, 1.0, 0.0)),
      CameraDirection::Right => Some(Vector3::new(1.0, 0.0, 0.0)),
      CameraDirection::Down => Some(Vector3::new(0.0, -1.0, 0.0)),
      CameraDirection::Left => Some(Vector3::new(-1.0, 0.0, 0.0)),

      CameraDirection::UpRight => Some(Vector3::new(1.0, 1.0, 0.).normalized()),
      CameraDirection::DownRight => Some(Vector3::new(1.0, -1.0, 0.).normalized()),
      CameraDirection::DownLeft => Some(Vector3::new(-1.0, -1.0, 0.).normalized()),
      CameraDirection::UpLeft => Some(Vector3::new(-1.0, 1.0, 0.).normalized()),

      CameraDirection::None => None,
    }
  }


  // pub fn to_vector_w_z(&self) -> Option<Vector3> {
  //   match *self {      
  //     CameraDirection::Up => Some(Vector3::new(0.0, 1.0, 0.0)),
  //     CameraDirection::Right => Some(Vector3::new(0.0, 0.0, -1.0)),
  //     CameraDirection::Down => Some(Vector3::new(0.0, -1.0, 0.0)),
  //     CameraDirection::Left => Some(Vector3::new(0.0, 0.0, -1.0)),

  //     CameraDirection::UpRight => Some(Vector3::new(0.0, 1.0, 1.).normalized()),
  //     CameraDirection::DownRight => Some(Vector3::new(0.0, -1.0, 1.).normalized()),
  //     CameraDirection::DownLeft => Some(Vector3::new(0.0, -1.0, -1.).normalized()),
  //     CameraDirection::UpLeft => Some(Vector3::new(0.0, 1.0, -1.).normalized()),

  //     CameraDirection::None => None,
  //   }
  // }
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
      camera_speed: 10.,
      looking_at: (0, 0),
      distance_from_globe_surface: 0.0,
    }
  }

  fn process(&mut self, delta: f64) {
    let mut transform = self.base().get_global_transform();
    let mut directions: Vec<CameraDirection> = vec![];
    let input = Input::singleton();


    let cam_location = self.base().get_global_position();
    let world_origin = Vector3::new(0.0, 0.0, 0.0);
    let radius = cam_location.distance_to(world_origin);
    let vector_to_origin = (world_origin - cam_location).normalized();
    let antipode = CoordinatesSystem::get_antipode(cam_location);

    if input.is_action_pressed("camera_up") {
      directions.push(CameraDirection::Up);
    }
    if input.is_action_pressed("camera_down") {
      directions.push(CameraDirection::Down);
    }
    if input.is_action_pressed("camera_left") {
      directions.push(CameraDirection::Left);
    }
    if input.is_action_pressed("camera_right") {
      directions.push(CameraDirection::Right);
    }

    if directions.len() > 2 { directions = vec![CameraDirection::None]; }
    let filtered_direction: CameraDirection;
    filtered_direction = match directions.as_slice() {
      [single_direction] => *single_direction,
      directions if directions.contains(&CameraDirection::Up) && directions.contains(&CameraDirection::Right) => CameraDirection::UpRight,
      directions if directions.contains(&CameraDirection::Up) && directions.contains(&CameraDirection::Left) => CameraDirection::UpLeft,
      directions if directions.contains(&CameraDirection::Down) && directions.contains(&CameraDirection::Right) => CameraDirection::DownRight,
      directions if directions.contains(&CameraDirection::Down) && directions.contains(&CameraDirection::Left) => CameraDirection::DownLeft,
      _ => CameraDirection::None,
    };

    // Get the direction as a Vector3 from the CameraDirection enum
    let mut direction_vector = filtered_direction.to_vector_x();

    // if self.base_mut().get_global_rotation_degrees().y < -45. || self.base_mut().get_global_rotation_degrees().y > 80. {
    //   direction_vector = filtered_direction.to_vector_w_z();
    // }
    if direction_vector.is_none() { return }

    let new_position = CoordinatesSystem::get_dynamic_geodesic_trajectory(
      cam_location,
      direction_vector.unwrap(),
      antipode,
      radius,
    );

    // let new_position = CoordinatesSystem::get_geodesic_trajectory_b(
    //   cam_location,
    //   antipode,
    //   direction_vector.unwrap(),
    //   radius,
    // );

    transform.origin = new_position;


    self.base_mut().set_global_transform(transform);
    self.base_mut().look_at(vector_to_origin);


    // TODO: Set proper camera up_vector to not flip the camera and lose WASD controls
    // let forward = vector_to_origin.normalized();
    // let up_vector = Vector3::new(0.0, 1.0, 0.0);
    // let up = up_vector.normalized();
    // let right = up.cross(forward).normalized();
    // let basis = Basis::from_cols(right, up, -forward);
    // self.base_mut().set_basis(basis);

    // self.base_mut().set_global_transform(Transform3D {
    //     origin: new_position,
    //     basis,
    //   }
    // );




    // godot_print_rich!("|||========================");
    // godot_print!("new_position: {:?}", new_position);
    // // godot_print!("get_global_rotation: {:?}",  self.base_mut().get_global_rotation());
    // godot_print!("get_global_rotation_degrees: {:?}",  self.base_mut().get_global_rotation_degrees());
    // godot_print_rich!("========================|||");


    // todo: implement zoom system
    // input.is_action_pressed("zoom_in");
    // input.is_action_pressed("zoom_out");

  }
}