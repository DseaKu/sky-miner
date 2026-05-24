use super::tile_generator;
use crate::terrain::*;

use super::consts;
use rand::{self, RngExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct ChunkGenerator {
    _perlin: noise::Perlin,
    center: ChunkCoord,
    chunks: HashMap<ChunkCoord, Chunk>,
    pub spawn_queue: Vec<(Chunk, ChunkCoord)>,
}

impl ChunkGenerator {
    fn generate_chunk(&mut self, coord: &ChunkCoord) -> Chunk {
        use consts::CHUNK_SIZE as CS;
        let tg = tile_generator::TileGenerator;
        let mut new_chunk = Chunk::new();

        for x in 0..CS {
            for y in 0..CS {
                let index = (y * CS + x) as usize;

                let local = LocalCoord::new(x, y);
                let global = local.to_global(*coord);

                new_chunk.tiles[index] = tg.generate_tile(global.x, global.y);
            }
        }

        new_chunk
    }
    pub fn update_chunks(&mut self) {
        use consts::RENDER_DISTANCE as RD;
        let c = &self.center.clone();

        for x in (c.x - RD)..=(c.x + RD) {
            for y in (c.y - RD)..=(c.y + RD) {
                let coord = ChunkCoord::new(x, y);

                if self.chunks.contains_key(&coord) {
                    continue;
                }

                let mut new_chunk = self.generate_chunk(&coord);

                if new_chunk.state == ChunkState::Unspawned {
                    self.spawn_queue
                        .push((new_chunk.clone(), ChunkCoord::new(x, y)));
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
            center: ChunkCoord::default(),
            chunks: HashMap::new(),
            spawn_queue: Vec::new(),
        }
    }
    pub fn has_center_changed(&self, new_chunk: &ChunkCoord) -> bool {
        *new_chunk != self.center
    }
    pub fn set_center_chunk(&mut self, new_chunk: ChunkCoord) {
        self.center = new_chunk;
    }
}
