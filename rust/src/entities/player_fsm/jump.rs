use super::constants::jump;
use crate::core::utils::FloatExt;
use crate::entities::player_fsm::{self, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "JUMP";

#[derive(Default)]
pub struct JumpState {
    timer: f64,
}

impl player_fsm::StateBehavior for JumpState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        player_fsm::macros::play_animation!(player, "jump");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        self.timer += delta;

        player_fsm::macros::flip_sprite!(player, direction);

        let mut velocity = player.get_velocity();

        if self.timer < jump::MAX_DURATION {
            velocity.y = FloatExt::lerp(velocity.y, jump::MAX_SPEED, jump::ACCEL * delta as f32);
        }
        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_poll_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        // let input = Input::singleton();
        // let direction = input.get_axis("left", "right");
        // if direction == 0.0 {
        //     return Some(State::Idle(player_fsm::idle::IdleState));
        // }
        None
    }
}
