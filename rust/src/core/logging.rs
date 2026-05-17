#[macro_export]
macro_rules! gd_print {
    ($($arg:tt)*) => {
        godot::prelude::godot_print!("[{}] {}", env!("CARGO_PKG_NAME"), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! gd_warn {
    ($($arg:tt)*) => {
        godot::prelude::godot_warn!("[{}] {}", env!("CARGO_PKG_NAME"), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! gd_error {
    ($($arg:tt)*) => {
        godot::prelude::godot_error!("[{}] {}", env!("CARGO_PKG_NAME"), format_args!($($arg)*));
    }
}
