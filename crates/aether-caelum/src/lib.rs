//! Aether Caelum — CSS layout engine.
//!
//! Implements **Flexbox** (CSS Flexbox Level 1), **Grid** (CSS Grid Level 1),
//! and **Block** layout algorithms. Used as the layout layer in the
//! [Aether Browser](https://github.com/SKYIOUS/aether-browser).
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
//!
//! # Feature Flags
//!
//! - `content_size` (default): tracks content overflow bounds per node in `LayoutOutput`.
//! - `debug_layout`: enables `eprintln!`-based debug logging for layout computation.
//!
//! # Known Limitations
//!
//! - **Safe alignment** not implemented — content may overflow with non-start alignment values in flexbox/grid.
//! - **Baseline alignment** partially implemented; last baseline and vertical text baselines not handled.
//! - **Float layout** has known edge cases: second float at same Y pushes subsequent content down.
//! - **Vertical writing modes** not supported.
//! - **`visibility: collapse`** not implemented.
//! - **Auto margins for absolute-positioned root** not supported.
//! - **Scrollbar gutter** side always right/bottom regardless of `direction`.
//! - **Grid track sizing** re-runs all tracks instead of only affected ones (performance).
//! - Table layout not implemented; `display: inline` treated as `display: block`.
//!
//! For CSS string parsing, use [`Style::from_css`] or construct [`Style`] fields directly.

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

#[cfg(test)]
mod integration_tests {
    use crate::prelude::*;
    use crate::geometry::Point;
    use crate::style::{Dimension, FlexWrap, Overflow};

    fn make_avail(w: f32, h: f32) -> Size<AvailableSpace> {
        Size { width: AvailableSpace::Definite(w), height: AvailableSpace::Definite(h) }
    }

    fn ctree() -> CaelumTree<()> { CaelumTree::new() }

    // --- Flexbox edge cases ---

    #[test]
    fn flexbox_items_larger_than_container() {
        let mut t = ctree();
        let a = t.new_leaf(Style { size: Size::from_lengths(200.0, 50.0), ..Default::default() }).unwrap();
        let b = t.new_leaf(Style { size: Size::from_lengths(50.0, 200.0), ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(100.0, 100.0), ..Default::default() }, &[a, b],
            ).unwrap();
        t.compute_layout(container, make_avail(100.0, 100.0)).unwrap();
        let layout = t.layout(container).unwrap();
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn flexbox_zero_size_items() {
        let mut t = ctree();
        let a = t.new_leaf(Style { size: Size::from_lengths(0.0, 0.0), ..Default::default() }).unwrap();
        let b = t.new_leaf(Style { size: Size::from_lengths(0.0, 0.0), ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, ..Default::default() }, &[a, b],
            ).unwrap();
        t.compute_layout(container, make_avail(500.0, 500.0)).unwrap();
        let layout = t.layout(container).unwrap();
        assert!(layout.size.width >= 0.0);
        assert!(layout.size.height >= 0.0);
    }

    #[test]
    fn flexbox_nested_containers() {
        let mut t = ctree();
        let child = t.new_leaf(Style { size: Size::from_lengths(50.0, 50.0), ..Default::default() }).unwrap();
        let inner = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(100.0, 100.0), ..Default::default() }, &[child],
            ).unwrap();
        let outer = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(300.0, 300.0), ..Default::default() }, &[inner],
            ).unwrap();
        t.compute_layout(outer, make_avail(300.0, 300.0)).unwrap();
        let inner_layout = t.layout(inner).unwrap();
        assert_eq!(inner_layout.size.width, 100.0);
        assert_eq!(inner_layout.size.height, 100.0);
    }

    #[test]
    fn flexbox_wrap_with_min_max() {
        let mut t = ctree();
        let ch = Style {
            size: Size::from_lengths(80.0, 50.0),
            min_size: Size::from_lengths(30.0, 30.0),
            max_size: Size::from_lengths(120.0, 100.0),
            ..Default::default()
        };
        let a = t.new_leaf(ch.clone()).unwrap();
        let b = t.new_leaf(ch.clone()).unwrap();
        let c = t.new_leaf(ch).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, flex_wrap: FlexWrap::Wrap,
                    size: Size::from_lengths(150.0, 200.0), ..Default::default() }, &[a, b, c],
            ).unwrap();
        t.compute_layout(container, make_avail(150.0, 200.0)).unwrap();
        let layout = t.layout(container).unwrap();
        assert_eq!(layout.size.width, 150.0);
    }

    // --- Grid tests ---

    #[test]
    fn grid_auto_placement() {
        let mut t = ctree();
        let items: Vec<_> = (0..4)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(50.0, 50.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Grid, size: Size::from_lengths(200.0, 200.0),
                    grid_template_columns: vec![fr(1.0), fr(1.0)],
                    grid_template_rows: vec![fr(1.0), fr(1.0)], ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        for item in &items {
            let layout = t.layout(*item).unwrap();
            assert!(layout.size.width > 0.0);
            assert!(layout.size.height > 0.0);
        }
    }

    #[test]
    fn grid_item_spanning_multiple_tracks() {
        let mut t = ctree();
        let wide = t.new_leaf(Style {
                grid_column: Line { start: GridPlacement::from_line_index(1),
                    end: GridPlacement::from_line_index(3) },
                ..Default::default()
            }).unwrap();
        let small = t.new_leaf(Style { ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Grid, size: Size::from_lengths(200.0, 200.0),
                    grid_template_columns: vec![fr(1.0), fr(1.0)],
                    grid_template_rows: vec![fr(1.0), fr(1.0)], ..Default::default() }, &[wide, small],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        let wide_layout = t.layout(wide).unwrap();
        assert_eq!(wide_layout.size.width, 200.0);
    }

    // --- Float + Block ---

    #[test]
    fn block_float_left() {
        let mut t = ctree();
        let float = t.new_leaf(Style { size: Size::from_lengths(80.0, 100.0),
                float: crate::Float::Left, ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(300.0, 200.0), ..Default::default() },
                &[float],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 200.0)).unwrap();
        let float_layout = t.layout(float).unwrap();
        assert_eq!(float_layout.location.x, 0.0);
    }

    #[test]
    fn block_clear_both() {
        let mut t = ctree();
        let left = t.new_leaf(Style { size: Size::from_lengths(80.0, 100.0),
                float: crate::Float::Left, ..Default::default() }).unwrap();
        let right = t.new_leaf(Style { size: Size::from_lengths(80.0, 50.0),
                float: crate::Float::Right, clear: crate::Clear::Both, ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(300.0, 300.0), ..Default::default() },
                &[left, right],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 300.0)).unwrap();
        let right_layout = t.layout(right).unwrap();
        assert!(right_layout.location.y >= 0.0);
    }

    // --- Edge cases ---

    #[test]
    fn zero_height_container() {
        let mut t = ctree();
        let child = t.new_leaf(Style { size: Size::from_lengths(50.0, 50.0), ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(100.0, 0.0), ..Default::default() }, &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(100.0, 0.0)).unwrap();
        let layout = t.layout(container).unwrap();
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn deeply_nested_layout() {
        let mut t = ctree();
        let mut parent = t.new_leaf(Style { size: Size::from_lengths(10.0, 10.0), ..Default::default() }).unwrap();
        for _ in 0..50 {
            let child = t.new_leaf(Style { size: Size::from_lengths(10.0, 10.0), ..Default::default() }).unwrap();
            let new_parent = t.new_with_children(
                    Style { display: Display::Flex, size: Size::from_lengths(50.0, 50.0), ..Default::default() },
                    &[parent, child],
                ).unwrap();
            parent = new_parent;
        }
        t.compute_layout(parent, make_avail(500.0, 500.0)).unwrap();
        let layout = t.layout(parent).unwrap();
        assert!(layout.size.width > 0.0);
        assert!(layout.size.height > 0.0);
    }

    #[test]
    fn aspect_ratio_sizing() {
        let mut t = ctree();
        let node = t.new_leaf(Style {
                size: Size { width: Dimension::from_length(200.0), height: Dimension::auto() },
                aspect_ratio: Some(2.0), ..Default::default()
            }).unwrap();
        t.compute_layout(node, make_avail(500.0, 500.0)).unwrap();
        let layout = t.layout(node).unwrap();
        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn overflow_hidden() {
        let mut t = ctree();
        let inner = t.new_leaf(Style { size: Size::from_lengths(200.0, 200.0), ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(100.0, 100.0),
                    overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden }, ..Default::default() }, &[inner],
            ).unwrap();
        t.compute_layout(container, make_avail(100.0, 100.0)).unwrap();
        let layout = t.layout(container).unwrap();
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    // --- from_css parser tests ---

    #[test]
    fn from_css_width_height() {
        let s = Style::<String>::from_css("width", "100px").unwrap();
        assert_eq!(s.size.width.into_option(), Some(100.0));
        let s = Style::<String>::from_css("height", "50%").unwrap();
        assert_eq!(s.size.height.tag(), Dimension::percent(0.5).tag());
        let s = Style::<String>::from_css("width", "auto").unwrap();
        assert!(s.size.width.is_auto());
    }

    #[test]
    fn from_css_display() {
        let s = Style::<String>::from_css("display", "flex").unwrap();
        assert_eq!(s.display, Display::Flex);
        let s = Style::<String>::from_css("display", "none").unwrap();
        assert_eq!(s.display, Display::None);
        assert!(Style::<String>::from_css("display", "bogus").is_err());
    }

    #[test]
    fn from_css_position() {
        let s = Style::<String>::from_css("position", "absolute").unwrap();
        assert_eq!(s.position, Position::Absolute);
    }

    #[test]
    fn from_css_margin_padding() {
        let s = Style::<String>::from_css("margin", "10px").unwrap();
        assert_eq!(s.margin.left.into_raw().value(), 10.0);
        let s = Style::<String>::from_css("margin", "10px 20px").unwrap();
        assert_eq!(s.margin.left.into_raw().value(), 20.0);
        assert_eq!(s.margin.top.into_raw().value(), 10.0);
        let s = Style::<String>::from_css("padding", "1px 2px 3px 4px").unwrap();
        assert_eq!(s.padding.top.into_raw().value(), 1.0);
        assert_eq!(s.padding.right.into_raw().value(), 2.0);
        assert_eq!(s.padding.bottom.into_raw().value(), 3.0);
        assert_eq!(s.padding.left.into_raw().value(), 4.0);
    }

    #[test]
    fn from_css_unknown_property() {
        assert!(Style::<String>::from_css("color", "red").is_err());
    }

    // --- Flexbox: wrap, shrink, alignment, grow, column, gap ---

    #[test]
    fn flexbox_wrap_items() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(80.0, 50.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, flex_wrap: FlexWrap::Wrap,
                    size: Size::from_lengths(150.0, 200.0), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(150.0, 200.0)).unwrap();
        let l0 = t.layout(items[0]).unwrap();
        let l1 = t.layout(items[1]).unwrap();
        let l2 = t.layout(items[2]).unwrap();
        assert_eq!(l0.location.x, 0.0);
        assert!(l1.location.y > l0.location.y, "wrapped item[1] y {} not > item[0] y {}", l1.location.y, l0.location.y);
        assert!(l2.location.y > l0.location.y, "wrapped item[2] y {} not > item[0] y {}", l2.location.y, l0.location.y);
    }

    #[test]
    fn flexbox_nowrap_shrink() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(100.0, 50.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(150.0, 50.0), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(150.0, 50.0)).unwrap();
        for (i, item) in items.iter().enumerate() {
            let l = t.layout(*item).unwrap();
            assert!(l.size.width <= 60.0, "item[{}] width {} > 60", i, l.size.width);
        }
        let last = t.layout(items[2]).unwrap();
        assert!((last.location.x + last.size.width - 150.0).abs() < 1.0);
    }

    #[test]
    fn flexbox_align_items_stretch() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style {
                    size: Size { width: Dimension::from_length(80.0), height: Dimension::auto() },
                    ..Default::default()
                }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(400.0, 100.0),
                    align_items: Some(AlignItems::Stretch), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(400.0, 100.0)).unwrap();
        for (i, item) in items.iter().enumerate() {
            let l = t.layout(*item).unwrap();
            assert_eq!(l.size.height, 100.0, "item[{}] height {} != 100", i, l.size.height);
        }
    }

    #[test]
    fn flexbox_align_items_center() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(60.0, 30.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(400.0, 100.0),
                    align_items: Some(AlignItems::Center), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(400.0, 100.0)).unwrap();
        for (i, item) in items.iter().enumerate() {
            let l = t.layout(*item).unwrap();
            assert!((l.location.y - 35.0).abs() < 1.0, "item[{}] y {} != 35", i, l.location.y);
        }
    }

    #[test]
    fn flexbox_justify_space_between() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(50.0, 30.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(400.0, 50.0),
                    justify_content: Some(JustifyContent::SpaceBetween), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(400.0, 50.0)).unwrap();
        let l0 = t.layout(items[0]).unwrap();
        let l2 = t.layout(items[2]).unwrap();
        assert_eq!(l0.location.x, 0.0);
        assert!((l2.location.x + l2.size.width - 400.0).abs() < 1.0,
            "last item right edge {} != 400", l2.location.x + l2.size.width);
    }

    #[test]
    fn flexbox_grow_ratio() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|i| t.new_leaf(Style { size: Size::from_lengths(50.0, 30.0),
                    flex_grow: i as f32, ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(300.0, 50.0), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 50.0)).unwrap();
        // flex_grow = [0, 1, 2], remaining = 150
        // item0: 50 + 0 = 50, item1: 50 + 50 = 100, item2: 50 + 100 = 150
        let l0 = t.layout(items[0]).unwrap();
        let l1 = t.layout(items[1]).unwrap();
        let l2 = t.layout(items[2]).unwrap();
        assert!((l0.size.width - 50.0).abs() < 1.0, "item0 width {}", l0.size.width);
        assert!((l1.size.width - 100.0).abs() < 1.0, "item1 width {}", l1.size.width);
        assert!((l2.size.width - 150.0).abs() < 1.0, "item2 width {}", l2.size.width);
        assert_eq!(l0.location.x, 0.0);
        assert_eq!(l1.location.x, 50.0);
        assert_eq!(l2.location.x, 150.0);
    }

    #[test]
    fn flexbox_column_direction() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(50.0, 80.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style { display: Display::Flex, flex_direction: FlexDirection::Column,
                    size: Size::from_lengths(100.0, 300.0), ..Default::default() }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(100.0, 300.0)).unwrap();
        for (i, item) in items.iter().enumerate() {
            let l = t.layout(*item).unwrap();
            assert_eq!(l.location.x, 0.0);
            assert!((l.location.y - (i as f32 * 80.0)).abs() < 1.0,
                "item[{}] y {} != {}", i, l.location.y, i * 80);
        }
    }

    #[test]
    fn flexbox_gap_between_items() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(80.0, 50.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style {
                    display: Display::Flex,
                    gap: Size { width: LengthPercentage::length(10.0), height: LengthPercentage::length(0.0) },
                    size: Size::from_lengths(300.0, 50.0),
                    ..Default::default()
                }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 50.0)).unwrap();
        let l0 = t.layout(items[0]).unwrap();
        let l1 = t.layout(items[1]).unwrap();
        let l2 = t.layout(items[2]).unwrap();
        assert_eq!(l0.location.x, 0.0);
        assert!((l1.location.x - 90.0).abs() < 1.0, "item1 x {}", l1.location.x);
        assert!((l2.location.x - 180.0).abs() < 1.0, "item2 x {}", l2.location.x);
    }

    // --- Grid: fr tracks, gap, auto rows, fixed tracks, explicit row heights ---

    #[test]
    fn grid_explicit_fr_columns() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![fr(1.0), fr(2.0), fr(1.0)],
                    grid_template_rows: vec![fr(1.0)],
                    size: Size::from_lengths(400.0, 100.0),
                    ..Default::default()
                }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(400.0, 100.0)).unwrap();
        let l0 = t.layout(items[0]).unwrap();
        let l1 = t.layout(items[1]).unwrap();
        let l2 = t.layout(items[2]).unwrap();
        assert!((l0.size.width - 100.0).abs() < 1.0, "col0 width {}", l0.size.width);
        assert!((l1.size.width - 200.0).abs() < 1.0, "col1 width {}", l1.size.width);
        assert!((l2.size.width - 100.0).abs() < 1.0, "col2 width {}", l2.size.width);
    }

    #[test]
    fn grid_column_gap() {
        let mut t = ctree();
        let items: Vec<_> = (0..4)
            .map(|_| t.new_leaf(Style { ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![fr(1.0), fr(1.0)],
                    grid_template_rows: vec![fr(1.0), fr(1.0)],
                    gap: Size { width: LengthPercentage::length(20.0), height: LengthPercentage::length(0.0) },
                    size: Size::from_lengths(220.0, 200.0),
                    ..Default::default()
                }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(220.0, 200.0)).unwrap();
        let col_w = (220.0 - 20.0) / 2.0;
        let l0 = t.layout(items[0]).unwrap();
        let l1 = t.layout(items[1]).unwrap();
        assert!((l0.size.width - col_w).abs() < 1.0);
        assert!((l1.location.x - (col_w + 20.0)).abs() < 1.0, "item1 x {}", l1.location.x);
    }

    #[test]
    fn grid_auto_rows() {
        let mut t = ctree();
        let items: Vec<_> = (0..4)
            .map(|_| t.new_leaf(Style { size: Size::from_lengths(50.0, 30.0), ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![fr(1.0), fr(1.0)],
                    size: Size::from_lengths(200.0, 200.0),
                    ..Default::default()
                }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        for (i, item) in items.iter().enumerate() {
            let l = t.layout(*item).unwrap();
            assert!(l.size.width > 0.0, "item[{}] width", i);
            assert!(l.size.height > 0.0, "item[{}] height", i);
        }
    }

    #[test]
    fn grid_fixed_px_columns() {
        let mut t = ctree();
        let items: Vec<_> = (0..3)
            .map(|_| t.new_leaf(Style { ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTemplateComponent::from_length(100.0),
                        GridTemplateComponent::from_length(200.0),
                        fr(1.0),
                    ],
                    grid_template_rows: vec![fr(1.0)],
                    size: Size::from_lengths(400.0, 100.0),
                    ..Default::default()
                }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(400.0, 100.0)).unwrap();
        let l0 = t.layout(items[0]).unwrap();
        let l1 = t.layout(items[1]).unwrap();
        assert!((l0.size.width - 100.0).abs() < 1.0, "col0 width {}", l0.size.width);
        assert!((l1.size.width - 200.0).abs() < 1.0, "col1 width {}", l1.size.width);
    }

    #[test]
    fn grid_explicit_row_height() {
        let mut t = ctree();
        let items: Vec<_> = (0..4)
            .map(|_| t.new_leaf(Style { ..Default::default() }).unwrap())
            .collect();
        let container = t.new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![fr(1.0), fr(1.0)],
                    grid_template_rows: vec![
                        GridTemplateComponent::from_length(80.0),
                        GridTemplateComponent::from_length(120.0),
                    ],
                    size: Size::from_lengths(200.0, 200.0),
                    ..Default::default()
                }, &items,
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        let l0 = t.layout(items[0]).unwrap();
        let l2 = t.layout(items[2]).unwrap();
        assert!((l0.size.height - 80.0).abs() < 1.0, "row1 height {}", l0.size.height);
        assert!((l2.size.height - 120.0).abs() < 1.0, "row2 height {}", l2.size.height);
    }

    // --- Block: percentage sizing, auto height, margins, negative margin, absolute ---

    #[test]
    fn block_percentage_width() {
        let mut t = ctree();
        let child = t.new_leaf(Style {
                size: Size { width: Dimension::percent(0.5), height: Dimension::from_length(50.0) },
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(400.0, 100.0), ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(400.0, 100.0)).unwrap();
        let l = t.layout(child).unwrap();
        assert!((l.size.width - 200.0).abs() < 1.0, "child width {} != 200", l.size.width);
    }

    #[test]
    fn block_auto_height_expands() {
        let mut t = ctree();
        let child = t.new_leaf(Style { size: Size::from_lengths(100.0, 50.0), ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block,
                    size: Size { width: Dimension::from_length(200.0), height: Dimension::auto() },
                    ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 600.0)).unwrap();
        let cl = t.layout(child).unwrap();
        let pl = t.layout(container).unwrap();
        assert!(pl.size.height >= cl.size.height,
            "container height {} < child height {}", pl.size.height, cl.size.height);
        assert!(pl.size.height > 0.0);
    }

    #[test]
    fn block_margin_top_pushes_down() {
        let mut t = ctree();
        let child = t.new_leaf(Style {
                size: Size::from_lengths(100.0, 50.0),
                margin: Rect { top: LengthPercentageAuto::length(20.0), ..Rect::zero() },
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(200.0, 200.0), ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        let l = t.layout(child).unwrap();
        assert_eq!(l.location.y, 20.0);
        assert_eq!(l.location.x, 0.0);
    }

    #[test]
    fn block_negative_margin_overlap() {
        let mut t = ctree();
        let a = t.new_leaf(Style { size: Size::from_lengths(100.0, 50.0), ..Default::default() }).unwrap();
        let b = t.new_leaf(Style {
                size: Size::from_lengths(100.0, 50.0),
                margin: Rect { top: LengthPercentageAuto::length(-10.0), ..Rect::zero() },
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(200.0, 200.0), ..Default::default() },
                &[a, b],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        let la = t.layout(a).unwrap();
        let lb = t.layout(b).unwrap();
        assert!(lb.location.y < la.location.y + la.size.height,
            "second item y {} not overlapping first item bottom {}",
            lb.location.y, la.location.y + la.size.height);
    }

    #[test]
    fn block_absolute_positioning() {
        let mut t = ctree();
        let child = t.new_leaf(Style {
                position: Position::Absolute,
                size: Size::from_lengths(50.0, 30.0),
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(200.0, 200.0), ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        let l = t.layout(child).unwrap();
        assert!(l.size.width > 0.0);
        assert!(l.size.height > 0.0);
    }

    // --- Float: multiple floats side-by-side, float right, clear left ---

    #[test]
    fn float_left_two_items() {
        let mut t = ctree();
        let f1 = t.new_leaf(Style { size: Size::from_lengths(80.0, 100.0),
                float: crate::Float::Left, ..Default::default() }).unwrap();
        let f2 = t.new_leaf(Style { size: Size::from_lengths(80.0, 100.0),
                float: crate::Float::Left, ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(300.0, 200.0), ..Default::default() },
                &[f1, f2],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 200.0)).unwrap();
        let l1 = t.layout(f1).unwrap();
        let l2 = t.layout(f2).unwrap();
        assert_eq!(l1.location.x, 0.0);
        assert_eq!(l2.location.x, 80.0);
        assert_eq!(l1.location.y, 0.0);
        assert_eq!(l2.location.y, 0.0);
    }

    #[test]
    fn float_right_position() {
        let mut t = ctree();
        let float = t.new_leaf(Style { size: Size::from_lengths(80.0, 100.0),
                float: crate::Float::Right, ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(300.0, 200.0), ..Default::default() },
                &[float],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 200.0)).unwrap();
        let l = t.layout(float).unwrap();
        assert!((l.location.x - 220.0).abs() < 1.0, "float right x {}", l.location.x);
    }

    #[test]
    fn float_clear_left_block() {
        let mut t = ctree();
        let f1 = t.new_leaf(Style { size: Size::from_lengths(80.0, 100.0),
                float: crate::Float::Left, ..Default::default() }).unwrap();
        let f2 = t.new_leaf(Style { size: Size::from_lengths(80.0, 50.0),
                float: crate::Float::Left, clear: crate::Clear::Left, ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Block, size: Size::from_lengths(300.0, 300.0), ..Default::default() },
                &[f1, f2],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 300.0)).unwrap();
        let l2 = t.layout(f2).unwrap();
        assert!(l2.location.y >= 100.0, "cleared float y {} < 100", l2.location.y);
    }

    // --- Edge cases: min-height, display:none, max-width, percent sizing ---

    #[test]
    fn min_height_overrides() {
        let mut t = ctree();
        let child = t.new_leaf(Style {
                size: Size::from_lengths(50.0, 0.0),
                min_size: Size { width: Dimension::auto(), height: Dimension::from_length(100.0) },
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(200.0, 200.0), ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 200.0)).unwrap();
        let l = t.layout(child).unwrap();
        assert!(l.size.height >= 100.0, "child height {} < 100", l.size.height);
    }

    #[test]
    fn display_none_child_takes_no_space() {
        let mut t = ctree();
        let visible = t.new_leaf(Style { size: Size::from_lengths(50.0, 50.0), ..Default::default() }).unwrap();
        let hidden = t.new_leaf(Style { display: Display::None, size: Size::from_lengths(100.0, 100.0), ..Default::default() }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(300.0, 100.0), ..Default::default() },
                &[visible, hidden],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 100.0)).unwrap();
        let lv = t.layout(visible).unwrap();
        let lh = t.layout(hidden).unwrap();
        assert_eq!(lh.size.width, 0.0);
        assert_eq!(lh.size.height, 0.0);
        assert_eq!(lv.location.x, 0.0);
    }

    #[test]
    fn max_width_constrains() {
        let mut t = ctree();
        let child = t.new_leaf(Style {
                size: Size::from_lengths(300.0, 50.0),
                max_size: Size { width: Dimension::from_length(100.0), height: Dimension::auto() },
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(200.0, 100.0), ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(200.0, 100.0)).unwrap();
        let l = t.layout(child).unwrap();
        assert!(l.size.width <= 101.0, "child width {} > 100", l.size.width);
    }

    #[test]
    fn percent_sized_flex_item() {
        let mut t = ctree();
        let child = t.new_leaf(Style {
                size: Size { width: Dimension::percent(0.5), height: Dimension::from_length(50.0) },
                ..Default::default()
            }).unwrap();
        let container = t.new_with_children(
                Style { display: Display::Flex, size: Size::from_lengths(300.0, 100.0), ..Default::default() },
                &[child],
            ).unwrap();
        t.compute_layout(container, make_avail(300.0, 100.0)).unwrap();
        let l = t.layout(child).unwrap();
        assert!((l.size.width - 150.0).abs() < 1.0, "child width {} != 150", l.size.width);
    }
}
