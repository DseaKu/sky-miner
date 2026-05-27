use super::tile_generator;
use crate::terrain::*;

use rand::{self, RngExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct ChunkGenerator {
    _perlin: noise::Perlin,
    center: ChunkCoord,
    chunks: HashMap<ChunkCoord, Chunk>,
    pub spawn_queue: Vec<(Chunk, ChunkCoord)>,
    pub despawn_queue: Vec<(Chunk, ChunkCoord)>,
    pub config: config::TerrainConfig,
}

impl ChunkGenerator {
    fn generate_chunk(&mut self, coord: &ChunkCoord) -> Chunk {
        let cs = self.config.chunk_size;
        let tg = tile_generator::TileGenerator;
        let mut new_chunk = Chunk::new(cs);

        for x in 0..cs {
            for y in 0..cs {
                let index = (y * cs + x) as usize;

                let local = LocalCoord::new(x, y);
                let global = local.to_global(*coord, cs);

                new_chunk.tiles[index] = tg.generate_tile(global.x, global.y);
            }
        }

        new_chunk
    }

    fn spawn_logic(&mut self, center: ChunkCoord, render_dist: i32) {
        let c = center;
        let rd = render_dist;
        for x in (c.x - rd)..=(c.x + rd) {
            for y in (c.y - rd)..=(c.y + rd) {
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

    fn despawn_logic(&mut self, center: ChunkCoord, render_dist: i32) {
        let c = center;
        let rd = render_dist;
        let to_remove: Vec<ChunkCoord> = self
            .chunks
            .iter()
            .filter(|(coord, chunk)| {
                let is_outside = (coord.x - c.x).abs() > rd || (coord.y - c.y).abs() > rd;
                is_outside && chunk.state != ChunkState::Modified
            })
            .map(|(coord, _)| *coord)
            .collect();

        for coord in to_remove {
            if let Some(chunk) = self.chunks.remove(&coord) {
                self.despawn_queue.push((chunk, coord));
            }
        }
    }
    pub fn update_chunks(&mut self) {
        let render_dist = self.config.render_distance;
        let center = self.center;

        self.spawn_logic(center, render_dist);
        self.despawn_logic(center, render_dist);
    }

    pub fn mark_dirty(&mut self, coord: &ChunkCoord) {
        if let Some(chunk) = self.chunks.get_mut(coord) {
            chunk.state = ChunkState::Modified;
        }
    }

    pub fn new(config: config::TerrainConfig) -> Self {
        let mut rng = rand::rng();
        let rnd_num: u32 = rng.random();
        crate::gd_print!("MapGenerator: Initialized with seed {}", rnd_num);

        Self {
            _perlin: noise::Perlin::new(rnd_num),
            center: ChunkCoord::default(),
            chunks: HashMap::new(),
            spawn_queue: Vec::new(),
            despawn_queue: Vec::new(),
            config,
        }
    }
    pub fn has_center_changed(&self, new_chunk: &ChunkCoord) -> bool {
        *new_chunk != self.center
    }
    pub fn set_center_chunk(&mut self, new_chunk: ChunkCoord) {
        self.center = new_chunk;
    }
}
