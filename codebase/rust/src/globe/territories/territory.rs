use std::collections::{HashMap, HashSet};
use std::fmt;

use godot::{builtin::Color, classes::{MeshInstance3D, StandardMaterial3D}, prelude::*};

use crate::player::player::PlayerId;
use crate::{globe::coordinates_system::surface_point::Coordinates, player::player::PlayerStaticInfo};

#[derive(Eq, PartialEq, Hash)]
pub enum Continent {
  Africa,
  Asia,
  Europe,
  NorthAmerica,
  Oceania,
  SouthAmerica,
  Antarctica,
  Special,
}

impl fmt::Display for Continent {
  /// allows to use `&Continent::Africa.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Continent::Africa => write!(f, "africa"),
      Continent::Asia => write!(f, "asia"),
      Continent::Europe => write!(f, "europe"),
      Continent::NorthAmerica => write!(f, "north_america"),
      Continent::Oceania => write!(f, "oceania"),
      Continent::SouthAmerica => write!(f, "south_america"),
      Continent::Antarctica => write!(f, "antarctica"),
      Continent::Special => write!(f, "special"),
    }
  }
}

#[derive(Eq, PartialEq, Hash)]
pub enum SubContinent {
  MiddleEast,
  InteriorAsia,
  IndianSubcontinent,
  SoutheastAsia,
  EastAsia,
  EuropeRelatedAsia,
}

impl fmt::Display for SubContinent {
  /// allows to use `&SubContinent::MiddleEast.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SubContinent::MiddleEast => write!(f, "middle_east"),
      SubContinent::InteriorAsia => write!(f, "interior_asia"),
      SubContinent::IndianSubcontinent => write!(f, "indian_subcontinent"),
      SubContinent::SoutheastAsia => write!(f, "southeast_asia"),
      SubContinent::EastAsia => write!(f, "east_asia"),
      SubContinent::EuropeRelatedAsia => write!(f, "europe_related_asia"),
    }
  }
}

pub struct Location {
  pub continent: Continent,
  pub sub_continent: Option<SubContinent>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Size {
  Tiny,
  Small,
  Medium,
  Large,
  Huge,
  Humongous,
  None,
}

impl fmt::Display for Size {
  /// allows to use `&Size::Tiny.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Size::Tiny => write!(f, "tiny"),
      Size::Small => write!(f, "small"),
      Size::Medium => write!(f, "medium"),
      Size::Large => write!(f, "large"),
      Size::Huge => write!(f, "huge"),
      Size::Humongous => write!(f, "humongous"),
      Size::None => write!(f, "none"),
    }
  }
}

pub type TerritoryId = String;
pub type Territories = HashMap<TerritoryId, Territory>;

/// TroopId is a string name, is the base().get_name().to_string() of a troop
pub type TroopId = String;

#[derive(Hash, Eq, PartialEq)]
pub enum TerritoryState {
  Unoccupied,
  UnoccupiedUnderConflict,
  OccupationInProgress,
  Occupied,
  OccupiedUnderConflict,
}

/// Not a Godot class, look at `land.rs`, `surface_point.rs` and
/// `virtual_planet.rs` for the Godot classes related to territories
pub struct Territory {
  pub territory_id: TerritoryId,
  pub location: Location,

  pub coordinates: Vec<Coordinates>,
  pub size: Size,

  pub organic_max_troops: i32,
  troops_growth_velocity: f32,
  pub seconds_to_spawn_troop: f64,
  pub spawner_location: Vector3,
  pub territory_states: HashSet<TerritoryState>,

  /// It counts which troops are deployed in the territory, not necessarily arrived
  all_troops_deployed: HashSet<TroopId>,
  /// It counts which troops are deployed in the territory, not necessarily arrived, filtering by player
  all_troops_deployed_by_player: HashMap<PlayerId, HashSet<TroopId>>,
  /// It counts which troops are have arrived to the territory
  pub all_troops_deployed_and_arrived: HashSet<TroopId>,
  /// It counts which troops are have arrived to the territory, filtering by player
  pub all_troops_deployed_and_arrived_by_player: HashMap<PlayerId, HashSet<TroopId>>,
  pub has_troops_from_different_players: bool,

  pub time_to_be_conquered: f64,
  pub conquering_progress_per_second: f64,
  /// TICKET: #93 This could possibly be a list of players trying to conquer the territory
  pub player_trying_to_conquer: Option<PlayerStaticInfo>,
  pub progress_to_reset_idle_conquering: f64,

  pub current_ruler: Option<PlayerStaticInfo>,
  pub next_troop_progress: f64,
  pub valid_seconds_elasped_since_last_troop: f64,
}

pub enum ColorChange {
  Lighten,
  Darken,
  SuperDarken,
  Exact,
}

impl Territory {
  /// It's a factor that helps setting how many troops a territory can generate
  /// the lower the value, the less troops a territory can generate
  const BASE_TROOP_NUMBER_PER_TERRITORY: f32 = 0.02;

  /// organic_max_troops is clamped between 1 and MAX_NUMBER_OF_TROOPS_GENERATED_PER_TERRITORY
  const MAX_NUMBER_OF_TROOPS_GENERATED_PER_TERRITORY: i32 = 20;

  /// It's a factor that helps setting how fast the troops grow in a territory
  /// the lower the value, the slower the troops grow
  const BASE_TROOP_GROWTH_VELOCITY: f32 = 0.001;

  /// It's a factor helping controlling the speed of the troops spawning
  /// the lower the value, the faster the troops spawn
  const BASE_SECONDS_FOR_A_TROOP_TO_SPAWN: f64 = 3.;

  /// It's a factor that helps setting how much time a player has to occupying a territory
  /// to take control of it and become a ruler
  /// the lower the value, the faster the territories are conquered
  const BASE_TERRITORY_OCCUPATION_TIME: f64 = 0.2;

  /// If user stops trying to conquer a territory, the progress to conquer it is reset
  const _SECONDS_TO_RESET_IDLE_CONQUERING: f64 = 10.;


  pub fn get_base_territory(territory_id: &str, continent: Continent, sub_continent: Option<SubContinent>) -> Territory {
    Territory {
      territory_id: territory_id.to_string(),
      location: Location { continent, sub_continent },

      // Fields below are filled on the fly
      coordinates: Vec::new(),
      size: Size::None,
      organic_max_troops: 0,
      troops_growth_velocity: 0.1,
      seconds_to_spawn_troop: 10.,
      spawner_location: Vector3::ZERO,

      territory_states: HashSet::from([
        TerritoryState::Unoccupied,
      ]),

      all_troops_deployed: HashSet::new(),
      all_troops_deployed_by_player: HashMap::new(),

      all_troops_deployed_and_arrived: HashSet::new(),
      all_troops_deployed_and_arrived_by_player: HashMap::new(),

      has_troops_from_different_players: false,

      time_to_be_conquered: 10.,
      conquering_progress_per_second: 0.1,
      player_trying_to_conquer: None,
      progress_to_reset_idle_conquering: 0.,

      current_ruler: None,
      next_troop_progress: 0.,
      valid_seconds_elasped_since_last_troop: 0.,
    }
  }

  fn continent_to_color(continent: &Continent) -> Color {
    match continent {
      Continent::Africa => /* Color::LIGHT_SLATE_GRAY */  Color::ROSY_BROWN.lightened(0.7),
      Continent::Asia => /* Color::GREEN_YELLOW */  Color::ROSY_BROWN.lightened(0.7),
      Continent::Europe => /* Color::SKY_BLUE */  Color::ROSY_BROWN.lightened(0.7),
      Continent::NorthAmerica => /* Color::INDIAN_RED */  Color::ROSY_BROWN.lightened(0.7),
      Continent::Oceania => /* Color::BURLYWOOD */  Color::ROSY_BROWN.lightened(0.7),
      Continent::SouthAmerica => /* Color::TOMATO */  Color::ROSY_BROWN.lightened(0.7),
      Continent::Antarctica => /* Color::GRAY */  Color::ROSY_BROWN.lightened(0.7),
      Continent::Special => /* Color::GOLD */  Color::ROSY_BROWN.lightened(0.7),
    }
  }

  pub fn get_territory_color(sub_continent: &Option<SubContinent>, continent: &Continent) -> Color {
    match sub_continent {
      Some(SubContinent::MiddleEast) => /* Color::GREEN */ Color::ROSY_BROWN.lightened(0.7),
      Some(SubContinent::InteriorAsia) => /* Color::WEB_GREEN */ Color::ROSY_BROWN.lightened(0.7),
      Some(SubContinent::IndianSubcontinent) => /* Color::LAWN_GREEN */ Color::ROSY_BROWN.lightened(0.7),
      Some(SubContinent::SoutheastAsia) => /* Color::LIME_GREEN */ Color::ROSY_BROWN.lightened(0.7),
      Some(SubContinent::EastAsia) => /* Color::GREEN_YELLOW */ Color::ROSY_BROWN.lightened(0.7),
      Some(SubContinent::EuropeRelatedAsia) => /* Color::LIGHT_GREEN */ Color::ROSY_BROWN.lightened(0.7),
      None => Self::continent_to_color(&continent)
    }
  }

  pub fn set_shade_color_to_territory(territory_mesh: Gd<MeshInstance3D>, color_change: ColorChange) {
    let base_color = territory_mesh.get_meta("current_base_color");
    let base_color = base_color.to::<Color>();
    
    let color = match color_change {
      ColorChange::Lighten => base_color.lightened(0.5),
      ColorChange::Darken => base_color.darkened(0.25),
      ColorChange::SuperDarken => base_color.darkened(0.5),
      ColorChange::Exact => base_color,
    };

    Self::set_color_to_active_material(&territory_mesh, color);
  }

  pub fn set_color_to_active_material(territory_mesh: &Gd<MeshInstance3D>, color: Color){
    territory_mesh
      .get_active_material(0)
      .expect("Expected to have an active material")
      .try_cast::<StandardMaterial3D>()
      .expect("Expected mesh territory's material to be castable to StandardMaterial3D")
      .set_albedo(color);
  }

  pub fn checking_territory(territory_mesh: Gd<MeshInstance3D>) {
    Self::set_shade_color_to_territory(territory_mesh, ColorChange::Darken);
  }

  pub fn unchecking_territory(territory_mesh: Gd<MeshInstance3D>) {
    Self::set_shade_color_to_territory(territory_mesh, ColorChange::Exact);
  }

  pub fn clicking_territory(territory_mesh: Gd<MeshInstance3D>) {
    Self::set_shade_color_to_territory(territory_mesh, ColorChange::SuperDarken);
  }

  /// Should be called when the coordinates of the territory are set
  pub fn set_territory_size(&mut self) {
    // TODO: Instead of using size, use coordinates.len to calculate organic_max_troops and troops_growth_velocity
    let num_of_coordinates = self.coordinates.len();

    self.size = match num_of_coordinates {
      1..=40 => Size::Tiny,
      41..=150 => Size::Small,
      151..=450 => Size::Medium,
      451..=750 => Size::Large,
      751..=1000 => Size::Huge,
      1001..=9999 => Size::Humongous,
      _ => Size::None,
    };
  }
  
  /// Should be called when the coordinates of the territory are set
  pub fn set_troops_growth_velocity_and_secs_to_spawn(&mut self) {
    let num_of_coordinates = self.coordinates.len();

    self.troops_growth_velocity = (Self::BASE_TROOP_GROWTH_VELOCITY * num_of_coordinates as f32)
      .clamp(0.01, 3.);

    self.seconds_to_spawn_troop = Self::BASE_SECONDS_FOR_A_TROOP_TO_SPAWN / self.troops_growth_velocity as f64;
  }

  /// Should be called when the coordinates of the territory are set
  pub fn set_organic_max_troops(&mut self) {
    let num_of_coordinates = self.coordinates.len();

    self.organic_max_troops = ((
      Self::BASE_TROOP_NUMBER_PER_TERRITORY * num_of_coordinates as f32) as i32)
      .clamp(1, Self::MAX_NUMBER_OF_TROOPS_GENERATED_PER_TERRITORY);
  }

  /// The greater the territory, the longer it takes to be conquered
  pub fn set_time_to_be_conquered(&mut self) {
    let num_of_coordinates = self.coordinates.len() as f64;
    self.time_to_be_conquered = num_of_coordinates * Self::BASE_TERRITORY_OCCUPATION_TIME;
  }

  pub fn add_territory_deployment(&mut self, troop_id: &TroopId, player_id: PlayerId) {
    self.all_troops_deployed.insert(troop_id.clone());

    self.all_troops_deployed_by_player
      .entry(player_id)
      .or_insert(HashSet::new())
      .insert(troop_id.to_string());
  }

  pub fn inform_troop_arrived(&mut self, troop_id: &TroopId, player_id: PlayerId) {
    self.all_troops_deployed_and_arrived.insert(troop_id.clone());

    self.all_troops_deployed_and_arrived_by_player
      .entry(player_id)
      .or_insert(HashSet::new())
      .insert(troop_id.to_string());

    self.set_troops_from_different_players_flag();
  }

  // TODO: potential bug here:
  // when troop is asked to runaway and it been asked to get back, it may be not showing at the hashmap again, needs to check this
  pub fn inform_territory_departure(&mut self, troop_id: &TroopId, player_id: PlayerId) {
    self.all_troops_deployed.remove(troop_id);
    self.all_troops_deployed_and_arrived.remove(troop_id);

    if let Some(player_troops) = self.all_troops_deployed_by_player.get_mut(&player_id) {
      player_troops.remove(troop_id);
    }

    if let Some(player_troops) = self.all_troops_deployed_and_arrived_by_player.get_mut(&player_id) {
      player_troops.remove(troop_id);
    }

    self.set_troops_from_different_players_flag();
  }

  /// It counts all the troops deployed and arrived to a territory, if there are troops from different players
  /// it sets true to has_troops_from_different_players
  /// That helps to know if a territory will be under conflict or when the conflict is finished
  pub fn set_troops_from_different_players_flag(&mut self) {
    let mut troops_by_player_counter = 0;
    self.all_troops_deployed_and_arrived_by_player.iter().for_each(|(_, troops)| {
      if troops.len() > 0 { troops_by_player_counter += 1 }
    });

    if troops_by_player_counter > 1 { self.has_troops_from_different_players = true; }
    else { self.has_troops_from_different_players = false; }
  }
}