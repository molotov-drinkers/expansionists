use godot::classes::{ILabel, Label};
use godot::prelude::*;

use crate::globe::territories::territory::Territory;


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
  pub fn set_text(&mut self, territory: &Territory) {
    self.showing_text = format!("{}\n{} ({})",
      territory.territory_id.clone(),
      territory.size.to_string(),
      territory.coordinates.len()
   );
  }

  pub fn clean_hud(&mut self) {
    self.showing_text = "".to_string();
  }
}