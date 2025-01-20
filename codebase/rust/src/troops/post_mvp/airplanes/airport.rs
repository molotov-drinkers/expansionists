struct AirPort {
  territory_id: String,
  in_game_name: String,
  actual_name: String,
  description: String,
}

impl AirPort {
  fn new(territory_id: &str, in_game_name: &str, actual_name: &str, description: &str) -> AirPort {
    AirPort {
      territory_id: territory_id.to_string(),
      in_game_name: in_game_name.to_string(),
      actual_name: actual_name.to_string(),
      description: description.to_string(),
    }
  }

  fn populate() {
    Self::new(
      "arabian_peninsula", "",
      "King Fahd International", "King Fahd International (DMM) - Dammam, Saudi Arabia - 780 km²"
    );
    Self::new(
      "great_wall", "",
      "Beijing Daxing International", "Beijing Daxing International (PKX) - Beijing, China - 46.6 km²"
    );
    Self::new(
      "chao_phraya_river", "",
      // The upcoming one should be 
      // Shanghai Pudong International (PVG) - Shanghai, China - 40 km²
      // China already had Beinjing so I skipped to the next biggest
      "Suvarnabhumi Airport", "Suvarnabhumi Airport (BKK) - Bangkok, Thailand - 32.4 km²"
    );
    Self::new(
      "north_america_desert", "",
      "Denver International", "Denver International (DEN) - Denver, CO, USA - 135.7 km²"
    );
    Self::new(
      "new_great_britain", "",
      // 2 and 3 are
      // Dallas/Fort Worth International (DFW) - Dallas, TX, USA - 69.6 km²
      // Orlando International (MCO) - Orlando, FL, USA - 53.8 km²
      // Denver will already have a airport for N.A. desert. And Florida will hold a spaceport
      // so to balance it, i picked the 4th biggest
      "Washington Dulles International", "Washington Dulles International (IAD) - Washington D.C., USA - 48.6 km²"
    );
    Self::new(
      "euro_romance_lands", "",
      "Charles de Gaulle Airport", "Charles de Gaulle Airport (CDG) - Paris, France - 32.4 km²"
    );
    Self::new(
      "asia_europe_connection", "",
      "Istanbul Airport", "Istanbul Airport (IST) - Istanbul, Turkey - 30 km²"
    );
    Self::new(
      "big_plain", "",
      "Sheremetyevo International Airport", "Sheremetyevo International Airport (SVO) - Moscow, Russia - 29.28 km²"
    );
    Self::new(
      "kangaroos", "",
      "Sydney Airport", "Sydney Airport (SYD) - Sydney, Australia - 20 km²"
    );
    Self::new(
      "atlantic_forest", "",
      "São Paulo/Guarulhos International Airport", "São Paulo/Guarulhos International Airport (GRU) - Guarulhos, Brazil - 14 km²"
    );
    Self::new(
      "latinos", "",
      "El Dorado International Airport", "El Dorado International Airport (BOG) - Bogotá, Colombia - 10 km²"
    );
    Self::new(
      "african_south_central_plateau", "",
      "O.R. Tambo International Airport", "O.R. Tambo International Airport (JNB) - Johannesburg, South Africa - 16.1 km²"
    );
    Self::new(
      "nile_river_region", "",
      "Cairo International (CAI)", "Cairo International (CAI) - Cairo, Egypt - 37 km"
    );
    Self::new(
      "everybodys_south", "",
      "Williams Field", "Airport type	Public - Location McMurdo Station, Antarctica"
    );
  }
}