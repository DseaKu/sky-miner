use super::constants::ground;
use crate::core::utils::FloatExt;
use crate::entities::player::{self, macros, State};
use godot::classes::{CharacterBody2D, Input, InputEvent};
use godot::prelude::*;
const STATE_NAME: &str = "RUN";

#[derive(Default)]
pub struct RunState;

impl player::StateBehavior for RunState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        macros::play_animation!(player, "run");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        let mut velocity = player.get_velocity();

        macros::flip_sprite!(player, direction);
        macros::apply_gravity!(velocity.y, delta);

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
            return Some(State::Jump(player::jump::JumpState::default()));
        }
        None
    }

    fn get_poll_transition(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        if !player.is_on_floor() {
            return Some(State::Fall(player::fall::FallState::default()));
        }

        if direction == 0.0 {
            return Some(State::Idle(player::idle::IdleState));
        }

        None
    }
}
