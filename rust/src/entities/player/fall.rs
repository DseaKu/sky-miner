use super::consts;
use crate::core::utils::FloatExt;
use crate::entities::player::{self, macros, State};
use godot::classes::{CharacterBody2D, Input, InputEvent};
use godot::prelude::*;
const STATE_NAME: &str = "FALL";

#[derive(Default)]
pub struct FallState;

impl player::StateBehavior for FallState {
    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>, data: &mut player::PlayerData) {
        macros::play_animation!(player, STATE_NAME.to_lowercase());
        if data.jumps_left == consts::v_move::jump::MAX_JUMPS {}
    }

    fn physics_update(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        delta: f64,
    ) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        let mut velocity = player.get_velocity();

        macros::flip_sprite!(player, direction);
        macros::apply_gravity!(velocity.y, delta);

        // Horizontal Movement
        if direction != 0.0 {
            velocity.x = FloatExt::lerp(
                velocity.x,
                consts::h_move::air::MAX_SPEED * direction,
                consts::h_move::air::ACCEL * delta as f32,
            );
        } else {
            velocity.x = FloatExt::move_toward(
                velocity.x,
                0.0,
                consts::h_move::air::FRICTION * delta as f32,
            )
        }

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_input_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        data: &mut player::PlayerData,
        event: Gd<InputEvent>,
    ) -> Option<State> {
        if data.jumps_left > 0 && event.is_action_pressed("jump") {
            return Some(State::Jump(player::jump::JumpState::default()));
        }
        None
    }

    fn get_poll_transition(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        _delta: f64,
    ) -> Option<State> {
        if player.is_on_floor() {
            return Some(State::Land(player::land::LandState::default()));
        }
        None
    }
}
