use crate::entities::player_fsm::{self, State};
use godot::classes::{CharacterBody2D, Input};
use godot::prelude::*;

const STATE_NAME: &str = "Idle State";

#[derive(Default)]
pub struct IdleState;

impl player_fsm::StateBehavior for IdleState {
    fn on_enter(&mut self, player: &mut Gd<CharacterBody2D>) {
        godot_print!("Enter {} ", STATE_NAME);
        if let Some(mut anim) = player.get_node_or_null("AnimationPlayer") {
            anim.call("play", &[Variant::from("idle")]);
        }
    }

    fn on_exit(&mut self, _player: &mut Gd<CharacterBody2D>) {
        godot_print!("Exit {} ", STATE_NAME);
    }

    fn physics_update(&mut self, player: &mut Gd<CharacterBody2D>, delta: f64) {
        let mut velocity = player.get_velocity();
        let friction = 3000.0;

        // Manual move_toward for f32
        let target = 0.0;
        let amount = (friction * delta) as f32;
        if (target - velocity.x).abs() <= amount {
            velocity.x = target;
        } else {
            velocity.x += (target - velocity.x).signum() * amount;
        }

        player.set_velocity(velocity);
        player.move_and_slide();
    }

    fn handle_transitions(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        let input = Input::singleton();

        if input.is_action_pressed("left") || input.is_action_pressed("right") {
            return Some(State::Run(player_fsm::run::RunState));
        }

        None
    }
}
