use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapGenNode {
    base: Base<Node>,
}

#[godot_api]
impl INode for MapGenNode {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}
