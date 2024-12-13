use std::fmt;

use godot::{
  classes::{BoxMesh, CharacterBody3D, ICharacterBody3D, MeshInstance3D, StandardMaterial3D}, prelude::*
};
use crate::{
  globe::coordinates_system::{
    coordinates_system::CoordinatesSystem,
    surface_point::{Coordinates, SurfacePoint},
    virtual_planet::VirtualPlanet,
  },
  root::root::RootScene,
};

pub enum LocationSituation {
  SelfLand,
  AllyLand,
  NeutralLand,
  EnemyLand,
}

#[derive(PartialEq, Debug)]
pub enum Surface {
  Land,
  Water,

  // future_version:
  // Air, // (Planes)
  // Space, // (Satellites)
}

impl fmt::Display for Surface {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Surface::Land => write!(f, "land"),
      Surface::Water =>  write!(f, "water"),
    }
  }
}

pub enum FighthingBehavior {
  /// will fight any non-ally troop who crosses by it doesn't matter the territory
  Beligerent,

  /// will only fight if attacked or if it's territory is attacked
  Pacifist,
}

pub struct CombatStats {
  pub in_combat: bool,
  pub in_after_combat: bool,

  pub damage: i32,
  pub hp: i32,
  pub speed: i32,
  pub alive: bool,

  pub fighting_behavior: FighthingBehavior,
}

const ORIGIN: &str = "atlantic_forest";
const DEST: &str = "horn";

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Troop {
  base: Base<CharacterBody3D>,

  pub located_at: Coordinates,
  pub location_situation: LocationSituation,
  pub surface: Surface,

  pub owner: String,

  pub combat_stats: CombatStats,

  /// used for animation inside of the territory
  pub is_moving: bool,
  pub randomly_walking_to: Coordinates,
  pub moving_speed: f32,
  pub walking_trajectory_points: Vec<Vector3>,
  pub current_trajectory_point: usize,
}

#[godot_api]
impl ICharacterBody3D for Troop {
  fn init(base: Base<CharacterBody3D>) -> Troop {

    Troop {
      base: base,
      
      located_at: (0, 0),
      location_situation: LocationSituation::NeutralLand,
      surface: Surface::Land,

      owner: "".to_string(),

      combat_stats: CombatStats {
        in_combat: false,
        in_after_combat: false,
        damage: 0,
        hp: 0,
        speed: 0,
        alive: false,
        fighting_behavior: FighthingBehavior::Beligerent,
      },

      is_moving: false,
      randomly_walking_to: (0, 0),
      moving_speed: 0.2,
      walking_trajectory_points: vec![],
      current_trajectory_point: 0,
    }
  }

  fn ready(&mut self) {
    self.set_custom_collision();
  }

  fn physics_process(&mut self, _delta: f64) {
    self.set_surface();
    self.check_and_change_mesh();
    self.set_orientation(None);
    self.maybe_populate_trajectory_points();
    self.maybe_move_along_the_trajectory_and_set_orientation();
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
  fn set_orientation(&mut self, trajectory_vector: Option<Vector3>) {
    // This is the "up" direction on the surface
    let normal = self.base().get_global_position().normalized();

    // If it's moving, gets trajectory forward vector
    let forward = if trajectory_vector.is_some() {
      trajectory_vector.unwrap()
    } else {
      // Choose a forward direction (assuming the character faces the -Z direction by default)
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

  #[func]
  fn set_surface(&mut self) {
    let surface_point = SurfacePoint::get_troop_surface_point(
      self
    );

    if surface_point.is_in_group(&Surface::Land.to_string()) {
      self.surface = Surface::Land;
    } else {
      self.surface = Surface::Water;
    }
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
    if
      // (self._is_on_self_land() || self._is_on_ally_land()) &&
      self.combat_stats.in_combat == false &&
      self.is_moving == false {

      let virtual_planet = self.get_virtual_planet_from_troop_scope();
      let randomly_walking_to = virtual_planet
        .bind()
        // .get_another_territory_coordinate(self.located_at);
        // TODO: get back to get_another_territory_coordinate implementation
        .get_an_random_territory_coordinate(
          DEST.into(),
        );

      let geodesic_trajectory = CoordinatesSystem::get_geodesic_trajectory(
        self.located_at,
        randomly_walking_to,
        &virtual_planet.bind().coordinate_map,
        VirtualPlanet::get_planet_radius() as f32
      );

      // self._highlight_geodesic_trajectory(&geodesic_trajectory);

      self.walking_trajectory_points = geodesic_trajectory;
      self.is_moving = true;
      self.randomly_walking_to = randomly_walking_to;

    }
  }

  fn maybe_move_along_the_trajectory_and_set_orientation(&mut self) {

    if self.walking_trajectory_points.len() != 0 {
      let current_target = self.walking_trajectory_points[self.current_trajectory_point];
      let current_position = self.base().get_global_transform().origin;
      
      let direction = (current_target - current_position).try_normalized();
      if direction.is_none() && self.current_trajectory_point < self.walking_trajectory_points.len() {
        self.current_trajectory_point = self.current_trajectory_point + 1;
        return;
      }

      let direction = direction.unwrap();
      let velocity = direction * self.moving_speed;

      self.set_orientation(Some(direction));
      self.base_mut().set_velocity(velocity);
      self.base_mut().move_and_slide();

      // Check if the Troop has reached the target (within a small tolerance)
      let current_distance = current_position.distance_to(current_target);

      if current_distance < 0.1 && self.current_trajectory_point < self.walking_trajectory_points.len() {
        self.current_trajectory_point = self.current_trajectory_point + 1;
      }

      // Finish the movement if the troop has reached the last waypoint
      if current_distance < 0.1 && self.current_trajectory_point == self.walking_trajectory_points.len() {
        godot_print!("--- Troop has reached the destination");
        self.is_moving = false;
        self.current_trajectory_point = 0;
        self.walking_trajectory_points.clear();


        self.combat_stats.in_combat = true;
      }
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


/// Called from root.rs
pub fn troop_spawner(root_scene: &mut RootScene, virtual_planet: &VirtualPlanet, troops_spawn: i8) {
  let temp_hard_coded_territory = ORIGIN.into();
  let coordinate = VirtualPlanet::get_an_random_territory_coordinate(
    &virtual_planet,
    temp_hard_coded_territory
  );

  let cartesian = virtual_planet
    .coordinate_map
    .get(&coordinate)
    .expect("Coordinate expected to exist")
    .cartesian;

  // STEP 3: SPAWNING TROOP
  let player_color = Color::FLORAL_WHITE;
  let mut material = StandardMaterial3D::new_gd();
  material.set_albedo(player_color);
  let new_troop: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
  let mut new_troop = new_troop.instantiate_as::<Troop>();

  // TICKET: #39 generate a troop ID base on: territory_id + player_id + timestamp
  let troop_id = format!("troop ... {:}-{:}", troops_spawn, temp_hard_coded_territory);
  new_troop.set_name(&troop_id.to_godot());
  new_troop.bind_mut().located_at = coordinate;

  // For organization matter, new_troops are spawn under /root_scene/troops
  root_scene.base()
    .find_child("troops") 
    .expect("troops to exist")
    .add_child(&new_troop);

  new_troop.set_position(cartesian);

  let troop_node = new_troop.find_child("default_mesh").expect("MeshInstance3D to exist");
  let mut troop_mesh = troop_node.cast::<MeshInstance3D>();
  troop_mesh.set_surface_override_material(0, &material);

}