pub mod chunk_gen {
    pub const CHUNK_SIZE: i32 = 16;
    pub const RENDER_DISTANCE: i32 = 10;
}
pub mod path {
    pub const PLAYER_NODE_PATH: &str = "../../Player";
    pub const TILE_MAP_LAYER_NODE_PATH: &str = "../TileMapLayer";
}

pub mod atlas_coords {
    pub const SOURCE_ID: i32 = 2;

    // (x,y)
    pub const DIRT: (i32, i32) = (0, 0);
    pub const STONE: (i32, i32) = (1, 0);
    pub const EMPTY_CELL: (i32, i32) = (3, 3);
}

pub mod tile_gen {
}
