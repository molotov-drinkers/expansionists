use godot::{classes::{CharacterBody3D, ICharacterBody3D, MeshInstance3D, StandardMaterial3D}, prelude::*};
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
  // Air,
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
    }
  }

  fn ready(&mut self) {
    // godot_print!("Troop ready");
  }

  fn physics_process(&mut self, _delta: f64) {
    Self::start_random_walk_within_territory(self);
  }
}

impl Troop {
  fn is_on_self_land(&self) -> bool {
    // TODO: implement
    self.located_at;
    true
  }

  fn is_on_ally_land(&self) -> bool {
    // TODO: implement
    false
  }


  fn start_random_walk_within_territory(&mut self) {
    if
      (self.is_on_self_land() || self.is_on_ally_land()) &&
      self.combat_stats.in_combat == false {

      let virtual_planet = self.get_virtual_planet_from_troop_scope();
      let randomly_walking_to = virtual_planet
        .bind()
        .get_another_territory_coordinate(self.located_at);

      self.is_moving = true;
      self.randomly_walking_to = randomly_walking_to;

      // TODO: call start the movement implementation
      CoordinatesSystem::_get_geodesic_trajectory(
        self.located_at,
        randomly_walking_to,
        &virtual_planet.bind().coordinate_map
      );

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
  let player_color = Color::DARK_CYAN;
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