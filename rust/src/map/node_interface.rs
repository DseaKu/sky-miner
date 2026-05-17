use super::consts::path::*;
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
impl INode for MapGenNode {
    fn init(base: Base<Node>) -> Self {
        godot_print!("{}Initializing...", MAP_GEN_NODE_STR);
        Self {
            player_node: None,
            tile_map_node: None,
            map_gen: generator::MapGenerator::new(),
            base,
        }
    }

    fn ready(&mut self) {
        // Try to fetch Player Node
        if let Some(player) = self.base().try_get_node_as::<Node2D>(PLAYER_NODE_PATH) {
            let mut player_to_connect = player.clone();
            player_to_connect.connect(
                "tree_exiting",
                &self.base().callable("on_player_tree_exiting"),
            );
            self.player_node = Some(player);
            godot_print!("{}Linked to Player node", MAP_GEN_NODE_STR);
        } else {
            godot_warn!(
                "{}Could not fetch Player node at {}",
                MAP_GEN_NODE_STR,
                PLAYER_NODE_PATH
            );
        }

        // Try to fetch Tile Map Node
        if let Some(tile_map) = self
            .base()
            .try_get_node_as::<TileMapLayer>(TILE_MAP_LAYER_NODE_PATH)
        {
            let mut tile_map_to_connect = tile_map.clone();
            tile_map_to_connect.connect(
                "tree_exiting",
                &self.base().callable("on_tile_map_layer_exiting"),
            );
            self.tile_map_node = Some(tile_map);
            godot_print!("{}Linked to TileMapLayer node", MAP_GEN_NODE_STR);
        } else {
            godot_warn!(
                "{}Could not fetch TileMapLayer node at {}",
                MAP_GEN_NODE_STR,
                TILE_MAP_LAYER_NODE_PATH
            );
        }
    }

    fn process(&mut self, delta: f64) {
        if let Some(grid_pos) = self.get_player_grid_pos() {
            self.map_gen.update(delta, grid_pos);
        }
    }
}

#[godot_api]
impl MapGenNode {
    #[func]
    fn on_player_tree_exiting(&mut self) {
        godot_print!(
            "{}Player node exiting tree. Stopping processing.",
            MAP_GEN_NODE_STR
        );
        self.player_node = None;
        self.base_mut().set_process(false);
    }

    #[func]
    fn on_tile_map_layer_exiting(&mut self) {
        godot_print!(
            "{}Tile Map Layer node exiting tree. Stopping processing.",
            MAP_GEN_NODE_STR
        );
        self.tile_map_node = None;
        self.base_mut().set_process(false);
    }

    fn get_player_pos(&self) -> Option<Vector2> {
        if let Some(player) = &self.player_node {
            if player.is_instance_valid() {
                return Some(player.get_global_position());
            }
        }
        None
    }

    fn get_player_grid_pos(&self) -> Option<Vector2i> {
        if let (Some(player), Some(tile_map)) = (&self.player_node, &self.tile_map_node) {
            if player.is_instance_valid() && tile_map.is_instance_valid() {
                let global_pos = player.get_global_position();
                let local_pos = tile_map.to_local(global_pos);
                let grid_pos = tile_map.local_to_map(local_pos);
                return Some(grid_pos);
            }
        }
        None
    }
}
