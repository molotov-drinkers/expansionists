
use godot::prelude::*;

pub mod macros;
pub mod root;
pub mod i18n;
pub mod globe;
pub mod camera;
pub mod troops;
pub mod player;
pub mod heads_up_display;

struct Expansionists;

#[gdextension]
unsafe impl ExtensionLibrary for Expansionists {}
