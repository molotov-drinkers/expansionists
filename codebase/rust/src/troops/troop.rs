
use godot::{
  classes::{BoxMesh, CharacterBody3D, ICharacterBody3D, MeshInstance3D, StandardMaterial3D}, prelude::*
};
use crate::globe::{coordinates_system::{
    coordinates_system::CoordinatesSystem,
    surface_point::{Coordinates, SurfacePoint, SurfacePointMetadata},
    virtual_planet::VirtualPlanet,
  }, territory::types::TerritoryId};

use super::{
  combat_engine::CombatStats,
  surface::Surface
};

const DEST: &str = "antartica_peninsula";
const IDLE_TIMER: f32 = 0.2;

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
  combat_stats: CombatStats,

  is_moving: bool,
  is_patrolling: bool,
  in_territory_moving_speed: f32,
  /// indicates the time the troop will wait before moving again while patrolling
  idle_timer: f32,

  walking_trajectory_points: Vec<Vector3>,
  current_trajectory_point: usize,

}

#[godot_api]
impl ICharacterBody3D for Troop {
  fn init(base: Base<CharacterBody3D>) -> Troop {

    Troop {
      base: base,
      touching_surface_point: SurfacePoint::get_blank_surface_point_metadata(),
      deployed_to_territory: "".to_string(),
      surface: Surface::Land,

      combat_stats: CombatStats::new(),

      is_moving: false,
      is_patrolling: false,
      in_territory_moving_speed: 0.05,
      idle_timer: IDLE_TIMER,

      walking_trajectory_points: vec![],
      current_trajectory_point: 0,
      
    }
  }

  fn ready(&mut self) {
    self.set_custom_collision();
  }

  fn physics_process(&mut self, delta: f64) {    
    // Sets orientation first, as we use default_mesh to get the global position
    // it's important to set the orientation before setting the surface troop
    self.set_orientation(None);

    self.set_surface_troop();
    self.check_and_change_mesh();
    self.maybe_populate_trajectory_points();
    self.maybe_move_along_the_trajectory_and_set_orientation();
    self.decrease_idle_timer(delta);
  }
}

#[godot_api]
impl Troop {
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

  /// Sets orientation to respect the globe trajectory and gravity
  /// if the troop is moving, it will set the orientation to the direction it's moving
  fn set_orientation(&mut self, trajectory_vector: Option<Vector3>) {
    // This is the "up" direction on the surface
    let normal = self.base().get_global_position().normalized();

    // If it's moving, gets trajectory forward vector
    let forward = if trajectory_vector.is_some() {
      trajectory_vector.unwrap()
    } else {
      // Choose a forward direction (assuming the troop faces the -Z direction by default)
      Vector3::new(0.0, 0.0, -1.0)
    };
  
    // Calculate the right vector using the cross product (normal x forward)
    let right = normal
      .cross(forward)
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
    if self.combat_stats.in_combat == false && self.is_moving == false {
      let virtual_planet = self.get_virtual_planet_from_troop_scope();
      let virtual_planet = virtual_planet.bind();

      let moving_to: Coordinates = match self.is_patrolling {
        true => virtual_planet
          .get_an_random_territory_coordinate(&self.deployed_to_territory),
        
        // TICKET: #12 This will be an order to move to other territory
        false => virtual_planet
          .get_an_random_territory_coordinate(DEST.into()),
      };

      let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
        self.touching_surface_point.cartesian,
        virtual_planet.get_cartesian_from_coordinates(&moving_to),
        VirtualPlanet::get_planet_radius() as f32
      );

      // self._highlight_geodesic_trajectory(&geodesic_trajectory);
      self.walking_trajectory_points = geodesic_trajectory;
      self.is_moving = true;

    }
  }

  fn maybe_move_along_the_trajectory_and_set_orientation(&mut self) {
    if self.walking_trajectory_points.len() != 0 && self.idle_timer == 0.0 {
      if self.have_future_invasion_in_the_trajectory() {
        self.reset_trajectory();
        return;
      }

      let current_target = self.walking_trajectory_points[self.current_trajectory_point];
      let current_position = self.base().get_global_transform().origin;
      let direction = (current_target - current_position).try_normalized();
      let on_the_last_waypoint = self.current_trajectory_point == (self.walking_trajectory_points.len() -1);

      // If the direction is None, it means the current position is the same as the target
      // so we should move to the next point in the trajectory
      if direction.is_none() && !on_the_last_waypoint {
        self.current_trajectory_point = self.current_trajectory_point + 1;
        return;
      }

      let direction = direction.unwrap();
      let velocity = direction * self.in_territory_moving_speed;
      self.set_orientation(Some(direction));
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
        self.reset_trajectory();
      }
    }
  }

  /// Avoids future invasion by checking if the next N points (buffer_checker) on the geodesic trajectory
  /// are on an different territory as the troop is patrolling at
  /// that happens because the point the troop is moving to is get randomly and the geodesic trajectory
  /// may pass through other territories
  fn have_future_invasion_in_the_trajectory(&mut self) -> bool {
    // Where N is troop position + buffer on the geodesic trajectory
    let buffer_checker = 5;

    if self.is_patrolling && (self.current_trajectory_point + buffer_checker) < self.walking_trajectory_points.len() -1 {
      let check_future_invasion = self.walking_trajectory_points[self.current_trajectory_point + buffer_checker];
      let world = self.base().get_world_3d().expect("World to exist");
      let surface_point = SurfacePoint::get_surface_point(check_future_invasion, world)
        .expect("Expected to get surface point");
      if surface_point.bind().get_surface_point_metadata().territory_id.clone()
        .is_some_and(|t| t != self.deployed_to_territory) {
          return true;
      }
    }
    false
  }

  fn reset_trajectory(&mut self) {
    self.is_moving = false;
    self.current_trajectory_point = 0;
    self.walking_trajectory_points.clear();
    self.reset_idle_timer();
  }

  fn reset_idle_timer(&mut self) {
    self.idle_timer = IDLE_TIMER;
  }

  fn decrease_idle_timer(&mut self, delta: f64) {
    if self.idle_timer <= 0.0 {
      self.idle_timer = 0.0; // Ensure it doesn't go negative
    } else {
      self.idle_timer -= delta as f32;
    }
  }

  /// Creates 3d Meshe Cubes all along the trajectory of the troop
  /// Used for debugging purposes
  fn _highlight_geodesic_trajectory(&mut self, geodesic_trajectory: &Vec<Vector3>) {
    let node_3d_name = "geodesic_mesh";

    // Delete existing geodesic mesh
    for child in self.base().get_children().iter_shared() {
      if child.get_name() == node_3d_name.into() {
        self.base_mut().remove_child(&child);
      }
    }

    let mut geodesic_mesh = Node3D::new_alloc();
    // Adding the geodesic mesh to the troop right away to be able to set the global position
    // This way we avoid having the geodesic mesh with the relative position of the troop
    self.base_mut().add_child(&geodesic_mesh);
    geodesic_mesh.set_name(node_3d_name);
    geodesic_mesh.set_global_position(Vector3::new(0.0, 0.0, 0.0));

    for point in geodesic_trajectory {
      let mut material = StandardMaterial3D::new_gd();
      let mut box_mesh = BoxMesh::new_gd();
      let mut geodesic_mesh_cube = MeshInstance3D::new_alloc();

      material.set_albedo(Color::GOLD);
      box_mesh.set_size(Vector3::new(0.02, 0.02, 0.02));
      box_mesh.set_material(&material);
      geodesic_mesh_cube.set_mesh(&box_mesh);
      geodesic_mesh_cube.set_position(*point);
      // geodesic_mesh_cube.set_surface_override_material(0, &material);
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
}
