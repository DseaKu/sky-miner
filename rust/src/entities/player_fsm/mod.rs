use enum_dispatch::enum_dispatch;
use godot::classes::CharacterBody2D;
use godot::prelude::*;

pub mod constants;
pub mod idle;
pub mod macros;
pub mod player_fsm_node;
pub mod run;

#[enum_dispatch]
pub trait StateBehavior {
    fn on_enter(&mut self, _player: &mut Gd<CharacterBody2D>) {}
    fn on_exit(&mut self, _player: &mut Gd<CharacterBody2D>) {}
    fn physics_update(&mut self, _player: &mut Gd<CharacterBody2D>, _delta: f64) {}
    fn handle_transitions(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _delta: f64,
    ) -> Option<State> {
        None
    }
}

#[enum_dispatch(StateBehavior)]
pub enum State {
    Idle(idle::IdleState),
    Run(run::RunState),
}

impl State {
    pub fn transition_to(&mut self, player: &mut Gd<CharacterBody2D>, mut new_state: State) {
        self.on_exit(player);
        new_state.on_enter(player);
        *self = new_state;
    }
}
