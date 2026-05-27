pub mod chunk_generator;
pub mod config;
pub mod consts;
pub mod node_interface;
pub mod tile_generator;

use godot::classes::class_macros::private::virtuals::Os::Vector2i;

use crate::core::utils::ToVector2i;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    #[default]
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
            Void => ac.empty_cell,
            Stone => ac.stone,
            Dirt => ac.dirt,
            Ore => ac.ore,
            Gem => ac.gem,
        }
        .to_vector2i()
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub tiles: Vec<TileType>,
    pub is_modified: bool,
}

impl Chunk {
    pub fn new(chunk_size: i32) -> Self {
        Self {
            tiles: vec![TileType::Void; (chunk_size * chunk_size) as usize],
            is_modified: false,
        }
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

#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct LocalCoord {
    pub x: i32,
    pub y: i32,
}

impl LocalCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_global(self, chunk: ChunkCoord, chunk_size: i32) -> GlobalCoord {
        GlobalCoord {
            x: (chunk.x * chunk_size) + self.x,
            y: (chunk.y * chunk_size) + self.y,
        }
    }
}

#[derive(Default, Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct GlobalCoord {
    pub x: i32,
    pub y: i32,
}

impl GlobalCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_chunk(self, chunk_size: i32) -> ChunkCoord {
        // Division truncates toward zero in Rust, but for procedural grids
        // spanning negative numbers, you generally want Euclidean division (div_euclid).
        ChunkCoord {
            x: self.x.div_euclid(chunk_size),
            y: self.y.div_euclid(chunk_size),
        }
    }
}
