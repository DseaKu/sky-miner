use super::consts;
use super::tile::{Chunk, TileType};
use godot::prelude::*;
use rand::{self, Rng};
use std::collections::HashMap;

#[derive(Default)]
pub struct MapGenerator {
    perlin: noise::Perlin,
    cur_player_chunk: Vector2i,
    chunks: HashMap<Vector2i, Chunk>,
}

impl MapGenerator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rnd_num: u32 = rng.gen();
        crate::gd_print!("MapGenerator: Initialized with seed {}", rnd_num);
        let perlin = noise::Perlin::new(rnd_num);

        Self {
            perlin,
            cur_player_chunk: Vector2i::default(),
            chunks: HashMap::new(),
        }
    }

    pub fn update(&mut self, _delta: f64, grid_pos: Vector2i) {
        use consts::gen::CHUNK_SIZE as S;
        let new_chunk_x = grid_pos.x / S;
        let new_chunk_y = grid_pos.y / S;
        let new_player_chunk = Vector2i::new(new_chunk_x, new_chunk_y);

        if new_player_chunk != self.cur_player_chunk {
            self.cur_player_chunk = new_player_chunk;
            self.update_chunks();
        }
    }

    fn update_chunks(&mut self) {
        use consts::gen::RENDER_DISTANCE as RD;
        let pc = self.cur_player_chunk;

        for cx in (pc.x - RD)..=(pc.x + RD) {
            for cy in (pc.y - RD)..=(pc.y + RD) {
                let chunk_coord = Vector2i::new(cx, cy);

                if !self.chunks.contains_key(&chunk_coord) {
                    self.gen_chunk(chunk_coord);
                }
            }
        }
    }

    fn gen_chunk(&mut self, coord: Vector2i) {
        use consts::gen::CHUNK_SIZE as S;
        let mut chunk = Chunk::new(S);

        // Basic generation logic (placeholder)
        for i in 0..chunk.tiles.len() {
            chunk.tiles[i] = TileType::Stone;
        }

        self.chunks.insert(coord, chunk);
        crate::gd_print!("MapGenerator: Generated chunk at {:?}", coord);
    }
}
