// TODO: remove this when implementation is done
#![allow(dead_code)]
#![allow(unused_variables)]

use godot::{classes::{IStaticBody3D, InputEvent, InputEventMouseButton, MeshInstance3D, StaticBody3D}, global::MouseButton, prelude::*};
use crate::globe::coordinates_system::surface_point::SurfacePoint;

/// Every territory should be a MeshInstance3D with the 
/// following "Ocean StaticBody3D" as a child
/// |-territory
/// |||- ocean
/// ||||- collision_shape
#[derive(GodotClass)]
#[class(base=StaticBody3D)]
pub struct Ocean {
  base: Base<StaticBody3D>,
}

#[godot_api]
impl IStaticBody3D for Ocean {
  fn init(base: Base<StaticBody3D>) -> Ocean {
    Ocean {
      base: base,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_ray_pickable(true);
    // godot_print!("Ocean ready");
  }

  fn input_event(
      &mut self,
      _camera: Option<Gd<Camera3D>>,
      event: Option<Gd<InputEvent>>,
      event_position: Vector3,
      _normal: Vector3,
      _shape_idx: i32
    ) {
    // Self::catch_right_unclick(self, event, event_position);
  }

}

impl Ocean {
  fn catch_right_unclick(&mut self, event: Option<Gd<InputEvent>>, event_position: Vector3) {
    if let Some(event) = event {
      if let Ok(mouse_click) = event.try_cast::<InputEventMouseButton>() {

        let mouse_button = mouse_click.get_button_index();
        let pressed = mouse_click.is_pressed();
        let territory = self.base().get_parent().expect("Parent to exist").cast::<MeshInstance3D>();

        match (mouse_button, pressed) {
          (MouseButton::RIGHT, false) => {
            let surface_point = SurfacePoint::get_surface_point(
              event_position,
              self.base().get_world_3d().expect("World to exist")
            );

            if let Some(surface_point) = surface_point {
              let bind = surface_point.bind();
              let metadata = bind.get_surface_point_metadata();
              godot_print!("Clicked at Ocean: {:?}", metadata);
            } else {
              godot_error!("Err: clicked at {:?} and {:?} didn't find any surface point", territory, event_position);
            }
          }
          _ => {}
        }
      }
    }
  }

}