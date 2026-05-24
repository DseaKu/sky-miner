use super::tile_generator;
use crate::terrain::*;

use super::consts;
use rand::{self, RngExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct ChunkGenerator {
    _perlin: noise::Perlin,
    center: Coord,
    chunks: HashMap<Coord, Chunk>,
    pub spawn_queue: Vec<(Chunk, Coord)>,
}

impl ChunkGenerator {
    fn generate_chunk(&mut self, _coord: &Coord) -> Chunk {
        use consts::CHUNK_SIZE as C_S;
        let t_g = tile_generator::TileGenerator;
        let mut new_chunk = Chunk::new();

        for x in 0..C_S {
            for y in 0..C_S {
                let index = (y * C_S + x) as usize;
                new_chunk.tiles[index] = t_g.generate_tile(x, y);
            }
        }

        new_chunk
    }

    pub fn update_chunks(&mut self) {
        use consts::RENDER_DISTANCE as R_D;
        let c = &self.center.clone();

        for x in (c.x - R_D)..=(c.x + R_D) {
            for y in (c.y - R_D)..=(c.y + R_D) {
                let coord = Coord::new(x, y);

                if self.chunks.contains_key(&coord) {
                    continue;
                }

                let mut new_chunk = self.generate_chunk(&coord);

                if new_chunk.state == ChunkState::Unspawned {
                    self.spawn_queue.push((new_chunk.clone(), Coord::new(x, y)));
                    new_chunk.state = ChunkState::PendingSpawn;
                }
                self.chunks.insert(coord, new_chunk);
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
            spawn_queue: Vec::new(),
        }
    }
    pub fn has_center_changed(&self, new_chunk: &Coord) -> bool {
        *new_chunk != self.center
    }
    pub fn set_center_chunk(&mut self, new_chunk: Coord) {
        self.center = new_chunk;
    }
}
