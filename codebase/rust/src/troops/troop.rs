use std::collections::HashSet;

use godot::{
  classes::{BoxMesh, CharacterBody3D, ICharacterBody3D, MeshInstance3D, Sprite3D, StandardMaterial3D}, prelude::*
};
use crate::globe::{coordinates_system::{
    coordinates_system::CoordinatesSystem,
    surface_point::{Coordinates, SurfacePoint, SurfacePointMetadata},
    virtual_planet::VirtualPlanet,
  }, territories::territory::TerritoryId};

use super::{
  combat_engine::CombatStats, speed::SpeedType, surface::Surface
};

#[derive(Hash, Eq, PartialEq)]
enum TroopState {
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

  /// Whenever the troop is being deployed to another territory
  /// other than the one it was before
  Deploying,

  /// Whenever the troop is in combat
  Combating,
}

type TroopActivities = HashSet<TroopState>;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Troop {
  base: Base<CharacterBody3D>,
  /// holds troop's current location
  touching_surface_point: SurfacePointMetadata,
  /// holds the territory id the troop belongs to
  pub deployed_to_territory: TerritoryId,
  // location_situation: LocationSituation,
  surface: Surface,

  // owner: String,
  _combat_stats: CombatStats,

  troop_activities: TroopActivities,
  adopted_speed: SpeedType,

  /// indicates the time the troop will wait before moving again while patrolling
  idle_timer: f32,

  moving_trajectory_points: [Vector3; CoordinatesSystem::NUM_OF_WAYPOINTS],
  moving_trajectory_is_set: bool,
  current_trajectory_point: usize,

  /// it turns true when the troop is spawned and the orientation is set
  initial_orientation_is_set: bool,
}

#[godot_api]
impl ICharacterBody3D for Troop {
  fn init(base: Base<CharacterBody3D>) -> Troop {

    Troop {
      base: base,
      touching_surface_point: SurfacePoint::get_blank_surface_point_metadata(),
      deployed_to_territory: "".to_string(),
      surface: Surface::Land,

      _combat_stats: CombatStats::new(),

      troop_activities: HashSet::from([
        TroopState::Idle,
        TroopState::Patrolling,
      ]),

      adopted_speed: SpeedType::Patrolling,

      idle_timer: Self::DEFAULT_IDLE_TIMER,

      moving_trajectory_points: [Vector3::ZERO; CoordinatesSystem::NUM_OF_WAYPOINTS],
      moving_trajectory_is_set: false,
      current_trajectory_point: 0,

      initial_orientation_is_set: false,
    }
  }

  fn ready(&mut self) {
    self.base_mut().add_to_group(Self::TROOP_CLASS_NAME);
    // self.base_mut().add_to_group(self.owner);
    self.set_custom_collision();
    self.set_selected_sprites_visibility(false);
  }

  fn process(&mut self, delta: f64) {
    // Sets orientation first, as we use default_mesh to get the global position
    // it's important to set the orientation before setting the surface troop
    self.set_initial_orientation();

    self.set_surface_troop();
    self.check_and_change_mesh();
    self.maybe_populate_trajectory_points();
    self.maybe_move_along_the_trajectory_and_set_orientation();
    self.decrease_idle_timer(delta);
  }
}

#[godot_api]
impl Troop {

  pub const TROOP_CLASS_NAME: &'static str = "troop";

  /// Defines the time the troop will wait before moving again while patrolling
  const DEFAULT_IDLE_TIMER: f32 = 0.7;

  /// Sets troop collision layer and mask are set to be separate.
  /// To avoid misbehaviors on geodesic movement
  fn set_custom_collision(&mut self) {
    let troop_collision_layer = 2;
    let troop_collision_mask = 2;
    self.base_mut().set_collision_mask(troop_collision_layer);
    self.base_mut().set_collision_layer(troop_collision_mask);
  }

  /// Sets troop surface according to the surface_point troop is touching
  fn set_surface_troop(&mut self) {
    let surface_point = SurfacePoint::get_troop_surface_point(
      self
    );

    // if it doesn't find a surface point, it doesn't panic, just keep the previous surface
    if surface_point.is_none() {
      return;
    }

    let surface_point = surface_point.unwrap();
    if surface_point.is_in_group(&Surface::Land.to_string()) {
      self.surface = Surface::Land;
    } else {
      self.surface = Surface::Water;
    }

    self.touching_surface_point = surface_point.bind().surface_point_metadata.clone();
  }

  /// Sets troop to show the proper mesh according to the surface the troop is touching
  fn check_and_change_mesh(&mut self) {
    let mut sea_mesh = self
      .base_mut()
      .find_child("sea")
      .expect("Expected to find sea troop")
      .cast::<Node3D>();

    let mut land_mesh = self
      .base_mut()
      .find_child("land")
      .expect("Expected to find land troop")
      .cast::<Node3D>();

    if self.surface == Surface::Land {
      sea_mesh.set_visible(false);
      land_mesh.set_visible(true);
    } else {
      sea_mesh.set_visible(true);
      land_mesh.set_visible(false);
    }
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
  fn set_orientation(&mut self, trajectory_vector: Vector3) {
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

  fn _is_on_self_land(&self) -> bool {
    // TICKET: #12
    true
  }

  fn _is_on_ally_land(&self) -> bool {
    // TICKET: #12
    false
  }

  fn maybe_populate_trajectory_points(&mut self) {
    if !self.troop_activities.contains(&TroopState::Combating) &&
      !self.troop_activities.contains(&TroopState::Moving) &&
      self.troop_activities.contains(&TroopState::Patrolling) {

      let virtual_planet = self.get_virtual_planet_from_troop_scope();
      let virtual_planet = virtual_planet.bind();

      let moving_to: Coordinates = virtual_planet
        .get_an_random_territory_coordinate(&self.deployed_to_territory);

      let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
        self.touching_surface_point.cartesian,
        virtual_planet.get_cartesian_from_coordinates(&moving_to),
        VirtualPlanet::get_planet_radius() as f32
      );

      // self.highlight_geodesic_trajectory(&geodesic_trajectory);
      self.moving_trajectory_points = geodesic_trajectory;
      self.moving_trajectory_is_set = true;
      self.troop_activities.insert(TroopState::Moving);
    }
  }

  pub fn set_order_to_move_to(&mut self, destination: Vector3, territory_id: &TerritoryId) {
    self.reset_trajectory(false);
    self.troop_activities.insert(TroopState::Moving);
    self.troop_activities.insert(TroopState::Deploying);
    self.troop_activities.remove(&TroopState::Patrolling);

    let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
      self.touching_surface_point.cartesian,
      destination,
      VirtualPlanet::get_planet_radius() as f32
    );

    self.moving_trajectory_points = geodesic_trajectory;
    self.moving_trajectory_is_set = true;
    self.adopted_speed = SpeedType::FightOrFlight;
    self.deployed_to_territory = territory_id.clone();
  }

  fn maybe_move_along_the_trajectory_and_set_orientation(&mut self) {
    if self.moving_trajectory_is_set && !self.troop_activities.contains(&TroopState::Idle) {
      if self.have_future_invasion_in_the_trajectory() {
        self.reset_trajectory(true);
        return;
      }

      let current_target = self.moving_trajectory_points[self.current_trajectory_point];
      let current_position = self.base().get_global_transform().origin;
      let direction = (current_target - current_position).try_normalized();
      let on_the_last_waypoint = self.current_trajectory_point == (self.moving_trajectory_points.len() -1);

      // If the direction is None, it means the current position is the same as the target
      // so we should move to the next point in the trajectory
      if direction.is_none() && !on_the_last_waypoint {
        self.current_trajectory_point = self.current_trajectory_point + 1;
        return;
      }

      let direction = direction.expect("Expected Troop direction to be a Vector3");
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
        self.reset_trajectory(true);
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
      let world = self.base().get_world_3d().expect("World to exist");
      let surface_point = SurfacePoint::get_surface_point(check_future_invasion, world ,None)
        .expect("Expected to get surface point to check future invasion");
      if surface_point.bind().get_surface_point_metadata().territory_id.clone()
        .is_some_and(|t| t != self.deployed_to_territory) {
          return true;
      }
    }
    false
  }

  fn reset_trajectory(&mut self, gets_back_to_patrolling: bool) {
    self.troop_activities.remove(&TroopState::Moving);
    self.troop_activities.remove(&TroopState::Deploying);
    self.troop_activities.insert(TroopState::Idle);

    if gets_back_to_patrolling {
      self.troop_activities.insert(TroopState::Patrolling);
    }

    self.adopted_speed = SpeedType::Patrolling;
    self.current_trajectory_point = 0;
    self.moving_trajectory_points = [Vector3::ZERO; CoordinatesSystem::NUM_OF_WAYPOINTS];
    self.moving_trajectory_is_set = false;
  }

  fn decrease_idle_timer(&mut self, delta: f64) {
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

  fn get_virtual_planet_from_troop_scope(&self) -> Gd<VirtualPlanet> {
    let root = self.base()
      .get_parent()
      .expect("troop parent to exist")
      .get_parent()
      .expect("root_scene to exist");

    let virtual_planet = root
      .find_child("virtual_planet")
      .expect("virtual_planet to exist")
      .cast::<VirtualPlanet>();

    virtual_planet
  }

  pub fn select_troop(&mut self) {
    // TICKET: #63 Put it on the HUD
    self.troop_activities.insert(TroopState::Selected);

    self.set_selected_sprites_visibility(true);
  }

  pub fn deselect_troop(&mut self) {
    self.troop_activities.remove(&TroopState::Selected);

    self.set_selected_sprites_visibility(false);
  }

  fn set_selected_sprites_visibility(&mut self, visible: bool) {
    let mut land_selected_sprite = self.base_mut().get_node_as::<Sprite3D>("land/selected");
    let mut sea_selected_sprite = self.base_mut().get_node_as::<Sprite3D>("sea/selected");

    land_selected_sprite.set_visible(visible);
    sea_selected_sprite.set_visible(visible);
  }
}
