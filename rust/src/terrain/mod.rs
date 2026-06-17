pub mod chunk_generator;
pub mod config;
pub mod consts;
pub mod io_handler;
pub mod node_interface;
pub mod tile_generator;

use godot::classes::class_macros::private::virtuals::Os::Vector2i;

use crate::core::utils::ToVector2i;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    #[default]
    Sky,
    Void,
    Stone,
    Dirt,
    Ore,
    Gem,
}
impl TileType {
    pub fn to_atlas_coords(self, config: &config::TerrainConfig) -> Vector2i {
        let ac = &config.atlas_coords;
        use TileType::*;

        match self {
            Sky => Vector2i::new(-1, -1),
            Void => ac.empty_cell.to_vector2i(),
            Stone => ac.stone.to_vector2i(),
            Dirt => ac.dirt.to_vector2i(),
            Ore => ac.ore.to_vector2i(),
            Gem => ac.gem.to_vector2i(),
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub tiles: Vec<TileType>,
    pub is_modified: bool,
}

impl Chunk {
    pub fn new(chunk_size: i32) -> Self {
        Self {
            tiles: vec![TileType::Sky; (chunk_size * chunk_size) as usize],
            is_modified: false,
        }
    }
}

#[derive(Default)]
pub struct ChunkData {
    pub chunk: Chunk,
    pub coord: ChunkCoord,
}
impl ChunkData {
    pub fn new(chunk: Chunk, coord: ChunkCoord) -> Self {
        Self { chunk, coord }
    }
}

#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn is_outside_render_distance(&self, center: &ChunkCoord, render_dist: i32) -> bool {
        (self.x - center.x).abs() > render_dist || (self.y - center.y).abs() > render_dist
    }
}

/// Tile Coordinate inside a chunk
#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct LocalTileCoord {
    pub x: i32,
    pub y: i32,
}

impl LocalTileCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_tile(self, chunk: ChunkCoord, chunk_size: i32) -> TileCoord {
        TileCoord {
            x: (chunk.x * chunk_size) + self.x,
            y: (chunk.y * chunk_size) + self.y,
        }
    }
}

/// Global Tile Coordinate
#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

impl TileCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_chunk(self, chunk_size: i32) -> ChunkCoord {
        ChunkCoord {
            x: self.x.div_euclid(chunk_size),
            y: self.y.div_euclid(chunk_size),
        }
    }
}

impl From<Vector2i> for TileCoord {
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<TileCoord> for Vector2i {
    fn from(t: TileCoord) -> Self {
        Self::new(t.x, t.y)
    }
}
