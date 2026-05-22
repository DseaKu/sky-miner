use crate::core::utils::ToVector2i;
use crate::terrain::consts::atlas_coords::{DIRT, SOURCE_ID};
use crate::terrain::Coord;

use super::consts;
use super::consts::path;
use super::generator;
use godot::classes::TileMapLayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TerrainGenerator {
    player_node: Option<Gd<Node2D>>,
    tile_map_node: Option<Gd<TileMapLayer>>,
    map_gen: generator::MapGenerator,
    base: Base<Node>,
}

#[godot_api]
impl TerrainGenerator {
    #[func]
    fn on_player_tree_exiting(&mut self) {
        crate::on_map_exit_stop_process!(self, player_node, "Player");
    }

    #[func]
    fn on_tile_map_layer_exiting(&mut self) {
        crate::on_map_exit_stop_process!(self, tile_map_node, "Tile Map Layer");
    }

    fn get_player_grid_pos(&self) -> Option<Vector2i> {
        if let (Some(player), Some(tile_map)) = (&self.player_node, &self.tile_map_node) {
            let global_pos = player.get_global_position();
            let local_pos = tile_map.to_local(global_pos);
            let grid_pos = tile_map.local_to_map(local_pos);
            return Some(grid_pos);
        }
        None
    }

    pub fn set_cell(&mut self) {
        if let Some(tile_map) = &mut self.tile_map_node {
            tile_map
                .set_cell_ex(Vector2i::new(0, 0))
                .source_id(SOURCE_ID)
                .atlas_coords(DIRT.to_vector2i())
                .done(); // Must call .done()
        }
    }
}

#[godot_api]
impl INode for TerrainGenerator {
    fn process(&mut self, _delta: f64) {
        if let Some(p_pos) = self.get_player_grid_pos() {
            use consts::gen::CHUNK_SIZE as C_S;

            let g = &mut self.map_gen;
            let p_chunk = Coord::new(p_pos.x / C_S, p_pos.y / C_S);

            if g.has_chunk_changed(&p_chunk) {
                g.set_cur_chunk(p_chunk);
                g.update_chunks();
            }
        }
    }
    fn init(base: Base<Node>) -> Self {
        crate::map_print!("Initializing...");
        Self {
            player_node: None,
            tile_map_node: None,
            map_gen: generator::MapGenerator::new(),
            base,
        }
    }

    fn ready(&mut self) {
        crate::link_map_node!(
            self,
            Node2D,
            path::PLAYER_NODE_PATH,
            player_node,
            "Player",
            "on_player_tree_exiting"
        );

        crate::link_map_node!(
            self,
            TileMapLayer,
            path::TILE_MAP_LAYER_NODE_PATH,
            tile_map_node,
            "TileMapLayer",
            "on_tile_map_layer_exiting"
        );

        self.set_cell();
    }
}
