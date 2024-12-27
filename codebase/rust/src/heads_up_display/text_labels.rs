use godot::classes::{Font, FontFile, ILabel, Label};
use godot::prelude::*;

use crate::troops::spawner_engine;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct TextLabels {
  base: Base<Label>,
}

#[godot_api]
impl ILabel for TextLabels {
  fn init(base: Base<Label>) -> TextLabels {

    TextLabels {
      base: base,
    }
  }

  fn ready(&mut self) {
    // let new_font = FontFile::
    // self.base_mut().add_theme_font_override(name, font);

    // self.base_mut().font
  }
    
}
