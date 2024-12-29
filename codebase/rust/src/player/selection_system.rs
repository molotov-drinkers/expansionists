// TODO: Remove this line once the file is implemented
#![allow(dead_code)]

use godot::{classes::{INinePatchRect, InputEvent, InputEventMagnifyGesture, InputEventMouseButton, NinePatchRect}, global::MouseButton, prelude::*};

use crate::troops::troop::Troop;


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SelectionSystem {
  dragging: bool,
}

#[godot_api]
impl INode3D for SelectionSystem {
  fn init(_base: Base<Node3D>) -> Self {
    SelectionSystem {
      dragging: false,
    }
  }

  fn ready(&mut self) {
  }
}


#[derive(GodotClass)]
#[class(base=NinePatchRect)]
pub struct UiDragBox {
  base: Base<NinePatchRect>,
  dragging: bool,
  in_rect_troops: Vec<Troop>,
  start_pos: Vector2,
}

#[godot_api]
impl INinePatchRect for UiDragBox {
  fn init(base: Base<NinePatchRect>) -> Self {
    UiDragBox {
      base: base,
      dragging: false,
      in_rect_troops: Vec::new(),
      start_pos: Vector2::ZERO,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_visible(false);
  }

  fn input(&mut self, event: Gd<InputEvent>) {
    if let Ok(mouse_click) = event.clone().try_cast::<InputEventMouseButton>() {
      let mouse_button = mouse_click.get_button_index();
      let pressed = mouse_click.is_pressed();
      let clicked_at = mouse_click.get_position();

      match (mouse_button, pressed) {
        (MouseButton::LEFT, true) => {
          self.dragging = true;
          self.start_pos = clicked_at;
          // self.base_mut().set_visible(true);
          self.base_mut().set_position(clicked_at);
        },
        (MouseButton::LEFT, false) => {
          self.dragging = false;
          self.base_mut().set_visible(false);
          self.cast_troop_selection()
        },
        _ => {}
      }
    }

    // handling mouse Pad Pinch events
    if let Ok(_magnify_gesture) = event.try_cast::<InputEventMagnifyGesture>() {
      todo!()
    }
  }

  fn process(&mut self, _delta: f64) {
    if self.dragging {
      let mouse_pos = self.base_mut().get_global_mouse_position();
      let size = mouse_pos - self.start_pos;

      // NinePatchRect doesn't take negative size, so we need to flip the scale
      // if size is negative if needed
      let positive_x = size.x > 0.;
      let positive_y = size.y > 0.;
      match (positive_x, positive_y) {
        (false, true) => self.base_mut().set_scale(Vector2::new(-1., 1.)),
        (true, false) => self.base_mut().set_scale(Vector2::new(1., -1.)),
        (false, false) => self.base_mut().set_scale(Vector2::new(-1., -1.)),
        _ => self.base_mut().set_scale(Vector2::new(1., 1.)),
      }

      // The size of the drag box is the absolute value of the size
      self.base_mut().set_size(size.abs());


      if self.base_mut().get_size().length_squared() > Self::MIN_DRAG_SQUARE {
        self.base_mut().set_visible(true);
      }
    }
  }

}

impl UiDragBox {
  const MIN_DRAG_SQUARE: f32 = 164.;

  fn cast_troop_selection(&mut self) {

  }
}

// TODO: Should check if troop is visible on camera before adding it to in_rect_troops
