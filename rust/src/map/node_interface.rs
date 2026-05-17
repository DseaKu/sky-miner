use super::generator;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapGenNode {
    map_gen: generator::MapGenerator,
    base: Base<Node>,
}

#[godot_api]
impl INode for MapGenNode {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Initilaize Map Generator Node");
        Self {
            map_gen: generator::MapGenerator::new(),
            base,
        }
    }

    fn ready(&mut self) {
        self.map_gen.initialize();
    }
    fn process(&mut self, delta: f64) {
        self.map_gen.update(delta);
    }
}
