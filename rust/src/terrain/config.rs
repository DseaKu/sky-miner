use serde::{Deserialize, Serialize};

const PRINT_PREFIX: &str = "TerrainConfig: ";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct TerrainConfig {
    pub atlas_coords: AtlasCoordsConfig,
    pub chunk_gen: ChunkGen,
    pub tile_gen: TileGen,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AtlasCoordsConfig {
    pub source_id: i32,
    pub dirt: (i32, i32),
    pub stone: (i32, i32),
    pub ore: (i32, i32),
    pub gem: (i32, i32),
    pub empty_cell: (i32, i32),
}

impl Default for AtlasCoordsConfig {
    fn default() -> Self {
        use super::consts::atlas_coords::*;
        Self {
            source_id: SOURCE_ID,
            dirt: DIRT,
            stone: STONE,
            ore: (2, 0),
            gem: (0, 1),
            empty_cell: EMPTY_CELL,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct ChunkGen {
    pub chunk_size: i32,
    pub render_distance: i32,
}

impl Default for ChunkGen {
    fn default() -> Self {
        use super::consts::chunk_gen::*;
        Self {
            chunk_size: CHUNK_SIZE,
            render_distance: RENDER_DISTANCE,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct TileGen {
    pub ground_level: i32,
    pub space_isle_ground: i32,
    pub height_penalty_step: f64,
    pub isle: IsleConfig,
    pub ore: OreConfig,
    pub gem: GemConfig,
    pub void: VoidConfig,
}

impl Default for TileGen {
    fn default() -> Self {
        Self {
            ground_level: 0,
            space_isle_ground: -2,
            height_penalty_step: 0.00001,
            isle: IsleConfig::default(),
            ore: OreConfig::default(),
            gem: GemConfig::default(),
            void: VoidConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct IsleConfig {
    pub spread: f64,
    pub threshold: f64,
    pub stretch_x: f64,
    pub stretch_y: f64,
}

impl Default for IsleConfig {
    fn default() -> Self {
        Self {
            spread: 0.0001,  // Decreased from 0.0013 to increase space between islands
            threshold: 0.15, // Increased from 0.15 to make islands more distinct
            stretch_x: 4.0,
            stretch_y: 40.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct OreConfig {
    pub spread: f64,
    pub init_threshold: f64,
    pub min_threshold: f64,
    pub curve_steepness: f64,
    pub dirt_threshold: f64,
}

impl Default for OreConfig {
    fn default() -> Self {
        Self {
            spread: 0.05,
            init_threshold: 1.00,
            min_threshold: 0.60,
            curve_steepness: 1.35,
            dirt_threshold: -0.3,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct GemConfig {
    pub spread: f64,
    pub init_threshold: f64,
    pub min_threshold: f64,
    pub curve_steepness: f64,
}

impl Default for GemConfig {
    fn default() -> Self {
        Self {
            spread: 0.35,
            init_threshold: 1.15,
            min_threshold: 0.50,
            curve_steepness: 1.35,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct VoidConfig {
    pub spread: f64,
    pub threshold: f64,
}

impl Default for VoidConfig {
    fn default() -> Self {
        Self {
            spread: 0.25,
            threshold: 0.23,
        }
    }
}

impl TerrainConfig {
    pub fn load() -> Self {
        use godot::classes::file_access::ModeFlags;
        use godot::classes::FileAccess;
        use godot::classes::ProjectSettings;
        use godot::obj::Singleton;

        let path = "user://terrain_config.toml";
        let absolute_path = ProjectSettings::singleton().globalize_path(path);
        let absolute_dir = ProjectSettings::singleton().globalize_path("user://");

        if FileAccess::file_exists(path) {
            let file = FileAccess::open(path, ModeFlags::READ);
            if let Some(file) = file {
                let content = file.get_as_text();
                match toml::from_str::<TerrainConfig>(&content.to_string()) {
                    Ok(config) => {
                        crate::node_print!(
                            PRINT_PREFIX,
                            "Successfully loaded from {}\n => \"{}\"",
                            path,
                            absolute_dir
                        );
                        return config;
                    }
                    Err(e) => {
                        crate::gd_error!(
                            "TerrainConfig: Failed to parse {} ({}) - Error: {}",
                            path,
                            absolute_path,
                            e
                        );
                    }
                }
            }
        }

        let default = Self::default();
        default.save();
        crate::gd_print!(
            "TerrainConfig: Created default template at {}\n => \"{}\"",
            path,
            absolute_dir
        );
        default
    }

    pub fn save(&self) {
        use godot::classes::file_access::ModeFlags;
        use godot::classes::FileAccess;
        use godot::classes::ProjectSettings;
        use godot::obj::Singleton;

        let path = "user://terrain_config.toml";

        if let Ok(content) = toml::to_string_pretty(self) {
            let file = FileAccess::open(path, ModeFlags::WRITE);
            if let Some(mut file) = file {
                file.store_string(&content);
                let absolute_dir = ProjectSettings::singleton().globalize_path("user://");
                crate::gd_print!("TerrainConfig: Saved to {}\n => \"{}\"", path, absolute_dir);
            }
        }
    }
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            atlas_coords: AtlasCoordsConfig::default(),
            chunk_gen: ChunkGen::default(),
            tile_gen: TileGen::default(),
        }
    }
}
