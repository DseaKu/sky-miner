use super::consts;
use crate::core::utils::FloatExt;
use crate::entities::player::{self, State};
use godot::classes::{Input, InputEvent};
use godot::prelude::*;

#[derive(Default)]
pub struct JumpState {
    timer: f64,
    jump_released: bool,
}

impl player::StateBehavior for JumpState {
    fn on_enter(&mut self, ctx: &mut player::PlayerContext) {
        if ctx.data.jumps_left == consts::v_move::jump::MAX_JUMPS {
            ctx.play_animation("jump");
        } else {
            ctx.play_animation("air_slam");
        }

        // Immediately apply upward impulse to prevent self-canceling transition to FallState
        let mut velocity = ctx.player.get_velocity();
        velocity.y = 0.0;
        ctx.player.set_velocity(velocity);
    }

    fn physics_update(&mut self, ctx: &mut player::PlayerContext, delta: f64) {
        use consts::v_move::jump as jmp;

        self.timer += delta;

        // Horizontal velocity
        ctx.handle_h_move(delta, true);

        // Vertical velocity
        let input = Input::singleton();

        if !input.is_action_pressed("jump") && self.timer > jmp::MIN_DURATION {
            self.jump_released = true;
        }

        // Adding force to the upwards momentum
        if self.timer < jmp::MAX_DURATION && !self.jump_released {
            let mut velocity = ctx.player.get_velocity();
            velocity.y = FloatExt::lerp(velocity.y, jmp::MAX_SPEED, jmp::ACCEL * delta as f32);
            ctx.player.set_velocity(velocity);

        // Reducing momentum
        } else {
            ctx.apply_gravity(delta);
        }

        ctx.move_and_slide();
    }

    fn get_input_transition(
        &mut self,
        ctx: &mut player::PlayerContext,
        event: Gd<InputEvent>,
    ) -> Option<State> {
        if ctx.data.jumps_left > 0 && event.is_action_pressed("jump") {
            return Some(State::Jump(player::jump::JumpState::default()));
        }
        None
    }

    fn on_exit(&mut self, ctx: &mut player::PlayerContext) {
        ctx.data.jumps_left -= 1;
    }

    fn get_poll_transition(
        &mut self,
        ctx: &mut player::PlayerContext,
        _delta: f64,
    ) -> Option<State> {
        // Only allow transitioning to Fall if we are actually moving downwards
        // AND we have finished the initial jump impulse duration.
        if self.timer >= consts::v_move::jump::MIN_DURATION && ctx.player.get_velocity().y >= 0.0 {
            return Some(State::Fall(player::fall::FallState));
        }
        None
    }
}
