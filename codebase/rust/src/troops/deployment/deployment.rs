use crate::{
  globe::{coordinates_system::virtual_planet::VirtualPlanet,
  territories::territory::TerritoryState}, player::player::Player, troops::troop::Troop
};
use godot::prelude::*;

impl Troop {
  /// Can trigger Combat or Colonization/Occupation/War
  pub fn get_deployment_next_action(&mut self, virtual_planet: &mut Gd<VirtualPlanet>) {
    if self.waiting_for_deployment_following_action {

      // Some if troop hit a land
      let Some(ref touching_territory_id) = self.touching_surface_point.territory_id else {
        return;
      };

      // troop arrived to the territory that has been deployed to
      if touching_territory_id == &self.deployed_to_territory {
        self.waiting_for_deployment_following_action = false;

        let mut virtual_planet = virtual_planet.bind_mut();
        let territory = virtual_planet.territories
          .get_mut(touching_territory_id)
          .expect(&format!("Expected to find territory {touching_territory_id}, at get_deployment_next_action"));

        territory.inform_troop_arrived(
          &self.base().get_name().to_string(),
          self.owner.player_id
        );

        let territory_current_ruler = territory
          .current_ruler
          .as_ref();

        if territory.territory_states.contains(&TerritoryState::Unoccupied) && !territory.has_troops_from_different_players {
          territory.territory_states.insert(TerritoryState::OccupationInProgress);
          territory.territory_states.remove(&TerritoryState::UnoccupiedUnderConflict);

          let root = self.get_root_from_troop();
          let player = Player::get_player_by_id(root, self.owner.player_id.clone());
          let player_static_info = player.bind().static_info.clone();
          territory.player_trying_to_conquer = Some(player_static_info);

        } else if territory_current_ruler.is_some_and(|ruler_static_info| ruler_static_info.player_id == self.owner.player_id) {
          // Entering own territory, could start patrolling or start defending it from invaders

        } else if territory_current_ruler.is_some_and(|ruler_static_info| ruler_static_info.player_id != self.owner.player_id) {
          // Entering enemy territory, could start combat or keep combatting until the territory is conquered
          territory.territory_states.insert(TerritoryState::OccupiedUnderConflict);
          territory.player_trying_to_conquer = Some(self.owner.clone());

        } else if territory.territory_states.contains(&TerritoryState::Unoccupied) && territory.has_troops_from_different_players {
          territory.territory_states.insert(TerritoryState::UnoccupiedUnderConflict);
          territory.player_trying_to_conquer = Some(self.owner.clone());
          // Entering a territory that started being occupied by someone else, should start combat and hold down the territory occupation
          // until the conflict is finished

          // TODO: implement this, for now battle is happening only if territory has a ruler

          godot_print!("Troop would start a combat or keep combating. Also would pause enemy occupation! ::: {}", touching_territory_id);

        } else {
          godot_error!("Troop has no idea what to do after the deployment! ::: {}", touching_territory_id);
        }
      };
    }
  }

}