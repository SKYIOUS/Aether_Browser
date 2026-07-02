#[allow(unused_macros)]
macro_rules! impl_parse_for_keyword_enum {
    ($e:ident, $($rest:tt)*) => {};
}

macro_rules! debug_log {
    ($($t:tt)*) => {};
}

macro_rules! debug_log_node {
    ($($t:tt)*) => {};
}

macro_rules! debug_push_node {
    ($node:expr) => {};
}

macro_rules! debug_pop_node {
    () => {};
}

pub mod compute;
pub mod geometry;
pub mod prelude;
pub mod style;
pub mod style_helpers;
pub mod tree;
pub mod util;

pub use compute::compute_block_layout;
pub use compute::compute_flexbox_layout;
pub use compute::compute_grid_layout;
pub use compute::detailed_info::*;
pub use compute::{
    compute_cached_layout, compute_hidden_layout, compute_leaf_layout, compute_root_layout, round_layout,
};
pub use style::Style;
pub use tree::traits::*;
pub use tree::CaelumTree;
pub use util::print_tree;

pub use compute::*;
pub use geometry::*;
pub use style::*;
pub use tree::*;
pub use util::*;
