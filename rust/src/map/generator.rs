use super::consts;
use godot::prelude::*;
use noise;
use rand::{self, Rng};

pub struct MapGenerator {
    perlin: noise::Perlin,
}

impl MapGenerator {
    pub fn new() -> Self {
        godot_print!("Initialize MapGenerator Module");
        let mut rng = rand::thread_rng();
        let rnd_num: u32 = rng.gen();
        godot_print!("Seed: {}", rnd_num);
        let perlin = noise::Perlin::new(rnd_num);
        Self { perlin }
    }
    pub fn initialize(&self) {}
    pub fn update(&self, _delta: f64) {}
}
