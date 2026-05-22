#[macro_export]
macro_rules! node_print {
    ($prefix:expr, $($arg:tt)*) => {
        $crate::gd_print!("{}{}", $prefix, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! node_warn {
    ($prefix:expr, $($arg:tt)*) => {
        $crate::gd_warn!("{}{}", $prefix, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! link_node {
    ($self:expr, $type:ty, $path:expr, $field:ident, $name:expr, $method:expr, $prefix:expr) => {
        if let Some(node) = $self.base().try_get_node_as::<$type>($path) {
            let mut node_to_connect = node.clone();
            node_to_connect.connect("tree_exiting", &$self.base().callable($method));
            $self.$field = Some(node);
            $crate::node_print!($prefix, "Linked to {} node", $name);
        } else {
            $crate::node_warn!($prefix, "Could not fetch {} node at {}", $name, $path);
        }
    };
}

#[macro_export]
macro_rules! on_exit_stop_process {
    ($self:expr, $field:ident, $name:expr, $prefix:expr) => {
        $crate::node_print!($prefix, "{} node exiting tree. Stopping processing.", $name);
        $self.$field = None;
        $self.base_mut().set_physics_process(false);
        $self.base_mut().set_process(false);
    };
}
