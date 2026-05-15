use super::constants::in_air;
use crate::core::utils::FloatExt;
use crate::entities::player::{self, macros, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "FALL";

pub struct FallState {
    jumps_left: i32,
}

impl Default for FallState {
    fn default() -> Self {
        Self {
            jumps_left: in_air::MAX_N_JUMP,
        }
    }
}
impl player::StateBehavior for FallState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        macros::play_animation!(player, "fall");
        self.jumps_left -= 1;
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        let mut velocity = player.get_velocity();

        macros::flip_sprite!(player, direction);
        macros::apply_gravity!(velocity.y, delta);

        // Horizontal velocity
        if direction != 0.0 {
            velocity.x = FloatExt::lerp(
                velocity.x,
                in_air::MAX_SPEED_X * direction,
                in_air::ACCEL_X * delta as f32,
            );
        } else {
            velocity.x = FloatExt::move_toward(velocity.x, 0.0, in_air::FRICTION * delta as f32)
        }

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_poll_transition(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        let input = Input::singleton();

        if self.jumps_left >= 0 && input.is_action_just_pressed("jump") {
            return Some(State::Jump(player::jump::JumpState::default()));
        }

        if player.is_on_floor() {
            return Some(State::Idle(player::idle::IdleState));
        }

        None
    }
}
