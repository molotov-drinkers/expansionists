
use godot::{classes::{IStaticBody3D, InputEvent, InputEventMouseButton, MeshInstance3D, StaticBody3D}, global::MouseButton, prelude::*};
use crate::{
  globe::{
    coordinates_system::virtual_planet::VirtualPlanet,
    territories::territory::Territory
  },
  heads_up_display::territory_hud::TerritoryHUD,
  // heads_up_display::temp_territory_hud::TerritoryHUD,
};

/// Every territory should be a MeshInstance3D with the 
/// following "Land StaticBody3D" as a child
/// |-territory
/// |||- land
/// ||||- collision_shape
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

  fn input_event(
      &mut self,
      _camera: Option<Gd<Camera3D>>,
      event: Option<Gd<InputEvent>>,
      _event_position: Vector3,
      _normal: Vector3,
      _shape_idx: i32
    ) {
    Self::catch_left_click(self, event);
  }

  fn mouse_enter(&mut self) {
    let territory_mesh = self.base()
      .get_parent()
      .expect("Parent to exist")
      .cast::<MeshInstance3D>();
    
    let mut territory_hud = self.get_territory_hud_from_land();
    let virtual_planet = self.get_virtual_planet_from_land();
    
    let territories = &virtual_planet.bind().territories;
    let territory = territories
      .get(&territory_mesh.get_name().to_string())
      .expect("Expected to find territory");

    territory_hud.bind_mut().activate_hud(territory);

    Territory::checking_territory(territory_mesh);
  }

  fn mouse_exit(&mut self) {
    let territory = self.base()
      .get_parent()
      .expect("Parent to exist")
      .cast::<MeshInstance3D>();

    let mut territory_hud = self.get_territory_hud_from_land();
    territory_hud.bind_mut().clean_hud();

    Territory::unchecking_territory(territory);
  }
}

impl Land {
  fn catch_left_click(&mut self, event: Option<Gd<InputEvent>>,) {
    if let Some(event) = event {
      if let Ok(mouse_click) = event.try_cast::<InputEventMouseButton>() {
        let mouse_button = mouse_click.get_button_index();
        let pressed = mouse_click.is_pressed();
        let territory = self.base().get_parent().expect("Parent to exist").cast::<MeshInstance3D>();

        match (mouse_button, pressed) {
          (MouseButton::LEFT, true) => Territory::clicking_territory(territory),
          (MouseButton::LEFT, false) => Territory::checking_territory(territory),
          _ => {}
        }
      }
    }
  }

  /// expects the following hierarchy:
  /// ```
  /// root_scene
  /// |-globe
  /// |||-territories
  /// ||||-territory
  /// |||||-land (receives land)
  /// ||||||-collision_shape
  /// ```
  fn get_root_from_land(&mut self) -> Gd<Node> {
    self
      .base()
      .get_parent().expect("Expected Land to have mesh Territory as parent")
      .get_parent().expect("Expected Mesh territory to have territories as parent")
      .get_parent().expect("Expected territories to have globe as parent")
      .get_parent().expect("Expected globe to have root as parent")
  }

  fn get_territory_hud_from_land(&mut self) -> Gd<TerritoryHUD> {
    let territory_hud = self
      .get_root_from_land()
      .try_get_node_as::<TerritoryHUD>("canvas_layer/territory_hud")
      .expect("Expected to find TerritoryHUD from RootScene");

    territory_hud
  }

  fn get_virtual_planet_from_land(&mut self) -> Gd<VirtualPlanet> {
    let territory_hud = self
      .get_root_from_land()
      .try_get_node_as::<VirtualPlanet>("virtual_planet")
      .expect("Expected to find VirtualPlanet from RootScene");

    territory_hud
  }
}