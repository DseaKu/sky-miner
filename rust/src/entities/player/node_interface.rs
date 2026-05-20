use crate::entities::player::{self, StateBehavior};
use godot::classes::{CharacterBody2D, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerFsmNode {
    fsm: player::State,
    data: player::PlayerData,
    base: Base<Node>,
}

#[godot_api]
impl PlayerFsmNode {
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

        crate::gd_print!("--- Player Configuration Info ---");
        crate::gd_print!("Config Path: {}player_config.json", user_path);
        crate::gd_print!("Current Settings:");
        crate::gd_print!(
            "  - Ground Speed: {}",
            self.data.config.h_move.ground.max_speed
        );
        crate::gd_print!(
            "  - Air Speed:    {}",
            self.data.config.h_move.air.max_speed
        );
        crate::gd_print!(
            "  - Jump Force:   {}",
            self.data.config.h_move.air.max_speed
        );
        crate::gd_print!(
            "  - Jumps Max:    {}",
            self.data.config.v_move.jump.max_jumps
        );
        crate::gd_print!("---------------------------------");
    }
}

#[godot_api]
impl INode for PlayerFsmNode {
    fn init(base: Base<Node>) -> Self {
        crate::gd_print!("PlayerFsmNode: Initializing...");
        let fsm = player::State::Idle(player::idle::IdleState);
        let config = player::config::PlayerConfig::load();
        let data = player::PlayerData {
            jumps_left: config.v_move.jump.max_jumps,
            config,
        };

        Self { fsm, data, base }
    }

    fn ready(&mut self) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            let mut ctx = player::PlayerContext::new(&mut player, &mut self.data);
            self.fsm.on_enter(&mut ctx);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            let mut ctx = player::PlayerContext::new(&mut player, &mut self.data);
            if let Some(next_state) = self.fsm.get_input_transition(&mut ctx, event) {
                self.fsm.transition_to(&mut ctx, next_state);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let parent = self.base().get_parent();
        if let Some(mut player) = parent.and_then(|p| p.try_cast::<CharacterBody2D>().ok()) {
            let mut ctx = player::PlayerContext::new(&mut player, &mut self.data);
            self.fsm.physics_update(&mut ctx, delta);
            if let Some(next_state) = self.fsm.get_poll_transition(&mut ctx, delta) {
                self.fsm.transition_to(&mut ctx, next_state);
            }
        }
    }
}
