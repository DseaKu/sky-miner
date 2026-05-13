use super::constants::ground;
use crate::core::utils::FloatExt;
use crate::entities::player_fsm::{self, macros, State};
use godot::classes::{CharacterBody2D, Input, InputEvent};
use godot::prelude::*;
const STATE_NAME: &str = "RUN";

#[derive(Default)]
pub struct RunState;

impl player_fsm::StateBehavior for RunState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        macros::play_animation!(player, "run");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        macros::flip_sprite!(player, direction);

        let mut velocity = player.get_velocity();

        // Boost acceleration when changing directions to overcome existing momentum quickly and make turning feel more responsive.
        let mut accel = ground::ACCEL;
        if direction.signum() != velocity.x.signum() && velocity.x != 0.0_f32 {
            accel = ground::ACCEL_TURN;
        }

        velocity.x = FloatExt::lerp(
            velocity.x,
            direction * ground::MAX_SPEED,
            accel * delta as f32,
        );
        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_input_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        event: Gd<InputEvent>,
    ) -> Option<State> {
        if event.is_action_pressed("jump") {
            return Some(State::Jump(player_fsm::jump::JumpState::default()));
        }
        None
    }

    fn get_poll_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        if direction == 0.0 {
            return Some(State::Idle(player_fsm::idle::IdleState));
        }

        None
    }
}
