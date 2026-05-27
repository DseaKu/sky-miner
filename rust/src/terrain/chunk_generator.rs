use super::tile_generator::TileGenerator;
use crate::terrain::*;
use godot::prelude::PackedByteArray;

use std::collections::HashMap;

const PRINT_PREFIX: &str = "ChunkGenerator: ";

#[derive(Default)]
pub struct ChunkGenerator {
    center: ChunkCoord,
    chunks: HashMap<ChunkCoord, Chunk>,
    tile_gen: tile_generator::TileGenerator,
    pub spawn_queue: Vec<(Chunk, ChunkCoord)>,
    pub despawn_queue: Vec<(Chunk, ChunkCoord)>,
    pub config: config::TerrainConfig,
}

impl ChunkGenerator {
    fn get_chunk_path(coord: &ChunkCoord) -> String {
        format!("user://chunks/chunk_{}_{}.bin", coord.x, coord.y)
    }

    fn save_chunk(coord: &ChunkCoord, chunk: &Chunk) {
        use godot::classes::file_access::ModeFlags;
        use godot::classes::{DirAccess, FileAccess};

        if let Some(mut dir) = DirAccess::open("user://") {
            if !dir.dir_exists("chunks") {
                dir.make_dir("chunks");
            }
        } else {
            crate::gd_error!("{}Failed to open user:// directory", PRINT_PREFIX);
            return;
        }

        let path = Self::get_chunk_path(coord);
        if let Ok(encoded) = bincode::serialize(chunk) {
            if let Some(mut file) = FileAccess::open(&path, ModeFlags::WRITE) {
                let bytes = PackedByteArray::from_iter(encoded);
                file.store_buffer(&bytes);
            }
        }
    }
    fn load_chunk(&self, coord: &ChunkCoord) -> Option<Chunk> {
        use godot::classes::file_access::ModeFlags;
        use godot::classes::FileAccess;

        let path = Self::get_chunk_path(coord);
        if FileAccess::file_exists(&path) {
            if let Some(mut file) = FileAccess::open(&path, ModeFlags::READ) {
                let len = file.get_length() as i64;
                let bytes = file.get_buffer(len).to_vec();
                if let Ok(chunk) = bincode::deserialize::<Chunk>(&bytes) {
                    return Some(chunk);
                }
            }
        }
        None
    }

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
                crate::gd_print!("ChunkGenerator: Cleared all saved chunks in\n => {}", path);
            }
        }
    }
    fn generate_chunk(&mut self, coord: &ChunkCoord) -> Chunk {
        if let Some(saved_chunk) = self.load_chunk(coord) {
            return saved_chunk;
        }

        let chunk_size = self.config.chunk_size;
        let mut new_chunk = Chunk::new(chunk_size);

        for x in 0..chunk_size {
            for y in 0..chunk_size {
                let index = (y * chunk_size + x) as usize;

                let local = LocalCoord::new(x, y);
                let global = local.to_global(*coord, chunk_size);

                new_chunk.tiles[index] = self.tile_gen.generate_tile(global.x, global.y);
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
            .filter(|(coord, _)| coord.is_outside_render_distance(&center, render_dist))
            .map(|(coord, _)| *coord)
            .collect();

        for coord in to_remove {
            if let Some(chunk) = self.chunks.remove(&coord) {
                if chunk.is_modified {
                    Self::save_chunk(&coord, &chunk);
                }
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

    pub fn set_tile(&mut self, grid_pos: Vector2i, tile: TileType) {
        let chunk_size = self.config.chunk_size;
        let coord = GlobalCoord::new(grid_pos.x, grid_pos.y).to_chunk(chunk_size);

        if let Some(chunk) = self.chunks.get_mut(&coord) {
            let local_x = grid_pos.x.rem_euclid(chunk_size);
            let local_y = grid_pos.y.rem_euclid(chunk_size);
            let index = (local_y * chunk_size + local_x) as usize;
            if index < chunk.tiles.len() {
                chunk.tiles[index] = tile;
                chunk.is_modified = true;
            }
        }
    }

    pub fn new(config: config::TerrainConfig) -> Self {
        Self {
            center: ChunkCoord::default(),
            chunks: HashMap::new(),
            tile_gen: TileGenerator::new(),
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
