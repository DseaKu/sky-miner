use super::consts::path;
use super::generator;
use godot::classes::TileMapLayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapGenNode {
    player_node: Option<Gd<Node2D>>,
    tile_map_node: Option<Gd<TileMapLayer>>,
    map_gen: generator::MapGenerator,
    base: Base<Node>,
}

#[godot_api]
impl MapGenNode {
    #[func]
    fn on_player_tree_exiting(&mut self) {
        crate::on_exit_stop_process!(self, player_node, "Player");
    }

    #[func]
    fn on_tile_map_layer_exiting(&mut self) {
        crate::on_exit_stop_process!(self, tile_map_node, "Tile Map Layer");
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
            // 3. Use set_cell_ex() for optional parameters, and remove named args
            tile_map
                .set_cell_ex(Vector2i::new(0, 0))
                .source_id(2)
                .atlas_coords(Vector2i::new(1, 0))
                .done(); // You must call .done()
        }
    }
}

#[godot_api]
impl INode for MapGenNode {
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
        crate::link_node!(
            self,
            Node2D,
            path::PLAYER_NODE_PATH,
            player_node,
            "Player",
            "on_player_tree_exiting"
        );

        crate::link_node!(
            self,
            TileMapLayer,
            path::TILE_MAP_LAYER_NODE_PATH,
            tile_map_node,
            "TileMapLayer",
            "on_tile_map_layer_exiting"
        );

        self.set_cell();
    }

    fn process(&mut self, delta: f64) {
        if let Some(grid_pos) = self.get_player_grid_pos() {
            self.map_gen.update(delta, grid_pos);
        }
    }
}
