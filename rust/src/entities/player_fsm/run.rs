use crate::entities::player_fsm;
use godot::prelude::*;

const STATE_NAME: &str = "Run State";

pub struct RunState;
impl player_fsm::StateBehavior for RunState {
    fn on_enter(&mut self) {
        godot_print!("Enter {} ", STATE_NAME);
    }

    fn on_exit(&mut self) {
        godot_print!("Exit {} ", STATE_NAME);
    }
}
