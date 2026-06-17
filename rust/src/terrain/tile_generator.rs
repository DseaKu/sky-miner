use crate::terrain::config;
use crate::terrain::TileType;
use noise::{NoiseFn, OpenSimplex};
use rand::{self, RngExt};

const PRINT_PREFIX: &str = "TileGenerator: ";

pub struct TileGenerator {
    island_noise: OpenSimplex,
    ore_noise: OpenSimplex,
    gem_noise: OpenSimplex,
    void_noise: OpenSimplex,
    config: config::TileGen,

    // Dynamic parameters
    height_penalty: f64,
    ore_threshold: f64,
    gem_threshold: f64,
}

impl TileGenerator {
    pub fn new(config: config::TileGen) -> Self {
        let mut rng = rand::rng();
        let seed_island: u32 = rng.random();
        let seed_ore: u32 = rng.random();
        let seed_gem: u32 = rng.random();
        let seed_void: u32 = rng.random();

        crate::node_print!(
            PRINT_PREFIX,
            "Initialized with seeds: Island={}, Ore={}, Gem={}, Void={}",
            seed_island,
            seed_ore,
            seed_gem,
            seed_void
        );

        Self {
            island_noise: OpenSimplex::new(seed_island),
            ore_noise: OpenSimplex::new(seed_ore),
            gem_noise: OpenSimplex::new(seed_gem),
            void_noise: OpenSimplex::new(seed_void),
            config,
            height_penalty: 0.0,
            ore_threshold: 0.0,
            gem_threshold: 0.0,
        }
    }

    pub fn update_dynamic_params(&mut self, player_y: f64) {
        let delta = player_y * self.config.height_penalty_step;
        self.height_penalty = self.move_toward_gd(self.config.isle.threshold, 0.0, delta);

        let drop_ore = ((self.height_penalty * self.config.ore.curve_steepness) + 1.0).ln();
        self.ore_threshold = (self.config.ore.init_threshold - drop_ore)
            .clamp(self.config.ore.min_threshold, self.config.ore.init_threshold);

        let drop_gem = ((self.height_penalty * self.config.gem.curve_steepness) + 1.0).ln();
        self.gem_threshold = (self.config.gem.init_threshold - drop_gem)
            .clamp(self.config.gem.min_threshold, self.config.gem.init_threshold);
    }

    fn move_toward_gd(&self, current: f64, target: f64, delta: f64) -> f64 {
        if delta >= 0.0 {
            if (target - current).abs() <= delta {
                target
            } else {
                current + (target - current).signum() * delta
            }
        } else {
            current - (target - current).signum() * delta.abs()
        }
    }

    pub fn generate_tile(&self, x: i32, y: i32) -> TileType {
        // Layer logic
        if y > self.config.ground_level {
            return TileType::Stone;
        }

        if y > self.config.space_isle_ground {
            return TileType::Void;
        }

        // Isle Generation
        let ix = (x as f64) * self.config.isle.stretch_x * self.config.isle.spread;
        let iy = (y as f64) * self.config.isle.stretch_y * self.config.isle.spread;
        let island_val = self.island_noise.get([ix, iy]);

        if island_val < self.height_penalty {
            return TileType::Sky;
        }

        // Inside Island: Apply voids, ore, gems
        let vx = x as f64 * self.config.void.spread;
        let vy = y as f64 * self.config.void.spread;
        let void_val = self.void_noise.get([vx, vy]);

        if void_val > self.config.void.threshold {
            return TileType::Void;
        }

        let ox = x as f64 * self.config.ore.spread;
        let oy = y as f64 * self.config.ore.spread;
        let ore_val = self.ore_noise.get([ox, oy]);

        if ore_val < self.config.ore.dirt_threshold {
            return TileType::Dirt;
        }

        if ore_val > self.ore_threshold {
            return TileType::Ore;
        }

        let gx = x as f64 * self.config.gem.spread;
        let gy = y as f64 * self.config.gem.spread;
        let gem_val = self.gem_noise.get([gx, gy]);

        if gem_val > self.gem_threshold {
            return TileType::Gem;
        }

        TileType::Stone
    }
}
