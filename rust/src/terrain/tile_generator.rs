use crate::terrain::TileType;
use rand::{self, RngExt};

const PRINT_PREFIX: &str = "TileGenerator";

#[derive(Default)]
pub struct TileGenerator {
    perlin: noise::Perlin,
}

impl TileGenerator {
    pub fn generate_tile(&self, _x: i32, y: i32) -> TileType {
        if y < 0 {
            return TileType::default();
        }
        TileType::Stone
    }
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let rnd_num: u32 = rng.random();
        crate::node_print!(PRINT_PREFIX, "Initialized with seed\n => {}", rnd_num);

        Self {
            perlin: noise::Perlin::new(rnd_num),
        }
    }
    // fn _calc_height_penalty(&self, cur_pos: &f32) -> f32 {
    //     use consts::{isle, HEIGHT_PENALTY};
    //     let move_toward = utils::FloatExt::move_toward;
    //
    //     move_toward(isle::ISLAND_THRESHOLD, 0.0, cur_pos * HEIGHT_PENALTY)
    // }
}
