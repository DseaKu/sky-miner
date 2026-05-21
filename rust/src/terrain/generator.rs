use crate::core::utils::FloatExt;
use crate::terrain::*;

use super::consts;
use godot::prelude::*;
use rand::{self, RngExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct MapGenerator {
    _perlin: noise::Perlin,
    cur_p_chunk: Coord,
    chunks: HashMap<Coord, Chunk>,
}

/*
* Toggle gd map gen
* Harden debug ui, should run without dependencies
* First get on block beneath the player
*
*
*/

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

    fn update_chunks(&mut self) {
        use consts::gen::RENDER_DISTANCE as RD;
        let pc = self.cur_p_chunk.clone();

        for cx in (pc.x - RD)..=(pc.x + RD) {
            for cy in (pc.y - RD)..=(pc.y + RD) {
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
            cur_p_chunk: Coord::default(),
            chunks: HashMap::new(),
        }
    }

    pub fn update(&mut self, _delta: f64, grid_pos: Vector2i) {
        use consts::gen::CHUNK_SIZE as CS;
        let new_p_chunk = Coord::new(grid_pos.x / CS, grid_pos.y / CS);

        // Set a new player chunk if the player moves out of the old one.
        if new_p_chunk != self.cur_p_chunk {
            self.cur_p_chunk = new_p_chunk;
            self.update_chunks();
        }
    }
}
