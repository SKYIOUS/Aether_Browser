//! Contains both a high-level interface to Caelum using a ready-made node tree, and a set of traits for defining custom node trees.
//!
//! - For documentation on the high-level API, see the [`CaelumTree`] struct.
//! - For documentation on the low-level trait-based API, see the [`traits`] module.

// Submodules
mod cache;
mod layout;
mod node;
pub mod traits;

pub use cache::{Cache, ClearState};
pub use layout::{CollapsibleMarginSet, Layout, LayoutInput, LayoutOutput, RequestedAxis, RunMode, SizingMode};
pub use node::NodeId;
pub(crate) use traits::LayoutPartialTreeExt;
pub use traits::{LayoutPartialTree, PrintTree, RoundTree, TraversePartialTree, TraverseTree};

pub use traits::LayoutFlexboxContainer;

pub use traits::LayoutGridContainer;

pub use traits::LayoutBlockContainer;

mod Caelum_tree;
pub use Caelum_tree::{CaelumError, CaelumResult, CaelumTree};

pub use layout::DetailedLayoutInfo;
