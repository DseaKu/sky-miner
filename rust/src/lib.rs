// #![deny(warnings)]
#![deny(clippy::redundant_clone)]
#![deny(clippy::unwrap_used)]
use godot::prelude::*;

mod core;
mod entities;
mod terrain;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
