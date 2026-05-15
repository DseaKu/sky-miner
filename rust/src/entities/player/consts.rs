/// Horizontal Movement
pub mod h_move {
    pub mod air {
        pub const ACCEL: f32 = 1.5;
        pub const MAX_SPEED: f32 = 400.0;
        pub const FRICTION: f32 = 300.0;
    }
    pub mod ground {
        pub const FRICTION: f32 = 3000.0;
        pub const MAX_SPEED: f32 = 450.0;
        pub const ACCEL_RUN: f32 = 1.4;
        pub const ACCEL_TURN: f32 = 10.0;
        pub const LAND_DURATION: f64 = 0.2;
    }
}
/// Vertical Movement
pub mod v_move {
    pub mod jump {
        pub const MAX_SPEED: f32 = -550.0;
        pub const ACCEL: f32 = 10.0;
        pub const MAX_DURATION: f64 = 0.2;
        pub const MIN_DURATION: f64 = 0.01;
        pub const MAX_JUMPS: i32 = 2;
        pub const IMMEDIATE_TURNING_SPEED: f32 = 100.0;
    }
    pub mod gravity {
        pub const ACCEL: f32 = 2.7;
        pub const MAX_SPEED: f32 = 700.0;
    }
}
