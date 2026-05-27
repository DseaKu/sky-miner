use super::tile_generator::TileGenerator;
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
        let chunk_size = self.config.chunk_size;
        let mut new_chunk = Chunk::new(chunk_size);

        for x in 0..chunk_size {
            for y in 0..chunk_size {
                let index = (y * chunk_size + x) as usize;

                let local = LocalCoord::new(x, y);
                let global = local.to_global(*coord, chunk_size);

                new_chunk.tiles[index] = TileGenerator.generate_tile(global.x, global.y);
            }
        }

        new_chunk
    }

    fn spawn_logic(&mut self, center: ChunkCoord, render_dist: i32) {
        for x in (center.x - render_dist)..=(center.x + render_dist) {
            for y in (center.y - render_dist)..=(center.y + render_dist) {
                let coord = ChunkCoord::new(x, y);

                if self.chunks.contains_key(&coord) {
                    continue;
                }

                let new_chunk = self.generate_chunk(&coord);
                self.spawn_queue
                    .push((new_chunk.clone(), ChunkCoord::new(x, y)));
                self.chunks.insert(coord, new_chunk);
            }
        }
    }

    fn despawn_logic(&mut self, center: ChunkCoord, render_dist: i32) {
        let to_remove: Vec<ChunkCoord> = self
            .chunks
            .iter()
            .filter(|(coord, chunk)| {
                let is_outside = (coord.x - center.x).abs() > render_dist
                    || (coord.y - center.y).abs() > render_dist;
                is_outside && !chunk.is_modified
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
            chunk.is_modified = true;
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
