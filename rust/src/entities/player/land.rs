use super::{consts, State};
use crate::entities::player;
use godot::classes::InputEvent;
use godot::prelude::*;

const STATE_NAME: &str = "Land";

#[derive(Default)]
pub struct LandState {
    timer: f64,
}

impl player::StateBehavior for LandState {
    fn on_enter(&mut self, ctx: &mut player::PlayerContext) {
        ctx.play_animation(&STATE_NAME.to_lowercase());
        ctx.data.jumps_left = consts::v_move::jump::MAX_JUMPS;
    }

    fn physics_update(&mut self, ctx: &mut player::PlayerContext, delta: f64) {
        self.timer += delta;
        ctx.apply_gravity(delta);
        ctx.handle_h_move(delta, false);
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

    fn get_poll_transition(&mut self, _ctx: &mut player::PlayerContext, _delta: f64) -> Option<State> {
        if self.timer > consts::h_move::ground::LAND_DURATION {
            return Some(State::Idle(player::idle::IdleState));
        }
        None
    }
}
