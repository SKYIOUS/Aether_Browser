#[cfg(feature = "debug_layout")]
macro_rules! debug_log {
    ($label:expr, dbg:$expr:expr $(,)?) => {
        eprintln!("{}: {} = {:?}", $label, stringify!($expr), $expr)
    };
    (dbg:$expr:expr $(,)?) => {
        eprintln!("{} = {:?}", stringify!($expr), $expr)
    };
    ($lit:literal $(,)?) => {
        eprintln!($lit)
    };
    ($expr:expr $(,)?) => {
        eprintln!("{:?}", $expr)
    };
    ($label:expr, $expr:expr $(,)?) => {
        eprintln!("{}: {:?}", $label, $expr)
    };
}
#[cfg(not(feature = "debug_layout"))]
macro_rules! debug_log { ($($t:tt)*) => {}; }

#[cfg(feature = "debug_layout")]
macro_rules! debug_log_node {
    ($expr:expr $(,)?) => {
        eprintln!("  node inputs: {:?}", $expr)
    };
}
#[cfg(not(feature = "debug_layout"))]
macro_rules! debug_log_node { ($($t:tt)*) => {}; }

#[cfg(feature = "debug_layout")]
macro_rules! debug_push_node {
    ($node:expr) => { eprintln!("  push: {:?}", $node) };
}
#[cfg(not(feature = "debug_layout"))]
macro_rules! debug_push_node { ($node:expr) => {}; }

#[cfg(feature = "debug_layout")]
macro_rules! debug_pop_node {
    () => { eprintln!("  pop") };
}
#[cfg(not(feature = "debug_layout"))]
macro_rules! debug_pop_node { () => {}; }

macro_rules! impl_parse_for_keyword_enum {
    ($e:ident, $($css:literal => $variant:ident),+ $(,)?) => {
        impl $e {
            #[allow(dead_code)]
            pub fn from_css_str(s: &str) -> Option<Self> {
                match s {
                    $($css => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}
