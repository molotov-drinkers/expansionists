use crate::{
  globe::{coordinates_system::{coordinates_system::CoordinatesSystem, virtual_planet::VirtualPlanet},
  territories::territory::{Territory, TerritoryId}},
  troops::{speed::SpeedType, troop::{Troop, TroopState}}
};
use godot::{classes::Sprite3D, prelude::*};


impl Troop {
  pub fn set_order_to_move_to(&mut self, destination: Vector3, dest_territory_id: &TerritoryId) {
    self.reset_trajectory(false);
    self.troop_activities.insert(TroopState::Moving);
    self.troop_activities.insert(TroopState::Deploying);
    self.troop_activities.remove(&TroopState::Patrolling);

    let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
      self.touching_surface_point.cartesian,
      destination,
      VirtualPlanet::get_planet_radius() as f32
    );

    let mut virtual_planet = self.get_virtual_planet_from_troop_scope();
    let mut virtual_planet = &mut virtual_planet.bind_mut();

    let origin_territory = self.get_territory(
      self.deployed_to_territory.clone(), &mut virtual_planet
    );

    origin_territory.inform_territory_departure(
      &self.base().get_name().to_string(),
      self.owner.player_id.clone()
    );

    self.arrived_to_territory = false;
    self.moving_trajectory_points = geodesic_trajectory;
    self.moving_trajectory_is_set = true;
    self.adopted_speed = SpeedType::FightOrFlight;
    self.deployed_to_territory = dest_territory_id.clone();

    let destination_territory = self.get_territory(
      self.deployed_to_territory.clone(), &mut virtual_planet
    );
    destination_territory.add_territory_deployment(
      &self.base().get_name().to_string(),
      self.owner.player_id.clone()
    );

    self.waiting_for_deployment_following_action = true;
  }

  fn get_territory<'a>(&mut self, territory_id: TerritoryId, virtual_planet: &'a mut GdMut<'_, VirtualPlanet>) -> &'a mut Territory {
    let territory = virtual_planet
      .get_mut_territory_from_virtual_planet(&territory_id);

    territory
  }
  
  pub fn select_troop(&mut self) {
    self.troop_activities.insert(TroopState::Selected);

    self.set_selected_sprites_visibility(true);
  }

  pub fn deselect_troop(&mut self) {
    self.troop_activities.remove(&TroopState::Selected);

    self.set_selected_sprites_visibility(false);
  }

  pub fn set_selected_sprites_visibility(&mut self, visible: bool) {
    let mut land_selected_sprite = self.base_mut().get_node_as::<Sprite3D>("land/selected");
    let mut sea_selected_sprite = self.base_mut().get_node_as::<Sprite3D>("sea/selected");

    land_selected_sprite.set_visible(visible);
    sea_selected_sprite.set_visible(visible);
  }

}