// Movement
// Ground
pub mod ground {
    pub const FRICTION: f32 = 3000.0;
    pub const MAX_SPEED: f32 = 400.0;
    pub const ACCEL: f32 = 1.5;
    pub const ACCEL_TURN: f32 = 10.0;
}
// In Air
pub mod in_air {
    pub const ACCEL: f32 = 3.3;
    pub const MAX_SPEED: f32 = 700.0;
}
// Jump
pub mod jump {
    pub const MAX_SPEED: f32 = -400.0;
    pub const ACCEL: f32 = 10.0;
    pub const MAX_DURATION: f64 = 0.2;
}
