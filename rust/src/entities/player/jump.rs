use super::consts;
use crate::core::utils::FloatExt;
use crate::entities::player::{self, macros, State};
use godot::classes::{CharacterBody2D, Input, InputEvent};
use godot::prelude::*;
const STATE_NAME: &str = "JUMP";

#[derive(Default)]
pub struct JumpState {
    timer: f64,
    jump_released: bool,
}

impl player::StateBehavior for JumpState {
    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>, _data: &mut player::PlayerData) {
        macros::play_animation!(player, STATE_NAME.to_lowercase());
    }

    fn physics_update(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        delta: f64,
    ) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        let mut velocity = player.get_velocity();

        self.timer += delta;

        // Horizontal velocity
        if direction != 0.0 {
            velocity.x = FloatExt::lerp(
                velocity.x,
                consts::h_move::air::MAX_SPEED * direction,
                consts::h_move::air::ACCEL * delta as f32,
            );
        } else {
            velocity.x = FloatExt::move_toward(
                velocity.x,
                0.0,
                consts::h_move::air::FRICTION * delta as f32,
            )
        }

        // Vertical velocity
        if input.is_action_just_released("jump") {
            self.jump_released = true;
        }

        if self.timer < consts::v_move::jump::DURATION && !self.jump_released {
            velocity.y = FloatExt::lerp(
                velocity.y,
                consts::v_move::jump::MAX_SPEED,
                consts::v_move::jump::ACCEL * delta as f32,
            );
        } else {
            macros::apply_gravity!(velocity.y, delta);
        }

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_input_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        data: &mut player::PlayerData,
        event: Gd<InputEvent>,
    ) -> Option<State> {
        if data.jumps_left > 0 && event.is_action_pressed("jump") {
            return Some(State::Jump(player::jump::JumpState::default()));
        }
        None
    }

    fn on_exit(&mut self, _player: &mut Gd<CharacterBody2D>, data: &mut player::PlayerData) {
        data.jumps_left -= 1;
    }

    fn get_poll_transition(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        _delta: f64,
    ) -> Option<State> {
        if player.get_velocity().y >= 0.0 {
            return Some(State::Fall(player::fall::FallState));
        }

        None
    }
}
