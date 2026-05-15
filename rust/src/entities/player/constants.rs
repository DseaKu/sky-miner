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
    use crate::entities::player::constants::ground;

    pub const ACCEL_GRAVITY: f32 = 0.3;
    pub const MAX_SPEED_GRAVITY: f32 = 700.0;

    pub const ACCEL_X: f32 = ground::ACCEL;
    pub const MAX_SPEED_X: f32 = ground::MAX_SPEED;
    pub const FRICTION: f32 = 300.0;
    pub const MAX_N_JUMP: i32 = 8;
}
// Jump
pub mod jump {
    pub const MAX_SPEED: f32 = -500.0;
    pub const ACCEL: f32 = 10.0;
    pub const MAX_DURATION: f64 = 0.2;
}
