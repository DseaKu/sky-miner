#[macro_export]
macro_rules! player_print {
    ($($arg:tt)*) => {
        $crate::gd_print!("{}{}", $crate::entities::player::consts::path::PLAYER_FSM_NODE_STR, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! player_warn {
    ($($arg:tt)*) => {
        $crate::gd_warn!("{}{}", $crate::entities::player::consts::path::PLAYER_FSM_NODE_STR, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! link_player_node {
    ($self:expr, $type:ty, $path:expr, $field:ident, $name:expr, $method:expr) => {
        if let Some(node) = $self.base().try_get_node_as::<$type>($path) {
            let mut node_to_connect = node.clone();
            node_to_connect.connect("tree_exiting", &$self.base().callable($method));
            $self.$field = Some(node);
            $crate::player_print!("Linked to {} node", $name);
        } else {
            $crate::player_warn!("Could not fetch {} node at {}", $name, $path);
        }
    };
}

#[macro_export]
macro_rules! on_player_exit_stop_process {
    ($self:expr, $field:ident, $name:expr) => {
        $crate::player_print!("{} node exiting tree. Stopping processing.", $name);
        $self.$field = None;
        $self.base_mut().set_physics_process(false);
        $self.base_mut().set_process(false);
    };
}
