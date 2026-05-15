use crate::entities::player::{self, State};
use godot::classes::InputEvent;
use godot::prelude::*;
const STATE_NAME: &str = "FALL";

#[derive(Default)]
pub struct FallState;

impl player::StateBehavior for FallState {
    fn on_enter(&mut self, ctx: &mut player::PlayerContext) {
        ctx.play_animation(&STATE_NAME.to_lowercase());
    }

    fn physics_update(&mut self, ctx: &mut player::PlayerContext, delta: f64) {
        ctx.apply_gravity(delta);
        ctx.handle_h_move(delta, true);
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

    fn get_poll_transition(&mut self, ctx: &mut player::PlayerContext, _delta: f64) -> Option<State> {
        if ctx.player.is_on_floor() {
            return Some(State::Land(player::land::LandState::default()));
        }
        None
    }
}
