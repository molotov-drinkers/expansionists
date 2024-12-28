use godot::prelude::*;
use crate::{globe::coordinates_system::surface_point::SurfacePoint, troops::troop::Troop};

pub struct Deployment {
  pub troops: Vec<Troop>,
  pub deploying_to: Gd<SurfacePoint>,
}

impl Deployment {
  
  
  // TICKET: #20
  pub fn new(troops: Vec<Troop>, deploying_to: Gd<SurfacePoint>) -> Self {

    // deploying_to could be generatedn from a click on the globe
    // then we would do something close to SurfacePoint::get_troop_surface_point(troop: &Troop)
    // however, instead of troop_position we would use the click_position

    // then patrolling would be turned to false and deploying_to
    // would be send on CoordinatesSystem::get_geodesic_trajectory()
    // at destination (Some changes on troop mod needs to be done to make this work)

    Deployment {
      troops,
      deploying_to,
    }
  }
}