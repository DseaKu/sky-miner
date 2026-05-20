use crate::core::utils::FloatExt;
use crate::entities::player::{self, State};
use godot::classes::Input;
use godot::prelude::*;

#[derive(Default)]
pub struct JumpState {
    timer: f64,
    jump_released: bool,
    is_midair: bool,
}

impl player::StateBehavior for JumpState {
    fn on_enter(&mut self, ctx: &mut player::PlayerContext) {
        self.is_midair = !ctx.player.is_on_floor();

        if !self.is_midair {
            ctx.play_animation("jump");
        } else {
            ctx.play_animation("air_slam");

            // Kill Vertical velocity, to avoid instant transitioning to FallState
            let mut velocity = ctx.player.get_velocity();
            velocity.y = 0.0;

            // Kill and add horizontal momentum for an instant turn
            let input_dir = ctx.get_input_axis();
            if input_dir != 0.0 && input_dir.signum() != velocity.x.signum() {
                velocity.x = ctx.data.config.v_move.jump.immediate_turning_speed * input_dir;
            }

            ctx.player.set_velocity(velocity);
        }
    }

    fn physics_update(&mut self, ctx: &mut player::PlayerContext, delta: f64) {
        let (min_duration, max_duration, max_speed, accel) = {
            let jmp = &ctx.data.config.v_move.jump;
            (jmp.min_duration, jmp.max_duration, jmp.max_speed, jmp.accel)
        };

        self.timer += delta;

        // Horizontal velocity
        ctx.handle_h_move(delta, true);

        // Vertical velocity
        let mut velocity = ctx.player.get_velocity();
        let input = Input::singleton();

        if !input.is_action_pressed("jump") && self.timer > min_duration {
            self.jump_released = true;
        }

        // Adding force to the upwards momentum
        if self.timer < max_duration && !self.jump_released {
            velocity.y = FloatExt::lerp(velocity.y, max_speed, accel * delta as f32);
            ctx.player.set_velocity(velocity);

        // Reducing momentum
        } else {
            ctx.apply_gravity(delta);
        }

        ctx.move_and_slide();
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
        if self.timer >= ctx.data.config.v_move.jump.min_duration && ctx.player.get_velocity().y >= 0.0 {
            return Some(State::Fall(player::fall::FallState));
        }
        None
    }
}
