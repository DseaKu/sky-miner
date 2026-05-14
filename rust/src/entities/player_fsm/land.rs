use crate::entities::player_fsm::{self, macros, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "LAND";

#[derive(Default)]
pub struct LandState;

impl player_fsm::StateBehavior for LandState {
    fn get_name(&self) -> Option<String> {
        Some(STATE_NAME.to_string())
    }

    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        macros::play_animation!(player, "jump");
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        macros::flip_sprite!(player, direction);

        let mut velocity = player.get_velocity();

        macros::apply_gravity!(velocity.y, delta);
        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_poll_transition(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        if player.is_on_floor() {
            return Some(State::Idle(player_fsm::idle::IdleState));
        }

        None
    }
}
