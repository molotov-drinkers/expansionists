
use godot::{classes::{IStaticBody3D, InputEvent, InputEventMouseButton, MeshInstance3D, StaticBody3D}, global::MouseButton, prelude::*};
use crate::globe::territories::territory::Territory;

/// Every territory should be a MeshInstance3D with the 
/// following "Land StaticBody3D" as a child
#[derive(GodotClass)]
#[class(base=StaticBody3D)]
pub struct Land {
  base: Base<StaticBody3D>,
}

#[godot_api]
impl IStaticBody3D for Land {
  fn init(base: Base<StaticBody3D>) -> Land {
    Land {
      base: base,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_ray_pickable(true);
  }

  fn input_event(
      &mut self,
      _camera: Option<Gd<Camera3D>>,
      event: Option<Gd<InputEvent>>,
      _event_position: Vector3,
      _normal: Vector3,
      _shape_idx: i32
    ) {
    Self::catch_left_click(self, event);
  }

  fn mouse_enter(&mut self) {
    let territory = self.base()
      .get_parent()
      .expect("Parent to exist")
      .cast::<MeshInstance3D>();

    Territory::checking_territory(territory);
  }

  fn mouse_exit(&mut self) {
    let territory = self.base()
      .get_parent()
      .expect("Parent to exist")
      .cast::<MeshInstance3D>();
    Territory::unchecking_territory(territory);
  }
}

impl Land {
  fn catch_left_click(&mut self, event: Option<Gd<InputEvent>>,) {
    if let Some(event) = event {
      if let Ok(mouse_click) = event.try_cast::<InputEventMouseButton>() {
        let mouse_button = mouse_click.get_button_index();
        let pressed = mouse_click.is_pressed();
        let territory = self.base().get_parent().expect("Parent to exist").cast::<MeshInstance3D>();

        match (mouse_button, pressed) {
          (MouseButton::LEFT, true) => Territory::clicking_territory(territory),
          (MouseButton::LEFT, false) => Territory::checking_territory(territory),
          _ => {}
        }
      }
    }
  }
}