use godot::prelude::*;

mod core;
mod entities;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
