use crate::entities::player_fsm;
use godot::prelude::*;

const STATE_NAME: &'static str = "Idle State";

pub struct IdleState;
impl player_fsm::StateBehavior for IdleState {
    fn on_enter(&mut self) {
        godot_print!("Enter {} ", STATE_NAME);
    }

    fn on_exit(&mut self) {
        godot_print!("Exit {} ", STATE_NAME);
    }
}
