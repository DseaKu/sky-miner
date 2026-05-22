#[macro_export]
macro_rules! player_print {
    ($($arg:tt)*) => {
        $crate::node_print!($crate::entities::player::consts::path::PLAYER_FSM_NODE_STR, $($arg)*);
    }
}

#[macro_export]
macro_rules! player_warn {
    ($($arg:tt)*) => {
        $crate::node_warn!($crate::entities::player::consts::path::PLAYER_FSM_NODE_STR, $($arg)*);
    }
}

#[macro_export]
macro_rules! link_player_node {
    ($self:expr, $type:ty, $path:expr, $field:ident, $name:expr, $method:expr) => {
        $crate::link_node!($self, $type, $path, $field, $name, $method, $crate::entities::player::consts::path::PLAYER_FSM_NODE_STR);
    };
}

#[macro_export]
macro_rules! on_player_exit_stop_process {
    ($self:expr, $field:ident, $name:expr) => {
        $crate::on_exit_stop_process!($self, $field, $name, $crate::entities::player::consts::path::PLAYER_FSM_NODE_STR);
    };
}
