use godot::classes::{IControl, Control};
use godot::prelude::*;

use super::text_labels::TextLabels;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SelectionHUD {
  base: Base<Control>,
}

#[godot_api]
impl IControl for SelectionHUD {
  fn init(base: Base<Control>) -> SelectionHUD {

    SelectionHUD {
      base: base,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_visible(false);
  }
}

impl SelectionHUD {
  pub fn activate_hud(&mut self) {
    self.base_mut().set_visible(true);
  }

  pub fn set_text_with_num_of_troops(&mut self, num_of_troops: usize) {
    let path: &str = "MarginContainer/PanelContainer/MarginContainer/HBoxContainer/TextLabels";
    let mut text_label = self
      .base_mut()
      .get_node_as::<TextLabels>(path);
    text_label.set_text(
      &format!(" {num_of_troops}x Troops Selected")
    );
  }

  pub fn deactivate_hud(&mut self) {
    self.base_mut().set_visible(false);
  }
}
