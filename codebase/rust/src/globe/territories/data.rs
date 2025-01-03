use super::territory::{Continent, Location, SubContinent, Size, Territories, Territory};

impl Territory {
  /// returns a map of territories
  /// called from GlobeScene::init and VirtualPlanet::init
  /// at VirtualPlanet we have coordinates filled in too
  pub fn get_map() -> Territories {
    let mut territories = Territories::new();
    for territory in Territory::list() {
      territories.insert(territory.territory_id.clone(), territory);
    }
    territories
  }

  fn list() -> Vec<Territory> {
    let mut territories = Vec::new();
    territories.extend(Territory::list_africa());
    territories.extend(Territory::list_antarctica());
    territories.extend(Territory::list_asia());
    territories.extend(Territory::list_europe());
    territories.extend(Territory::list_north_america());
    territories.extend(Territory::list_oceania());
    territories.extend(Territory::list_south_america());
    territories.extend(Territory::list_special());
    territories
  }

  fn get_base_territory(territory_id: &str, continent: Continent, sub_continent: Option<SubContinent>) -> Territory {
    Territory {
      territory_id: territory_id.to_string(),
      location: Location { continent, sub_continent },

      // Fields below are filled on the fly
      coordinates: Vec::new(),
      size: Size::None,
      organic_max_troops: 0,
      troops_growth_velocity: 0.1,

      current_ruler: None,
      current_troops: Vec::new(),
    }
  }

  fn list_africa() -> [Territory; 13] {
    [
      Self::get_base_territory("horn", Continent::Africa, None),
      Self::get_base_territory("sahel", Continent::Africa, None),
      Self::get_base_territory("africa_rainforest", Continent::Africa, None),
      Self::get_base_territory("namid_desert", Continent::Africa, None),
      Self::get_base_territory("kalahari", Continent::Africa, None),
      Self::get_base_territory("sahara", Continent::Africa, None),
      Self::get_base_territory("east_savanna", Continent::Africa, None),
      Self::get_base_territory("african_south_central_plateau", Continent::Africa, None),
      Self::get_base_territory("nile_river_region", Continent::Africa, None),
      Self::get_base_territory("the_greatest_african_island", Continent::Africa, None),
      Self::get_base_territory("niger_river", Continent::Africa, None),
      Self::get_base_territory("volta_lake", Continent::Africa, None),
      Self::get_base_territory("africa_west_region", Continent::Africa, None),
    ]
  }

  fn list_south_america() -> [Territory; 9] {
    [
      Self::get_base_territory("amazon", Continent::SouthAmerica, None),
      Self::get_base_territory("andes", Continent::SouthAmerica, None),
      Self::get_base_territory("atlantic_forest", Continent::SouthAmerica, None),
      Self::get_base_territory("caatinga", Continent::SouthAmerica, None),
      Self::get_base_territory("incas", Continent::SouthAmerica, None),
      Self::get_base_territory("latinos", Continent::SouthAmerica, None),
      Self::get_base_territory("pampas", Continent::SouthAmerica, None),
      Self::get_base_territory("patagonia", Continent::SouthAmerica, None),
      Self::get_base_territory("tropical_highlands", Continent::SouthAmerica, None),
    ]
  }

  fn list_north_america() -> [Territory; 21] {
    [
      Self::get_base_territory("californias", Continent::NorthAmerica, None),
      Self::get_base_territory("caribbean_islands", Continent::NorthAmerica, None),
      Self::get_base_territory("north_america_desert", Continent::NorthAmerica, None),
      Self::get_base_territory("great_lakes", Continent::NorthAmerica, None),
      Self::get_base_territory("artic_territories", Continent::NorthAmerica, None),
      Self::get_base_territory("baffin_bay", Continent::NorthAmerica, None),
      Self::get_base_territory("labrador_sea_neighbors", Continent::NorthAmerica, None),
      Self::get_base_territory("new_great_britain", Continent::NorthAmerica, None),
      Self::get_base_territory("mississippi_way", Continent::NorthAmerica, None),
      Self::get_base_territory("romance_speaking_territory", Continent::NorthAmerica, None),
      Self::get_base_territory("southern_north", Continent::NorthAmerica, None),
      Self::get_base_territory("parallel_49th", Continent::NorthAmerica, None),
      Self::get_base_territory("great_bear_lake", Continent::NorthAmerica, None),
      Self::get_base_territory("slave_lake", Continent::NorthAmerica, None),
      Self::get_base_territory("mount_columbia", Continent::NorthAmerica, None),
      Self::get_base_territory("thousand_lakes_region", Continent::NorthAmerica, None),
      Self::get_base_territory("hudson_bay_viewers", Continent::NorthAmerica, None),
      Self::get_base_territory("north_pacific_civilization", Continent::NorthAmerica, None),
      Self::get_base_territory("aztecas", Continent::NorthAmerica, None),
      Self::get_base_territory("mayas", Continent::NorthAmerica, None),
      Self::get_base_territory("cocibolca_lake", Continent::NorthAmerica, None),
    ]
  }

  fn list_asia() -> [Territory; 49] {
    [
      // Middle East
      Self::get_base_territory("arabian_peninsula", Continent::Asia, Some(SubContinent::MiddleEast)),
      Self::get_base_territory("suez_canal", Continent::Asia, Some(SubContinent::MiddleEast)),
      Self::get_base_territory("east_dead_sea", Continent::Asia, Some(SubContinent::MiddleEast)),
      Self::get_base_territory("lut_desert", Continent::Asia, Some(SubContinent::MiddleEast)),
      Self::get_base_territory("monotheist_realms", Continent::Asia, Some(SubContinent::MiddleEast)),
      Self::get_base_territory("zagros_mountains", Continent::Asia, Some(SubContinent::MiddleEast)),

      // Europe Related Asia
      Self::get_base_territory("caspian_coast", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("caucasus", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("east_siberia", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("west_siberia", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("lake_balkhash", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("amu_darya_river", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("aral_sea", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),
      Self::get_base_territory("zeya_dam", Continent::Asia, Some(SubContinent::EuropeRelatedAsia)),

      // East Asia
      Self::get_base_territory("han_land", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("shibuya", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("daisetsuzan", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("korean_peninsula", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("cantonese_lands", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("great_wall", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("manchuria", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("mount_fuji", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("gobi_desert", Continent::Asia, Some(SubContinent::EastAsia)),
      Self::get_base_territory("loess_plateau", Continent::Asia, Some(SubContinent::InteriorAsia)),

      // Southeast Asia
      Self::get_base_territory("borneo_island", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("gede_pangrango", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("banda_arc", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("das_visayas_sea", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("tri_an_lake", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("asia_southeast_peninsula", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("chao_phraya_river", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("tonle_sap", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("phou_bia", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("red_river", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("irrawaddy_river", Continent::Asia, Some(SubContinent::SoutheastAsia)),
      Self::get_base_territory("east_new_guinea", Continent::Asia, Some(SubContinent::SoutheastAsia)),

      // Indian Subcontinent
      Self::get_base_territory("balimela_dam", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("central_sub_continent_highlands", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("ganges_delta_region", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("indo_river", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("kaveri_river", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("mount_pidurutalagala", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("thar_desert", Continent::Asia, Some(SubContinent::IndianSubcontinent)),
      Self::get_base_territory("western_ghats", Continent::Asia, Some(SubContinent::IndianSubcontinent)),

      // Interior Asia
      Self::get_base_territory("himalayas", Continent::Asia, Some(SubContinent::InteriorAsia)),
      Self::get_base_territory("k2_mountain", Continent::Asia, Some(SubContinent::InteriorAsia)),
      Self::get_base_territory("registan_desert", Continent::Asia, Some(SubContinent::InteriorAsia)),
      Self::get_base_territory("tian_shan_mountains", Continent::Asia, Some(SubContinent::InteriorAsia)),
      Self::get_base_territory("lake_baikal", Continent::Asia, Some(SubContinent::InteriorAsia)),
    ]
  }

  fn list_europe() -> [Territory; 11] {
    [
      Self::get_base_territory("nordics", Continent::Europe, None),
      Self::get_base_territory("the_islands", Continent::Europe, None),
      Self::get_base_territory("rhine_region", Continent::Europe, None),
      Self::get_base_territory("balkan_peninsula", Continent::Europe, None),
      Self::get_base_territory("euro_romance_lands", Continent::Europe, None),
      Self::get_base_territory("west_slavs", Continent::Europe, None),
      Self::get_base_territory("baltics", Continent::Europe, None),
      Self::get_base_territory("big_plain", Continent::Europe, None),
      Self::get_base_territory("north_black_sea", Continent::Europe, None),
      Self::get_base_territory("urau_mountains", Continent::Europe, None),
      Self::get_base_territory("white_sea", Continent::Europe, None),
    ]
  }

  fn list_oceania() -> [Territory; 8] {
    [
      Self::get_base_territory("maoris", Continent::Oceania, None),
      Self::get_base_territory("aussie_desert", Continent::Oceania, None),
      Self::get_base_territory("coral_sea_coast", Continent::Oceania, None),
      Self::get_base_territory("daintree_rainforest", Continent::Oceania, None),
      Self::get_base_territory("kangaroos", Continent::Oceania, None),
      Self::get_base_territory("most_isolated_city", Continent::Oceania, None),
      Self::get_base_territory("temperate_land", Continent::Oceania, None),
      Self::get_base_territory("south_pacific_islands", Continent::Oceania, None),
    ]
  }

  fn list_special() -> [Territory; 2] {
    [
      Self::get_base_territory("diomede_islands", Continent::Special, None),
      Self::get_base_territory("asia_europe_connection", Continent::Special, None),
      // Self::get_base_territory("rest_of_world", Continent::Special, None),
    ]
  }

  fn list_antarctica() -> [Territory; 7] {
    [
      Self::get_base_territory("west_antarctica", Continent::Antarctica, None),
      Self::get_base_territory("somov_sea", Continent::Antarctica, None),
      Self::get_base_territory("east_antarctica", Continent::Antarctica, None),
      Self::get_base_territory("riiser_larsen_ice_shelf", Continent::Antarctica, None),
      Self::get_base_territory("everybodys_south", Continent::Antarctica, None),
      Self::get_base_territory("antartica_peninsula", Continent::Antarctica, None),
      Self::get_base_territory("unclaimed_area", Continent::Antarctica, None),
    ]
  }
}