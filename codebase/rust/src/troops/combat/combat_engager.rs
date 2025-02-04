use crate::{
  globe::coordinates_system::virtual_planet::VirtualPlanet,
  troops::troop::{Troop, TroopState}
};
use godot::prelude::*;

use super::combat_stats::CombatTypes;

impl Troop {
  pub fn trigger_combat_engage_if_needed(&mut self, virtual_planet: &Gd<VirtualPlanet>) {
    let Some(ref touching_territory_id) = self.touching_surface_point.territory_id else {
      return;
    };

    let virtual_planet = virtual_planet.bind();
    let territory = virtual_planet.territories
      .get(touching_territory_id)
      .expect(&format!("Expected to find territory {touching_territory_id}, at engage_combat_if_needed"));
    
    if territory.has_troops_from_different_players && self.is_the_territory_deployed_to(touching_territory_id) {
      self.base_mut().add_to_group(Self::TROOP_COMBATTING);
      self.troop_activities.remove(&TroopState::Patrolling);
      self.troop_activities.remove(&TroopState::Idle);

      // Setting the combat type and combating states on the troops
      if territory.current_ruler.is_none() {
        self.troop_activities.insert(TroopState::Combating(CombatTypes::FightingOverUnoccupiedTerritory));
        // TODO: Still should be able to understand who attacked and who was already there        
      } else {
        territory.current_ruler.as_ref().map(|ruler_static_info| {
          if ruler_static_info.player_id != self.owner.player_id {
            self.troop_activities.insert(TroopState::Combating(CombatTypes::Attacking));
          } else {
            self.troop_activities.insert(TroopState::Combating(CombatTypes::Defending));
          }
        });
      }

    } else if self.base().is_in_group(Self::TROOP_COMBATTING) {
      self.troop_activities.insert(TroopState::Patrolling);
      self.remove_combatting_states();
      // todo: when it's combatting from the water, it seems like it's keep combatting endlessly
      // self.reset_trajectory(true);
      self.combat_stats.in_after_combat = true;
    }
  }

  pub fn troop_is_combatting(&self) -> bool {
    for combat_type in CombatTypes::iter() {
      if self.troop_activities.contains(&TroopState::Combating(combat_type)) {
        return true;
      }
    }

    return false;
  }

  fn remove_combatting_states(&mut self) {
    self.base_mut().remove_from_group(Self::TROOP_COMBATTING);
    for combat_type in CombatTypes::iter() {
      self.troop_activities.remove(&TroopState::Combating(combat_type));
    }
  }

  fn is_the_territory_deployed_to(&self, touching_territory_id: &String) -> bool {
    &self.deployed_to_territory == touching_territory_id
  }
  // fn update_combat_stats

}