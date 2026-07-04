//! A between-the-lines layout engine for Rust.
//!
//! Caelum implements **Flexbox**, **Grid**, and **Block** layout algorithms
//! for building UI frameworks, browsers, and renderers. It is the layout
//! layer powering the [Aether Browser](https://github.com/SKYIOUS/aether-browser).
//!
//! # Quick Start
//!
//! ```rust
//! use aether_caelum::prelude::*;
//!
//! let mut tree = CaelumTree::<()>::new();
//! let root = tree.new_leaf(Style::default()).unwrap();
//! tree.compute_layout(root, Size {
//!     width: AvailableSpace::Definite(800.0),
//!     height: AvailableSpace::Definite(600.0),
//! }).unwrap();
//! let layout = tree.layout(root).unwrap();
//! println!("root: {}x{} at ({},{})", layout.size.width, layout.size.height, layout.location.x, layout.location.y);
//! ```

#[macro_use]
mod macros;

pub mod compute;
pub mod geometry;
pub mod prelude;
pub mod style;
pub mod style_helpers;
pub mod tree;
pub mod util;

pub use compute::*;
pub use geometry::*;
pub use style::*;
pub use tree::*;
pub use tree::traits::CacheTree;
pub use util::*;
