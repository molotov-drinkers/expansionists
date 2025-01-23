use crate::{globe::{coordinates_system::virtual_planet::VirtualPlanet, territories::territory::TroopId}, troops::troop::{Troop, TroopState}};
use godot::prelude::*;

use super::projectile::Projectile;

/// Combat types the troop can engage
/// Needs to populate CombatTypes::iter() method
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CombatTypes {
  Attacking,
  Defending,
  FightingOverUnoccupiedTerritory,
}

impl CombatTypes {
  pub fn iter() -> impl Iterator<Item = CombatTypes> {
    [
      CombatTypes::Attacking,
      CombatTypes::Defending,
      CombatTypes::FightingOverUnoccupiedTerritory,
    ]
    .iter()
    .cloned()
  }
}


impl Troop {
  pub fn trigger_combat_engage_if_needed(&mut self, virtual_planet: &Gd<VirtualPlanet>) {
    let Some(ref touching_territory_id) = self.touching_surface_point.territory_id else {
      return;
    };

    let virtual_planet = virtual_planet.bind();
    let territory = virtual_planet.territories
      .get(touching_territory_id)
      .expect(&format!("Expected to find territory {touching_territory_id}, at engage_combat_if_needed"));
    
    if territory.has_troops_from_different_players {

      self.base_mut().add_to_group(Self::TROOP_COMBATTING);


      // self.troop_activities.remove(&TroopState::Patrolling);
      // self.troop_activities.remove(&TroopState::Idle);

      if territory.current_ruler.is_none() {
        self.troop_activities.insert(TroopState::Combating(CombatTypes::FightingOverUnoccupiedTerritory));

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
      self.remove_combatting_states();

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

  /// Every troop scene should have a child node named `projectile_spawner`
  /// This method returns the >Global position< of the `projectile_spawner` node
  fn get_projectile_spawner_position(&self) -> Transform3D {
    let projectile_spawner = self
      .base()
      .get_node_as::<Node3D>("projectile_spawner");

    projectile_spawner.get_global_transform()
  }

  pub fn get_combat_target_position(&self) -> Vector3 {
    let target_position = self.get_projectile_spawner_position().origin;
    target_position
  }

  pub fn keep_fighting_if_combatting(&mut self, virtual_planet: &Gd<VirtualPlanet>) {
    if !self.troop_is_combatting() {
      return;
    }

    // todo: try to find a enemy within the radius, then pick the closest one
    // if no enemy in the radius, get any in the territory
    let Some(enemy_troop) = self.find_optimal_enemy_troop_to_be_attacked(virtual_planet) else {
      return
    };

    // let _updated_target_position = enemy_troop.get_global_transform().origin;

    self.open_fire_on_the_enemy(enemy_troop)

  }

  /// TODO: doc
  fn find_optimal_enemy_troop_to_be_attacked(&mut self, virtual_planet: &Gd<VirtualPlanet>) -> Option<Gd<Troop>> {
    let Some(ref touching_territory_id) = self.touching_surface_point.territory_id else {
      return None;
    };

    let virtual_planet: GdRef<'_, VirtualPlanet> = virtual_planet.bind();
    let territory = virtual_planet.territories
      .get(touching_territory_id)
      .expect(&format!("Expected to find territory {touching_territory_id}, at engage_combat_if_needed"));
    
    let Some(enemy_player_id) = territory.all_troops_deployed_and_arrived_by_player
      .keys()
      .find(|player_id| {
        **player_id != self.owner.player_id
      }) else {
        godot_print!("No enemy troops found in territory {touching_territory_id}");
        return None;
      };

    let Some(enemy_troops) = territory
      .all_troops_deployed_and_arrived_by_player
      .get(enemy_player_id) else {
        godot_print!("No enemy troops found for enemy_player_id {enemy_player_id}");
        return None;
      };

    let positon = &self.base().get_global_transform().origin;

    let troop_found = enemy_troops
      .iter()
      .find_map(|enemy_troop_id| {

        if let Some(enemy_troop) = Self::get_troop_by_id(&virtual_planet, enemy_troop_id) {
          let enemy_position = enemy_troop.get_global_transform().origin;
          let distance = positon.distance_to(enemy_position);

          if distance <= self.combat_stats.cannon.range {
            return Some(enemy_troop);
          }
        }

        // TODO: should get the closest enemy troop, probably by distance_to
        // Maybe that's not that performatic, so it could be defined a const like COMBAT_RADIUS
        // if any troop within the radius if found, should be considered to be attacked
        // if not, should any troop
        // and move_and_slide towards it until it's within the radius
        None
      });

    troop_found
  }

  fn get_troop_by_id(virtual_planet: &GdRef<'_, VirtualPlanet>, troop_id: &TroopId) -> Option<Gd<Troop>> {
    let root = virtual_planet.get_root_from_virtual_planet();

    let Some(troop) = root
      .try_get_node_as::<Troop>(&format!("troops/{troop_id}")) else {
        godot_error!("Didn't find troop {troop_id}");
        return None;
      };
    
    Some(troop)
  }

  fn open_fire_on_the_enemy(&mut self, enemy_troop: Gd<Troop>) {
    if self.combat_stats.cannon.cooling_down {
      return
    }

    let target_position = enemy_troop.get_global_transform().origin;

    let mut projectile_spawner = self
      .base()
      .get_node_as::<Node3D>("projectile_spawner");

    let projectile: Gd<PackedScene> = load("res://scenes/troops/combat/projectile.tscn");
    let mut projectile = projectile.instantiate_as::<Projectile>();

    projectile.bind_mut().up_to_date_target_position = target_position; 

    // projectile.set_target_position(target_position);
    projectile_spawner.add_child(&projectile);
  }

  // fn update_combat_stats

}