use crate::core::utils::FloatExt;
use crate::entities::player_fsm::{self, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "IDLE";

#[derive(Default)]
pub struct IdleState;

impl player_fsm::StateBehavior for IdleState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        player_fsm::macros::play_animation!(player, "idle");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let mut velocity = player.get_velocity();
        velocity.x = FloatExt::move_toward(
            velocity.x,
            0.0,
            player_fsm::constants::FRICTION * delta as f32,
        );

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn handle_transitions(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        let input = Input::singleton();

        if input.is_action_just_pressed("jump") {
            return Some(State::Jump(player_fsm::jump::JumpState));
        }

        // Stop switching back and forth between run states if the left and right buttons are pressed.
        if input.is_action_pressed("left") && input.is_action_pressed("right") {
            return None;
        }

        if input.is_action_pressed("left") || input.is_action_pressed("right") {
            return Some(State::Run(player_fsm::run::RunState));
        }

        None
    }
}
