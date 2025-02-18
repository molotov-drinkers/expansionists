use crate::{
  globe::coordinates_system::{coordinates_system::CoordinatesSystem, virtual_planet::VirtualPlanet},
  troops::{combat::combat_stats::CombatStats, surface::surface::Surface, troop::{Troop, TroopId, TroopState}},
  visual_debug
};
use godot::prelude::*;

use super::{combat_stats::CombatTypes, projectile::Projectile};

impl Troop {
  pub fn keep_fighting_if_combatting(&mut self, delta: f64, virtual_planet: &Gd<VirtualPlanet>) {
    if !self.troop_is_combatting() {
      return;
    }

    let virtual_planet = &virtual_planet.bind();

    // TODO: Combat:
    // How it should be?
    // 1. Attacker invades it
    // 2. Defender approaches the Attacker within the combat radius
    // 3. Defender open fire
    // 4. Attacker returns fire

    // Caveat:
    // Should be aware a attacker can be ordered directly to attack a defender, (Right click on the defender)
    // so the defender should be able to open fire

    if self.troop_activities.contains(&TroopState::Combating(CombatTypes::Attacking)) {
      self.handle_attacking_behavior(delta, virtual_planet);

    } else if self.troop_activities.contains(&TroopState::Combating(CombatTypes::Defending)) {
      self.handle_defensive_behavior(delta, virtual_planet);

    } else {
      godot_error!("Troop is combatting but haven't defined if it's attacking or defending");
    }
  }

  fn handle_defensive_behavior(&mut self, delta: f64, virtual_planet: &GdRef<'_, VirtualPlanet>) {
    let enemy_troop = if self.combat_stats.opening_fire_on_troop.is_some() {
      Self::get_troop_by_id(
        &virtual_planet,
        &self.combat_stats.opening_fire_on_troop.as_ref().unwrap()
      )
    } else {
      self.find_closest_enemy_troop_to_be_attacked(virtual_planet, false)
    };

    self.handle_combat(delta, virtual_planet, enemy_troop);
  }

  fn handle_attacking_behavior(&mut self, delta: f64, virtual_planet: &GdRef<'_, VirtualPlanet>) {
    // TODO: This may be filled by:
    // 1. the right click
    // 2. when the defender approaches the attacker
    // 3. when it invades the enemy territory and defender is in the radius

    let enemy_troop = if self.combat_stats.opening_fire_on_troop.is_some() {
      Self::get_troop_by_id(
        &virtual_planet,
        &self.combat_stats.opening_fire_on_troop.as_ref().unwrap()
      )
    } else {
      self.find_closest_enemy_troop_to_be_attacked(virtual_planet, true)
    };

    self.handle_combat(delta, virtual_planet, enemy_troop);
  }

  fn handle_combat(&mut self, delta: f64, virtual_planet: &GdRef<'_, VirtualPlanet>, enemy_troop: Option<Gd<Troop>>) {
    let Some(enemy_troop) = enemy_troop else {
      godot_print!("No enemy troop found to keep fighting, cleaning combat_stats.opening_fire_on_troop");
      self.combat_stats.opening_fire_on_troop = None;
      return;
    };

    // Being sure isn't targetting some troop no longer combatting on that territory
    if enemy_troop.bind().deployed_to_territory != self.deployed_to_territory {
      self.combat_stats.opening_fire_on_troop = None;
      return;
    };

    self.combat_stats.opening_fire_on_troop = Some(enemy_troop.get_name().to_string());

    // Troop might be deployed while combatting, so it should not keep firing
    // whenever it's being deployed to another surface_point
    if self.troop_activities.contains(&TroopState::Deploying) {
      return;
    }

    let target_position = enemy_troop.get_global_transform().origin;
    let self_position = &self.base().get_global_transform().origin;
    let troops_distance = self_position.distance_to(target_position);

    if troops_distance > self.combat_stats.cannon.range {
      self.set_trajectory_to_get_closer_to_enemy(target_position, virtual_planet);

    } else {
      self.reset_trajectory();
      self.set_orientation(target_position.normalized());
      self.moving_and_combating = false;

      if self.has_cool_down_finished(delta) {
        self.open_fire_on_the_enemy(enemy_troop, virtual_planet)
      }
    }
  }

  fn find_closest_enemy_troop_to_be_attacked(
    &mut self,
    virtual_planet: &GdRef<'_, VirtualPlanet>,
    close_to_the_cannon_range: bool,
  ) -> Option<Gd<Troop>> {
    if !self.arrived_to_territory {
      // won't look for enemy troops if it hasn't arrived to the territory
      return None
    }
    let touching_territory_id = &self.deployed_to_territory;

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
      .fold(None, |current_closest: Option<(Gd<Troop>, f32)>, enemy_troop_id| {
        if let Some(enemy_troop) = Self::get_troop_by_id(&virtual_planet, enemy_troop_id) {
          let enemy_position = enemy_troop.get_global_transform().origin;
          let new_comparable_distance = self_position.distance_to(enemy_position);

          // That's to ensure an attacking troop would be able to fight back even if
          // it's just a lil bit further than the cannon range
          // We had situations when neighbor troops were combating while some others
          // were just watching, which wasn't natural
          let cannon_range_plus_buffer = self.combat_stats.cannon.range + (self.combat_stats.cannon.range/8.);

          if close_to_the_cannon_range && new_comparable_distance > cannon_range_plus_buffer {
            return current_closest;
          }

          match current_closest {
            // Keep the previous closest if it's better
            Some((_, current_min_distance))
              if new_comparable_distance >= current_min_distance => current_closest,
            
            // Otherwise, update the closest:
            _ => Some((enemy_troop, new_comparable_distance)),
          }

        } else {
          // If there's no enemy found, keep the current closest
          current_closest
        }
      })
      .map(|(enemy_troop, _closest_distance)| enemy_troop);

    return closest_enemy_troop;
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

  fn set_trajectory_to_get_closer_to_enemy(&mut self, target_position: Vector3, virtual_planet: &GdRef<'_, VirtualPlanet>) {
    if !self.moving_trajectory_is_set {

      let mut heat_map_dictionary = self
        .base()
        .get_meta("heat_map_for_within_territory_trajectory")
        .to::<Dictionary>();
      heat_map_dictionary.clear();

      let in_the_frontiers_trajectory = CoordinatesSystem::get_in_the_frontiers_trajectory(
        self.touching_surface_point.cartesian,
        target_position,
        VirtualPlanet::get_planet_radius() as f32,
        self.base().get_world_3d().expect("World to exist"),
        &self.deployed_to_territory,
        virtual_planet,
        self.base(),
      );

      visual_debug!({
        self.highlight_trajectory(&in_the_frontiers_trajectory);
      });

      self.moving_trajectory_points = in_the_frontiers_trajectory;
      self.current_trajectory_point = 0;
      self.moving_and_combating = true;
      self.moving_trajectory_is_set = true;
    }

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

    let position_to_spawn_projectile = self.get_projectile_spawner_position();
    projectile.bind_mut().up_to_date_target_position = target_position; 
    projectile.bind_mut().fired_by = self.base().get_name().to_string();
    projectile.set_global_transform(position_to_spawn_projectile);

    projectiles_node.add_child(&projectile);
  }

}