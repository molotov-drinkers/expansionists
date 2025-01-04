use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

use crate::globe::coordinates_system::virtual_planet::VirtualPlanet;
use crate::player::color::PlayerColor;
use crate::player::player::Player;
use crate::troops::spawner_engine;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RootScene {
  base: Base<Node3D>,
  troops_spawn: i32,
  hack_bool: bool,
}

#[godot_api]
impl INode3D for RootScene {
  fn init(base: Base<Node3D>) -> RootScene {

    RootScene {
      base: base,
      troops_spawn: 0,
      hack_bool: false,
    }
  }

  fn process(&mut self, _delta: f64) {
    let mut virtual_planet = self.base_mut()
      .find_child("virtual_planet")
      .expect("Expected to find virtual_planet")
      .cast::<VirtualPlanet>();
    let mut virtual_planet = virtual_planet.bind_mut();

    if virtual_planet.are_surface_points_matched && self.base().is_node_ready() {
      self.startup_troops_spawn(&mut virtual_planet);
    }
  }
}

const ORIGIN_A: &str = "baffin_bay";
const ORIGIN_B: &str = "unclaimed_area";
const ORIGIN_C: &str = "east_savanna";
const ORIGIN_D: &str = "korean_peninsula";

impl RootScene {

  pub fn hardcoded_players(&mut self) -> Vec<Gd<Player>> {
    let mut players_node = self
      .base_mut()
      .find_child("players")
      .expect("Expected players to be found in RootScene");

    let mut player_1 = Player::new_alloc();
    player_1.bind_mut().set_player(
      1,
      "Player 1".to_owned(),
      PlayerColor::Red,
      ORIGIN_A.to_owned()
    );

    let mut cpu_2 = Player::new_alloc();
    cpu_2.bind_mut().set_player(
      2,
      "Hawk".to_owned(),
      PlayerColor::Blue,
      ORIGIN_B.to_owned()
    );

    let mut cpu_3 = Player::new_alloc();
    cpu_3.bind_mut().set_player(
      3,
      "Eagle".to_owned(),
      PlayerColor::Yellow,
      ORIGIN_C.to_owned()
    );

    let mut cpu_4 = Player::new_alloc();
    cpu_4.bind_mut().set_player(
      4,
      "Tiger".to_owned(),
      PlayerColor::Green,
      ORIGIN_D.to_owned()
    );

    players_node.add_child(&player_1);
    players_node.add_child(&cpu_2);
    players_node.add_child(&cpu_3);
    players_node.add_child(&cpu_4);

    [player_1, cpu_2, cpu_3, cpu_4].to_vec()
  }

  pub fn startup_troops_spawn(&mut self, virtual_planet: &mut VirtualPlanet) {
    // TODO: this hack bool should go away
    if self.hack_bool == false {
      for player in self.hardcoded_players() {
        virtual_planet.set_new_territory_ruler(player);
      }
      self.hack_bool = true;
    }

    // TODO: this spanwer should be done differently
    let max_troops = 5;
    while self.troops_spawn < max_troops {
      spawner_engine::troop_spawner(
        self,
        &virtual_planet,
        self.troops_spawn,
        ORIGIN_A.to_owned(),
      );

      spawner_engine::troop_spawner(
        self,
        &virtual_planet,
        self.troops_spawn,
        ORIGIN_B.to_owned(),
      );

      spawner_engine::troop_spawner(
        self,
        &virtual_planet,
        self.troops_spawn,
        ORIGIN_C.to_owned(),
      );

      spawner_engine::troop_spawner(
        self,
        &virtual_planet,
        self.troops_spawn,
        ORIGIN_D.to_owned(),
      );

      self.troops_spawn+=1;
    }
  }
}
