use crate::entities::player_fsm::{self, StateBehavior};
use godot::prelude::*;
use godot::classes::CharacterBody2D;

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
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            self.fsm.on_enter(&mut player);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            if let Some(next_state) = self.fsm.physics_update(&mut player, delta) {
                self.fsm.transition_to(&mut player, next_state);
            }
        }
    }
}
