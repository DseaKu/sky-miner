pub mod chunk_generator;
pub mod consts;
pub mod node_interface;

use consts::CHUNK_SIZE as CS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Void,
    _Stone,
}

#[derive(Default, Eq, PartialEq, Clone, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Chunk {
    pub _tiles: Vec<TileType>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            _tiles: vec![TileType::Void; (CS * CS) as usize],
        }
    }
}
