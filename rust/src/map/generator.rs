use godot::prelude::*;
use rand::{self, Rng};

pub struct MapGenerator {
    perlin: noise::Perlin,
}

impl MapGenerator {
    pub fn new() -> Self {
        // Setup Perlin
        let mut rng = rand::thread_rng();
        let rnd_num: u32 = rng.gen();
        crate::gd_print!("MapGenerator: Initialized with seed {}", rnd_num);
        let perlin = noise::Perlin::new(rnd_num);

        Self { perlin }
    }
    pub fn update(&self, _delta: f64, _grid_pos: Vector2i) {}
}
