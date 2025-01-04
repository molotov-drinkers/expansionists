use godot::builtin::Color;

pub enum PlayerColor {
  Red,
  Blue,
  Green,
  Yellow,
  Purple,
  Orange,
  Black,
  White,
}

impl PlayerColor {
  pub fn get_player_color(color: &PlayerColor) -> Color {
    match color {
      PlayerColor::Red => Color::RED,
      PlayerColor::Blue => Color::BLUE,
      PlayerColor::Green => Color::GREEN,
      PlayerColor::Yellow => Color::YELLOW,
      PlayerColor::Purple => Color::PURPLE,
      PlayerColor::Orange => Color::ORANGE,
      PlayerColor::Black => Color::BLACK,
      PlayerColor::White => Color::WHITE,
    }
  }

  pub fn get_troop_player_color(color: &PlayerColor) -> Color {
    let color = Self::get_player_color(color);
    let color = color.darkened(0.2);

    color
  }

  pub fn get_banner_player_color(color: &PlayerColor) -> Color {
    let color = Self::get_player_color(color);
    let color = color.lightened(0.2);

    color
  }

  pub fn get_land_color(color: &PlayerColor) -> Color {
    let color = Self::get_player_color(color);
    let color = color.lightened(0.5);

    color
  }

}

