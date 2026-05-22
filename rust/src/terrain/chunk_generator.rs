use crate::terrain::*;

use super::consts;
use rand::{self, RngExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct ChunkGenerator {
    _perlin: noise::Perlin,
    center: Coord,
    chunks: HashMap<Coord, Chunk>,
}

impl ChunkGenerator {
    fn generate_chunk(&mut self, _coord: &Coord) -> Chunk {
        Chunk::new()
    }

    pub fn update_chunks(&mut self) {
        use consts::RENDER_DISTANCE as R_D;
        let c = &self.center.clone();

        for x in (c.x - R_D)..=(c.x + R_D) {
            for y in (c.y - R_D)..=(c.y + R_D) {
                let coord = Coord::new(x, y);

                if !self.chunks.contains_key(&coord) {
                    let new_chunk = self.generate_chunk(&coord);
                    self.chunks.insert(coord, new_chunk);
                }
            }
        }
    }

    pub fn new() -> Self {
        let mut rng = rand::rng();
        let rnd_num: u32 = rng.random();
        crate::gd_print!("MapGenerator: Initialized with seed {}", rnd_num);

        Self {
            _perlin: noise::Perlin::new(rnd_num),
            center: Coord::default(),
            chunks: HashMap::new(),
        }
    }
    pub fn has_center_changed(&self, new_chunk: &Coord) -> bool {
        *new_chunk != self.center
    }
    pub fn set_center_chunk(&mut self, new_chunk: Coord) {
        self.center = new_chunk;
    }
    // fn _calc_height_penalty(&self, cur_pos: &f32) -> f32 {
    //     use consts::{isle, HEIGHT_PENALTY};
    //     let move_toward = utils::FloatExt::move_toward;
    //
    //     move_toward(isle::ISLAND_THRESHOLD, 0.0, cur_pos * HEIGHT_PENALTY)
    // }
}
