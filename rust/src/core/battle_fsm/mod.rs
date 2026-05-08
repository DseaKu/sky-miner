use enum_dispatch::enum_dispatch;

pub mod enemy_turn;
pub mod initial;
pub mod player_turn;

#[enum_dispatch]
pub trait Behavior {
    fn name(&self) -> String;
    fn on_enter(&mut self) {}
    fn on_exit(&mut self) {}
    fn handle_event(&mut self, _event: &str) -> Option<State>;
}

#[enum_dispatch(Behavior)]
pub enum State {
    Initial(initial::State),
    PlayerTurn(player_turn::State),
    EnemyTurn(enemy_turn::State),
}

impl State {
    pub fn process_event(&mut self, event: &str) -> bool {
        if let Some(new_state) = self.handle_event(event) {
            self.transition_to(new_state);
            return true;
        }
        false
    }

    fn transition_to(&mut self, mut new_state: State) {
        self.on_exit();
        new_state.on_enter();
        *self = new_state;
    }
}
