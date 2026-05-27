use crate::core::utils::ToVector2i;
use crate::terrain::ChunkCoord;
use crate::terrain::GlobalCoord;
use crate::terrain::LocalCoord;
use crate::terrain::TileType;

use super::chunk_generator;
use super::consts;
use godot::classes::TileMapLayer;
use godot::prelude::*;

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

    fn get_player_grid_pos(&self) -> Option<Vector2i> {
        if let (Some(player), Some(tile_map)) = (&self.player_node, &self.tile_map_node) {
            let g_pos = player.get_global_position();
            let l_pos = tile_map.to_local(g_pos);
            let grid_pos = tile_map.local_to_map(l_pos);
            return Some(grid_pos);
        }
        None
    }

    pub fn get_player_coord(&self, player_pos: &Vector2i) -> ChunkCoord {
        GlobalCoord::new(player_pos.x, player_pos.y).to_chunk(self.config.chunk_size)
    }

    pub fn evaluate_chunks(&mut self) {
        if let Some(player_pos) = self.get_player_grid_pos() {
            let player_chunk = self.get_player_coord(&player_pos);
            let generator = &mut self.chunk_generator;

            if generator.has_center_changed(&player_chunk) {
                generator.set_center_chunk(player_chunk);
                generator.update_chunks();
            }
        }
    }

    pub fn process_spawning_queue(&mut self) {
        let chunk_size = self.config.chunk_size;

        if let Some(tile_map) = &mut self.tile_map_node {
            let spawn_queue = &mut self.chunk_generator.spawn_queue;

            if spawn_queue.is_empty() {
                return;
            }

            for (chunk, coord) in spawn_queue.drain(..) {
                for (index, tile_type) in chunk.tiles.iter().enumerate() {
                    if *tile_type == TileType::Void {
                        continue;
                    }

                    let local = LocalCoord::new(
                        (index % chunk_size as usize) as i32,
                        (index / chunk_size as usize) as i32,
                    );

                    let global = local.to_global(coord, chunk_size);

                    tile_map
                        .set_cell_ex(Vector2i::new(global.x, global.y))
                        .source_id(self.config.atlas_coords.source_id)
                        .atlas_coords(tile_type.to_atlas_coords(&self.config))
                        .done();
                }
            }
        }
    }

    pub fn process_despawning_queue(&mut self) {
        let chunk_size = self.config.chunk_size;

        if let Some(tile_map) = &mut self.tile_map_node {
            let despawn_queue = &mut self.chunk_generator.despawn_queue;

            if despawn_queue.is_empty() {
                return;
            }

            for (chunk, coord) in despawn_queue.drain(..) {
                for index in 0..chunk.tiles.len() {
                    let local = LocalCoord::new(
                        (index % chunk_size as usize) as i32,
                        (index / chunk_size as usize) as i32,
                    );

                    let global = local.to_global(coord, chunk_size);

                    tile_map.set_cell(Vector2i::new(global.x, global.y));
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

        // Check if there is a tile there
        let target_cell = tile_map.get_cell_atlas_coords(grid_pos);
        let empty_cell = self.config.atlas_coords.empty_cell.to_vector2i();
        let non_existing_cell = Vector2i::new(-1, -1);

        if target_cell != empty_cell && target_cell != non_existing_cell {
            let atlas_coords = &self.config.atlas_coords;
            if target_cell == atlas_coords.ore.to_vector2i() {
                crate::gd_print!("Mined Ore!");
            } else if target_cell == atlas_coords.stone.to_vector2i() {
                crate::gd_print!("Mined Stone.");
            } else if target_cell == atlas_coords.dirt.to_vector2i() {
                crate::gd_print!("Mined Dirt.");
            } else if target_cell == atlas_coords.gem.to_vector2i() {
                crate::gd_print!("Mined Gem!");
            }

            tile_map
                .set_cell_ex(grid_pos)
                .source_id(self.config.atlas_coords.source_id)
                .atlas_coords(empty_cell)
                .done();

            self.mark_chunk_dirty(grid_pos);

            return true;
        }

        false
    }

    #[func]
    pub fn mark_chunk_dirty(&mut self, grid_pos: Vector2i) {
        let coord = GlobalCoord::new(grid_pos.x, grid_pos.y).to_chunk(self.config.chunk_size);
        self.chunk_generator.mark_dirty(&coord);
    }

    #[func]
    pub fn save_config(&self) {
        self.config.save();
    }
}

#[godot_api]
impl INode for TerrainGenerator {
    fn process(&mut self, _delta: f64) {
        self.evaluate_chunks();
        self.process_spawning_queue();
        self.process_despawning_queue();
    }

    fn init(base: Base<Node>) -> Self {
        crate::node_print!(PRINT_PREFIX, "Initializing...");
        let config = crate::terrain::config::TerrainConfig::load();
        Self {
            player_node: None,
            tile_map_node: None,
            chunk_generator: chunk_generator::ChunkGenerator::new(config.clone()),
            config,
            base,
        }
    }

    fn ready(&mut self) {
        use consts::path as p;

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

        // Spawn chunks initial
        if let Some(player_pos) = self.get_player_grid_pos() {
            let player_chunk = self.get_player_coord(&player_pos);

            self.chunk_generator.set_center_chunk(player_chunk);
            self.chunk_generator.update_chunks();
        }
    }
}
