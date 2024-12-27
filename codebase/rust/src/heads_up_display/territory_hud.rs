use godot::classes::{Control, HBoxContainer, IControl};
use godot::prelude::*;

use crate::globe::territories::territory::Territory;

use super::text_labels::TextLabels;


#[derive(GodotClass)]
#[class(base=Control)]
pub struct TerritoryHUD {
  base: Base<Control>,
}

#[godot_api]
impl IControl for TerritoryHUD {
  fn init(base: Base<Control>) -> TerritoryHUD {

    TerritoryHUD {
      base: base,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_visible(false);
  }
}

impl TerritoryHUD {
  pub fn activate_hud(&mut self, territory: &Territory) {
    self.base_mut().set_visible(true);
    
    self.activate_territory_part(territory);
    self.activate_ruler_part(territory);
  }

  fn activate_territory_part(&mut self, territory: &Territory) {
    let shared_path = "territory_margin_container/PanelContainer/MarginContainer/VBoxContainer/";

    let mut name = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "Label")
    );
    let mut size_info = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "size_info/Label")
    );
    let mut continent = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "continent/Label")
    );

    let formatted_size = territory.size.to_string().to_uppercase();
    let formatted_growth = &territory.troops_growth_velocity;
    let max_troops = &territory.organic_max_troops;

    let formatted_continent = territory.location.continent.to_string().to_uppercase().replace("_", " ");
    let formatted_sub_continent = if let Some(sub_continent) = &territory.location.sub_continent {
      let sub = sub_continent.to_string().to_uppercase().replace("_", " ");
      format!(" - {sub}")
    } else {
      "".to_string()
    };
    
    name.set_text(&territory.territory_id.clone().to_uppercase().replace("_", " "));
    name.bind_mut().set_font_size(32);

    size_info.set_text(&format!("{formatted_size} [{formatted_growth:.2} -> {max_troops}]"));
    continent.set_text(&format!("{formatted_continent}{formatted_sub_continent}"));
  }

  fn activate_ruler_part(&mut self, territory: &Territory) {
    let shared_path = "ruler_margin_container/PanelContainer/MarginContainer/VBoxContainer/";
    let mut occupied = self.base().get_node_as::<HBoxContainer>(
      &(shared_path.to_owned() + "occupied")
    );

    let mut unoccupied = self.base().get_node_as::<HBoxContainer>(
      &(shared_path.to_owned() + "unoccupied")
    );
    if territory.current_ruler.is_some() {
      // TODO: Create HUD for ruler once it has one
    }
    occupied.set_visible(false);
    unoccupied.set_visible(true);
  }

  pub fn clean_hud(&mut self) {
    self.base_mut().set_visible(false);
  }
}