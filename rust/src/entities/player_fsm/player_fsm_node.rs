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
        let fsm = player_fsm::State::Idle(player_fsm::idle::IdleState);

        Self { fsm, base }
    }

    fn ready(&mut self) {
        self.fsm.on_enter();
    }
}
