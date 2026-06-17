use crate::terrain::config;
use crate::terrain::TileType;
use noise::NoiseFn;
use rand::{self, RngExt};

const PRINT_PREFIX: &str = "TileGenerator";

#[derive(Default)]
pub struct TileGenerator {
    perlin: noise::Perlin,
    config: config::TileGen,
}

impl TileGenerator {
    pub fn generate_tile(&self, x: i32, y: i32) -> TileType {
        if y > self.config.ground_level {
            return TileType::Stone;
        }
        // Multiply by spread (frequency) and stretch directly in the coordinates
        let island_spread = 0.0013;
        let nx = (x as f64) * island_spread * self.config.isle.stretch_x;
        let ny = (y as f64) * island_spread * self.config.isle.stretch_y;

        // The noise crate takes an array of f64s
        let island_val = self.perlin.get([nx, ny]);
        // If the noise value is a high peak, make it land
        if island_val > 0.25 {
            TileType::Stone
        } else {
            // Otherwise, it is a valley and should be empty sky
            TileType::default()
        }
    }
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let rnd_num: u32 = rng.random();
        crate::node_print!(PRINT_PREFIX, "Initialized with seed\n => {}", rnd_num);

        Self {
            perlin: noise::Perlin::new(rnd_num),
            config: config::TileGen::default(),
        }
    }
    // fn _calc_height_penalty(&self, cur_pos: &f32) -> f32 {
    //     use consts::{isle, HEIGHT_PENALTY};
    //     let move_toward = utils::FloatExt::move_toward;
    //
    //     move_toward(isle::ISLAND_THRESHOLD, 0.0, cur_pos * HEIGHT_PENALTY)
    // }
}
