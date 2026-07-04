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

#[allow(unused_macros)]
macro_rules! impl_parse_for_keyword_enum {
    ($e:ident, $($rest:tt)*) => {};
}
