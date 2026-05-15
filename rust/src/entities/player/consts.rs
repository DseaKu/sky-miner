/// Horizontal Movement
pub mod h_move {
    pub mod air {
        pub const ACCEL: f32 = 1.5;
        pub const MAX_SPEED: f32 = 400.0;
        pub const FRICTION: f32 = 300.0;
    }
    pub mod ground {
        pub const FRICTION: f32 = 3000.0;
        pub const MAX_SPEED: f32 = 400.0;
        pub const ACCEL_RUN: f32 = 1.5;
        pub const ACCEL_TURN: f32 = 10.0;
        pub const LAND_DURATION: f64 = 0.2;
    }
}
/// Vertical Movement
pub mod v_move {
    pub mod jump {
        pub const MAX_SPEED: f32 = -500.0;
        pub const ACCEL: f32 = 10.0;
        pub const MAX_DURATION: f64 = 0.2;
        pub const MIN_DURATION: f64 = 0.05;

        pub const MAX_JUMPS: i32 = 2;
    }
    pub mod gravity {
        pub const ACCEL: f32 = 2.3;
        pub const MAX_SPEED: f32 = 700.0;
    }
}
