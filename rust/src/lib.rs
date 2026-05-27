// #![deny(warnings)]
#![deny(clippy::redundant_clone)]
#![deny(clippy::unwrap_used)]
use godot::prelude::*;

mod core;
mod entities;
mod terrain;
mod ui;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
