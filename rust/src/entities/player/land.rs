use super::constants::{ground, in_air};
use crate::core::utils::FloatExt;
use crate::entities::player::{self, macros, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;
const STATE_NAME: &str = "Land";

#[derive(Default)]
pub struct LandState {
    timer: f64,
}

impl player::StateBehavior for LandState {
    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>, data: &mut player::PlayerData) {
        macros::play_animation!(player, STATE_NAME.to_lowercase());
        data.jumps_left = in_air::MAX_N_JUMP;
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

        self.timer += delta;

        macros::flip_sprite!(player, direction);
        macros::apply_gravity!(velocity.y, delta);

        // Horizontal velocity
        if direction != 0.0 {
            velocity.x = FloatExt::lerp(
                velocity.x,
                ground::MAX_SPEED * direction,
                ground::ACCEL * delta as f32,
            );
        } else {
            velocity.x = FloatExt::move_toward(velocity.x, 0.0, in_air::FRICTION * delta as f32)
        }

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn get_poll_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _data: &mut player::PlayerData,
        _delta: f64,
    ) -> Option<State> {
        if self.timer > ground::LAND_DURATION {
            return Some(State::Idle(player::idle::IdleState));
        }
        None
    }
}
