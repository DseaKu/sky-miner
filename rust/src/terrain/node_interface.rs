use crate::core::utils::ToVector2i;
use crate::terrain::consts::atlas_coords;
use crate::terrain::Coord;
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

    pub fn get_player_coord(player_pos: &Vector2i) -> Coord {
        use consts::CHUNK_SIZE as C_S;
        Coord::new(player_pos.x / C_S, player_pos.y / C_S)
    }
    pub fn evalute_chunks(&mut self) {
        if let Some(p_pos) = self.get_player_grid_pos() {
            let p_chunk = Self::get_player_coord(&p_pos);
            let c_g = &mut self.chunk_generator;

            if c_g.has_center_changed(&p_chunk) {
                c_g.set_center_chunk(p_chunk);
                c_g.update_chunks();
            }
        }
    }
    pub fn set_cell(&mut self, x: i32, y: i32, atlas_coords: Vector2i) {
        if let Some(tile_map) = &mut self.tile_map_node {
            use consts::atlas_coords as a_c;
            tile_map
                .set_cell_ex(Vector2i::new(x, y))
                .source_id(a_c::SOURCE_ID)
                .atlas_coords(atlas_coords)
                .done(); // Must call .done()
        }
    }

    pub fn process_spawning_queue(&self) {
        use consts::CHUNK_SIZE as C_S;
        let s_q = &self.chunk_generator.spawn_queue;

        if s_q.is_empty() {
            return;
        }
        for (chunk, coord) in s_q {
            for (index, tile_type) in chunk.tiles.iter().enumerate() {
                if *tile_type == TileType::Void {
                    continue;
                }
                // set cell
                //
            }
        }
    }
}

#[godot_api]
impl INode for TerrainGenerator {
    fn process(&mut self, _delta: f64) {
        Self::evalute_chunks(self);
        Self::process_spawning_queue(self);
    }

    fn init(base: Base<Node>) -> Self {
        crate::node_print!(PRINT_PREFIX, "Initializing...");
        Self {
            player_node: None,
            tile_map_node: None,
            chunk_generator: chunk_generator::ChunkGenerator::new(),
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
        if let Some(p_pos) = self.get_player_grid_pos() {
            let p_chunk = Self::get_player_coord(&p_pos);

            self.chunk_generator.set_center_chunk(p_chunk);
            self.chunk_generator.update_chunks();
        }

        self.set_cell(0, 0, TileType::Stone.to_atlas_coords());
    }
}
