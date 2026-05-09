use enum_dispatch::enum_dispatch;

mod idle;
mod player_fsm_node;
mod run;

#[enum_dispatch]
pub trait StateBehavior {
    fn on_enter(&mut self) {}
    fn on_exit(&mut self) {}
}

#[enum_dispatch(StateBehavior)]
pub enum State {
    Idle(idle::IdleState),
    run(run::RunState),
}

impl State {
    fn transition_to(&mut self, mut new_state: State) {
        self.on_exit();
        new_state.on_enter();
        *self = new_state;
    }
}
