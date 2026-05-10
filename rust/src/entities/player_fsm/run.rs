use crate::entities::player_fsm::{self, State};
use godot::prelude::*;
use godot::classes::{CharacterBody2D, Input};

const STATE_NAME: &str = "Run State";

#[derive(Default)]
pub struct RunState;

impl player_fsm::StateBehavior for RunState {
    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        godot_print!("Enter {} ", STATE_NAME);
        player_fsm::play_animation!(player, "run");
    }

    fn on_exit(&mut self, _player: &mut Gd<CharacterBody2D>) {
        godot_print!("Exit {} ", STATE_NAME);
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        let mut velocity = player.get_velocity();
        let max_speed = 400.0;
        let accel = 1.5;

        let target_x = direction * max_speed;
        velocity.x = velocity.x + (target_x - velocity.x) * (accel * delta) as f32;

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn handle_transitions(&mut self, _player: &mut Gd<CharacterBody2D>, _delta: f64) -> Option<State> {
        let input = Input::singleton();
        let direction = input.get_axis("left", "right");

        if direction == 0.0 {
            return Some(State::Idle(player_fsm::idle::IdleState::default()));
        }

        None
    }
    }

