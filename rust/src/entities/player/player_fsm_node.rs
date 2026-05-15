use crate::entities::player::{self, StateBehavior};
use godot::classes::{CharacterBody2D, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerFsmNode {
    fsm: player::State,
    data: player::PlayerData,
    base: Base<Node>,
}

#[godot_api]
impl PlayerFsmNode {
    #[func]
    pub fn get_state_name(&self) -> StringName {
        match &self.fsm {
            player::State::Idle(_) => StringName::from("IDLE"),
            player::State::Run(_) => StringName::from("RUN"),
            player::State::Jump(_) => StringName::from("JUMP"),
            player::State::Fall(_) => StringName::from("FALL"),
            player::State::Land(_) => StringName::from("LAND"),
        }
    }

    #[func]
    pub fn get_jumps_left(&self) -> i32 {
        self.data.jumps_left
    }
}

#[godot_api]
impl INode for PlayerFsmNode {
    fn init(base: Base<Node>) -> Self {
        let fsm = player::State::Idle(player::idle::IdleState);
        let data = player::PlayerData {
            jumps_left: player::constants::in_air::MAX_N_JUMP,
        };

        Self { fsm, data, base }
    }

    fn ready(&mut self) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            self.fsm.on_enter(&mut player, &mut self.data);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            if let Some(next_state) =
                self.fsm
                    .get_input_transition(&mut player, &mut self.data, event)
            {
                self.fsm
                    .transition_to(&mut player, &mut self.data, next_state);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            self.fsm.physics_update(&mut player, &mut self.data, delta);
            if let Some(next_state) =
                self.fsm
                    .get_poll_transition(&mut player, &mut self.data, delta)
            {
                self.fsm
                    .transition_to(&mut player, &mut self.data, next_state);
            }
        }
    }
}
