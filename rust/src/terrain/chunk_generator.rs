use super::tile_generator::TileGenerator;
use crate::terrain::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

const PRINT_PREFIX: &str = "ChunkGenerator: ";

pub struct ChunkGenerator {
    center: ChunkCoord,
    chunk_hash_map: HashMap<ChunkCoord, Chunk>,
    tile_gen: tile_generator::TileGenerator,
    pub spawn_queue: Vec<ChunkData>,
    pub despawn_queue: Vec<ChunkData>,
    pub config: config::TerrainConfig,
    io_handler: Option<io_handler::IOHandler>,
    loading_set: HashSet<ChunkCoord>,
}

impl ChunkGenerator {
    pub fn clear_saved_chunks() {
        use godot::classes::DirAccess;

        let path = "user://chunks";
        if DirAccess::dir_exists_absolute(path) {
            if let Some(mut dir) = DirAccess::open(path) {
                dir.list_dir_begin();
                let mut file_name = dir.get_next();
                while !file_name.is_empty() {
                    if !dir.current_is_dir() {
                        dir.remove(&file_name);
                    }
                    file_name = dir.get_next();
                }
                dir.list_dir_end();
                crate::node_print!(PRINT_PREFIX, "Cleared all saved chunks in\n => {}", path);
            }
        }
    }

    fn generate_chunk_internal(&mut self, coord: &ChunkCoord) -> Chunk {
        let chunk_size = self.config.chunk_gen.chunk_size;
        let cs_usize = chunk_size as usize;
        let mut new_chunk = Chunk::new(chunk_size);

        new_chunk
            .tiles
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, tile_type)| {
                let local =
                    LocalTileCoord::new((index % cs_usize) as i32, (index / cs_usize) as i32);

                let tile = local.to_tile(*coord, chunk_size);

                *tile_type = self.tile_gen.generate_tile(tile.x, tile.y);
            });

        new_chunk
    }

    pub fn poll_io(&mut self) {
        let mut responses = Vec::new();
        if let Some(io) = &self.io_handler {
            while let Some(response) = io.poll() {
                responses.push(response);
            }
        }

        for response in responses {
            match response {
                io_handler::IOResponse::Loaded(coord, chunk) => {
                    self.loading_set.remove(&coord);
                    self.spawn_queue.push(ChunkData::new(chunk.clone(), coord));
                    self.chunk_hash_map.insert(coord, chunk);
                }
                io_handler::IOResponse::NotFound(coord) => {
                    self.loading_set.remove(&coord);
                    let new_chunk = self.generate_chunk_internal(&coord);
                    self.spawn_queue
                        .push(ChunkData::new(new_chunk.clone(), coord));
                    self.chunk_hash_map.insert(coord, new_chunk);
                }
            }
        }
    }

    fn spawn_logic(&mut self, center: ChunkCoord, render_dist: i32) {
        for x in (center.x - render_dist)..=(center.x + render_dist) {
            for y in (center.y - render_dist)..=(center.y + render_dist) {
                let coord = ChunkCoord::new(x, y);

                if self.chunk_hash_map.contains_key(&coord) || self.loading_set.contains(&coord) {
                    continue;
                }

                if let Some(io) = &self.io_handler {
                    io.load(coord);
                    self.loading_set.insert(coord);
                } else {
                    let new_chunk = self.generate_chunk_internal(&coord);
                    self.spawn_queue
                        .push(ChunkData::new(new_chunk.clone(), coord));
                    self.chunk_hash_map.insert(coord, new_chunk);
                }
            }
        }
    }

    fn despawn_logic(&mut self, center: ChunkCoord, render_dist: i32) {
        let to_remove: Vec<ChunkCoord> = self
            .chunk_hash_map
            .iter()
            .filter(|(coord, _)| coord.is_outside_render_distance(&center, render_dist))
            .map(|(coord, _)| *coord)
            .collect();

        for coord in to_remove {
            if let Some(chunk) = self.chunk_hash_map.remove(&coord) {
                if chunk.is_modified {
                    if let Some(io) = &self.io_handler {
                        io.save(coord, chunk.clone());
                    }
                }
                self.despawn_queue.push(ChunkData::new(chunk, coord));
            }
        }
    }
    pub fn update_chunks(&mut self) {
        let render_dist = self.config.chunk_gen.render_distance;
        let center = self.center;

        self.spawn_logic(center, render_dist);
        self.despawn_logic(center, render_dist);
    }

    pub fn mark_dirty(&mut self, coord: &ChunkCoord) {
        if let Some(chunk) = self.chunk_hash_map.get_mut(coord) {
            chunk.is_modified = true;
        }
    }

    pub fn set_tile(&mut self, grid_pos: Vector2i, tile: TileType) {
        let chunk_size = self.config.chunk_gen.chunk_size;
        let tile_coord = TileCoord::from(grid_pos);
        let coord = tile_coord.to_chunk(chunk_size);

        if let Some(chunk) = self.chunk_hash_map.get_mut(&coord) {
            let local_x = grid_pos.x.rem_euclid(chunk_size);
            let local_y = grid_pos.y.rem_euclid(chunk_size);
            let index = (local_y * chunk_size + local_x) as usize;
            if index < chunk.tiles.len() {
                chunk.tiles[index] = tile;
                chunk.is_modified = true;
            }
        }
    }

    pub fn new(config: config::TerrainConfig, base_path: Option<PathBuf>) -> Self {
        let io_handler = base_path.map(io_handler::IOHandler::new);
        Self {
            center: ChunkCoord::default(),
            chunk_hash_map: HashMap::new(),
            tile_gen: TileGenerator::new(config.tile_gen.clone()),
            spawn_queue: Vec::new(),
            despawn_queue: Vec::new(),
            config,
            io_handler,
            loading_set: HashSet::new(),
        }
    }
    pub fn has_center_changed(&self, new_chunk: &ChunkCoord) -> bool {
        *new_chunk != self.center
    }
    pub fn set_center_chunk(&mut self, new_chunk: ChunkCoord) {
        self.center = new_chunk;
    }
    pub fn update_dynamic_params(&mut self, player_y: f64) {
        self.tile_gen.update_dynamic_params(player_y);
    }
}
