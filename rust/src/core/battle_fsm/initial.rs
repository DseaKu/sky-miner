use crate::core::battle_fsm::{self, Behavior, player_turn};
use godot::prelude::*;

pub struct State;

impl Behavior for State {
    fn name(&self) -> String {
        "Initialize".to_string()
    }
    
    fn on_enter(&mut self) {
        godot_print!("Initializing battle...");
    }
    
    fn handle_event(&mut self, event: &str) -> Option<battle_fsm::State> {
        if event == "StartBattle" {
            return Some(player_turn::State.into());
        }
        None
    }
}
