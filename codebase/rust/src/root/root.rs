use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::i18n::base::AvailableLanguage;
use crate::player::color::PlayerColor;
use crate::player::player::{Player, PlayerType, TroopMeshes};
use crate::troops::mesh_map::MeshId;
use crate::troops::spawner_engine;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RootScene {
  base: Base<Node3D>,
  initial_spawn_started: bool,
}

#[godot_api]
impl INode3D for RootScene {
  fn init(base: Base<Node3D>) -> RootScene {

    RootScene {
      base: base,
      initial_spawn_started: false,
    }
  }

  fn process(&mut self, _delta: f64) {
    let virtual_planet = self.base()
      .find_child("virtual_planet")
      .expect("Expected to find virtual_planet")
      .cast::<VirtualPlanet>();

    if virtual_planet.bind().are_surface_points_matched && self.base().is_node_ready() {
      self.startup_troops_spawn();
    }
  }
}

impl RootScene {

  pub fn hardcoded_players(&mut self) -> Vec<Gd<Player>> {
    let mut players_node = self
      .base_mut()
      .find_child("players")
      .expect("Expected players to be found in RootScene");

    let mut player_1 = Player::new_alloc();
    player_1.bind_mut().set_player(
      1,
      "Torque".to_owned(),
      PlayerColor::Blue,
      "baffin_bay".to_owned(),
      PlayerType::MainPlayer,
      TroopMeshes {
        land: MeshId::Tank1,
        sea: MeshId::Boat6,
      },
      AvailableLanguage::InternationalEnglish,
    );

    let mut cpu_2 = Player::new_alloc();
    cpu_2.bind_mut().set_player(
      2,
      "Eastest".to_owned(),
      PlayerColor::Red,
      "west_siberia".to_owned(),
      PlayerType::Bot,
      TroopMeshes {
        land: MeshId::Tonk,
        sea: MeshId::Galleon,
      },      
      AvailableLanguage::InternationalEnglish,
    );

    let mut cpu_3 = Player::new_alloc();
    cpu_3.bind_mut().set_player(
      3,
      "Eagle".to_owned(),
      PlayerColor::Yellow,
      "east_savanna".to_owned(),
      PlayerType::Bot,
      TroopMeshes {
        land: MeshId::Truck1,
        sea: MeshId::Boat6,
      },      
      AvailableLanguage::InternationalEnglish,
    );

    let mut cpu_4 = Player::new_alloc();
    cpu_4.bind_mut().set_player(
      4,
      "Tiger".to_owned(),
      PlayerColor::Green,
      "korean_peninsula".to_owned(),
      PlayerType::Bot,
      TroopMeshes {
        land: MeshId::Cannon,
        sea: MeshId::Boat5,
      },      
      AvailableLanguage::InternationalEnglish,
    );

    players_node.add_child(&player_1);
    players_node.add_child(&cpu_2);
    players_node.add_child(&cpu_3);
    players_node.add_child(&cpu_4);

    let players = [
      player_1,
      cpu_2,
      cpu_3,
      cpu_4,
    ].to_vec();
    players
  }

  pub fn startup_troops_spawn(&mut self) {
    if self.initial_spawn_started == false {
      self.initial_spawn_started = true;
      
      let hardcoded_players = self.hardcoded_players();

      let mut virtual_planet = self.base()
        .find_child("virtual_planet")
        .expect("Expected to find virtual_planet")
        .cast::<VirtualPlanet>();
      let mut virtual_planet = virtual_planet.bind_mut();

      for mut player in hardcoded_players {
        let player_static_info = {
          let player_binding = player.bind();
          player_binding.static_info.clone()
        };
        player.set_name(&player_static_info.player_id.to_string());

        let territory_id = &player_static_info.initial_territory;
        let mut territory = virtual_planet.get_mut_territory_from_virtual_planet(territory_id);
        VirtualPlanet::set_new_territory_ruler(territory, &mut player);

        let mut troops_spawn = 0;
        let num_of_initial_troops = 1;
        while troops_spawn < num_of_initial_troops {
          spawner_engine::spawn_troop(
            self,
            &mut player,
            &mut territory,
          );
          troops_spawn+=1;
        }

      }
    }

  }
}
