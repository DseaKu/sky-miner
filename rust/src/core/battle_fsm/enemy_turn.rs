use crate::core::battle_fsm::{self, player_turn, Behavior};
use godot::prelude::*;

pub struct State;

impl Behavior for State {
    fn name(&self) -> String {
        "EnemyTurn".to_string()
    }

    fn on_enter(&mut self) {
        godot_print!("Enemy Turn Started!");
    }

    fn handle_event(&mut self, event: &str) -> Option<battle_fsm::State> {
        if event == "EnemyFinished" {
            godot_print!("Enemy finished turn.");
            return Some(player_turn::State.into());
        }
        None
    }
}
