
use godot::prelude::*;

pub mod globe;
pub mod player;

struct Expansinists;

#[gdextension]
unsafe impl ExtensionLibrary for Expansinists {}
