#[macro_export]
macro_rules! map_print {
    ($($arg:tt)*) => {
        $crate::node_print!($crate::terrain::consts::path::MAP_GEN_NODE_STR, $($arg)*);
    }
}

#[macro_export]
macro_rules! map_warn {
    ($($arg:tt)*) => {
        $crate::node_warn!($crate::terrain::consts::path::MAP_GEN_NODE_STR, $($arg)*);
    }
}

#[macro_export]
macro_rules! link_map_node {
    ($self:expr, $type:ty, $path:expr, $field:ident, $name:expr, $method:expr) => {
        $crate::link_node!($self, $type, $path, $field, $name, $method, $crate::terrain::consts::path::MAP_GEN_NODE_STR);
    };
}

#[macro_export]
macro_rules! on_map_exit_stop_process {
    ($self:expr, $field:ident, $name:expr) => {
        $crate::on_exit_stop_process!($self, $field, $name, $crate::terrain::consts::path::MAP_GEN_NODE_STR);
    };
}
