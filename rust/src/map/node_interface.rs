use super::generator;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapGenNode {
    player_node: Option<Gd<Node>>,
    map_gen: generator::MapGenerator,
    base: Base<Node>,
}

#[godot_api]
impl INode for MapGenNode {
    fn init(base: Base<Node>) -> Self {
        godot_print!("\n##############################");
        godot_print!("###### Init MapGen Node ######");
        godot_print!("##############################");
        Self {
            player_node: None,
            map_gen: generator::MapGenerator::new(),
            base,
        }
    }

    fn ready(&mut self) {
        // Try to fetch Player Node
        let player_path = "../../Player";
        let map_node = " => MapGenNode: ";
        if let Some(player) = self.base().get_node_or_null(player_path) {
            godot_print!("{}Fetched Player Node", map_node);
            self.player_node = Some(player);
        } else {
            godot_warn!("{}Could not find Player node: {}", map_node, player_path);
        }

        self.map_gen.initialize();
    }
    fn process(&mut self, delta: f64) {
        self.map_gen.update(delta);
    }
}
