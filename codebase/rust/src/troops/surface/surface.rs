use std::fmt;

use godot::prelude::*;

use crate::{
  globe::coordinates_system::surface_point::SurfacePoint,
  troops::troop::Troop
};

#[derive(PartialEq, Debug)]
pub enum Surface {
  Land,
  Sea,

  // future_version:
  // Air, // (Planes)
  // Space, // (Satellites)
}

impl fmt::Display for Surface {
  /// allows to use `&Surface::Land.to_string()`
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Surface::Land => write!(f, "land"),
      Surface::Sea =>  write!(f, "sea"),
    }
  }
}

impl Troop {
  /// Sets troop surface according to the surface_point troop is touching
  pub fn set_surface_troop(&mut self) {
    let surface_point = SurfacePoint::get_troop_surface_point(
      self
    );

    // if it doesn't find a surface point, it doesn't panic, just keep the previous surface
    if surface_point.is_none() {
      return;
    }

    let surface_point = surface_point.unwrap();
    if surface_point.is_in_group(&Surface::Land.to_string()) && self.surface != Surface::Land {
      self.surface_type_changed = true;
      self.surface = Surface::Land;

    } else if !surface_point.is_in_group(&Surface::Land.to_string()) && self.surface != Surface::Sea {
      self.surface_type_changed = true;
      self.surface = Surface::Sea;
    }

    self.touching_surface_point = surface_point.bind().surface_point_metadata.clone();
  }

  fn get_sea_and_land_mesh(&self) -> (Gd<Node3D>, Gd<Node3D>) {
    let sea_mesh = self
      .base()
      .get_node_as::<Node3D>("sea");

    let land_mesh =  self
      .base()
      .get_node_as::<Node3D>("land");

    (sea_mesh, land_mesh)
  }

  pub fn set_troop_visibility(&mut self) {
    let (mut sea_mesh, mut land_mesh) = self.get_sea_and_land_mesh();

    sea_mesh.set_visible(false);
    land_mesh.set_visible(true);
  }

  /// Sets troop to show the proper mesh according to the surface the troop is touching
  pub fn check_and_change_mesh(&mut self) {
    if self.surface_type_changed {
      self.surface_type_changed = false;

      let (mut sea_mesh, mut land_mesh) = self.get_sea_and_land_mesh();

      if self.surface == Surface::Land {
        sea_mesh.set_visible(false);
        land_mesh.set_visible(true);
      } else {
        sea_mesh.set_visible(true);
        land_mesh.set_visible(false);
      }

    }
  }
}