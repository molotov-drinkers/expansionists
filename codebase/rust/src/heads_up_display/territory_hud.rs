use godot::classes::{ColorRect, Control, HBoxContainer, IControl};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::globe::territories::territory::{Territory, TerritoryId};
use crate::player::color::PlayerColor;

use super::text_labels::TextLabels;


#[derive(GodotClass)]
#[class(base=Control)]
pub struct TerritoryHUD {
  base: Base<Control>,
  current_territory: Option<TerritoryId>,
}

#[godot_api]
impl IControl for TerritoryHUD {
  fn init(base: Base<Control>) -> TerritoryHUD {

    TerritoryHUD {
      base: base,
      current_territory: None,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_visible(false);
  }

  fn process(&mut self, _delta: f64) {
    if self.current_territory.is_some() {
      let virtual_planet = self.get_virtual_planet_from_territory_hud();
      let virtual_planet = virtual_planet.bind();

      let territory_id = self.current_territory.as_ref().unwrap();
      let territory = virtual_planet.get_territory_from_virtual_planet(&territory_id);

      self.activate_territory_part(territory);
      self.activate_ruler_part(territory);
    }
  }
}

impl TerritoryHUD {
  pub fn activate_hud(&mut self, territory: &Territory) {
    self.base_mut().set_visible(true);
    self.current_territory = Some(territory.territory_id.clone());

    self.activate_ruler_part(territory);
  }

  fn activate_territory_part(&mut self, territory: &Territory) {
    let shared_path = "territory_margin_container/PanelContainer/MarginContainer/VBoxContainer/";

    let mut name = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "TextLabels")
    );
    let mut size_info = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "size_info/TextLabels")
    );
    let mut continent = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "continent/TextLabels")
    );

    let formatted_size = territory.size.to_string().to_uppercase();
    let formatted_secs_to_troop = &territory.seconds_to_spawn_troop;
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

    size_info.set_text(&format!("{formatted_size} [{formatted_secs_to_troop:.1} SECS -> {max_troops}]"));
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

    let mut ruler_label = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "HBoxContainer/TextLabels")
    );

    if territory.current_ruler.is_some() {
      // TODO: Create HUD for ruler once it has one
      occupied.set_visible(true);
      unoccupied.set_visible(false);

      let ruler = territory.current_ruler.as_ref().unwrap();
      let ruler_color = PlayerColor::get_banner_player_color(&ruler.color);
      let mut ruler_banner = occupied.get_node_as::<ColorRect>("banner");
      
      ruler_banner.set_color(ruler_color);
      ruler_label.set_text(&ruler.user_name);

      let num_of_troops = territory.all_troops_deployed_and_arrived.len();
      occupied.get_node_as::<TextLabels>("VBoxContainer/troops/TextLabels")
        .set_text(&format!("{:?}x", num_of_troops));

      return
    }

    occupied.set_visible(false);
    unoccupied.set_visible(true);
    ruler_label.set_text("Ruler");
  }

  pub fn clean_hud(&mut self) {
    self.base_mut().set_visible(false);
    self.current_territory = None;
  }

  fn get_root_from_territory_hud(&mut self) -> Gd<Node> {
    self
      .base()
      .get_parent().expect("Expected TerritoryHUD to have ui as parent")
      .get_parent().expect("Expected ui to have root as parent")
  }

  fn get_virtual_planet_from_territory_hud(&mut self) -> Gd<VirtualPlanet> {
    let virtual_planet = self
      .get_root_from_territory_hud()
      .try_get_node_as::<VirtualPlanet>("virtual_planet")
      .expect("Expected to find VirtualPlanet from RootScene");

    virtual_planet
  }
}