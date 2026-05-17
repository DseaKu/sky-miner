use godot::prelude::*;

mod core;
mod entities;
mod map;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
