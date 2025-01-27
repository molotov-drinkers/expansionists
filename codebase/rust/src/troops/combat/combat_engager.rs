use crate::{globe::coordinates_system::{coordinates_system::CoordinatesSystem, virtual_planet::VirtualPlanet}, troops::{combat::combat_stats::CombatStats, speed::SpeedType, surface::surface::Surface, troop::{Troop, TroopId, TroopState}}};
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

  /// Every troop scene should have a child node named `projectile_spawner`
  /// This method returns the >Global position< of the `projectile_spawner` node
  fn get_projectile_spawner_position(&self) -> Transform3D {
    let path = if self.surface == Surface::Land {
      "land/composable_mesh/projectile_spawner"
    } else {
      "sea/composable_mesh/projectile_spawner"
    };

    let projectile_spawner = self
      .base()
      .get_node_as::<Node3D>(path);

    projectile_spawner.get_global_transform()
  }

  pub fn get_combat_target_position(&self) -> Vector3 {
    let target_position = self.get_projectile_spawner_position().origin;
    target_position
  }

  pub fn keep_fighting_if_combatting(&mut self, delta: f64, virtual_planet: &Gd<VirtualPlanet>) {
    if !self.troop_is_combatting() {
      return;
    }

    let virtual_planet = &virtual_planet.bind();
    let enemy_troop = if self.combat_stats.troop_being_attacked.is_some() {
      Self::get_troop_by_id(
        &virtual_planet,
        &self.combat_stats.troop_being_attacked.as_ref().unwrap()
      )
    } else {
      // todo: try to find a enemy within the radius, then pick the closest one
      // if no enemy in the radius, get any in the territory
      self.find_optimal_enemy_troop_to_be_attacked(virtual_planet)
    };
    let Some(enemy_troop) = enemy_troop else {
      godot_print!("No enemy troop found to keep fighting, cleaning combat_stats.troop_being_attacked");
      self.combat_stats.troop_being_attacked = None;
      return;
    };

    self.combat_stats.troop_being_attacked = Some(enemy_troop.get_name().to_string());

    let target_position = enemy_troop.get_global_transform().origin;
    let self_position = &self.base().get_global_transform().origin;
    

    let troops_distance = self_position.distance_to(target_position);
    if troops_distance > self.combat_stats.cannon.range {

      // todos: Lotta issues here =(
      self.set_trajectory_to_get_closer_to_enemy(target_position);
      self.get_closer_to_attack(target_position, *self_position);

    } else {
      // self.combat_stats.cannon.firing = true;
      // todo if this works, could be a troop state
      self.combat_stats.moving_while_fighting = false;

      // self.base_mut().look_at(target_position.normalized());
      self.set_orientation(target_position.normalized());

      if self.has_cool_down_finished(delta) {
        self.open_fire_on_the_enemy(enemy_troop, virtual_planet)
      }
    }

  }

  fn set_trajectory_to_get_closer_to_enemy(&mut self, target_position: Vector3) {
    if self.combat_stats.moving_while_fighting {
      return;
    }
    let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
      self.touching_surface_point.cartesian,
      target_position,
      VirtualPlanet::get_planet_radius() as f32
    );

    // todo if this works, could be a troop state
    self.combat_stats.moving_while_fighting = true;

    self.moving_trajectory_points = geodesic_trajectory;
    // self.moving_trajectory_is_set = true;
    // self.troop_activities.insert(TroopState::Moving);
  }

  fn get_closer_to_attack(&mut self, target_position: Vector3, current_position: Vector3) {
    let current_target = self.moving_trajectory_points[self.current_trajectory_point];
    let direction = (current_target - current_position).try_normalized();
    let Some(direction) = direction else {
      // godot_error!("direction was None at get_closer_to_attack");
      return
    };

    let velocity = direction * self.adopted_speed.get_speed();
    self.set_orientation(direction);
    self.base_mut().set_velocity(velocity);

    let current_distance = current_position.distance_to(current_target);
    let too_close_to_the_waypoint = current_distance < 0.1;

    let on_the_last_waypoint = self.current_trajectory_point == (self.moving_trajectory_points.len() -1);
    if too_close_to_the_waypoint && !on_the_last_waypoint {
      self.current_trajectory_point = self.current_trajectory_point + 1;
    }

  }

  /// TODO: doc
  fn find_optimal_enemy_troop_to_be_attacked(&mut self, virtual_planet: &GdRef<'_, VirtualPlanet>) -> Option<Gd<Troop>> {
    let Some(ref touching_territory_id) = self.touching_surface_point.territory_id else {
      return None;
    };

    let territory = virtual_planet.territories
      .get(touching_territory_id)
      .expect(&format!("Expected to find territory {touching_territory_id}, at engage_combat_if_needed"));
    
    let Some(enemy_player_id) = territory.all_troops_deployed_and_arrived_by_player
      .keys()
      .find(|player_id| {**player_id != self.owner.player_id })
        else {
          godot_print!("No enemy troops found in territory {touching_territory_id}");
          return None;
        };

    let Some(enemy_troops) = territory
      .all_troops_deployed_and_arrived_by_player
      .get(enemy_player_id) else {
        godot_print!("No enemy troops found for enemy_player_id {enemy_player_id}");
        return None;
      };

    let self_position = &self.base().get_global_transform().origin;

    let closest_enemy_troop = enemy_troops
      .iter()
      .fold(None, |closest, enemy_troop_id| {
        if let Some(enemy_troop) = Self::get_troop_by_id(&virtual_planet, enemy_troop_id) {
          let enemy_position = enemy_troop.get_global_transform().origin;
          let distance = self_position.distance_to(enemy_position);

          if distance <= self.combat_stats.cannon.range {
              match closest {
                  Some((_, min_distance)) if distance >= min_distance => closest, // Keep the previous closest if it's better
                  _ => Some((enemy_troop, distance)), // Otherwise, update the closest
              }
          } else {
            // Skip if the enemy is out of range
            closest
          }
        } else {
          // If there's no enemy found, keep the current closest
          closest
        }
      })
      .map(|(enemy_troop, _closest_distance)| enemy_troop);



    // let troop_found = enemy_troops
    //   .iter()
    //   .find_map(|enemy_troop_id| {
    //     if let Some(enemy_troop) = Self::get_troop_by_id(&virtual_planet, enemy_troop_id) {
    //       let enemy_position = enemy_troop.get_global_transform().origin;
    //       let distance = self_position.distance_to(enemy_position);

    //       if distance <= self.combat_stats.cannon.range {
    //         return Some(enemy_troop);
    //       }
    //     }

    //     None
    //   });

    closest_enemy_troop
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

  fn has_cool_down_finished(&mut self, delta: f64) -> bool {
    self.combat_stats.cannon.cooling_down_counter += delta;
    if self.combat_stats.cannon.cooling_down_counter >= CombatStats::COOL_DOWN_TIMER_IN_SECS {
      self.combat_stats.cannon.cooling_down_counter = 0.;
      return true;
    }

    false
  }

  fn open_fire_on_the_enemy(&mut self, enemy_troop: Gd<Troop>, virtual_planet: &GdRef<'_, VirtualPlanet>) {
    let root = virtual_planet.get_root_from_virtual_planet();

    let target_position = enemy_troop.get_global_transform().origin;

    let mut projectiles_node = root
      .get_node_as::<Node3D>("troops/projectiles");

    let projectile: Gd<PackedScene> = load("res://scenes/troops/combat/projectile.tscn");
    let mut projectile = projectile.instantiate_as::<Projectile>();

    projectile.bind_mut().up_to_date_target_position = target_position; 
    let position_to_spawn_projectile = self.get_projectile_spawner_position();
    projectile.set_global_transform(position_to_spawn_projectile);

    projectiles_node.add_child(&projectile);
  }

  // fn update_combat_stats

}