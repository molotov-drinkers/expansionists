use godot::{classes::{BoxMesh, CharacterBody3D, ICharacterBody3D, MeshInstance3D, StandardMaterial3D}, prelude::*};
use crate::{globe::coordinates_system::{coordinates_system::CoordinatesSystem, surface_point::Coordinates, virtual_planet::VirtualPlanet}, root::root::RootScene};

pub enum LocationSituation {
  SelfLand,
  AllyLand,
  NeutralLand,
  EnemyLand,
}

pub enum Surface {
  Land,
  Water,

  // future_version:
  // Air, (Planes)
  // Space, (Satellites)
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

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Troop {
  base: Base<CharacterBody3D>,

  pub located_at: Coordinates,
  pub location_situation: LocationSituation,
  pub surface_type: Surface,

  pub owner: String,

  pub combat_stats: CombatStats,

  // used for animation inside of the territory
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
      surface_type: Surface::Land,

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
      moving_speed: 0.5,
      walking_trajectory_points: vec![],
      current_trajectory_point: 0,
    }
  }

  fn ready(&mut self) {
    // godot_print!("Troop ready");
    // To avoid misbehaviors on geodesic movement, the troop collision layer and mask 
    // are set to be separate 
    let troop_collision_layer = 2;
    let troop_collision_mask = 2;

    self.base_mut().set_collision_mask(troop_collision_layer);
    self.base_mut().set_collision_layer(troop_collision_mask);
  }

  fn physics_process(&mut self, _delta: f64) {
    self.set_orientation(None);
    self.maybe_populate_trajectory_points();
    self.maybe_move_along_the_trajectory_and_set_orientation();
  }
}

impl Troop {
  fn set_orientation(&mut self, trajectory_vector: Option<Vector3>) {
    let normal = self.base().get_global_position().normalized();  // This is the "up" direction on the surface

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

  fn _is_on_self_land(&self) -> bool {
    // TODO: implement
    // self.located_at;
    true
  }

  fn _is_on_ally_land(&self) -> bool {
    // TODO: implement
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
          // "great_lakes"
          // "kangaroos"
          // "unclaimed_area"
          // "latinos"
          "west_slavs"
          // "nordics"
          // "most_isolated_city"
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

      godot_print!("Troop is moving from {:?}", self.located_at);
      godot_print!("Troop is moving to {:?}", randomly_walking_to);
      // godot_print!("Trajectory is {:?}", self.walking_trajectory_points);
    }
  }

  fn maybe_move_along_the_trajectory_and_set_orientation(&mut self) {
    if !self.walking_trajectory_points.is_empty() {

      let current_target = self.walking_trajectory_points[self.current_trajectory_point];
      let current_position = self.base().get_global_transform().origin;

      let direction = (current_target - current_position).try_normalized();
      if direction.is_none() { return; }

      let direction = direction.unwrap();
      let velocity = direction * self.moving_speed;

      self.set_orientation(Some(direction));
      self.base_mut().set_velocity(velocity);
      self.base_mut().move_and_slide();

      // Check if the Troop has reached the target (within a small tolerance)
      let current_distance = current_position.distance_to(current_target);

      // godot_print!("Distance to target: {:?}", current_distance);
      if current_distance < 0.05 && self.current_trajectory_point < self.walking_trajectory_points.len() {
        // Move to the next waypoint
        // godot_print!("-- Moving to the next waypoint");
        self.current_trajectory_point = self.current_trajectory_point + 1;
      }

      // godot_print!("self.current_trajectory_point: {:?}", self.current_trajectory_point);
      // godot_print!("self.walking_trajectory_points.len(): {:?}", self.walking_trajectory_points.len());
      // Finish the movement if the troop has reached the last waypoint
      if current_distance < 0.05 && self.current_trajectory_point == self.walking_trajectory_points.len() /* - 1 */ {
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
pub fn troop_spawner(root_scene: &mut RootScene) {
    
  // TODO: Refactor this spaghetti
  // STEP 1: GETTING TERRITORIES
  let virtual_planet = root_scene.base_mut()
    .get_node_as::<VirtualPlanet>("virtual_planet");
  let virtual_planet = &virtual_planet.bind();

  // STEP 2: GETTING TERRITORY LAND LOCATION TO SPAWN TROOP
  let hard_coded_territory = "atlantic_forest";
  let coordinate = VirtualPlanet::get_an_random_territory_coordinate(
    &virtual_planet,
    hard_coded_territory
  );

  let coordinate_metadata = virtual_planet.coordinate_map
    .get(&coordinate)
    .expect("Coordinate expected to exist");
  let cartesian = coordinate_metadata.cartesian;

  // STEP 3: SPAWNING TROOP
  let player_color = Color::FLORAL_WHITE;
  let mut material = StandardMaterial3D::new_gd();
  material.set_albedo(player_color);
  let scene: Gd<PackedScene> = load("res://scenes/troop_scene.tscn");
  let mut new_troop = scene.instantiate_as::<Troop>();
  //TODO: generate a troop ID base on: territory_id + player_id + timestamp
  //TODO: use troop_id to acknologe the troop location along the planet
  let troop_id = "troop";
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