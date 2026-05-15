use crate::entities::player::{self, StateBehavior};
use godot::classes::{CharacterBody2D, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerFsmNode {
    fsm: player::State,
    base: Base<Node>,
}

#[godot_api]
impl PlayerFsmNode {
    #[func]
    pub fn get_state_name(&self) -> String {
        self.fsm.get_name().unwrap_or_else(|| "Unknown".to_string())
    }
}

#[godot_api]
impl INode for PlayerFsmNode {
    fn init(base: Base<Node>) -> Self {
        let fsm = player::State::Idle(player::idle::IdleState);

        Self { fsm, base }
    }

    fn ready(&mut self) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            self.fsm.on_enter(&mut player);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            if let Some(next_state) = self.fsm.get_input_transition(&mut player, event) {
                self.fsm.transition_to(&mut player, next_state);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            self.fsm.physics_update(&mut player, delta);
            if let Some(next_state) = self.fsm.get_poll_transition(&mut player, delta) {
                self.fsm.transition_to(&mut player, next_state);
            }
        }
    }
}
