struct SpacePort {
  territory_id: String,
  in_game_name: String,
  actual_name: String,
  description: String,
}

impl SpacePort {
  fn new(territory_id: &str, in_game_name: &str, actual_name: &str, description: &str) -> SpacePort {
    SpacePort {
      territory_id: territory_id.to_string(),
      in_game_name: in_game_name.to_string(),
      actual_name: actual_name.to_string(),
      description: description.to_string(),
    }
  }

  fn populate() {
    Self::new(
      "californias", "west_coast_launcher",
      "Vandenberg Space Force Base", "USA  ->  West Coast launch site known for its diverse missions, including national security, commercial, and scientific launches."
    );
    Self::new(
      "amazon", "far_from_europe_base",
      "Guiana Space Centre", "EU / French Guiana  ->  European Space Agency's primary launch site, strategically located near the equator for efficient launches."
    );
    Self::new(
      "aral_sea", "the_first_spaceport",
      "Baikonur Cosmodrome", "Kazakhstan  ->  World's first and largest spaceport, leased and operated by Russia, with a rich history in space exploration."
    );
    Self::new(
      "loess_plateau", "1958_base",
      "Jiuquan Satellite Launch Center", "China  ->  Primary launch site for China's human spaceflight program, including the Shenzhou missions and Tiangong space station."
    );
    Self::new(
      "mount_fuji", "rocket_island",
      "Tanegashima Space Center", "Japan  ->  Japan Aerospace Exploration Agency's launch site for large rockets, responsible for deploying satellites and interplanetary probes."
    );
    Self::new(
      "kaveri_river", "bay_of_bengal_launchpad",
      "Satish Dhawan Space Centre", "India  ->  Indian Space Research Organisation's launch site, known for its cost-effective launch vehicles and ambitious space program."
    );
    Self::new(
      "white_sea", "polar_orbit_launcher",
      "Plesetsk Cosmodrome", "Russia  ->  Northernmost launch site globally, strategically positioned for polar and high-inclination orbits."
    );
    Self::new(
      "aussie_desert", "remote_senders",
      "Woomera Prohibited Area", "Australia  ->  Historically significant launch site, currently undergoing redevelopment to become a major spaceport for commercial and research activities."
    );
    Self::new(
      "caatinga", "gateway_of_the_equator",
      "Alcantara Launch Center", "Brazil  ->  Located near the equator, offering advantages for launching geostationary satellites, with growing commercial partnerships."
    );
    Self::new(
      "caspian_coast", "the_desert_base",
      "Semnan Space Center", "Iran  ->  Iran's primary space launch facility, used for launching satellites and conducting research in space technology."
    );
  }
}