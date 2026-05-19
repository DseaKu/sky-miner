use crate::core::utils::FloatExt;
use crate::terrain::*;

use super::consts;
use godot::prelude::*;
// use rand::{self, Rng};
use std::collections::HashMap;

#[derive(Default)]
pub struct MapGenerator {
    // perlin: noise::Perlin,
    cur_player_chunk: Coord,
    chunks: HashMap<Coord, Chunk>,
}

impl MapGenerator {
    fn calc_height_penalty(&self, cur_pos: f32) -> f32 {
        use consts::gen as G;
        FloatExt::move_toward(G::isle::ISLAND_THRESHOLD, 0.0, cur_pos * G::HEIGHT_PENALTY)
    }

    fn generate_chunk(&mut self, coord: Coord) {
        use consts::gen::CHUNK_SIZE as S;
        let mut chunk = Chunk::new(S);

        let h_penalty = self.calc_height_penalty(coord.y as f32);

        // Basic generation logic (placeholder)
        for i in 0..chunk.tiles.len() {
            chunk.tiles[i] = TileType::Stone;
        }

        self.chunks.insert(coord, chunk);
    }

    fn update_chunks(&mut self) {
        use consts::gen::RENDER_DISTANCE as RD;
        let pc = self.cur_player_chunk.clone();

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
        // let mut rng = rand::thread_rng();
        // let rnd_num: u32 = rng.gen();
        // crate::gd_print!("MapGenerator: Initialized with seed {}", rnd_num);
        // let perlin = noise::Perlin::new(rnd_num);

        Self {
            // perlin,
            cur_player_chunk: Coord::default(),
            chunks: HashMap::new(),
        }
    }

    pub fn update(&mut self, _delta: f64, grid_pos: Vector2i) {
        use consts::gen::CHUNK_SIZE as S;
        let new_chunk_x = grid_pos.x / S;
        let new_chunk_y = grid_pos.y / S;
        let new_player_chunk = Coord::new(new_chunk_x, new_chunk_y);

        if new_player_chunk != self.cur_player_chunk {
            self.cur_player_chunk = new_player_chunk;
            self.update_chunks();
        }
    }
}
