use godot::prelude::*;

pub mod camera;
pub mod globe;
pub mod heads_up_display;
pub mod i18n;
pub mod macros;
pub mod player;
pub mod root;
pub mod troops;

struct Expansionists;

#[gdextension]
unsafe impl ExtensionLibrary for Expansionists {}
