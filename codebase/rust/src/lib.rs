
use godot::prelude::*;

pub mod root;
pub mod globe;
pub mod camera;
pub mod troops;
pub mod player;

struct Expansinists;

#[gdextension]
unsafe impl ExtensionLibrary for Expansinists {}
