use enum_dispatch::enum_dispatch;
use godot::classes::{CharacterBody2D, InputEvent};
use godot::prelude::*;

pub mod constants;
pub mod fall;
pub mod idle;
pub mod jump;
pub mod macros;
pub mod player_fsm_node;
pub mod run;

#[enum_dispatch]
pub trait StateBehavior {
    fn on_enter(&mut self, _player: &mut Gd<CharacterBody2D>, _data: &mut PlayerData) {}
    fn on_exit(&mut self, _player: &mut Gd<CharacterBody2D>, _data: &mut PlayerData) {}
    fn physics_update(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _data: &mut PlayerData,
        _delta: f64,
    ) {
    }
    fn get_input_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _data: &mut PlayerData,
        _event: Gd<InputEvent>,
    ) -> Option<State> {
        None
    }
    fn get_poll_transition(
        &mut self,
        _player: &mut Gd<CharacterBody2D>,
        _data: &mut PlayerData,
        _delta: f64,
    ) -> Option<State> {
        None
    }
    fn get_name(&self) -> Option<String> {
        None
    }
}

pub struct PlayerData {
    pub jumps_left: i32,
}

#[enum_dispatch(StateBehavior)]
pub enum State {
    Idle(idle::IdleState),
    Run(run::RunState),
    Jump(jump::JumpState),
    Fall(fall::FallState),
}

impl State {
    pub fn transition_to(
        &mut self,
        player: &mut Gd<CharacterBody2D>,
        data: &mut PlayerData,
        mut new_state: State,
    ) {
        self.on_exit(player, data);
        new_state.on_enter(player, data);
        *self = new_state;
    }
}
