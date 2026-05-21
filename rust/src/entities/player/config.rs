use serde::{Deserialize, Serialize};

/// Main configuration container for all player physics and movement settings.
/// Serialize/Deserialize allows converting this entire tree to/from TOML.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerConfig {
    pub h_move: HMoveConfig,
    pub v_move: VMoveConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HMoveConfig {
    pub air: AirConfig,
    pub ground: GroundConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AirConfig {
    pub accel: f32,
    pub max_speed: f32,
    pub friction: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroundConfig {
    pub friction: f32,
    pub max_speed: f32,
    pub accel_run: f32,
    pub accel_turn: f32,
    pub land_duration: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VMoveConfig {
    pub jump: JumpConfig,
    pub gravity: GravityConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JumpConfig {
    pub max_speed: f32,
    pub accel: f32,
    pub max_duration: f64,
    pub min_duration: f64,
    pub max_jumps: i32,
    pub immediate_turning_speed: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GravityConfig {
    pub accel: f32,
    pub max_speed: f32,
}

impl PlayerConfig {
    /// Attempts to load config from 'user://player_config.toml'.
    /// Returns default constants if the file is missing or corrupted.
    pub fn load() -> Self {
        use godot::classes::file_access::ModeFlags;
        use godot::classes::FileAccess;
        use godot::classes::ProjectSettings;
        use godot::obj::Singleton;

        let path = "user://player_config.toml";
        let absolute_path = ProjectSettings::singleton().globalize_path(path);
        let absolute_dir = ProjectSettings::singleton().globalize_path("user://");

        if FileAccess::file_exists(path) {
            let file = FileAccess::open(path, ModeFlags::READ);
            if let Some(file) = file {
                let content = file.get_as_text();
                match toml::from_str::<PlayerConfig>(&content.to_string()) {
                    Ok(config) => {
                        crate::gd_print!(
                            "PlayerConfig: Successfully loaded from {}\n => \"{}\"",
                            path,
                            absolute_dir
                        );
                        return config;
                    }
                    Err(e) => {
                        crate::gd_error!(
                            "PlayerConfig: Failed to parse {} ({}) - Error: {}",
                            path,
                            absolute_path,
                            e
                        );
                    }
                }
            }
        }

        // If file doesn't exist, create it with defaults so the user has a template to edit
        let default = Self::default();
        default.save();
        crate::gd_print!(
            "PlayerConfig: Created default template at {}\n => \"{}\"",
            path,
            absolute_dir
        );
        default
    }

    /// Saves the current configuration as a pretty-printed TOML file.
    pub fn save(&self) {
        use godot::classes::file_access::ModeFlags;
        use godot::classes::FileAccess;
        use godot::classes::ProjectSettings;
        use godot::obj::Singleton;

        let path = "user://player_config.toml";

        // Convert Rust structs into a human-readable TOML string
        if let Ok(content) = toml::to_string_pretty(self) {
            let file = FileAccess::open(path, ModeFlags::WRITE);
            if let Some(mut file) = file {
                file.store_string(&content);
                let absolute_dir = ProjectSettings::singleton().globalize_path("user://");
                crate::gd_print!("PlayerConfig: Saved to {}\n => \"{}\"", path, absolute_dir);
            }
        }
    }
}

impl Default for PlayerConfig {
    /// Provides the baseline values from consts.rs.
    fn default() -> Self {
        use super::consts::*;
        Self {
            h_move: HMoveConfig {
                air: AirConfig {
                    accel: h_move::air::ACCEL,
                    max_speed: h_move::air::MAX_SPEED,
                    friction: h_move::air::FRICTION,
                },
                ground: GroundConfig {
                    friction: h_move::ground::FRICTION,
                    max_speed: h_move::ground::MAX_SPEED,
                    accel_run: h_move::ground::ACCEL_RUN,
                    accel_turn: h_move::ground::ACCEL_TURN,
                    land_duration: h_move::ground::LAND_DURATION,
                },
            },
            v_move: VMoveConfig {
                jump: JumpConfig {
                    max_speed: v_move::jump::MAX_SPEED,
                    accel: v_move::jump::ACCEL,
                    max_duration: v_move::jump::MAX_DURATION,
                    min_duration: v_move::jump::MIN_DURATION,
                    max_jumps: v_move::jump::MAX_JUMPS,
                    immediate_turning_speed: v_move::jump::IMMEDIATE_TURNING_SPEED,
                },
                gravity: GravityConfig {
                    accel: v_move::gravity::ACCEL,
                    max_speed: v_move::gravity::MAX_SPEED,
                },
            },
        }
    }
}
