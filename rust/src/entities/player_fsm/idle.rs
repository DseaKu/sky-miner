use crate::entities::player_fsm;
use godot::prelude::*;

const STATE_NAME: &str = "Idle State";

pub struct IdleState;
impl player_fsm::StateBehavior for IdleState {
    fn on_enter(&mut self) {
        godot_print!("Enter {} ", STATE_NAME);
    }

    fn on_exit(&mut self) {
        godot_print!("Exit {} ", STATE_NAME);
        // 1. Create the new state instance
        let next_state = State::Run(run::RunState::default());

        // 2. Call the transition method
        // 'current_state' would be the variable holding your current State enum instance
        current_state.transition_to(next_state);
    }
}
