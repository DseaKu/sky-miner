use super::constants::{in_air, jump};
use crate::core::utils::FloatExt;
use crate::entities::player::{self, macros, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "JUMP";

pub struct JumpState {
    timer: f64,
    jump_released: bool,
}

impl Default for JumpState {
    fn default() -> Self {
        Self {
            timer: 0.0,
            jump_released: false,
        }
    }
}
impl player::StateBehavior for JumpState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        macros::play_animation!(player, "jump");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        self.timer += delta;
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        let mut velocity = player.get_velocity();

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

        // Vertical velocity
        if input.is_action_just_released("jump") {
            self.jump_released = true;
        }

        if self.timer < jump::MAX_DURATION && !self.jump_released {
            velocity.y = FloatExt::lerp(velocity.y, jump::MAX_SPEED, jump::ACCEL * delta as f32);
        } else {
            macros::apply_gravity!(velocity.y, delta);
        }

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_poll_transition(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        if player.get_velocity().y >= 0.0 {
            return Some(State::Land(player::land::LandState::default()));
        }

        None
    }
}
