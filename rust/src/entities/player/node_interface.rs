use super::consts::{path, spawn};
use crate::entities::player::{self, StateBehavior};
use godot::classes::{CharacterBody2D, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerFsmNode {
    player_node: Option<Gd<CharacterBody2D>>,
    fsm: player::State,
    data: player::PlayerData,
    base: Base<Node>,
}

#[godot_api]
impl PlayerFsmNode {
    #[func]
    fn on_player_tree_exiting(&mut self) {
        crate::on_player_exit_stop_process!(self, player_node, "Player");
    }

    #[func]
    pub fn get_state_name(&self) -> StringName {
        match &self.fsm {
            player::State::Idle(_) => StringName::from("IDLE"),
            player::State::Run(_) => StringName::from("RUN"),
            player::State::Jump(_) => StringName::from("JUMP"),
            player::State::Fall(_) => StringName::from("FALL"),
            player::State::Land(_) => StringName::from("LAND"),
        }
    }

    #[func]
    pub fn get_jumps_left(&self) -> i32 {
        self.data.jumps_left
    }

    #[func]
    pub fn save_config(&self) {
        self.data.config.save();
    }

    #[func]
    pub fn print_config_info(&self) {
        use godot::classes::ProjectSettings;
        let user_path = ProjectSettings::singleton().globalize_path("user://");

        crate::player_print!("--- Player Configuration Info ---");
        crate::player_print!("Config Path: {}player_config.json", user_path);
        crate::player_print!("Current Settings:");
        crate::player_print!(
            "  - Ground Speed: {}",
            self.data.config.h_move.ground.max_speed
        );
        crate::player_print!(
            "  - Air Speed:    {}",
            self.data.config.h_move.air.max_speed
        );
        crate::player_print!(
            "  - Jump Force:   {}",
            self.data.config.h_move.air.max_speed
        );
        crate::player_print!(
            "  - Jumps Max:    {}",
            self.data.config.v_move.jump.max_jumps
        );
        crate::player_print!("---------------------------------");
    }
}

#[godot_api]
impl INode for PlayerFsmNode {
    fn init(base: Base<Node>) -> Self {
        let fsm = player::State::Idle(player::idle::IdleState);
        let config = player::config::PlayerConfig::load();
        let data = player::PlayerData {
            jumps_left: config.v_move.jump.max_jumps,
            config,
        };

        Self {
            player_node: None,
            fsm,
            data,
            base,
        }
    }

    fn ready(&mut self) {
        use spawn::position as pos;
        crate::player_print!("Initializing...");
        crate::link_player_node!(
            self,
            CharacterBody2D,
            path::PARENT_NODE_PATH,
            player_node,
            "Player",
            "on_player_tree_exiting"
        );

        if let Some(player) = self.player_node.as_mut() {
            let mut ctx = player::PlayerContext::new(player, &mut self.data);
            self.fsm.on_enter(&mut ctx);
            player.set_position(Vector2::new(pos::X, pos::Y));
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(player) = self.player_node.as_mut() {
            let mut ctx = player::PlayerContext::new(player, &mut self.data);
            if let Some(next_state) = self.fsm.get_input_transition(&mut ctx, event) {
                self.fsm.transition_to(&mut ctx, next_state);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(player) = self.player_node.as_mut() {
            let mut ctx = player::PlayerContext::new(player, &mut self.data);
            self.fsm.physics_update(&mut ctx, delta);
            if let Some(next_state) = self.fsm.get_poll_transition(&mut ctx, delta) {
                self.fsm.transition_to(&mut ctx, next_state);
            }
        }
    }
}
