//! Helpful misc. utilities such as a function to debug print a tree
mod math;
mod resolve;
pub(crate) mod sys;

pub use math::MaybeMath;
pub use resolve::{MaybeResolve, ResolveOrZero};

#[doc(hidden)]
#[macro_use]
pub(crate) mod debug;

mod print;
pub use print::print_tree;
pub use print::write_tree;

