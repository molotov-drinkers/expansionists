use super::types::{Continent, Location, SubContinent, Territories, Territory};

impl Territory {
  pub fn get_map() -> Territories {
    let mut territories = Territories::new();
    for territory in Territory::list() {
      territories.insert(territory.base_name.clone(), territory);
    }
    territories
  }

  pub fn list() -> Vec<Territory> {
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

  pub fn list_africa() -> Vec<Territory> {
    vec![
      Territory {
        base_name: "horn".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "sahel".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "africa_rainforest".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "namid_desert".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "kalahari".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "sahara".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "east_savanna".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "african_south_central_plateau".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "nile_river_region".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "the_greatest_african_island".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "niger_river".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "volta_lake".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "africa_west_region".to_string(),
        location: Location {
          continent: Continent::Africa,
          sub_continent: None,
        },
      },
    ]
  }

  pub fn list_south_america() -> Vec<Territory> {
    vec![
      Territory {
        base_name: "amazon".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "andes".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "atlantic_forest".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "caatinga".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "incas".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "latinos".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "pampas".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "patagonia".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
      Territory {
        base_name: "tropical_highlands".to_string(),
        location: Location {
          continent: Continent::SouthAmerica,
          sub_continent: None,
        },
      },
    ]
  }

  pub fn list_north_america() -> Vec<Territory> { vec![
    Territory {
      base_name: "californias".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "caribbean_islands".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "north_america_desert".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "great_lakes".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "artic_territories".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "baffin_bay".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "labrador_sea_neighbors".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "new_great_britain".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "mississippi_way".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "romance_speaking_territory".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "southern_north".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "parallel_49th".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "great_bear_lake".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "slave_lake".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "mount_columbia".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "thousand_lakes_region".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "hudson_bay_viewers".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "north_pacific_civilization".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "aztecas".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "mayas".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "cocibolca_lake".to_string(),
      location: Location {
        continent: Continent::NorthAmerica,
        sub_continent: None,
      },
    },
  ]}

  pub fn list_asia() -> Vec<Territory> { vec![
    // Asia - Middle East
    Territory {
      base_name: "arabian_peninsula".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::MiddleEast),
      },
    },
    Territory {
      base_name: "suez_canal".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::MiddleEast),
      },
    },
    Territory {
      base_name: "east_dead_sea".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::MiddleEast),
      },
    },
    Territory {
      base_name: "lut_desert".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::MiddleEast),
      },
    },
    Territory {
      base_name: "monotheist_realms".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::MiddleEast),
      },
    },
    Territory {
      base_name: "zagros_mountains".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::MiddleEast),
      },
    },

    // Asia - EuropeRelatedAsia
    Territory {
      base_name: "caspian_coast".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },
    Territory {
      base_name: "caucasus".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },
    Territory {
      base_name: "east_siberia".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },
    Territory {
      base_name: "west_siberia".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },
    Territory {
      base_name: "lake_balkhash".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },
    Territory {
      base_name: "amu_darya_river".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },
    Territory {
      base_name: "aral_sea".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::EuropeRelatedAsia),
      },
    },

    Territory {
      base_name: "borneo_island".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "gede_pangrango".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "banda_arc".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "das_visayas_sea".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "tri_an_lake".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "asia_southeast_peninsula".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "east_new_guinea".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "han_land".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "shibuya".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "daisetsuzan".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "korean_peninsula".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "zeya_dam".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "cantonese_lands".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "phou_bia".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "red_river".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "tonle_sap".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "great_wall".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "manchuria".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "mount_fuji".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "chao_phraya_river".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },
    Territory {
      base_name: "irrawaddy_river".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::PacificAndSoutheastAsia),
      },
    },

    // Asia - Indian Subcontinent
    Territory {
      base_name: "balimela_dam".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "central_sub_continent_highlands".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "ganges_delta_region".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "indo_river".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "kaveri_river".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "mount_pidurutalagala".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "thar_desert".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },
    Territory {
      base_name: "western_ghats".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::IndianSubcontinent),
      },
    },

    // Asia - Interior Asia
    Territory {
      base_name: "gobi_desert".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
    Territory {
      base_name: "himalayas".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
    Territory {
      base_name: "k2_mountain".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
    Territory {
      base_name: "loess_plateau".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
    Territory {
      base_name: "registan_desert".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
    Territory {
      base_name: "tian_shan_mountains".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
    Territory {
      base_name: "lake_baikal".to_string(),
      location: Location {
        continent: Continent::Asia,
        sub_continent: Some(SubContinent::InteriorAsia),
      },
    },
  ]}

  pub fn list_europe() -> Vec<Territory> { vec![
    Territory {
      base_name: "nordics".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "the_islands".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "rhine_region".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "balkan_peninsula".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "latin_variations".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "west_slavs".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "baltics".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "big_plain".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "north_black_sea".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "urau_mountains".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "white_sea".to_string(),
      location: Location {
        continent: Continent::Europe,
        sub_continent: None,
      },
    },
  ]}

  pub fn list_oceania() -> Vec<Territory> { vec![
    Territory {
      base_name: "maoris".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "aussie_desert".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "coral_sea_coast".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "daintree_rainforest".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "kangaroos".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "most_isolated_city".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "temperate_land".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "south_pacific_islands".to_string(),
      location: Location {
        continent: Continent::Oceania,
        sub_continent: None,
      },
    },
  ]}

  pub fn list_special() -> Vec<Territory> { vec![
    Territory {
      base_name: "diomede_islands".to_string(),
      location: Location {
        continent: Continent::Special,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "asia_europe_connection".to_string(),
      location: Location {
        continent: Continent::Special,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "rest_of_world".to_string(),
      location: Location {
        continent: Continent::Special,
        sub_continent: None,
      },
    },
  ]}

  pub fn list_antarctica() -> Vec<Territory> { vec![
    Territory {
      base_name: "west_antarctica".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "somov_sea".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "east_antarctica".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "riiser_larsen_ice_shelf".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "everybodys_south".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "antartica_peninsula".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
    Territory {
      base_name: "unclaimed_area".to_string(),
      location: Location {
        continent: Continent::Antarctica,
        sub_continent: None,
      },
    },
  ]}
}