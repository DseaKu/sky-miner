use crate::entities::player_fsm::{self, StateBehavior};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerFsmNode {
    fsm: player_fsm::State,
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerFsmNode {
    fn init(base: Base<Node>) -> Self {
        let mut fsm = player_fsm::State::Idle(player_fsm::idle::IdleState);
        fsm.on_enter();

        Self { fsm, base }
    }
}
