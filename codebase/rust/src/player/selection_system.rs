use godot::{classes::{INinePatchRect, InputEvent, InputEventMouseButton, NinePatchRect}, global::MouseButton, prelude::*};
use crate::{camera::player_camera::PlayerCamera, globe::territories::{land::Land, territory::TerritoryId}, heads_up_display::selection_hud::SelectionHUD, troops::troop::Troop};

#[derive(GodotClass)]
#[class(base=NinePatchRect)]
pub struct UiDragBox {
  base: Base<NinePatchRect>,
  dragging: bool,
  in_rect_troops: Vec<Gd<Troop>>,
  start_pos: Vector2,
  released_at: Vector2,
  positive_x: bool,
  positive_y: bool,
}

#[godot_api]
impl INinePatchRect for UiDragBox {
  fn init(base: Base<NinePatchRect>) -> Self {
    UiDragBox {
      base: base,
      dragging: false,
      in_rect_troops: Vec::new(),
      start_pos: Vector2::ZERO,
      released_at: Vector2::ZERO,
      positive_x: true,
      positive_y: true,
    }
  }

  fn ready(&mut self) {
    self.base_mut().set_visible(false);
    self.set_reception_for_right_click_on_lands_signal();
  }

  fn input(&mut self, event: Gd<InputEvent>) {
    if let Ok(mouse_click) = event.clone().try_cast::<InputEventMouseButton>() {
      let mouse_button = mouse_click.get_button_index();
      let pressed = mouse_click.is_pressed();
      let clicked_at = mouse_click.get_position();

      match (mouse_button, pressed) {
        (MouseButton::LEFT, true) => {
          // TODO: add a if shift isnt pressed, and clear in_rect_troops sole if it isnt
          self.deselect_troops();
          self.dragging = true;
          self.start_pos = clicked_at;
          self.base_mut().set_position(clicked_at);
        },
        (MouseButton::LEFT, false) => {
          self.dragging = false;
          self.released_at = clicked_at;
          self.base_mut().set_visible(false);
          self.cast_troop_selection()
        },
        _ => {}
      }
    }
  }

  fn process(&mut self, _delta: f64) {
    if self.dragging {
      let mouse_pos = self.base_mut().get_global_mouse_position();
      let size = mouse_pos - self.start_pos;

      // NinePatchRect doesn't take negative size, so we need to flip the scale
      // if size is negative if needed
      self.positive_x = size.x > 0.;
      self.positive_y = size.y > 0.;

      let x_scale = if self.positive_x { 1. } else { -1. };
      let y_scale = if self.positive_y { 1. } else { -1. };
      self.base_mut().set_scale(Vector2::new(x_scale, y_scale));

      // The size of the drag box is the absolute value of the size
      self.base_mut().set_size(size.abs());

      if self.base_mut().get_size().length_squared() > Self::MIN_DRAG_SQUARE {
        self.base_mut().set_visible(true);
      }
    }
  }

}
#[godot_api]
impl UiDragBox {
  const MIN_DRAG_SQUARE: f32 = 164.;

  fn cast_troop_selection(&mut self) {
    self.in_rect_troops.clear();

    let mut player_camera = self.get_camera_from_ui_drag_box();
    let ui_drag_box_rect = self.get_ui_drag_box_rect();

    for troop in self.get_player_troops().iter() {
      let troop_position = troop.get_global_position();
      if !player_camera.bind_mut().is_body_visible_on_camera(troop_position) {
        continue;
      }

      let troop_2d_position = player_camera.bind_mut()
        .get_vector_2_from_vector_3(troop_position);
      let in_the_rect = ui_drag_box_rect.has_point(troop_2d_position);

      if in_the_rect {
        let mut troop = troop.clone();
        troop.bind_mut().select_troop();
        self.in_rect_troops.push(troop);
        
        let mut selection_hud = self.get_hud_from_ui_drag_box();
        selection_hud.bind_mut().activate_hud();
        selection_hud.bind_mut().set_text_with_num_of_troops(self.in_rect_troops.len());
      }
    }

  }

  fn deselect_troops(&mut self) {
    self.get_player_troops()
      .iter()
      .for_each(|troop| {
        let mut troop = troop.clone();
        troop.bind_mut().deselect_troop();
      });
    self.in_rect_troops.clear();

    let mut selection_hud = self.get_hud_from_ui_drag_box();
    selection_hud.bind_mut().deactivate_hud();
  }

  fn get_player_troops(&mut self) -> Vec<Gd<Troop>> {
    let all_troops = self.get_root_from_ui_drag_box()
      .get_tree()
      .expect("Expected tree to be found from root in UiDragBox::ready")
      .get_nodes_in_group(Troop::MAIN_PLAYER_TROOPS);

    let mut selectable_troops = Vec::new();
    for troop in all_troops.iter_shared() {
      let troop = troop.cast::<Troop>();
      selectable_troops.push(troop);
    }

    selectable_troops
  }

  fn set_reception_for_right_click_on_lands_signal(&mut self) {
    let all_territory_lands = self.get_root_from_ui_drag_box()
      .get_tree()
      .expect("Expected tree to be found from root in UiDragBox::ready")
      .get_nodes_in_group(Land::LAND_CLASS_NAME);

    for land in all_territory_lands.iter_shared() {
      let mut land = land.cast::<Land>();
      let callable = self.base_mut().callable(
        "move_selected_troops"
      );
      land.connect(Land::LAND_RIGHT_CLICKED, &callable);
    }
  }

  #[func]
  fn move_selected_troops(&mut self, moving_to: Vector3, territory_id: TerritoryId) {
    self.in_rect_troops
      .iter()
      .for_each(|troop| {
        let mut troop = troop.clone();
        troop.bind_mut().set_order_to_move_to(
          moving_to,
          &territory_id);
      });
  }

  /// expects the following hierarchy:
  /// ```
  /// root_scene
  /// |-playable
  /// |||-selection_system
  /// ||||-ui_drag_box
  /// ```
  fn get_root_from_ui_drag_box(&mut self) -> Gd<Node> {
    self
      .base()
      .get_parent().expect("Expected UiDragBox to have SelectionSystem as parent")
      .get_parent().expect("Expected SelectionSystem to have playable as parent")
      .get_parent().expect("Expected playable to have root as parent")
  }


  fn get_camera_from_ui_drag_box(&mut self) -> Gd<PlayerCamera> {
    self
      .get_root_from_ui_drag_box()
      .get_node_as::<PlayerCamera>("player_camera")
  }

  fn get_hud_from_ui_drag_box(&mut self) -> Gd<SelectionHUD> {
    self
      .get_root_from_ui_drag_box()
      .try_get_node_as::<SelectionHUD>("ui/selection_hud")
      .expect("Expected to find SelectionHUD from RootScene")
  }

  fn get_ui_drag_box_rect(&mut self) -> Rect2 {
    let ui_drag_box_rect_position = match (self.positive_x, self.positive_y) {
      (true, true) => self.start_pos,
      (true, false) => self.base_mut().get_rect().abs().position,
      (false, false) => self.released_at,
      (false, true) => {
        Vector2::new(
          self.released_at.x,
          self.base_mut().get_rect().position.y,
        )
      },
    };

    Rect2::new(
      ui_drag_box_rect_position,
      self.base_mut().get_rect().abs().size
    )
  }
}

