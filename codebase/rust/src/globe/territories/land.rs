
use godot::{classes::{IStaticBody3D, MeshInstance3D, StaticBody3D}, prelude::*};
use crate::globe::territories::territory::Territory;

/// Every territory should be a MeshInstance3D with the 
/// following "Land StaticBody3D" as a child
#[derive(GodotClass)]
#[class(base=StaticBody3D)]
pub struct Land {
  base: Base<StaticBody3D>,
}

#[godot_api]
impl IStaticBody3D for Land {
  fn init(base: Base<StaticBody3D>) -> Land {
    Land {
      base: base,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_ray_pickable(true);
  }

  fn mouse_enter(&mut self) {
    let territory = self.base()
      .get_parent()
      .expect("Parent to exist")
      .cast::<MeshInstance3D>();

    Territory::checking_territory(territory);
  }

  fn mouse_exit(&mut self) {
    let territory = self.base()
      .get_parent()
      .expect("Parent to exist")
      .cast::<MeshInstance3D>();
    Territory::unchecking_territory(territory);
  }
}