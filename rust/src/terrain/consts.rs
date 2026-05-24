pub const CHUNK_SIZE: i32 = 4;
pub const RENDER_DISTANCE: i32 = 1;
// pub const HEIGHT_PENALTY: f32 = 0.00001;

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
    pub const EMPTY_CELL: (i32, i32) = (3, 3);
}
// const CHUNK_SIZE = 16
// const RENDER_DISTANCE = 10
//
// # Atlas Coords
// const TILE_SOURCE_ID = 2
// const DIRT = Vector2i(0, 0)
// const STONE = Vector2i(1, 0)
// const ORE = Vector2i(2, 0)
// const GEM = Vector2i(0, 1)
// const EMPTY_CELL = Vector2i(3, 3)
// const NONE_EXISTING_CELL = Vector2i(-1, -1)
//
// # Ore
// # const ORE_SEED = 1
// const ORE_SPREAD = 0.05
// const ORE_INIT_THRESHOLD = 1.00
// const ORE_MIN_THRESHOLD = 0.60
// const ORE_CURVE_STEEP_THRESH = 1.35
// const DIRT_THRESHOLD = -0.3
//
// # Void
// # const EMPTY_CELLS_SEED = 2
// const EMPTY_CELLS_SPREAD = 0.25
// const EMPTY_CELLS_THRESHOLD = 0.23
//
// # Gems
// # const GEM_SEED = 3
// const GEM_SPREAD = 0.35
// const GEM_INIT_THRESHOLD = 1.15
// const GEM_MIN_THRESHOLD = 0.50
// const GEM_CURVE_STEEP_THRESH = 1.35
//
// # Isle
// # const ISLAND_SEED = 4
// const ISLAND_SPREAD = 0.0013
// const ISLAND_THRESHOLD = 0.25
// const ISLAND_STRETCH_X = 4.0
// const ISLAND_STRETCH_Y = 40.0
// const SPACE_ISLE_GROUND = -4
// const HEIGHT_PENALTY = .00001
// const RARITY_HEIGHT_IMPACT := 3.0
