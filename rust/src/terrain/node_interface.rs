use crate::core::utils::ToVector2i;
use crate::terrain::ChunkCoord;
use crate::terrain::ChunkData;
use crate::terrain::LocalTileCoord;
use crate::terrain::TileCoord;
use crate::terrain::TileType;

use super::chunk_generator;
use super::consts;
use godot::classes::TileMapLayer;
use godot::prelude::*;
use std::path::PathBuf;

const PRINT_PREFIX: &str = "TerrainGenerator: ";

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TerrainGenerator {
    player_node: Option<Gd<Node2D>>,
    tile_map_node: Option<Gd<TileMapLayer>>,
    chunk_generator: chunk_generator::ChunkGenerator,
    config: crate::terrain::config::TerrainConfig,
    base: Base<Node>,
}

#[godot_api]
impl TerrainGenerator {
    #[func]
    fn on_player_tree_exiting(&mut self) {
        crate::on_exit_stop_process!(self, player_node, "Player", PRINT_PREFIX);
    }

    #[func]
    fn on_tile_map_layer_exiting(&mut self) {
        crate::on_exit_stop_process!(self, tile_map_node, "Tile Map Layer", PRINT_PREFIX);
    }

    fn get_player_info(&self) -> Option<(ChunkCoord, f64)> {
        let player = self.player_node.as_ref()?;
        let tile_map = self.tile_map_node.as_ref()?;

        let g_pos = player.get_global_position();
        let l_pos = tile_map.to_local(g_pos);
        let grid_pos = tile_map.local_to_map(l_pos);

        let chunk_coord = TileCoord::from(grid_pos).to_chunk(self.config.chunk_gen.chunk_size);
        Some((chunk_coord, g_pos.y as f64))
    }

    pub fn evaluate_chunks(&mut self) {
        if let Some((player_chunk, player_y)) = self.get_player_info() {
            let generator = &mut self.chunk_generator;
            generator.update_dynamic_params(player_y);

            if generator.has_center_changed(&player_chunk) {
                generator.set_center_chunk(player_chunk);
                generator.update_chunks();
            }
        }
    }

    pub fn process_spawning_queue(&mut self) {
        let tile_map = match &mut self.tile_map_node {
            Some(tm) => tm,
            None => return,
        };

        let spawn_queue = &mut self.chunk_generator.spawn_queue;

        let chunk_size = self.config.chunk_gen.chunk_size;
        if spawn_queue.is_empty() {
            return;
        }

        for ChunkData { chunk, coord } in spawn_queue.drain(..) {
            for (index, tile_type) in chunk.tiles.iter().enumerate() {
                let local = LocalTileCoord::new(
                    (index % chunk_size as usize) as i32,
                    (index / chunk_size as usize) as i32,
                );

                let tile = local.to_tile(coord, chunk_size);

                let source_id = if *tile_type == TileType::Sky {
                    -1
                } else {
                    self.config.atlas_coords.source_id
                };

                tile_map
                    .set_cell_ex(Vector2i::from(tile))
                    .source_id(source_id)
                    .atlas_coords(tile_type.to_atlas_coords(&self.config))
                    .done();
            }
        }
    }

    pub fn process_despawning_queue(&mut self) {
        let tile_map = match &mut self.tile_map_node {
            Some(tm) => tm,
            None => return,
        };
        let chunk_size = self.config.chunk_gen.chunk_size;

        let despawn_queue = &mut self.chunk_generator.despawn_queue;

        if despawn_queue.is_empty() {
            return;
        }

        for ChunkData { chunk: _, coord } in despawn_queue.drain(..) {
            for x in 0..chunk_size {
                for y in 0..chunk_size {
                    let local = LocalTileCoord::new(x, y);
                    let tile = local.to_tile(coord, chunk_size);
                    tile_map.set_cell(Vector2i::from(tile));
                }
            }
        }
    }

    #[func]
    pub fn mine_tile(&mut self, world_position: Vector2) -> bool {
        let tile_map = match &mut self.tile_map_node {
            Some(tm) => tm,
            None => return false,
        };

        let local_pos = tile_map.to_local(world_position);
        let grid_pos = tile_map.local_to_map(local_pos);

        let target_cell = tile_map.get_cell_atlas_coords(grid_pos);
        let empty_cell = self.config.atlas_coords.empty_cell.to_vector2i();
        let non_existing_cell = Vector2i::new(-1, -1);

        if target_cell != empty_cell && target_cell != non_existing_cell {
            tile_map
                .set_cell_ex(grid_pos)
                .source_id(self.config.atlas_coords.source_id)
                .atlas_coords(empty_cell)
                .done();

            self.chunk_generator.set_tile(grid_pos, TileType::Void);

            return true;
        }

        false
    }

    #[func]
    pub fn mark_chunk_dirty(&mut self, grid_pos: Vector2i) {
        let coord = TileCoord::from(grid_pos).to_chunk(self.config.chunk_gen.chunk_size);
        self.chunk_generator.mark_dirty(&coord);
    }

    #[func]
    pub fn save_config(&self) {
        self.config.save();
    }

    #[func]
    pub fn get_chunk_size(&self) -> i32 {
        self.config.chunk_gen.chunk_size
    }
}

#[godot_api]
impl INode for TerrainGenerator {
    fn process(&mut self, _delta: f64) {
        self.chunk_generator.poll_io();
        self.evaluate_chunks();
        self.process_despawning_queue();
        self.process_spawning_queue();
    }

    fn init(base: Base<Node>) -> Self {
        use godot::classes::ProjectSettings;
        use godot::obj::Singleton;

        crate::node_print!(PRINT_PREFIX, "Initializing...");
        let config = crate::terrain::config::TerrainConfig::load();

        chunk_generator::ChunkGenerator::clear_saved_chunks();

        let absolute_dir = ProjectSettings::singleton().globalize_path("user://chunks");
        crate::node_print!(
            PRINT_PREFIX,
            "Chunks will be saved to:\n => \"{}\"",
            absolute_dir
        );

        Self {
            player_node: None,
            tile_map_node: None,
            chunk_generator: chunk_generator::ChunkGenerator::new(
                config.clone(),
                Some(PathBuf::from(absolute_dir.to_string())),
            ),
            config,
            base,
        }
    }

    fn ready(&mut self) {
        use consts::path as p;

        self.base_mut().add_to_group("terrain");

        crate::link_node!(
            self,
            Node2D,
            p::PLAYER_NODE_PATH,
            player_node,
            "Player",
            "on_player_tree_exiting",
            PRINT_PREFIX
        );

        crate::link_node!(
            self,
            TileMapLayer,
            p::TILE_MAP_LAYER_NODE_PATH,
            tile_map_node,
            "TileMapLayer",
            "on_tile_map_layer_exiting",
            PRINT_PREFIX
        );

        if let Some((player_chunk, player_y)) = self.get_player_info() {
            self.chunk_generator.update_dynamic_params(player_y);
            self.chunk_generator.set_center_chunk(player_chunk);
            self.chunk_generator.update_chunks();
        }
    }
}
