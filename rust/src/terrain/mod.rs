pub mod consts;
pub mod generator;
pub mod macros;
pub mod node_interface;

use consts::gen::CHUNK_SIZE as CS;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Void,
    Stone,
}
impl TileType {
    // pub fn new() {
    //
    // }
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
    pub position: Coord,
    pub tiles: [TileType; (CS * CS) as usize],
}

impl Chunk {
    pub fn new(size: i32) -> Self {
        let count = (size * size) as usize;
        Self {
            position: Coord::default(),
            tiles: vec![TileType::Void; count],
        }
    }
}
