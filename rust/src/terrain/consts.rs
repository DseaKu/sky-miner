pub mod chunk_gen {
    pub const CHUNK_SIZE: i32 = 6;
    pub const RENDER_DISTANCE: i32 = 3;
}
pub mod path {
    pub const PLAYER_NODE_PATH: &str = "../../Player";
    pub const TILE_MAP_LAYER_NODE_PATH: &str = "../TileMapLayer";
}

// pub mod isle {
//     pub const ISLAND_THRESHOLD: f32 = 0.25;
// }

pub mod atlas_coords {
    pub const SOURCE_ID: i32 = 2;

    // (x,y)
    pub const DIRT: (i32, i32) = (0, 0);
    pub const STONE: (i32, i32) = (1, 0);
    // pub const ORE: (i32, i32) = (2, 0);
    // pub const GEM: (i32, i32) = (0, 1);
    pub const EMPTY_CELL: (i32, i32) = (-1, -1);
}

pub mod tile_gen {
    pub const GROUND_LEVEL: i32 = 0;

    pub mod isle {
        pub const SPAWN_LIMIT: f64 = 0.25;
        pub const STRETCH_X: f64 = 4.0;
        pub const STRETCH_Y: f64 = 40.0;
    }
    // pub const HEIGHT_PENALTY: f32 = 0.00001;
}
