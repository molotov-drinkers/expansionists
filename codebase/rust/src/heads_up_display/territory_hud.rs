use godot::classes::{ILabel, Label};
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=Label)]
pub struct TerritoryHUD {
  base: Base<Label>,
  showing_text: String,
}

#[godot_api]
impl ILabel for TerritoryHUD {
  fn init(base: Base<Label>) -> TerritoryHUD {

    TerritoryHUD {
      base: base,
      showing_text: "".to_string(),
    }
  }

  fn process(&mut self, _delta: f64) {
    let text = self.showing_text
      .clone()
      .to_uppercase()
      .replace("_", " ");

    self.base_mut().set_text(&text);
  }
}

impl TerritoryHUD {
  pub fn set_text(&mut self, text: String) {
    self.showing_text = text;
  }

  pub fn clean_hud(&mut self) {
    self.showing_text = "".to_string();
  }
}