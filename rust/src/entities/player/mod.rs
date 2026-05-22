use enum_dispatch::enum_dispatch;
use godot::classes::{CharacterBody2D, InputEvent};
use godot::prelude::*;

pub mod config;
pub mod consts;
pub mod fall;
pub mod idle;
pub mod jump;
pub mod land;
pub mod node_interface;
pub mod run;

#[enum_dispatch]
pub trait StateBehavior {
    fn on_enter(&mut self, _ctx: &mut PlayerContext) {}
    fn on_exit(&mut self, _ctx: &mut PlayerContext) {}
    fn physics_update(&mut self, _ctx: &mut PlayerContext, _delta: f64) {}
    fn get_input_transition(
        &mut self,
        _ctx: &mut PlayerContext,
        _event: Gd<InputEvent>,
    ) -> Option<State> {
        None
    }
    fn get_poll_transition(&mut self, _ctx: &mut PlayerContext, _delta: f64) -> Option<State> {
        None
    }
}

pub struct PlayerData {
    pub jumps_left: i32,
    pub config: config::PlayerConfig,
}

pub struct PlayerContext<'a> {
    pub player: &'a mut Gd<CharacterBody2D>,
    pub data: &'a mut PlayerData,
}

impl<'a> PlayerContext<'a> {
    pub fn new(player: &'a mut Gd<CharacterBody2D>, data: &'a mut PlayerData) -> Self {
        Self { player, data }
    }

    pub fn play_animation(&mut self, anim: &str) {
        if let Some(mut anim_node) = self.player.get_node_or_null("AnimationPlayer") {
            anim_node.call("play", &[Variant::from(anim)]);
        }
    }

    pub fn apply_gravity(&mut self, delta: f64) {
        let mut velocity = self.player.get_velocity();
        let grav = &self.data.config.v_move.gravity;
        velocity.y = crate::core::utils::FloatExt::lerp(
            velocity.y,
            grav.max_speed,
            grav.accel * delta as f32,
        );
        self.player.set_velocity(velocity);
    }

    fn get_input_axis(&self) -> f32 {
        godot::classes::Input::singleton().get_axis("left", "right")
    }

    pub fn handle_h_move(&mut self, delta: f64, is_air: bool) {
        let direction = self.get_input_axis();
        let mut velocity = self.player.get_velocity();

        if let Some(mut sprite) = self.player.get_node_or_null("Sprite2D") {
            if direction > 0.0 {
                sprite.set("flip_h", &Variant::from(false));
            } else if direction < 0.0 {
                sprite.set("flip_h", &Variant::from(true));
            }
        }

        if is_air {
            let air = &self.data.config.h_move.air;
            if direction != 0.0 {
                velocity.x = crate::core::utils::FloatExt::lerp(
                    velocity.x,
                    air.max_speed * direction,
                    air.accel * delta as f32,
                );
            } else {
                velocity.x = crate::core::utils::FloatExt::move_toward(
                    velocity.x,
                    0.0,
                    air.friction * delta as f32,
                );
            }
        } else {
            let gnd = &self.data.config.h_move.ground;
            if direction != 0.0 {
                let mut accel = gnd.accel_run;
                if direction.signum() != velocity.x.signum() && velocity.x != 0.0_f32 {
                    accel = gnd.accel_turn;
                }
                velocity.x = crate::core::utils::FloatExt::lerp(
                    velocity.x,
                    direction * gnd.max_speed,
                    accel * delta as f32,
                );
            } else {
                velocity.x = crate::core::utils::FloatExt::move_toward(
                    velocity.x,
                    0.0,
                    gnd.friction * delta as f32,
                );
            }
        }

        self.player.set_velocity(velocity);
    }

    pub fn move_and_slide(&mut self) {
        self.player.move_and_slide();
    }
}

#[enum_dispatch(StateBehavior)]
pub enum State {
    Idle(idle::IdleState),
    Run(run::RunState),
    Jump(jump::JumpState),
    Fall(fall::FallState),
    Land(land::LandState),
}

impl State {
    pub fn transition_to(&mut self, ctx: &mut PlayerContext, mut new_state: State) {
        self.on_exit(ctx);
        new_state.on_enter(ctx);
        *self = new_state;
    }
}
