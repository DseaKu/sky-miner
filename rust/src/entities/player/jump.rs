use super::consts;
use crate::core::utils::FloatExt;
use crate::entities::player::{self, State};
use godot::classes::text_server::Direction;
use godot::classes::{Input, InputEvent};
use godot::prelude::*;

#[derive(Default)]
pub struct JumpState {
    timer: f64,
    jump_released: bool,
    is_in_air_jump: bool,
}

impl player::StateBehavior for JumpState {
    fn on_enter(&mut self, ctx: &mut player::PlayerContext) {
        if ctx.player.is_on_floor() {
            ctx.play_animation("jump");
        } else {
            ctx.play_animation("air_slam");
            self.is_in_air_jump = true;
        }
    }

    fn physics_update(&mut self, ctx: &mut player::PlayerContext, delta: f64) {
        use consts::v_move::jump as jmp;

        self.timer += delta;

        let mut velocity = ctx.player.get_velocity();

        // Horizontal velocity
        ctx.handle_h_move(delta, true);

        // Vertical velocity
        let input = Input::singleton();

        if !input.is_action_pressed("jump") && self.timer > jmp::MIN_DURATION {
            self.jump_released = true;
        }

        // Immediately null downwards velocity to prevent self-canceling transition to FallState
        if self.is_in_air_jump {
            velocity.y = 0.0;
            self.is_in_air_jump = false;
            let direction = ctx.get_input_axis();
            if direction != 0.0 {
                use consts::h_move::ground as gnd;
                let mut accel = gnd::ACCEL_RUN;
                if direction.signum() != velocity.x.signum() && velocity.x != 0.0_f32 {
                    velocity.x = 0.0;
                    accel = gnd::ACCEL_TURN;
                }
                velocity.x = crate::core::utils::FloatExt::lerp(
                    velocity.x,
                    direction * gnd::MAX_SPEED,
                    accel * delta as f32,
                );
            }
        }
        // Adding force to the upwards momentum
        if self.timer < jmp::MAX_DURATION && !self.jump_released {
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
