pub mod chunk_generator;
pub mod consts;
pub mod node_interface;
pub mod tile_generator;

use consts::CHUNK_SIZE as CS;
use godot::classes::class_macros::private::virtuals::Os::Vector2i;

use crate::core::utils::ToVector2i;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    #[default]
    Void,
    Stone,
    // Dirt,
}
impl TileType {
    pub fn to_atlas_coords(self) -> Vector2i {
        use consts::atlas_coords as a_c;
        use TileType::*;

        match self {
            Void => a_c::EMPTY_CELL,
            Stone => a_c::STONE,
            // Dirt => a_c::DIRT,
        }
        .to_vector2i()
    }
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    #[default]
    Unspawned,
    PendingSpawn,
    _Spawned,
    _PendingDespawn,
}

#[derive(Clone)]
pub struct Chunk {
    pub tiles: Vec<TileType>,
    pub state: ChunkState,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; (CS * CS) as usize],
            state: ChunkState::default(),
        }
    }
}

#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct LocalCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct GlobalCoord {
    pub x: i32,
    pub y: i32,
}
impl LocalCoord {
    /// Converts a local chunk index into an absolute global map position
    pub fn to_global(self, chunk: ChunkCoord) -> GlobalCoord {
        use crate::terrain::consts::gen::CHUNK_SIZE as CS;

        GlobalCoord {
            x: (chunk.x * CS) + self.x,
            y: (chunk.y * CS) + self.y,
        }
    }
}

impl GlobalCoord {
    /// Converts an absolute global map position down into a Chunk coordinate
    pub fn to_chunk(self) -> ChunkCoord {
        use crate::terrain::consts::gen::CHUNK_SIZE as CS;

        // Division truncates toward zero in Rust, but for procedural grids
        // spanning negative numbers, you generally want Euclidean division (div_euclid).
        ChunkCoord {
            x: self.x.div_euclid(CS),
            y: self.y.div_euclid(CS),
        }
    }
}
