use godot::classes::text_server::AutowrapMode;
use godot::classes::{FontFile, ILabel, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct TextLabels {
  base: Base<Label>,
  font: Gd<FontFile>
}

#[godot_api]
impl ILabel for TextLabels {
  fn init(base: Base<Label>) -> TextLabels {
    let mut quantico_bold = FontFile::new_gd();
    quantico_bold.load_dynamic_font("res://assets/font/Quantico-Bold.ttf");
    quantico_bold.set_fixed_size(24);

    TextLabels {
      base: base,
      font: quantico_bold
    }
  }

  fn ready(&mut self) {
    let font_clone = self.font.clone();
    self.base_mut().add_theme_font_override("font", &font_clone);
  }
}

impl TextLabels {
  pub fn set_font_size(&mut self, size: i32) {
    self.font.set_fixed_size(size);
    let font_clone = self.font.clone();
    self.base_mut().add_theme_font_override("font", &font_clone);
    self.base_mut().set_autowrap_mode(AutowrapMode::WORD_SMART);
  }
}
