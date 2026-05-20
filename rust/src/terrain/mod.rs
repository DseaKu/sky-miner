pub mod consts;
pub mod generator;
pub mod macros;
pub mod node_interface;

use consts::gen::CHUNK_SIZE as CS;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Void,
    Stone,
}
impl TileType {
    // pub fn new() {
    //
    // }
}

#[derive(Default, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub _position: Coord,
    pub tiles: Vec<TileType>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            _position: Coord::default(),
            tiles: vec![TileType::Void; (CS * CS) as usize],
        }
    }
}
