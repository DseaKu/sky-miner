#[macro_export]
macro_rules! map_print {
    ($($arg:tt)*) => {
        $crate::gd_print!("{}{}", $crate::terrain::consts::path::MAP_GEN_NODE_STR, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! map_warn {
    ($($arg:tt)*) => {
        $crate::gd_warn!("{}{}", $crate::terrain::consts::path::MAP_GEN_NODE_STR, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! link_node {
    ($self:expr, $type:ty, $path:expr, $field:ident, $name:expr, $method:expr) => {
        if let Some(node) = $self.base().try_get_node_as::<$type>($path) {
            let mut node_to_connect = node.clone();
            node_to_connect.connect("tree_exiting", &$self.base().callable($method));
            $self.$field = Some(node);
            $crate::map_print!("Linked to {} node", $name);
        } else {
            $crate::map_warn!("Could not fetch {} node at {}", $name, $path);
        }
    };
}

#[macro_export]
macro_rules! on_exit_stop_process {
    ($self:expr, $field:ident, $name:expr) => {
        $crate::map_print!("{} node exiting tree. Stopping processing.", $name);
        $self.$field = None;
        $self.base_mut().set_process(false);
    };
}
