
use godot::classes::{Engine, ILabel, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct DebuggerHUD {
  base: Base<Label>,
}

#[godot_api]
impl ILabel for DebuggerHUD {
  fn init(base: Base<Label>) -> DebuggerHUD {

    DebuggerHUD {
      base: base,
    }
  }

  fn process(&mut self, _delta: f64) {
    let fps = Engine::singleton().get_frames_per_second();
    let text = format!("FPS: {}", fps);

    let color = match fps {
      fps if fps < 25.0 => Color::RED,
      fps if fps < 30.0 => Color::YELLOW,
      fps if fps < 60.0 => Color::from_rgba(0.6, 1.0, 0.6, 1.0),
      _ => Color::GREEN,
    };

    self.base_mut().set_modulate(color);
    self.base_mut().set_text(&text);
  }
}
