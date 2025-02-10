use std::collections::HashSet;

use godot::{
  classes::{BoxMesh, CharacterBody3D, ICharacterBody3D, MeshInstance3D, StandardMaterial3D}, prelude::*
};
use crate::{
  globe::{
    coordinates_system::{
      coordinates_system::CoordinatesSystem,
      surface_point::{Coordinates, SurfacePoint, SurfacePointMetadata},
      virtual_planet::VirtualPlanet,
    },
    territories::territory::TerritoryId
  },
  player::player::{Player, PlayerStaticInfo},
  root::root::RootScene
};

use super::{
  combat::{combat_stats::CombatTypes, combat_stats::CombatStats},
  speed::SpeedType,
  surface::surface::Surface
};

#[derive(Hash, Eq, PartialEq)]
pub enum TroopState {
  /// Whenever the troop is moving it doesn't matter the place nor reason
  Moving,

  /// Whenever the troop is patrolling in its territory
  Patrolling,
  
  /// Pauses in between movements while it patrols
  Idle,

  /// Like a patrolling but the troop is rotating in place
  /// (TODO:) Maybe it could be used as Idle instead
  // Rotating,
  
  /// If the troop is selected by the player
  Selected,

  /// Whenever the troop is being deployed to another surface_point
  /// other than the one it was before by the order of its player
  Deploying,

  /// Whenever the troop is in combat
  Combating(CombatTypes),
}

type TroopActivities = HashSet<TroopState>;

/// TroopId is a string name, is the base().get_name().to_string() of a troop
pub type TroopId = String;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Troop {
  pub base: Base<CharacterBody3D>,
  /// holds troop's current location, updated every frame
  pub touching_surface_point: SurfacePointMetadata,

  /// holds the territory id the troop is deployed to
  /// it changes when the troop is deployed to another territory
  pub deployed_to_territory: TerritoryId,
  /// indicates troop has arrived to the territory it was deployed to
  pub arrived_to_territory: bool,

  pub surface: Surface,
  /// If it changes, needs to swap in between sea and land mesh
  pub surface_type_changed: bool,

  pub owner: PlayerStaticInfo,
  pub combat_stats: CombatStats,

  pub troop_activities: TroopActivities,
  pub adopted_speed: SpeedType,

  /// indicates the time the troop will wait before moving again while patrolling
  idle_timer: f32,

  pub moving_trajectory_points: Vec<Vector3>,
  pub moving_trajectory_is_set: bool,
  pub current_trajectory_point: usize,

  /// it turns true when the troop is spawned and the orientation is set
  initial_orientation_is_set: bool,

  /// it turns true when the troop receives the deployment order
  /// and false when troop arrives to the deployed territory
  pub waiting_for_deployment_following_action: bool,
}

#[godot_api]
impl ICharacterBody3D for Troop {
  fn init(base: Base<CharacterBody3D>) -> Troop {

    Troop {
      base: base,
      touching_surface_point: SurfacePoint::get_blank_surface_point_metadata(),
      deployed_to_territory: "".to_string(),
      arrived_to_territory: true,
      surface: Surface::Land,
      surface_type_changed: false,

      owner: Player::get_blank_static_info(),
      combat_stats: CombatStats::new(),

      troop_activities: HashSet::from([
        TroopState::Idle,
        TroopState::Patrolling,
      ]),

      adopted_speed: SpeedType::Patrolling,

      idle_timer: Self::DEFAULT_IDLE_TIMER,

      moving_trajectory_points: Vec::new(),
      moving_trajectory_is_set: false,
      current_trajectory_point: 0,

      initial_orientation_is_set: false,

      waiting_for_deployment_following_action: false,
    }
  }

  fn ready(&mut self) {
    self.base_mut().add_to_group(Self::TROOP_CLASS_NAME);
    self.set_custom_collision();
    self.set_selected_sprites_visibility(false);
    self.set_troop_visibility();
  }

  fn process(&mut self, delta: f64) {
    // Sets orientation first, as we use default_mesh to get the global position
    // it's important to set the orientation before setting the surface troop
    self.set_initial_orientation();

    let virtual_planet = &mut self.get_virtual_planet_from_troop_scope();
    self.set_surface_troop();
    self.check_and_change_mesh();
    self.maybe_populate_trajectory_points(virtual_planet);
    self.maybe_move_along_the_trajectory_and_set_orientation();
    self.decrease_idle_timer_if_idling(delta);
    self.get_deployment_next_action(virtual_planet);

    self.trigger_combat_engage_if_needed(virtual_planet);
    self.keep_fighting_if_combatting(delta, virtual_planet);
  }
}

#[godot_api]
impl Troop {

  /// Group, Used to add represent the troop itself
  pub const TROOP_CLASS_NAME: &'static str = "troop";

  /// Group, Used to add represent the troop belongs to the player itself and
  /// it's not some other player's troop
  pub const MAIN_PLAYER_TROOPS: &'static str = "main_player_troops";
  pub const BOT_TROOPS: &'static str = "bot_troops";

  /// Group, Used to add represent the troop is in combat
  pub const TROOP_COMBATTING: &'static str = "troop_is_combatting";

  /// Defines the time the troop will wait before moving again while patrolling
  const DEFAULT_IDLE_TIMER: f32 = 0.7;

  pub const EVENT_TROOP_SPAWNED: &'static str = "troop_spawned";
  pub const EVENT_TROOP_DOWN: &'static str = "troop_down";

  #[signal]
  fn troop_spawned(&self) {}

  #[signal]
  fn troop_down(&self) {}

  pub fn set_ownership(&mut self, player: &PlayerStaticInfo) {
    self.owner = player.clone();
  }

  /// Sets troop collision layer and mask are set to be separate.
  /// To avoid misbehaviors on geodesic movement
  fn set_custom_collision(&mut self) {
    let troop_collision_layer = 2;
    let troop_collision_mask = 2;
    self.base_mut().set_collision_mask(troop_collision_layer);
    self.base_mut().set_collision_layer(troop_collision_mask);
  }

  /// Sets the initial orientation of the troop when it's spawned
  fn set_initial_orientation(&mut self) {
    if !self.initial_orientation_is_set {
      // Choose a forward direction (assuming the troop faces the -Z direction by default)
      let initial_orientation = Vector3::new(0.0, 0.0, -1.0);
      self.set_orientation(initial_orientation);
      self.initial_orientation_is_set = true;
    }
  }

  /// Sets orientation to respect the globe trajectory and gravity
  /// if the troop is moving, it will set the orientation to the direction it's moving
  pub fn set_orientation(&mut self, trajectory_vector: Vector3) {
    // This is the "up" direction on the surface
    let normal = self.base().get_global_position().normalized();

    // Calculate the right vector using the cross product (normal x forward)
    let right = normal
      .cross(trajectory_vector)
      .try_normalized()
      .expect("normal and forward expected to exist");
  
    // Calculate the new forward vector as the cross product of right and normal
    let new_forward = right
      .cross(normal)
      .try_normalized()
      .expect("right vector expected to exist");
  
    // Create a new rotation basis
    let basis = Basis::new_looking_at(new_forward, normal, true);

    let origin = self.base().get_global_position();
    self.base_mut().set_global_transform(Transform3D::new(
      basis, 
      origin
    ));
  }

  fn maybe_populate_trajectory_points(&mut self, virtual_planet: &Gd<VirtualPlanet>) {
    if !self.troop_is_combatting() &&
      !self.troop_activities.contains(&TroopState::Moving) &&
      self.troop_activities.contains(&TroopState::Patrolling) {

      let virtual_planet = virtual_planet.bind();

      let moving_to: Coordinates = virtual_planet
        .get_an_random_territory_coordinate(&self.deployed_to_territory);

      let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
        self.touching_surface_point.cartesian,
        virtual_planet.get_cartesian_from_coordinates(&moving_to),
        VirtualPlanet::get_planet_radius() as f32
      );

      // self.highlight_geodesic_trajectory(&geodesic_trajectory);
      self.moving_trajectory_points = geodesic_trajectory.to_vec();
      self.moving_trajectory_is_set = true;
      self.troop_activities.insert(TroopState::Moving);
    }
  }

  fn maybe_move_along_the_trajectory_and_set_orientation(&mut self) {
    if self.moving_trajectory_is_set &&
      !self.troop_activities.contains(&TroopState::Idle) {

      if self.have_future_invasion_in_the_trajectory() {
        self.no_combat_reset_trajectory(true);
        return;
      }

      let current_target = {
        let Some(current_target) = self.moving_trajectory_points
          .get(self.current_trajectory_point) else {
            godot_print!("Resetting the trajectory because the current target is None");
            self.reset_trajectory();
            return;
          };
        *current_target
      };

      let current_position = self.base().get_global_transform().origin;
      let direction = (current_target - current_position).try_normalized();
      let on_the_last_waypoint = self.current_trajectory_point == (self.moving_trajectory_points.len() -1);

      // If the direction is None, it means the current position is the same as the target
      // so we should move to the next point in the trajectory
      if direction.is_none() && !on_the_last_waypoint {
        self.current_trajectory_point = self.current_trajectory_point + 1;
        return;
      }

      let Some(direction) = direction else {
        godot_error!("Expected Troop direction to be a Vector3");
        let patrolling = self.troop_activities.contains(&TroopState::Patrolling);
        self.no_combat_reset_trajectory(patrolling);
        return
      };

      let velocity = direction * self.adopted_speed.get_speed();
      self.set_orientation(direction);
      self.base_mut().set_velocity(velocity);
      self.base_mut().move_and_slide();

      // Check if the Troop has reached the target (within a small tolerance)
      let current_distance = current_position.distance_to(current_target);
      let too_close_to_the_waypoint = current_distance < 0.1;

      if too_close_to_the_waypoint && !on_the_last_waypoint {
        self.current_trajectory_point = self.current_trajectory_point + 1;
      }

      // Finish the movement if the troop has reached the last waypoint
      if too_close_to_the_waypoint && on_the_last_waypoint {
        self.no_combat_reset_trajectory(true);
      }
    }
  }

  /// Avoids future invasion by checking if the next N points (buffer_checker) on the geodesic trajectory
  /// are on an different territory as the troop is patrolling at
  /// that happens because the point the troop is moving to is get randomly and the geodesic trajectory
  /// may pass through other territories
  fn have_future_invasion_in_the_trajectory(&mut self) -> bool {
    // Where N is {troop position} + {buffer} on the geodesic trajectory
    let buffer_checker = (CoordinatesSystem::NUM_OF_WAYPOINTS as f32 * 0.3) as usize;

    if self.troop_activities.contains(&TroopState::Patrolling) &&
      (self.current_trajectory_point + buffer_checker) < self.moving_trajectory_points.len() -1 {
      let check_future_invasion = self.moving_trajectory_points[self.current_trajectory_point + buffer_checker];
      let mut world = self.base().get_world_3d().expect("World to exist");
      let surface_point = SurfacePoint::get_surface_point(check_future_invasion, &mut world ,None)
        .expect("Expected to get surface point to check future invasion");
      if surface_point.bind().get_surface_point_metadata().territory_id.clone()
        .is_some_and(|t| t != self.deployed_to_territory) {
          return true;
      }
    }
    false
  }

  /// Resets all the states needed to reset when the troop is not in combat
  pub fn no_combat_reset_trajectory(&mut self, gets_back_to_patrolling: bool) {
    self.troop_activities.remove(&TroopState::Moving);
    self.troop_activities.remove(&TroopState::Deploying);
    self.troop_activities.insert(TroopState::Idle);

    if gets_back_to_patrolling {
      self.troop_activities.insert(TroopState::Patrolling);
    }

    self.adopted_speed = SpeedType::Patrolling;
    self.reset_trajectory();
  }

  pub fn reset_trajectory(&mut self) {
    self.current_trajectory_point = 0;
    self.moving_trajectory_points = Vec::new();
    self.moving_trajectory_is_set = false;
  }

  fn decrease_idle_timer_if_idling(&mut self, delta: f64) {
    if self.troop_activities.contains(&TroopState::Idle) {
      self.idle_timer -= delta as f32;
    }
    
    if self.idle_timer <= 0.0 {
      self.idle_timer = Self::DEFAULT_IDLE_TIMER;
      self.troop_activities.remove(&TroopState::Idle);
    } 
  }

  #[allow(dead_code)]
  /// Creates 3d Mesh Cubes all along the trajectory of the troop
  /// Used for debugging purposes
  fn highlight_geodesic_trajectory(&mut self, geodesic_trajectory: &[Vector3; CoordinatesSystem::NUM_OF_WAYPOINTS]) {
    let node_3d_name = "geodesic_mesh";

    let mut highlighted_trajectories = self.base_mut()
      .get_parent()
      .expect("Parent to exist")
      .find_child("highlighted_trajectories")
      .expect("Expected to find highlighted_trajectories");

    // Delete existing geodesic mesh
    for node in highlighted_trajectories.get_children().iter_shared() {
      highlighted_trajectories.remove_child(&node);
    }

    let mut geodesic_mesh = Node3D::new_alloc();
    highlighted_trajectories.add_child(&geodesic_mesh);
    geodesic_mesh.set_name(node_3d_name);
    geodesic_mesh.set_global_position(Vector3::new(0.0, 0.0, 0.0));

    for point in geodesic_trajectory {
      let mut material = StandardMaterial3D::new_gd();
      let mut box_mesh = BoxMesh::new_gd();
      let mut geodesic_mesh_cube = MeshInstance3D::new_alloc();

      material.set_albedo(Color::LIGHT_PINK);
      box_mesh.set_size(Vector3::new(0.02, 0.02, 0.02));
      box_mesh.set_material(&material);
      geodesic_mesh_cube.set_mesh(&box_mesh);
      geodesic_mesh_cube.set_position(*point);
      geodesic_mesh.add_child(&geodesic_mesh_cube);
    }
  }

  pub fn get_root_from_troop(&self) -> Gd<RootScene> {
    let root = self.base()
      .get_parent()
      .expect("troop parent to exist")
      .get_parent()
      .expect("root_scene to exist")
      .cast::<RootScene>();

    root
  }

  pub fn get_virtual_planet_from_troop_scope(&self) -> Gd<VirtualPlanet> {
    self
      .get_root_from_troop()
      .get_node_as::<VirtualPlanet>("virtual_planet")
  }
  
}
