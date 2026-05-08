use crate::core::battle_fsm::{self, Behavior, enemy_turn};
use godot::prelude::*;

pub struct State;

impl Behavior for State {
    fn name(&self) -> String {
        "PlayerTurn".to_string()
    }

    fn on_enter(&mut self) {
        godot_print!("Player Turn Started!");
    }

    fn handle_event(&mut self, event: &str) -> Option<battle_fsm::State> {
        if event == "EndTurnPressed" {
            return Some(enemy_turn::State.into());
        }
        None
    }
}
