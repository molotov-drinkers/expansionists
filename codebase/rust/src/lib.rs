
use godot::prelude::*;

pub mod root;
pub mod globe;
pub mod i18n;
pub mod camera;
pub mod troops;
pub mod player;
pub mod heads_up_display;

struct Expansionists;

#[gdextension]
unsafe impl ExtensionLibrary for Expansionists {}
