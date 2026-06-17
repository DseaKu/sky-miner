use serde::{Deserialize, Serialize};

const PRINT_PREFIX: &str = "PlayerConfig: ";
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerrainConfig {
    pub atlas_coords: AtlasCoordsConfig,
    pub chunk_gen: ChunkGen,
    pub tile_gen: TileGen,
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct AtlasCoordsConfig {
    pub source_id: i32,
    pub dirt: (i32, i32),
    pub stone: (i32, i32),
    pub ore: (i32, i32),
    pub gem: (i32, i32),
    pub empty_cell: (i32, i32),
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ChunkGen {
    pub chunk_size: i32,
    pub render_distance: i32,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TileGen {
    pub ground_level: i32,
    pub isle: Isle,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Isle {
    pub spawn_limit: f64,
    pub stretch_x: f64,
    pub stretch_y: f64,
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
        use super::consts::*;
        Self {
            atlas_coords: AtlasCoordsConfig {
                source_id: atlas_coords::SOURCE_ID,
                dirt: atlas_coords::DIRT,
                stone: atlas_coords::STONE,
                ore: (2, 0),
                gem: (0, 1),
                empty_cell: atlas_coords::EMPTY_CELL,
            },
            chunk_gen: ChunkGen {
                chunk_size: chunk_gen::CHUNK_SIZE,
                render_distance: chunk_gen::RENDER_DISTANCE,
            },
            tile_gen: TileGen {
                ground_level: tile_gen::GROUND_LEVEL,
                isle: Isle {
                    spawn_limit: tile_gen::isle::SPAWN_LIMIT,
                    stretch_x: tile_gen::isle::STRETCH_X,
                    stretch_y: tile_gen::isle::STRETCH_Y,
                },
            },
        }
    }
}
