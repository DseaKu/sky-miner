use super::generator;
use godot::classes::TileMapLayer;
use godot::prelude::*;

const MAP_GEN_NODE_STR: &str = "[mine_world] MapGenNode: ";

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
        let player_path = "../../Player";
        if let Some(player) = self.base().try_get_node_as::<Node2D>(player_path) {
            godot_print!("{}Linked to Player node", MAP_GEN_NODE_STR);
            self.player_node = Some(player);
        } else {
            godot_warn!(
                "{}Could not fetch Player node at {}",
                MAP_GEN_NODE_STR,
                player_path
            );
        }

        // Try to fetch Tile Map Node
        let tile_map_path = "../TileMapLayer";
        if let Some(tile_map) = self.base().try_get_node_as::<TileMapLayer>(tile_map_path) {
            godot_print!("{}Linked to TileMapLayer node", MAP_GEN_NODE_STR);
            self.tile_map_node = Some(tile_map);
        } else {
            godot_warn!(
                "{}Could not fetch TileMapLayer node at {}",
                MAP_GEN_NODE_STR,
                tile_map_path
            );
        }
    }

    fn process(&mut self, delta: f64) {
        self.map_gen.update(delta);
    }
}

impl MapGenNode {
    pub fn get_player_pos(&self) -> Option<Vector2> {
        if let Some(player) = &self.player_node {
            if player.is_instance_valid() {
                return Some(player.get_global_position());
            }
        }
        godot_warn!(
            "{}Cannot get player position. Player node is no longer available",
            MAP_GEN_NODE_STR
        );
        None
    }
}
