use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Surface {
  Land,
  Water,

  // future_version:
  // Air, // (Planes)
  // Space, // (Satellites)
}

impl fmt::Display for Surface {
  /// allows to use `&Surface::Land.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Surface::Land => write!(f, "land"),
      Surface::Water =>  write!(f, "water"),
    }
  }
}
