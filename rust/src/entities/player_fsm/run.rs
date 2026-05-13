use crate::core::utils::FloatExt;
use crate::entities::player_fsm::{self, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "Run";

#[derive(Default)]
pub struct RunState;

impl player_fsm::StateBehavior for RunState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        player_fsm::macros::play_animation!(player, "run");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        let mut velocity = player.get_velocity();

        velocity.x = FloatExt::lerp(
            velocity.x,
            direction * player_fsm::constants::MAX_SPEED,
            player_fsm::constants::ACCEL * delta as f32,
        );
        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn handle_transitions(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        if direction == 0.0 {
            return Some(State::Idle(player_fsm::idle::IdleState));
        }
        None
    }
}
