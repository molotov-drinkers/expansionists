use godot::classes::{ColorRect, Control, HBoxContainer, IControl};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::globe::territories::territory::{Territory, TerritoryId, TerritoryState};
use crate::i18n::base::{AvailableLanguage, I18nDefaultDictionary};
use crate::player::color::PlayerColor;
use crate::player::player::Player;
use crate::root::root::RootScene;

use super::text_labels::TextLabels;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TerritoryHUD {
  base: Base<Control>,
  current_territory: Option<TerritoryId>,
  chosen_language: AvailableLanguage,
  language_set: bool,
}

#[godot_api]
impl IControl for TerritoryHUD {
  fn init(base: Base<Control>) -> TerritoryHUD {

    TerritoryHUD {
      base: base,
      current_territory: None,
      chosen_language: AvailableLanguage::InternationalEnglish,
      language_set: false,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_visible(false);
  }

  fn process(&mut self, _delta: f64) {
    if self.current_territory.is_some() {   
      // TODO: remove hardcoded player id
      if !self.language_set {
        const MAIN_PLAYER_ID: i32 = 1;
        self.chosen_language = Player::get_player_language(
          self.get_root_from_territory_hud(),
          MAIN_PLAYER_ID
        );
        self.language_set = true;
      }

      let virtual_planet = self.get_virtual_planet_from_territory_hud();
      let virtual_planet = virtual_planet.bind();

      let territory_id = self.current_territory.as_ref().unwrap();
      let territory = virtual_planet.get_territory_from_virtual_planet(&territory_id);

      self.activate_ruler_part(territory);
    }
  }
}

impl TerritoryHUD {
  pub fn activate_hud(&mut self, territory: &Territory) {
    self.base_mut().set_visible(true);
    self.current_territory = Some(territory.territory_id.clone());

    self.activate_territory_part(territory);
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

    let formatted_secs_to_troop = &territory.seconds_to_spawn_troop;
    let formatted_secs_to_troop = format!("{:.0}", formatted_secs_to_troop);
    let max_troops = &territory.organic_max_troops;

    let base_dictionaries = self.chosen_language.get_translations();
    let territories_dictionary = &base_dictionaries.get_territory_dictionary();
    let translated_territory = territories_dictionary
      .get(&territory.territory_id as &str)
      .expect("Expected to find territory_id in dictionary");
    
    let sizes_dictionary = &base_dictionaries.get_sizes();
    let translated_size = sizes_dictionary
      .get(&territory.size)
      .expect("Expected to find size in dictionary");

    let continents_dictionary = &base_dictionaries.get_continents();
    let translated_continent = continents_dictionary
      .get(&territory.location.continent)
      .expect("Expected to find continent in dictionary");

    let sub_continents_dictionary = &base_dictionaries.get_sub_continents();
    let translated_sub_continent = if let Some(sub_continent) = &territory.location.sub_continent {
      let sub_continent = sub_continents_dictionary
        .get(sub_continent)
        .expect("Expected to find sub_continent in dictionary");
      format!(" - {sub_continent}")
    } else {
      String::new()
    };

    let general_dictionary = &base_dictionaries.get_general_dictionary();
    let translated_every_x_secs = general_dictionary
      .get("every_x_secs")
      .expect("Expected to find every_x_secs in dictionary");
    let translated_every_x_secs = translated_every_x_secs
      .replace("{x}", formatted_secs_to_troop.as_str());

    
    name.set_text(&translated_territory.to_godot());
    name.bind_mut().set_font_size(32);

    size_info.set_text(&format!("{translated_size} [{translated_every_x_secs} -> +{max_troops}]"));
    continent.set_text(&format!("{translated_continent}{translated_sub_continent}"));
  }

  fn activate_ruler_part(&mut self, territory: &Territory) {
    let (
      mut ruler_label,
      mut unoccupied,
      mut under_conflict,
      mut occupation_in_progress,
      mut occupied,
    ) = self.get_ruler_states_godot_classes();

    let base_dictionaries = self.chosen_language.get_translations();
    let general_dictionary = &base_dictionaries.get_general_dictionary();

    if territory.territory_states.contains(&TerritoryState::UnoccupiedUnderConflict) {
      Self::show_updated_unoccupied_under_conflict_ruler_hud(
        &mut under_conflict,
        &mut ruler_label,
        general_dictionary,
      );

    } else if territory.territory_states.contains(&TerritoryState::OccupationInProgress) {
      Self::show_updated_occupation_in_progress_ruler_hud(
        &mut occupation_in_progress,
        &mut ruler_label,
        general_dictionary,
      );

    } else if territory.territory_states.contains(&TerritoryState::Unoccupied) {
      Self::show_updated_unoccupied_ruler_hud(
        &mut unoccupied,
        &mut ruler_label,
        general_dictionary,
      );

    } else if territory.territory_states.contains(&TerritoryState::Occupied) {
      Self::show_updated_occupied_ruler_hud(
        &mut occupied,
        &mut ruler_label,
        general_dictionary,
        &territory,
      );

    } else if territory.territory_states.contains(&TerritoryState::OccupiedUnderConflict) {
      Self::show_updated_occupied_under_conflict_ruler_hud(
        &mut under_conflict,
        &mut ruler_label,
        general_dictionary,
        &territory,
      );
    }
  }

  pub fn clean_hud(&mut self) {
    self.base_mut().set_visible(false);
    self.current_territory = None;
  }

  fn get_root_from_territory_hud(&mut self) -> Gd<RootScene> {
    self
      .base()
      .get_parent().expect("Expected TerritoryHUD to have ui as parent")
      .get_parent().expect("Expected ui to have root as parent")
      .cast::<RootScene>()
  }

  fn get_virtual_planet_from_territory_hud(&mut self) -> Gd<VirtualPlanet> {
    let virtual_planet = self
      .get_root_from_territory_hud()
      .try_get_node_as::<VirtualPlanet>("virtual_planet")
      .expect("Expected to find VirtualPlanet from RootScene");

    virtual_planet
  }

  fn get_ruler_states_godot_classes(&mut self) -> (Gd<TextLabels>, Gd<HBoxContainer>, Gd<HBoxContainer>, Gd<HBoxContainer>, Gd<HBoxContainer>) {
    let shared_path = "ruler_margin_container/PanelContainer/MarginContainer/VBoxContainer/";
    let ruler_label = self.base().get_node_as::<TextLabels>(
      &(shared_path.to_owned() + "HBoxContainer/TextLabels")
    );
    
    let mut occupied = self.base().get_node_as::<HBoxContainer>(
      &(shared_path.to_owned() + "occupied")
    );

    let mut unoccupied = self.base().get_node_as::<HBoxContainer>(
      &(shared_path.to_owned() + "unoccupied")
    );

    let mut occupation_in_progress = self.base().get_node_as::<HBoxContainer>(
      &(shared_path.to_owned() + "occupation_in_progress")
    );

    let mut under_conflict = self.base().get_node_as::<HBoxContainer>(
      &(shared_path.to_owned() + "under_conflict")
    );

    occupied.set_visible(false);
    unoccupied.set_visible(false);
    occupation_in_progress.set_visible(false);
    under_conflict.set_visible(false);

    (
      ruler_label,
      unoccupied,
      under_conflict,
      occupation_in_progress,
      occupied,
    )
  }

  fn show_updated_unoccupied_under_conflict_ruler_hud(under_conflict: &mut Gd<HBoxContainer>, ruler_label: &mut Gd<TextLabels>, general_dictionary: &I18nDefaultDictionary) {
    under_conflict.set_visible(true);
    ruler_label.set_text(
      *general_dictionary
      .get("unoccupied_under_conflict")
      .expect("Expected general_dictionary to have unoccupied_under_conflict")
    );
  }

  fn show_updated_occupation_in_progress_ruler_hud(occupation_in_progress: &mut Gd<HBoxContainer>, ruler_label: &mut Gd<TextLabels>, general_dictionary: &I18nDefaultDictionary) {
    occupation_in_progress.set_visible(true);
    ruler_label.set_text(
      *general_dictionary
      .get("occupation_in_progress")
      .expect("Expected general_dictionary to have occupation_in_progress")
    );
  }

  fn show_updated_unoccupied_ruler_hud(unoccupied: &mut Gd<HBoxContainer>, ruler_label: &mut Gd<TextLabels>, general_dictionary: &I18nDefaultDictionary) {
    unoccupied.set_visible(true);
    ruler_label.set_text(
      *general_dictionary
      .get("unoccupied_territory")
      .expect("Expected general_dictionary to have unoccupied_territory")
    );
    
    let mut land_description = unoccupied.get_node_as::<TextLabels>("land_description");
    land_description.set_text(
      *general_dictionary
      .get("empty_land")
      .expect("Expected general_dictionary to have empty_land")
    );
  }

  fn show_updated_occupied_ruler_hud(occupied: &mut Gd<HBoxContainer>, ruler_label: &mut Gd<TextLabels>, _general_dictionary: &I18nDefaultDictionary, territory: &Territory) {
    occupied.set_visible(true);
    let ruler = territory.current_ruler.as_ref().unwrap();
    let ruler_color = PlayerColor::get_banner_player_color(&ruler.color);
    let mut ruler_banner = occupied.get_node_as::<ColorRect>("banner");
    
    ruler_banner.set_color(ruler_color);
    ruler_label.set_text(&ruler.user_name);

    let num_of_troops = territory.all_troops_deployed_and_arrived.len();
    occupied.get_node_as::<TextLabels>("VBoxContainer/troops/TextLabels")
      .set_text(&format!("{:?}x", num_of_troops));
  }

  fn show_updated_occupied_under_conflict_ruler_hud(under_conflict: &mut Gd<HBoxContainer>, ruler_label: &mut Gd<TextLabels>, general_dictionary: &I18nDefaultDictionary, territory: &Territory) {
    under_conflict.set_visible(true);
    let ruler = territory.current_ruler.as_ref().unwrap();

    let translated_occupied_under_conflict = general_dictionary
      .get("occupied_under_conflict")
      .expect("Expected to find occupied_under_conflict in dictionary");
    let translated_occupied_under_conflict = translated_occupied_under_conflict
      .replace("{x}", &ruler.user_name.as_str());
    ruler_label.set_text(&translated_occupied_under_conflict);
  }

}