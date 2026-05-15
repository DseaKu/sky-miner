use super::{consts, macros, State};

use crate::core::utils::FloatExt;
use crate::entities::player;

use godot::classes::{CharacterBody2D, Input, InputEvent};
use godot::prelude::*;

const STATE_NAME: &str = "Land";

#[derive(Default)]
pub struct LandState {
    timer: f64,
}

impl player::StateBehavior for LandState {
    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>, data: &mut player::PlayerData) {
        macros::play_animation!(player, STATE_NAME.to_lowercase());
        data.jumps_left = consts::v_move::jump::MAX_JUMPS;
    }

    fn physics_update(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        delta: f64,
    ) {
        use consts::h_move::air;
        use consts::h_move::ground as gnd;

        let input = Input::singleton();
        let direction = input.get_axis("left", "right");
        let mut velocity = player.get_velocity();

        self.timer += delta;

        macros::flip_sprite!(player, direction);
        macros::apply_gravity!(velocity.y, delta);

        // Horizontal velocity
        if direction != 0.0 {
            velocity.x = FloatExt::lerp(
                velocity.x,
                gnd::MAX_SPEED * direction,
                gnd::ACCEL_RUN * delta as f32,
            );
        } else {
            velocity.x = FloatExt::move_toward(velocity.x, 0.0, air::FRICTION * delta as f32)
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
        _player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        _delta: f64,
    ) -> Option<State> {
        use consts::h_move::ground as gnd;

        if self.timer > gnd::LAND_DURATION {
            return Some(State::Idle(player::idle::IdleState));
        }
        None
    }
}
