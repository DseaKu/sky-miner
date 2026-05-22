use crate::core::utils::FloatExt;
use crate::terrain::*;

use super::consts;
use rand::{self, RngExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct MapGenerator {
    _perlin: noise::Perlin,
    cur_chunk: Coord,
    chunks: HashMap<Coord, Chunk>,
}

impl MapGenerator {
    fn calc_height_penalty(&self, cur_pos: &f32) -> f32 {
        use consts::gen as G;
        FloatExt::move_toward(G::isle::ISLAND_THRESHOLD, 0.0, cur_pos * G::HEIGHT_PENALTY)
    }

    fn generate_chunk(&mut self, coord: Coord) {
        let mut chunk = Chunk::new();

        let cur_pos = &(coord.y as f32);
        let _h_penalty = self.calc_height_penalty(cur_pos);

        // Basic generation logic (placeholder)
        for i in 0..chunk.tiles.len() {
            chunk.tiles[i] = TileType::Stone;
        }

        self.chunks.insert(coord, chunk);
    }

    pub fn update_chunks(&mut self) {
        use consts::gen::RENDER_DISTANCE as R_D;
        let pc = self.cur_chunk.clone();

        for cx in (pc.x - R_D)..=(pc.x + R_D) {
            for cy in (pc.y - R_D)..=(pc.y + R_D) {
                let chunk_coord = Coord::new(cx, cy);

                if !self.chunks.contains_key(&chunk_coord) {
                    self.generate_chunk(chunk_coord);
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
            cur_chunk: Coord::default(),
            chunks: HashMap::new(),
        }
    }
    pub fn has_chunk_changed(&self, new_chunk: &Coord) -> bool {
        *new_chunk != self.cur_chunk
    }
    pub fn set_cur_chunk(&mut self, new_chunk: Coord) {
        self.cur_chunk = new_chunk;
    }
}
