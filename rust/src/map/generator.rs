use noise;

pub struct MapGenerator {
    perlin: noise::Perlin,
}

pub const NOISE_SEED: u32 = 1337;
impl MapGenerator {
    pub fn new() -> Self {
        let perlin = noise::Perlin::new(NOISE_SEED);
        Self { perlin }
    }
    pub fn initialize(&self) {}
    pub fn update(&self, _delta: f64) {}
}
