//! Layout engine abstraction for Vayu Browser.
//!
//! This crate defines the interface between the browser's layout pipeline
//! and concrete layout engine implementations (Taffy, native, etc.).

#[cfg(all(test, feature = "taffy-backend"))]
mod diff_tests;

use aether_css::{
    AlignContent as CssAlignContent, AlignItems as CssAlignItems, AlignSelf as CssAlignSelf,
    Display as CssDisplay, FlexDirection as CssFlexDirection, FlexWrap as CssFlexWrap,
    JustifyContent as CssJustifyContent, Position as CssPosition,
};

/// Box sizing model
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BoxSizing {
    #[default]
    ContentBox,
    BorderBox,
}

/// Grid track sizing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridTrack {
    Fixed(f32),
    Fr(f32),
    Auto,
}

/// Grid placement (1-indexed line numbers)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GridPlacement {
    pub start: Option<u16>,
    pub end: Option<u16>,
    pub span: Option<u16>,
}

impl GridPlacement {
    pub fn auto() -> Self {
        Self {
            start: None,
            end: None,
            span: None,
        }
    }
    pub fn line(start: u16) -> Self {
        Self {
            start: Some(start),
            end: None,
            span: None,
        }
    }
    pub fn span(span: u16) -> Self {
        Self {
            start: None,
            end: None,
            span: Some(span),
        }
    }
    pub fn start_end(start: u16, end: u16) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
            span: None,
        }
    }
}

/// Grid auto flow
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GridAutoFlow {
    #[default]
    Row,
    Column,
    Dense,
}

/// Input for a single layout element
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutElementInput {
    /// CSS display type
    pub display: CssDisplay,
    /// CSS position type
    pub position: CssPosition,
    /// Flex direction (for flex containers)
    pub flex_direction: Option<CssFlexDirection>,
    /// Flex wrap (for flex containers)
    pub flex_wrap: Option<CssFlexWrap>,
    /// Align items (for flex/grid containers)
    pub align_items: Option<CssAlignItems>,
    /// Align self (for flex/grid items)
    pub align_self: Option<CssAlignSelf>,
    /// Justify content (for flex/grid containers)
    pub justify_content: Option<CssJustifyContent>,
    /// Align content (for multi-line flex containers)
    pub align_content: Option<CssAlignContent>,
    /// Box sizing model
    pub box_sizing: BoxSizing,
    /// Flex grow factor
    pub flex_grow: f32,
    /// Flex shrink factor
    pub flex_shrink: f32,
    /// Flex basis
    pub flex_basis: Option<f32>,
    /// Grid template columns
    pub grid_template_columns: Option<Vec<GridTrack>>,
    /// Grid template rows
    pub grid_template_rows: Option<Vec<GridTrack>>,
    /// Grid column placement
    pub grid_column: Option<GridPlacement>,
    /// Grid row placement
    pub grid_row: Option<GridPlacement>,
    /// Grid auto flow
    pub grid_auto_flow: Option<GridAutoFlow>,
    /// Gap (row_gap, column_gap)
    pub gap: Option<(f32, f32)>,
    /// Width constraint
    pub width: Option<f32>,
    /// Height constraint
    pub height: Option<f32>,
    /// Min width constraint
    pub min_width: Option<f32>,
    /// Max width constraint
    pub max_width: Option<f32>,
    /// Min height constraint
    pub min_height: Option<f32>,
    /// Max height constraint
    pub max_height: Option<f32>,
    /// Margin: top, right, bottom, left
    pub margin: [Option<f32>; 4],
    /// Padding: top, right, bottom, left
    pub padding: [f32; 4],
    /// Border width: top, right, bottom, left
    pub border_width: [f32; 4],
    /// Inset for positioned elements: top, right, bottom, left
    pub inset: [f32; 4],
    /// Parent index in the element list (None = root)
    pub parent_index: Option<usize>,
    /// Whether this element is a text node (affects inline layout)
    pub is_text: bool,
    /// Text content (for text measurement if needed)
    pub text: String,
    /// Font size for text measurement
    pub font_size: f32,
    /// Line height multiplier
    pub line_height: f32,
    /// Whether element has content (text, wrapped lines, image, explicit size) - affects min-height logic
    pub has_content: bool,
}

/// Complete layout input
#[derive(Debug, Clone)]
pub struct LayoutInput {
    /// Container width
    pub container_width: f32,
    /// Viewport height
    pub viewport_height: f32,
    /// All elements to lay out
    pub elements: Vec<LayoutElementInput>,
}

/// A fragment within a line (text run or inline box)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutFragment {
    /// Element index this fragment belongs to
    pub element_index: usize,
    /// X position within line
    pub x: f32,
    /// Y position (baseline-relative)
    pub y: f32,
    /// Width of fragment
    pub width: f32,
    /// Height of fragment
    pub height: f32,
    /// Baseline offset from top
    pub baseline: f32,
    /// Text content (for text fragments)
    pub text: Option<String>,
    /// Font size
    pub font_size: f32,
    /// Line height multiplier
    pub line_height: f32,
}

/// A line of inline content
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutLine {
    /// Line X position (relative to parent content box)
    pub x: f32,
    /// Line Y position (relative to parent content box)
    pub y: f32,
    /// Line width
    pub width: f32,
    /// Line height
    pub height: f32,
    /// Baseline offset from line top
    pub baseline: f32,
    /// Fragments in this line
    pub fragments: Vec<LayoutFragment>,
}

/// Output for a single layout element
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutElementOutput {
    /// Absolute X position
    pub x: f32,
    /// Absolute Y position
    pub y: f32,
    /// Computed width
    pub width: f32,
    /// Computed height
    pub height: f32,
    /// Lines generated for inline content (empty for block elements)
    pub lines: Vec<LayoutLine>,
}

/// Complete layout output
#[derive(Debug, Clone)]
pub struct LayoutOutput {
    /// Layout results for each input element (same order)
    pub elements: Vec<LayoutElementOutput>,
}

/// Trait for layout engine implementations
/// Note: Engine instances are thread-local due to Taffy's internal raw pointers
pub trait LayoutEngine {
    /// Compute layout for the given input
    fn compute_layout(&self, input: &LayoutInput) -> LayoutOutput;

    /// Engine name for debugging
    fn name(&self) -> &'static str;
}

#[cfg(feature = "taffy-backend")]
mod taffy_impl {
    use super::*;
    use taffy::{
        AlignItems, AlignSelf, AvailableSpace, BoxSizing as TaffyBoxSizing, Dimension, Display,
        FlexDirection, FlexWrap, JustifyContent, LengthPercentage, LengthPercentageAuto, NodeId,
        Position, Size as TaffySize, Style, TaffyTree,
    };

    pub struct TaffyLayoutEngine;

    impl TaffyLayoutEngine {
        pub fn new() -> Self {
            Self
        }

        fn css_display_to_taffy(d: CssDisplay) -> Display {
            match d {
                CssDisplay::Flex | CssDisplay::InlineFlex => Display::Flex,
                CssDisplay::Grid => Display::Grid,
                CssDisplay::None => Display::None,
                _ => Display::Block,
            }
        }

        fn css_position_to_taffy(p: CssPosition) -> Position {
            match p {
                CssPosition::Absolute | CssPosition::Fixed => Position::Absolute,
                _ => Position::Relative,
            }
        }

        fn css_flex_direction_to_taffy(d: CssFlexDirection) -> FlexDirection {
            match d {
                CssFlexDirection::RowReverse => FlexDirection::RowReverse,
                CssFlexDirection::Column => FlexDirection::Column,
                CssFlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
                CssFlexDirection::Row => FlexDirection::Row,
            }
        }

        fn css_flex_wrap_to_taffy(w: CssFlexWrap) -> FlexWrap {
            match w {
                CssFlexWrap::WrapReverse => FlexWrap::WrapReverse,
                CssFlexWrap::Wrap => FlexWrap::Wrap,
                CssFlexWrap::NoWrap => FlexWrap::NoWrap,
            }
        }

        fn css_align_items_to_taffy(a: CssAlignItems) -> Option<AlignItems> {
            match a {
                CssAlignItems::Center => Some(AlignItems::CENTER),
                CssAlignItems::FlexEnd => Some(AlignItems::FLEX_END),
                CssAlignItems::Baseline => Some(AlignItems::BASELINE),
                CssAlignItems::Stretch | CssAlignItems::FlexStart => Some(AlignItems::STRETCH),
            }
        }

        fn css_align_self_to_taffy(a: CssAlignSelf) -> Option<AlignSelf> {
            match a {
                CssAlignSelf::Center => Some(AlignSelf::CENTER),
                CssAlignSelf::FlexEnd => Some(AlignSelf::FLEX_END),
                CssAlignSelf::Baseline => Some(AlignSelf::BASELINE),
                CssAlignSelf::Stretch => Some(AlignSelf::STRETCH),
                CssAlignSelf::Auto => None, // Default "auto" behavior = not set
                CssAlignSelf::FlexStart => Some(AlignSelf::FLEX_START),
            }
        }

        fn css_justify_content_to_taffy(j: CssJustifyContent) -> Option<JustifyContent> {
            match j {
                CssJustifyContent::Center => Some(JustifyContent::CENTER),
                CssJustifyContent::FlexEnd => Some(JustifyContent::FLEX_END),
                CssJustifyContent::SpaceBetween => Some(JustifyContent::SPACE_BETWEEN),
                CssJustifyContent::SpaceAround => Some(JustifyContent::SPACE_AROUND),
                CssJustifyContent::SpaceEvenly => Some(JustifyContent::SPACE_EVENLY),
                CssJustifyContent::FlexStart => Some(JustifyContent::FLEX_START),
            }
        }

        fn css_box_sizing_to_taffy(b: BoxSizing) -> TaffyBoxSizing {
            match b {
                BoxSizing::BorderBox => TaffyBoxSizing::BorderBox,
                BoxSizing::ContentBox => TaffyBoxSizing::ContentBox,
            }
        }

        fn opt_length(v: Option<f32>) -> Dimension {
            v.map(Dimension::length).unwrap_or(Dimension::auto())
        }

        fn opt_length_percentage(v: f32) -> LengthPercentage {
            LengthPercentage::length(v)
        }

        fn opt_length_percentage_auto(v: Option<f32>) -> LengthPercentageAuto {
            v.map(LengthPercentageAuto::length)
                .unwrap_or(LengthPercentageAuto::auto())
        }

        fn opt_length_percentage_auto_direct(v: f32) -> LengthPercentageAuto {
            LengthPercentageAuto::length(v)
        }
    }

    impl Default for TaffyLayoutEngine {
        fn default() -> Self {
            Self::new()
        }
    }

    impl LayoutEngine for TaffyLayoutEngine {
        fn compute_layout(&self, input: &LayoutInput) -> LayoutOutput {
            let mut tree: TaffyTree<()> = TaffyTree::new();

            // Create root
            let root_style = Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: TaffySize {
                    width: Dimension::length(input.container_width),
                    height: Dimension::auto(),
                },
                align_items: Some(AlignItems::STRETCH),
                ..Default::default()
            };
            let root_node = tree.new_leaf(root_style).expect("root leaf");

            let n = input.elements.len();
            let mut node_ids: Vec<Option<NodeId>> = vec![None; n];

            // Create nodes
            for (i, el) in input.elements.iter().enumerate() {
                if el.display == CssDisplay::None {
                    continue;
                }

                let has_content = el.has_content || el.width.is_some() || el.height.is_some();
                let min_width =
                    if el.display == CssDisplay::Block && !has_content && el.width.is_none() {
                        Some(1.0)
                    } else {
                        el.min_width
                    };
                let min_height =
                    if el.display == CssDisplay::Block && !has_content && el.height.is_none() {
                        Some(1.0)
                    } else {
                        el.min_height
                    };

                let style = Style {
                    display: Self::css_display_to_taffy(el.display),
                    position: Self::css_position_to_taffy(el.position),
                    margin: taffy::Rect {
                        top: Self::opt_length_percentage_auto(el.margin[0]),
                        right: Self::opt_length_percentage_auto(el.margin[1]),
                        bottom: Self::opt_length_percentage_auto(el.margin[2]),
                        left: Self::opt_length_percentage_auto(el.margin[3]),
                    },
                    padding: taffy::Rect {
                        top: Self::opt_length_percentage(el.padding[0]),
                        right: Self::opt_length_percentage(el.padding[1]),
                        bottom: Self::opt_length_percentage(el.padding[2]),
                        left: Self::opt_length_percentage(el.padding[3]),
                    },
                    border: taffy::Rect {
                        top: Self::opt_length_percentage(el.border_width[0]),
                        right: Self::opt_length_percentage(el.border_width[1]),
                        bottom: Self::opt_length_percentage(el.border_width[2]),
                        left: Self::opt_length_percentage(el.border_width[3]),
                    },
                    size: TaffySize {
                        width: Self::opt_length(el.width),
                        height: Self::opt_length(el.height),
                    },
                    min_size: TaffySize {
                        width: Self::opt_length(min_width),
                        height: Self::opt_length(min_height),
                    },
                    max_size: TaffySize {
                        width: Self::opt_length(el.max_width),
                        height: Self::opt_length(el.max_height),
                    },
                    inset: taffy::Rect {
                        top: Self::opt_length_percentage_auto_direct(el.inset[0]),
                        right: Self::opt_length_percentage_auto_direct(el.inset[1]),
                        bottom: Self::opt_length_percentage_auto_direct(el.inset[2]),
                        left: Self::opt_length_percentage_auto_direct(el.inset[3]),
                    },
                    flex_direction: el
                        .flex_direction
                        .map(Self::css_flex_direction_to_taffy)
                        .unwrap_or(FlexDirection::Row),
                    flex_wrap: el
                        .flex_wrap
                        .map(Self::css_flex_wrap_to_taffy)
                        .unwrap_or(FlexWrap::NoWrap),
                    align_items: el.align_items.and_then(Self::css_align_items_to_taffy),
                    align_self: el.align_self.and_then(Self::css_align_self_to_taffy),
                    justify_content: el
                        .justify_content
                        .and_then(Self::css_justify_content_to_taffy),
                    flex_grow: el.flex_grow,
                    flex_shrink: el.flex_shrink,
                    flex_basis: el
                        .flex_basis
                        .map(Dimension::length)
                        .unwrap_or(Dimension::auto()),
                    box_sizing: Self::css_box_sizing_to_taffy(el.box_sizing),
                    ..Default::default()
                };

                if let Ok(nid) = tree.new_leaf(style) {
                    node_ids[i] = Some(nid);
                }
            }

            // Build tree
            for i in 0..n {
                if input.elements[i].display == CssDisplay::None {
                    continue;
                }
                let child_id = match node_ids[i] {
                    Some(id) => id,
                    None => continue,
                };

                let parent_id = match input.elements[i].parent_index {
                    Some(pidx) if pidx < n && pidx != i => match node_ids[pidx] {
                        Some(id) => id,
                        None => root_node,
                    },
                    _ => root_node,
                };

                if parent_id != child_id {
                    let _ = tree.add_child(parent_id, child_id);
                }
            }

            // Compute layout
            let _ = tree.compute_layout(
                root_node,
                TaffySize {
                    width: AvailableSpace::Definite(input.container_width),
                    height: AvailableSpace::Definite(input.viewport_height),
                },
            );

            // Read relative positions from Taffy
            let mut rel_x: Vec<f32> = vec![0.0; n];
            let mut rel_y: Vec<f32> = vec![0.0; n];
            let mut widths: Vec<f32> = vec![0.0; n];
            let mut heights: Vec<f32> = vec![0.0; n];
            for i in 0..n {
                let nid = match node_ids[i] {
                    Some(id) => id,
                    None => continue,
                };
                if let Ok(layout) = tree.layout(nid) {
                    rel_x[i] = layout.location.x;
                    rel_y[i] = layout.location.y;
                    widths[i] = layout.size.width.max(0.0);
                    heights[i] = layout.size.height.max(0.0);
                }
            }

            // Convert to absolute positions by walking parent chain
            // Matches original apply_taffy_layout behavior
            let mut abs_x: Vec<f32> = vec![0.0; n];
            let mut abs_y: Vec<f32> = vec![0.0; n];
            for i in 0..n {
                let mut x = rel_x[i];
                let mut y = rel_y[i];
                let mut current = input.elements[i].parent_index;
                let mut steps = 0;
                while let Some(pidx) = current {
                    if pidx < n && pidx != i && steps < n {
                        x += rel_x[pidx];
                        y += rel_y[pidx];
                        current = input.elements[pidx].parent_index;
                        steps += 1;
                    } else {
                        break;
                    }
                }
                abs_x[i] = x;
                abs_y[i] = y;
            }

            // Build output
            let mut output_elements = Vec::with_capacity(n);
            for i in 0..n {
                output_elements.push(LayoutElementOutput {
                    x: abs_x[i],
                    y: abs_y[i],
                    width: widths[i],
                    height: heights[i],
                    lines: Vec::new(),
                });
            }

            LayoutOutput {
                elements: output_elements,
            }
        }

        fn name(&self) -> &'static str {
            "taffy"
        }
    }
}

#[cfg(feature = "native-backend")]
mod native_impl {
    use super::*;
    use std::collections::HashMap;

    // Simple text measurement cache
    struct TextMeasureCache {
        cache: HashMap<(String, u32), f32>, // (text, font_size * 100) -> width
    }

    impl TextMeasureCache {
        fn new() -> Self {
            Self {
                cache: HashMap::new(),
            }
        }

        fn measure(&mut self, text: &str, font_size: f32) -> f32 {
            let key = (text.to_string(), (font_size * 100.0) as u32);
            if let Some(&w) = self.cache.get(&key) {
                return w;
            }
            // Simple approximation: average character width ~ 0.6 * font_size
            // For more accuracy, this would use the actual font shaping
            let avg_char_width = font_size * 0.6;
            let width = text.chars().count() as f32 * avg_char_width;
            self.cache.insert(key, width);
            width
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct ComputedStyle {
        margin: [f32; 4], // top, right, bottom, left
        padding: [f32; 4],
        border: [f32; 4],
        width: Option<f32>,
        height: Option<f32>,
        min_width: Option<f32>,
        min_height: Option<f32>,
        max_width: Option<f32>,
        max_height: Option<f32>,
        box_sizing: BoxSizing,
    }

    impl ComputedStyle {
        fn from_input(el: &LayoutElementInput) -> Self {
            Self {
                margin: [
                    el.margin[0].unwrap_or(0.0),
                    el.margin[1].unwrap_or(0.0),
                    el.margin[2].unwrap_or(0.0),
                    el.margin[3].unwrap_or(0.0),
                ],
                padding: el.padding,
                border: el.border_width,
                width: el.width,
                height: el.height,
                min_width: el.min_width,
                min_height: el.min_height,
                max_width: el.max_width,
                max_height: el.max_height,
                box_sizing: el.box_sizing,
            }
        }

        fn content_width(&self, available: f32) -> f32 {
            match self.box_sizing {
                BoxSizing::ContentBox => self.width.unwrap_or(available),
                BoxSizing::BorderBox => {
                    let w = self.width.unwrap_or(available);
                    let sub = self.padding[1] + self.padding[3] + self.border[1] + self.border[3];
                    (w - sub).max(0.0)
                }
            }
        }

        fn content_height(&self, available: f32) -> f32 {
            match self.box_sizing {
                BoxSizing::ContentBox => self.height.unwrap_or(available),
                BoxSizing::BorderBox => {
                    let h = self.height.unwrap_or(available);
                    let sub = self.padding[0] + self.padding[2] + self.border[0] + self.border[2];
                    (h - sub).max(0.0)
                }
            }
        }
    }

    /// Layout context for block formatting context
    struct BlockLayoutContext {
        container_width: f32,
        viewport_height: f32,
        elements: Vec<LayoutElementInput>,
        outputs: Vec<LayoutElementOutput>,
        computed_styles: Vec<ComputedStyle>,
        children: Vec<Vec<usize>>,
        roots: Vec<usize>,
        text_cache: TextMeasureCache,
    }

    impl BlockLayoutContext {
        fn new(input: &LayoutInput) -> Self {
            let n = input.elements.len();
            let mut children = vec![Vec::new(); n];
            let mut roots = Vec::new();

            for i in 0..n {
                if input.elements[i].display == CssDisplay::None {
                    continue;
                }
                match input.elements[i].parent_index {
                    Some(pidx)
                        if pidx < n
                            && pidx != i
                            && input.elements[pidx].display != CssDisplay::None =>
                    {
                        children[pidx].push(i);
                    }
                    _ => roots.push(i),
                }
            }

            let computed_styles = input
                .elements
                .iter()
                .map(ComputedStyle::from_input)
                .collect();

            Self {
                container_width: input.container_width,
                viewport_height: input.viewport_height,
                elements: input.elements.clone(),
                outputs: vec![
                    LayoutElementOutput {
                        x: 0.0,
                        y: 0.0,
                        width: 0.0,
                        height: 0.0,
                        lines: Vec::new()
                    };
                    n
                ],
                computed_styles,
                children,
                roots,
                text_cache: TextMeasureCache::new(),
            }
        }

        fn layout_flex_container(
            &mut self,
            idx: usize,
            container_x: f32,
            container_y: f32,
            available_width: f32,
        ) {
            if self.elements[idx].display == CssDisplay::None {
                return;
            }
            let style = self.computed_styles[idx].clone();
            let margin_top = style.margin[0];
            let margin_right = style.margin[1];
            let margin_bottom = style.margin[2];
            let margin_left = style.margin[3];
            let avail_width = available_width - margin_left - margin_right;
            let content_width = style.content_width(avail_width.max(0.0)).max(0.0);
            let _content_x = container_x + margin_left;
            let _content_y = container_y + margin_top + style.padding[0];
            let content_height_available = style.content_height(self.viewport_height);
            let is_row = matches!(
                self.elements[idx]
                    .flex_direction
                    .unwrap_or(CssFlexDirection::Row),
                CssFlexDirection::Row | CssFlexDirection::RowReverse
            );
            let available_main = if is_row {
                content_width
            } else {
                content_height_available.max(0.0)
            };
            let available_cross = if is_row {
                content_height_available.max(0.0)
            } else {
                content_width
            };
            // Set container output first so flex items can position relative to it
            let container_height = style.content_height(self.viewport_height).max(0.0);
            let total_width = content_width
                + style.padding[1]
                + style.padding[3]
                + style.border[1]
                + style.border[3]
                + margin_left
                + margin_right;
            // Temporarily set container with estimated height; will update after laying out children
            self.outputs[idx] = LayoutElementOutput {
                x: container_x + margin_left,
                y: container_y + margin_top,
                width: total_width,
                height: container_height
                    + style.padding[0]
                    + style.padding[2]
                    + style.border[0]
                    + style.border[2]
                    + margin_top
                    + margin_bottom,
                lines: Vec::new(),
            };
            {
                let mut flex_ctx =
                    FlexLayoutContext::new(self, idx, available_main, available_cross);
                flex_ctx.layout();
            }
            // Absolute children are out of flow — position via containing block (layout_block handles inset)
            let absolute_children: Vec<usize> = self.children[idx]
                .iter()
                .copied()
                .filter(|&c| {
                    self.elements[c].position == CssPosition::Absolute
                        && self.elements[c].display != CssDisplay::None
                })
                .collect();
            for &child_idx in &absolute_children {
                self.layout_block(child_idx, 0.0, 0.0, available_width);
            }
            // Update container height if auto to contain flex items — absolute children out of flow, skip
            if style.height.is_none() {
                let children = self.children[idx].clone();
                let mut max_cross_bottom: f32 = 0.0;
                let mut max_main_bottom: f32 = 0.0;
                for &child_idx in &children {
                    if self.elements[child_idx].display == CssDisplay::None {
                        continue;
                    }
                    if self.elements[child_idx].position == CssPosition::Absolute {
                        continue;
                    }
                    let out = &self.outputs[child_idx];
                    if is_row {
                        max_cross_bottom =
                            max_cross_bottom.max(out.y + out.height - (container_y + margin_top));
                        max_main_bottom =
                            max_main_bottom.max(out.x + out.width - (container_x + margin_left));
                    } else {
                        max_cross_bottom =
                            max_cross_bottom.max(out.x + out.width - (container_x + margin_left));
                        max_main_bottom =
                            max_main_bottom.max(out.y + out.height - (container_y + margin_top));
                    }
                }
                let content_h = if is_row {
                    max_cross_bottom
                } else {
                    max_main_bottom
                };
                let total_h = content_h
                    + style.padding[0]
                    + style.padding[2]
                    + style.border[0]
                    + style.border[2]
                    + margin_top
                    + margin_bottom;
                self.outputs[idx].height = total_h.max(self.outputs[idx].height);
                // Also need to ensure width is at least content
                if is_row {
                    // For row, width already set to content_width; keep it
                } else {
                    let content_w = max_cross_bottom;
                    self.outputs[idx].width = content_w
                        + style.padding[1]
                        + style.padding[3]
                        + style.border[1]
                        + style.border[3]
                        + margin_left
                        + margin_right;
                }
            }
            // Recursively layout flex item children that are themselves containers (for nested flex/block)
            let children = self.children[idx].clone();
            for &child_idx in &children {
                if self.elements[child_idx].display == CssDisplay::None {
                    continue;
                }
                // If flex item has its own children, layout them
                if !self.children[child_idx].is_empty() {
                    // Determine available width for nested layout
                    let child_out = self.outputs[child_idx].clone();
                    let child_available = child_out.width
                        - self.computed_styles[child_idx].padding[1]
                        - self.computed_styles[child_idx].padding[3]
                        - self.computed_styles[child_idx].border[1]
                        - self.computed_styles[child_idx].border[3];
                    // For simplicity, treat nested as block
                    let nested_children = self.children[child_idx].clone();
                    for &grand_child in &nested_children {
                        if self.elements[grand_child].display == CssDisplay::Block
                            || self.elements[grand_child].display == CssDisplay::Flex
                        {
                            self.layout_block(
                                grand_child,
                                child_out.x,
                                child_out.y + self.computed_styles[child_idx].padding[0],
                                child_available.max(0.0),
                            );
                        }
                    }
                }
            }
        }

        fn layout_grid_container(
            &mut self,
            idx: usize,
            container_x: f32,
            container_y: f32,
            available_width: f32,
        ) {
            if self.elements[idx].display == CssDisplay::None {
                return;
            }
            let style = self.computed_styles[idx].clone();
            let margin_top = style.margin[0];
            let margin_right = style.margin[1];
            let margin_bottom = style.margin[2];
            let margin_left = style.margin[3];
            let avail_width = available_width - margin_left - margin_right;
            let content_width = style.content_width(avail_width.max(0.0)).max(0.0);
            let content_height_available = style.content_height(self.viewport_height);
            let total_width = content_width
                + style.padding[1]
                + style.padding[3]
                + style.border[1]
                + style.border[3]
                + margin_left
                + margin_right;
            let container_height = if style.height.is_some() {
                style.content_height(self.viewport_height).max(0.0)
            } else {
                400.0
            };
            let total_height = container_height
                + style.padding[0]
                + style.padding[2]
                + style.border[0]
                + style.border[2]
                + margin_top
                + margin_bottom;
            self.outputs[idx] = LayoutElementOutput {
                x: container_x + margin_left,
                y: container_y + margin_top,
                width: total_width,
                height: if style.height.is_some() {
                    total_height
                } else {
                    total_height
                },
                lines: Vec::new(),
            };
            let available_height = if style.height.is_some() {
                content_height_available
            } else {
                400.0
            };
            {
                let mut grid_ctx =
                    GridLayoutContext::new(self, idx, content_width, available_height);
                grid_ctx.layout();
            }
            if style.height.is_none() {
                let children = self.children[idx].clone();
                let mut max_bottom: f32 = 0.0;
                for &child_idx in &children {
                    if self.elements[child_idx].display == CssDisplay::None {
                        continue;
                    }
                    let out = &self.outputs[child_idx];
                    max_bottom = max_bottom.max(out.y + out.height - (container_y + margin_top));
                }
                let content_h = max_bottom;
                let total_h = content_h
                    + style.padding[0]
                    + style.padding[2]
                    + style.border[0]
                    + style.border[2]
                    + margin_top
                    + margin_bottom;
                self.outputs[idx].height = total_h.max(self.outputs[idx].height);
            }
            let children = self.children[idx].clone();
            for &child_idx in &children {
                if self.elements[child_idx].display == CssDisplay::None {
                    continue;
                }
                if !self.children[child_idx].is_empty() {
                    let child_out = self.outputs[child_idx].clone();
                    let child_available = child_out.width
                        - self.computed_styles[child_idx].padding[1]
                        - self.computed_styles[child_idx].padding[3]
                        - self.computed_styles[child_idx].border[1]
                        - self.computed_styles[child_idx].border[3];
                    let nested_children = self.children[child_idx].clone();
                    for &grand_child in &nested_children {
                        if self.elements[grand_child].display == CssDisplay::Block
                            || self.elements[grand_child].display == CssDisplay::Flex
                            || self.elements[grand_child].display == CssDisplay::Grid
                        {
                            self.layout_block(
                                grand_child,
                                child_out.x,
                                child_out.y + self.computed_styles[child_idx].padding[0],
                                child_available.max(0.0),
                            );
                        }
                    }
                }
            }
        }

        fn find_containing_block(&self, idx: usize) -> Option<usize> {
            let mut cur = self.elements[idx].parent_index;
            while let Some(pidx) = cur {
                if pidx < self.elements.len() && self.elements[pidx].position != CssPosition::Static
                {
                    return Some(pidx);
                }
                cur = self.elements.get(pidx).and_then(|e| e.parent_index);
            }
            None
        }

        fn layout_block(
            &mut self,
            idx: usize,
            container_x: f32,
            container_y: f32,
            available_width: f32,
        ) {
            if self.elements[idx].display == CssDisplay::None {
                return;
            }
            // Absolute-positioned elements are taken out of flow; position relative to containing block
            // CSS semantics: containing block is nearest positioned ancestor's padding edge; child at inset offsets
            if self.elements[idx].position == CssPosition::Absolute {
                let el = self.elements[idx].clone();
                let style = self.computed_styles[idx].clone();
                let (cb_x, cb_y) = if let Some(cb_idx) = self.find_containing_block(idx) {
                    // containing block's content origin (output position + padding + border)
                    let cb_style = &self.computed_styles[cb_idx];
                    let cb_out = &self.outputs[cb_idx];
                    // padding edge = output x + margin is already included, so add padding/border to get content?
                    // For our fixtures CB has no padding/border, so just use cb_out position
                    (
                        cb_out.x + cb_style.border[3] + cb_style.padding[3],
                        cb_out.y + cb_style.border[0] + cb_style.padding[0],
                    )
                } else {
                    (0.0, 0.0)
                };
                let margin_left = style.margin[3];
                let margin_top = style.margin[0];
                let x = cb_x + el.inset[3] + margin_left;
                let y = cb_y + el.inset[0] + margin_top;
                let content_width = style.content_width(available_width).max(0.0);
                let content_height = if style.height.is_some() {
                    style.content_height(self.viewport_height).max(0.0)
                } else if el.has_content && !el.text.is_empty() {
                    let text_w = self.text_cache.measure(&el.text, el.font_size);
                    let avail = content_width.max(20.0);
                    let lines = if text_w <= avail {
                        1.0
                    } else {
                        (text_w / avail).ceil()
                    };
                    lines * el.font_size * el.line_height
                } else if el.has_content {
                    20.0
                } else {
                    0.0
                };
                let total_width = content_width
                    + style.padding[1]
                    + style.padding[3]
                    + style.border[1]
                    + style.border[3]
                    + margin_left
                    + style.margin[1];
                let total_height = content_height
                    + style.padding[0]
                    + style.padding[2]
                    + style.border[0]
                    + style.border[2]
                    + margin_top
                    + style.margin[2];
                self.outputs[idx] = LayoutElementOutput {
                    x,
                    y,
                    width: total_width,
                    height: total_height,
                    lines: Vec::new(),
                };
                // layout children of absolute element inside it
                if !self.children[idx].is_empty() {
                    let child_available = content_width;
                    let child_x = x + style.border[3] + style.padding[3];
                    let child_y = y + style.border[0] + style.padding[0];
                    let children = self.children[idx].clone();
                    for &child_idx in &children {
                        if self.elements[child_idx].display == CssDisplay::None {
                            continue;
                        }
                        self.layout_block(child_idx, child_x, child_y, child_available.max(0.0));
                    }
                }
                return;
            }
            // If this is a flex container, delegate to flex layout
            if self.elements[idx].display == CssDisplay::Flex
                || self.elements[idx].display == CssDisplay::InlineFlex
            {
                self.layout_flex_container(idx, container_x, container_y, available_width);
                return;
            }
            if self.elements[idx].display == CssDisplay::Grid {
                self.layout_grid_container(idx, container_x, container_y, available_width);
                return;
            }

            // Extract needed values before mutable operations
            let style = self.computed_styles[idx].clone();
            let el = self.elements[idx].clone();
            let has_children = !self.children[idx].is_empty();

            // Calculate margins
            let margin_top = style.margin[0];
            let margin_right = style.margin[1];
            let margin_bottom = style.margin[2];
            let margin_left = style.margin[3];

            // Available width for this element
            let avail_width = available_width - margin_left - margin_right;
            let content_width = style.content_width(avail_width.max(0.0)).max(0.0);

            // Content box starts after top margin
            let content_y = container_y + margin_top;

            // CSS relative positioning: offset the box visually from its normal-flow position.
            // Children are still positioned at the normal-flow content origin (no offset).
            let (rel_x, rel_y) = if self.elements[idx].position == CssPosition::Relative {
                (el.inset[3], el.inset[0]) // left, top
            } else {
                (0.0, 0.0)
            };

            // Layout children at parent's padding-box (normal-flow origin, no relative offset)
            let children_x = container_x + margin_left + style.border[3] + style.padding[3];
            let children_y = content_y + style.border[0] + style.padding[0];
            let children_bottom_y =
                self.layout_children(idx, children_x, children_y, content_width);

            // Content height from children or explicit height
            let content_height = if has_children {
                // Children were laid out starting at content_y + padding_top
                // Content height = bottom of last child - content_y
                let child_content_height = (children_bottom_y - content_y).max(0.0);
                // Only use explicit height if it's larger than children require
                if style.height.is_some() {
                    child_content_height.max(style.content_height(self.viewport_height).max(0.0))
                } else {
                    child_content_height
                }
            } else if style.height.is_some() {
                style.content_height(self.viewport_height).max(0.0)
            } else if el.has_content {
                // Intrinsic text height: measure text and wrap per available width
                if !el.text.is_empty() {
                    let text_w = self.text_cache.measure(&el.text, el.font_size);
                    let avail = content_width.max(20.0);
                    let lines = if text_w <= avail {
                        1.0
                    } else {
                        (text_w / avail).ceil()
                    };
                    lines * el.font_size * el.line_height
                } else {
                    // has_content but no text (e.g., image or explicit size) — use min
                    20.0
                }
            } else {
                0.0
            };

            let total_height = content_height
                + style.padding[0]
                + style.padding[2]
                + style.border[0]
                + style.border[2]
                + margin_bottom;

            // Calculate width
            let total_width = content_width
                + style.padding[1]
                + style.padding[3]
                + style.border[1]
                + style.border[3]
                + margin_left
                + margin_right;

            // Store output (rel_x/rel_y already computed above for relative positioning)
            self.outputs[idx] = LayoutElementOutput {
                x: container_x + margin_left + rel_x,
                y: container_y + margin_top + rel_y,
                width: total_width,
                height: total_height,
                lines: Vec::new(),
            };
        }

        fn layout_children(
            &mut self,
            parent_idx: usize,
            x: f32,
            mut y: f32,
            available_width: f32,
        ) -> f32 {
            let children = self.children[parent_idx].clone();
            let padding_box_top = y; // y is content_y + padding_top = padding-box top

            // Separate inline, block and absolute children — absolute is out of flow per CSS
            let mut inline_children = Vec::new();
            let mut block_children = Vec::new();
            let mut absolute_children = Vec::new();

            for child_idx in children {
                if self.elements[child_idx].display == CssDisplay::None {
                    continue;
                }
                if self.elements[child_idx].position == CssPosition::Absolute {
                    absolute_children.push(child_idx);
                    continue;
                }
                match self.elements[child_idx].display {
                    CssDisplay::Inline | CssDisplay::InlineBlock => inline_children.push(child_idx),
                    CssDisplay::Block | CssDisplay::Flex => block_children.push(child_idx),
                    _ => block_children.push(child_idx),
                }
            }

            // Layout inline children first (they flow in lines)
            if !inline_children.is_empty() {
                y = self.layout_inline_children(
                    parent_idx,
                    x,
                    y,
                    available_width,
                    &inline_children,
                );
            }

            // Then layout block children (they stack vertically) — CSS margin collapse
            // Invariant: prev_bottom_margin_edge = bottom margin edge of last processed child
            let mut prev_bottom_margin_edge: Option<f32> = None;
            let mut prev_mb: f32 = 0.0;
            for &child_idx in &block_children {
                let mt = self.computed_styles[child_idx].margin[0];
                let mb = self.computed_styles[child_idx].margin[2];
                let child_y = match prev_bottom_margin_edge {
                    Some(prev_bme) => {
                        // Collapse: next top margin edge = prev bottom margin edge - min(prev_mb, next_mt)
                        let top_margin_edge = prev_bme - prev_mb.min(mt);
                        // layout_block adds mt internally, so pass top_margin_edge - mt
                        top_margin_edge - mt
                    }
                    None => y, // First child: start at parent content box top
                };
                self.layout_block(child_idx, x, child_y, available_width);
                // layout_block height = content + pad + border + mb. Add mt for margin-box height.
                self.outputs[child_idx].height += mt;
                // Bottom margin edge = y (margin-box top) + margin-box height
                let bottom_margin_edge = self.outputs[child_idx].y + self.outputs[child_idx].height;
                prev_bottom_margin_edge = Some(bottom_margin_edge);
                prev_mb = mb;
            }
            // After block children: y = bottom margin edge of last child (for parent height)
            if let Some(bme) = prev_bottom_margin_edge {
                y = bme;
            }

            // Absolute children are out of flow — position via containing block's padding box, do not affect y
            for child_idx in absolute_children {
                self.layout_block(child_idx, x, padding_box_top, available_width);
            }

            y
        }

        /// Layout inline children with line breaking
        fn layout_inline_children(
            &mut self,
            parent_idx: usize,
            x: f32,
            y: f32,
            available_width: f32,
            inline_children: &[usize],
        ) -> f32 {
            let mut lines = Vec::new();
            let mut current_line = LayoutLine {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                baseline: 0.0,
                fragments: Vec::new(),
            };

            let mut line_x: f32 = 0.0;
            let mut line_max_ascent: f32 = 0.0;
            let mut line_max_descent: f32 = 0.0;

            for &child_idx in inline_children {
                let el = &self.elements[child_idx];
                let style = &self.computed_styles[child_idx];
                let fs = el.font_size;
                let line_h = el.line_height;
                let ml = style.margin[3]; // margin-left
                let mr = style.margin[1]; // margin-right

                if el.is_text || !el.text.is_empty() {
                    // Text fragment — apply margins for the inline box containing this text
                    let text = &el.text;
                    let words: Vec<&str> = text.split_whitespace().collect();

                    // Add margin-left before first word on this line
                    line_x += ml;

                    for word in words {
                        let word_width = self.text_cache.measure(word, fs);
                        let space_width = self.text_cache.measure(" ", fs);
                        let word_total = word_width + space_width;

                        // Check if word fits on current line
                        if line_x + word_total > available_width
                            && current_line.fragments.is_empty()
                        {
                            // Word too long for empty line - force it
                            let baseline = fs * 0.8;
                            let height = fs * line_h;
                            let fragment = LayoutFragment {
                                element_index: child_idx,
                                x: line_x,
                                y: 0.0,
                                width: word_width,
                                height: height,
                                baseline,
                                text: Some(word.to_string()),
                                font_size: fs,
                                line_height: line_h,
                            };
                            current_line.fragments.push(fragment);
                            line_x += word_total;
                            line_max_ascent = line_max_ascent.max(baseline);
                            line_max_descent = line_max_descent.max(height - baseline);
                        } else if line_x + word_total > available_width
                            && !current_line.fragments.is_empty()
                        {
                            // Finish current line, start new one
                            current_line.width = line_x - space_width; // Remove trailing space
                            current_line.height = line_max_ascent + line_max_descent;
                            current_line.baseline = line_max_ascent;
                            lines.push(current_line);

                            // Start new line
                            current_line = LayoutLine {
                                x: 0.0,
                                y: y + lines.iter().map(|l| l.height).sum::<f32>(),
                                width: 0.0,
                                height: 0.0,
                                baseline: 0.0,
                                fragments: Vec::new(),
                            };
                            line_x = 0.0;
                            line_max_ascent = 0.0;
                            line_max_descent = 0.0;

                            // Re-add margin-left on new line
                            line_x += ml;

                            // Place word on new line
                            let baseline = fs * 0.8;
                            let height = fs * line_h;
                            let fragment = LayoutFragment {
                                element_index: child_idx,
                                x: line_x,
                                y: 0.0,
                                width: word_width,
                                height: height,
                                baseline,
                                text: Some(word.to_string()),
                                font_size: fs,
                                line_height: line_h,
                            };
                            current_line.fragments.push(fragment);
                            line_x += word_total;
                            line_max_ascent = line_max_ascent.max(baseline);
                            line_max_descent = line_max_descent.max(height - baseline);
                        } else {
                            // Word fits
                            let baseline = fs * 0.8;
                            let height = fs * line_h;
                            let fragment = LayoutFragment {
                                element_index: child_idx,
                                x: line_x,
                                y: 0.0,
                                width: word_width,
                                height: height,
                                baseline,
                                text: Some(word.to_string()),
                                font_size: fs,
                                line_height: line_h,
                            };
                            current_line.fragments.push(fragment);
                            line_x += word_total;
                            line_max_ascent = line_max_ascent.max(baseline);
                            line_max_descent = line_max_descent.max(height - baseline);
                        }
                    }
                    // Add margin-right after last word
                    line_x += mr;
                } else {
                    // Inline element (not text) - treat as atomic inline box; if its own text empty, look at text children (span+text hierarchy)
                    let element_width = if el.width.is_some() {
                        el.width.unwrap()
                    } else if !el.text.is_empty() {
                        self.text_cache.measure(&el.text, fs)
                    } else {
                        let mut max_w: f32 = 0.0;
                        // clone to avoid borrow conflict with text_cache
                        let grandchildren = self.children[child_idx].clone();
                        for grand in grandchildren {
                            let ge = &self.elements[grand];
                            if !ge.text.is_empty() {
                                let w = self.text_cache.measure(&ge.text, ge.font_size);
                                if w > max_w {
                                    max_w = w;
                                }
                            }
                        }
                        max_w
                    };
                    let box_width = ml + element_width + mr;
                    let element_height = el.height.unwrap_or(fs * line_h);

                    if line_x + box_width > available_width && current_line.fragments.is_empty() {
                        // Too wide for empty line, force it
                        let baseline = fs * 0.8;
                        let height = element_height;
                        let fragment = LayoutFragment {
                            element_index: child_idx,
                            x: line_x + ml,
                            y: 0.0,
                            width: element_width,
                            height: height,
                            baseline,
                            text: if el.is_text {
                                Some(el.text.clone())
                            } else {
                                None
                            },
                            font_size: fs,
                            line_height: line_h,
                        };
                        current_line.fragments.push(fragment);
                        line_x += box_width;
                        line_max_ascent = line_max_ascent.max(baseline);
                        line_max_descent = line_max_descent.max(height - baseline);
                    } else if line_x + box_width > available_width
                        && !current_line.fragments.is_empty()
                    {
                        // Finish current line
                        current_line.width = line_x;
                        current_line.height = line_max_ascent + line_max_descent;
                        current_line.baseline = line_max_ascent;
                        lines.push(current_line);

                        // Start new line
                        current_line = LayoutLine {
                            x: 0.0,
                            y: y + lines.iter().map(|l| l.height).sum::<f32>(),
                            width: 0.0,
                            height: 0.0,
                            baseline: 0.0,
                            fragments: Vec::new(),
                        };
                        line_max_ascent = 0.0;
                        line_max_descent = 0.0;

                        // Place element on new line
                        let baseline = fs * 0.8;
                        let height = element_height;
                        let fragment = LayoutFragment {
                            element_index: child_idx,
                            x: ml,
                            y: 0.0,
                            width: element_width,
                            height: height,
                            baseline,
                            text: if el.is_text {
                                Some(el.text.clone())
                            } else {
                                None
                            },
                            font_size: fs,
                            line_height: line_h,
                        };
                        current_line.fragments.push(fragment);
                        line_x = box_width;
                        line_max_ascent = line_max_ascent.max(baseline);
                        line_max_descent = line_max_descent.max(height - baseline);
                    } else {
                        // Fits on current line
                        let baseline = fs * 0.8;
                        let height = element_height;
                        let fragment = LayoutFragment {
                            element_index: child_idx,
                            x: line_x + ml,
                            y: 0.0,
                            width: element_width,
                            height: height,
                            baseline,
                            text: if el.is_text {
                                Some(el.text.clone())
                            } else {
                                None
                            },
                            font_size: fs,
                            line_height: line_h,
                        };
                        current_line.fragments.push(fragment);
                        line_x += box_width;
                        line_max_ascent = line_max_ascent.max(baseline);
                        line_max_descent = line_max_descent.max(height - baseline);
                    }
                }
            }

            // Push the last line if it has content
            if !current_line.fragments.is_empty() {
                current_line.width = line_x;
                current_line.height = line_max_ascent + line_max_descent;
                current_line.baseline = line_max_ascent;
                lines.push(current_line);
            }

            // Position lines and convert to absolute coordinates
            let mut line_y = y;
            for line in &mut lines {
                line.y = line_y;
                line_y += line.height;
            }

            // Update parent element's lines and set inline child outputs (position/size per CSS inline flow)
            self.outputs[parent_idx].lines.extend(lines.iter().cloned());
            for line in &lines {
                for frag in &line.fragments {
                    let idx = frag.element_index;
                    // also propagate to text children of inline wrappers (span + text hierarchy)
                    let grandchildren = self.children[idx].clone();
                    for &grand in &grandchildren {
                        if self.elements[grand].is_text || !self.elements[grand].text.is_empty() {
                            self.outputs[grand] = LayoutElementOutput {
                                x: x + line.x + frag.x,
                                y: line.y + frag.y,
                                width: frag.width,
                                height: frag.height,
                                lines: Vec::new(),
                            };
                        }
                    }
                    // fragment x is relative to line (line.x is 0), y is 0 relative to line
                    // absolute child position = parent content x + line.x + frag.x, parent y + line.y + frag.y
                    self.outputs[idx] = LayoutElementOutput {
                        x: x + line.x + frag.x,
                        y: line.y + frag.y,
                        width: frag.width,
                        height: frag.height,
                        lines: Vec::new(),
                    };
                }
            }

            // Return bottom of last line
            if !lines.is_empty() {
                y + lines.last().unwrap().y + lines.last().unwrap().height
            } else {
                y
            }
        }
    }

    // Flex layout context
    struct FlexLayoutContext<'a> {
        container_width: f32,
        container_height: f32,
        elements: &'a Vec<LayoutElementInput>,
        outputs: &'a mut Vec<LayoutElementOutput>,
        computed_styles: &'a Vec<ComputedStyle>,
        children: &'a Vec<Vec<usize>>,
        flex_container_idx: usize,
        available_main: f32,
        available_cross: f32,
        is_row: bool,
        is_wrap: bool,
    }

    impl<'a> FlexLayoutContext<'a> {
        fn new(
            ctx: &'a mut BlockLayoutContext,
            flex_container_idx: usize,
            available_main: f32,
            available_cross: f32,
        ) -> Self {
            let el = &ctx.elements[flex_container_idx];
            let flex_direction = el.flex_direction.unwrap_or(CssFlexDirection::Row);
            let flex_wrap = el.flex_wrap.unwrap_or(CssFlexWrap::NoWrap);
            let is_row = matches!(
                flex_direction,
                CssFlexDirection::Row | CssFlexDirection::RowReverse
            );
            let is_wrap = matches!(flex_wrap, CssFlexWrap::Wrap | CssFlexWrap::WrapReverse);

            Self {
                container_width: ctx.container_width,
                container_height: ctx.viewport_height,
                elements: &ctx.elements,
                outputs: &mut ctx.outputs,
                computed_styles: &ctx.computed_styles,
                children: &ctx.children,
                flex_container_idx,
                available_main,
                available_cross: available_cross.max(0.0),
                is_row,
                is_wrap,
            }
        }

        fn layout(&mut self) {
            let child_indices = &self.children[self.flex_container_idx];
            if child_indices.is_empty() {
                return;
            }

            // Collect flex items — absolute children are out of flow per CSS, skip here
            let mut flex_items: Vec<FlexItem> = Vec::new();
            for &child_idx in child_indices {
                if self.elements[child_idx].display == CssDisplay::None {
                    continue;
                }
                if self.elements[child_idx].position == CssPosition::Absolute {
                    continue;
                }
                let style = &self.computed_styles[child_idx];
                let is_row = self.is_row;
                let content_main = if is_row {
                    style.content_width(self.available_main)
                } else {
                    style.content_height(self.available_main)
                };
                let content_cross = if is_row {
                    style.content_height(self.available_cross)
                } else {
                    style.content_width(self.available_cross)
                };
                let flex_basis = self.elements[child_idx].flex_basis;
                let base_main = flex_basis.unwrap_or(content_main);
                let base_cross = content_cross;
                flex_items.push(FlexItem {
                    idx: child_idx,
                    flex_grow: self.elements[child_idx].flex_grow,
                    flex_shrink: self.elements[child_idx].flex_shrink,
                    flex_basis: self.elements[child_idx].flex_basis,
                    main_min: if is_row {
                        self.elements[child_idx].min_width.unwrap_or(0.0)
                    } else {
                        self.elements[child_idx].min_height.unwrap_or(0.0)
                    },
                    main_max: if is_row {
                        self.elements[child_idx].max_width.unwrap_or(f32::INFINITY)
                    } else {
                        self.elements[child_idx].max_height.unwrap_or(f32::INFINITY)
                    },
                    cross_min: if is_row {
                        self.elements[child_idx].min_height.unwrap_or(0.0)
                    } else {
                        self.elements[child_idx].min_width.unwrap_or(0.0)
                    },
                    cross_max: if is_row {
                        self.elements[child_idx].max_height.unwrap_or(f32::INFINITY)
                    } else {
                        self.elements[child_idx].max_width.unwrap_or(f32::INFINITY)
                    },
                    margin_main_start: if self.is_row {
                        style.margin[3]
                    } else {
                        style.margin[0]
                    },
                    margin_main_end: if self.is_row {
                        style.margin[1]
                    } else {
                        style.margin[2]
                    },
                    margin_cross_start: if is_row {
                        style.margin[0]
                    } else {
                        style.margin[3]
                    },
                    margin_cross_end: if is_row {
                        style.margin[2]
                    } else {
                        style.margin[1]
                    },
                    main_size: base_main,
                    cross_size: base_cross,
                });
            }

            // Perform flex layout
            self.layout_flex(flex_items);
        }

        fn layout_flex(&mut self, mut flex_items: Vec<FlexItem>) {
            let item_count = flex_items.len();
            if item_count == 0 {
                return;
            }

            // Calculate base sizes and free space
            let mut total_basis = 0.0;
            let mut total_grow = 0.0;
            let mut total_shrink_weighted = 0.0;

            for item in &flex_items {
                total_basis += item.main_size;
                total_grow += item.flex_grow;
                total_shrink_weighted += item.flex_shrink * item.main_size;
            }

            let free_space = self.available_main - total_basis;

            // Apply flex grow/shrink
            let mut final_main_sizes = Vec::with_capacity(flex_items.len());
            if free_space > 0.0 && total_grow > 0.0 {
                // Distribute free space proportionally to flex-grow
                for item in &mut flex_items {
                    let grow_share = (item.flex_grow / total_grow) * free_space;
                    item.main_size += grow_share;
                    final_main_sizes.push(item.main_size);
                }
            } else if free_space < 0.0 && total_shrink_weighted > 0.0 {
                // Shrink items proportionally to flex-shrink * base size
                for item in &mut flex_items {
                    let shrink_share =
                        (item.flex_shrink * item.main_size / total_shrink_weighted) * (-free_space);
                    item.main_size = (item.main_size - shrink_share)
                        .max(item.main_min)
                        .min(item.main_max);
                    final_main_sizes.push(item.main_size);
                }
            } else {
                // No free space to distribute, just use base sizes
                for item in &flex_items {
                    final_main_sizes.push(item.main_size.clamp(item.main_min, item.main_max));
                }
            }

            // Apply min/max constraints
            for i in 0..flex_items.len() {
                flex_items[i].main_size =
                    final_main_sizes[i].clamp(flex_items[i].main_min, flex_items[i].main_max);
            }

            // Handle cross-axis sizing
            for item in &mut flex_items {
                // Cross size: use explicit size or content-based
                if item.cross_size > 0.0 {
                    item.cross_size = item.cross_size.clamp(item.cross_min, item.cross_max);
                } else {
                    // Auto cross size - use available cross size for stretch
                    item.cross_size = self.available_cross.clamp(item.cross_min, item.cross_max);
                }
            }

            // Apply alignment
            let cross_start = if self.is_row {
                self.outputs[self.flex_container_idx].y
            } else {
                self.outputs[self.flex_container_idx].x
            };
            let align_items = self.elements[self.flex_container_idx]
                .align_items
                .unwrap_or(CssAlignItems::Stretch);
            let _align_content = self.elements[self.flex_container_idx]
                .align_content
                .unwrap_or(CssAlignContent::Stretch);
            let justify_content = self.elements[self.flex_container_idx]
                .justify_content
                .unwrap_or(CssJustifyContent::FlexStart);

            // Apply justify-content (main axis)
            let used_main = flex_items
                .iter()
                .map(|i| i.main_size + i.margin_main_start + i.margin_main_end)
                .sum::<f32>();
            let main_free = (self.available_main - used_main).max(0.0);

            let mut main_pos = 0.0;
            match justify_content {
                CssJustifyContent::FlexStart => main_pos = 0.0,
                CssJustifyContent::FlexEnd => main_pos = main_free,
                CssJustifyContent::Center => main_pos = main_free / 2.0,
                CssJustifyContent::SpaceBetween => {
                    if item_count > 1 {
                        main_pos = 0.0;
                        // Space will be distributed between items
                    }
                }
                CssJustifyContent::SpaceAround => {
                    main_pos = main_free / (item_count * 2) as f32;
                }
                CssJustifyContent::SpaceEvenly => {
                    main_pos = main_free / (item_count + 1) as f32;
                }
            }

            // Layout each item
            let mut current_main = main_pos;
            for item in &mut flex_items {
                let main_start = current_main + item.margin_main_start;
                let cross_start = self.calculate_cross_start(item, cross_start, align_items);

                if self.is_row {
                    self.outputs[item.idx].x = self.outputs[self.flex_container_idx].x
                        + self.computed_styles[self.flex_container_idx].padding[3]
                        + self.computed_styles[self.flex_container_idx].border[3]
                        + main_start;
                    self.outputs[item.idx].y = self.outputs[self.flex_container_idx].y
                        + self.computed_styles[self.flex_container_idx].padding[0]
                        + self.computed_styles[self.flex_container_idx].border[0]
                        + cross_start
                        + item.margin_cross_start;
                    self.outputs[item.idx].width = item.main_size;
                    self.outputs[item.idx].height = item.cross_size;
                } else {
                    self.outputs[item.idx].x = self.outputs[self.flex_container_idx].x
                        + self.computed_styles[self.flex_container_idx].padding[3]
                        + self.computed_styles[self.flex_container_idx].border[3]
                        + cross_start
                        + item.margin_cross_start;
                    self.outputs[item.idx].y = self.outputs[self.flex_container_idx].y
                        + self.computed_styles[self.flex_container_idx].padding[0]
                        + self.computed_styles[self.flex_container_idx].border[0]
                        + main_start;
                    self.outputs[item.idx].width = item.cross_size;
                    self.outputs[item.idx].height = item.main_size;
                }

                // Update position for next item
                current_main += item.main_size + item.margin_main_start + item.margin_main_end;
                if matches!(
                    justify_content,
                    CssJustifyContent::SpaceBetween
                        | CssJustifyContent::SpaceAround
                        | CssJustifyContent::SpaceEvenly
                ) {
                    if justify_content == CssJustifyContent::SpaceBetween && item_count > 1 {
                        current_main += main_free / (item_count - 1) as f32;
                    } else if justify_content == CssJustifyContent::SpaceAround {
                        current_main += main_free / item_count as f32;
                    } else if justify_content == CssJustifyContent::SpaceEvenly {
                        current_main += main_free / (item_count + 1) as f32;
                    }
                }
            }
        }

        fn calculate_cross_start(
            &self,
            item: &FlexItem,
            _container_cross_start: f32,
            align_items: CssAlignItems,
        ) -> f32 {
            let container_cross = self.available_cross;
            let item_cross = item.cross_size + item.margin_cross_start + item.margin_cross_end;

            let _align = match item.margin_cross_start > 0.0 || item.margin_cross_end > 0.0 {
                true => align_items, // If explicit margins, respect them
                false => align_items,
            };

            let item_align = if item.cross_size > 0.0 {
                align_items
            } else {
                align_items
            };

            match item_align {
                CssAlignItems::FlexStart => 0.0,
                CssAlignItems::FlexEnd => container_cross - item_cross,
                CssAlignItems::Center => (container_cross - item_cross) / 2.0,
                CssAlignItems::Stretch => 0.0, // Will be stretched to fill
                CssAlignItems::Baseline => 0.0, // Baseline alignment not implemented yet
            }
        }
    }

    // Flex item representation
    struct FlexItem {
        idx: usize,
        flex_grow: f32,
        flex_shrink: f32,
        flex_basis: Option<f32>,
        main_size: f32,
        cross_size: f32,
        main_min: f32,
        main_max: f32,
        cross_min: f32,
        cross_max: f32,
        margin_main_start: f32,
        margin_main_end: f32,
        margin_cross_start: f32,
        margin_cross_end: f32,
    }

    // Grid layout context
    struct GridLayoutContext<'a> {
        elements: &'a Vec<LayoutElementInput>,
        outputs: &'a mut Vec<LayoutElementOutput>,
        computed_styles: &'a Vec<ComputedStyle>,
        children: &'a Vec<Vec<usize>>,
        grid_container_idx: usize,
        container_width: f32,
        container_height: f32,
        cols: Vec<GridTrack>,
        rows: Vec<GridTrack>,
        col_gap: f32,
        row_gap: f32,
    }

    impl<'a> GridLayoutContext<'a> {
        fn new(
            ctx: &'a mut BlockLayoutContext,
            grid_container_idx: usize,
            available_width: f32,
            available_height: f32,
        ) -> Self {
            let el = &ctx.elements[grid_container_idx];
            let cols = el
                .grid_template_columns
                .clone()
                .unwrap_or_else(|| vec![GridTrack::Auto]);
            let rows = el
                .grid_template_rows
                .clone()
                .unwrap_or_else(|| vec![GridTrack::Auto]);
            let (row_gap, col_gap) = el.gap.unwrap_or((0.0, 0.0));
            Self {
                elements: &ctx.elements,
                outputs: &mut ctx.outputs,
                computed_styles: &ctx.computed_styles,
                children: &ctx.children,
                grid_container_idx,
                container_width: available_width,
                container_height: available_height,
                cols,
                rows,
                col_gap,
                row_gap,
            }
        }

        fn intrinsic_for_item(&self, child_idx: usize, is_column: bool) -> f32 {
            let el = &self.elements[child_idx];
            let style = &self.computed_styles[child_idx];
            // Use explicit size if present, otherwise text measurement or fallback
            let explicit = if is_column {
                el.width.or(style.width)
            } else {
                el.height.or(style.height)
            };
            if let Some(v) = explicit {
                return v;
            }
            if !el.text.is_empty() {
                // Simple text measurement: ~0.6*font_size per char
                return el.text.chars().count() as f32 * el.font_size * 0.6;
            }
            if el.has_content {
                return if is_column { 100.0 } else { 30.0 };
            }
            // For items with no explicit size and no text, use min size or 0
            0.0
        }

        fn layout(&mut self) {
            let n = self.children[self.grid_container_idx].len() as u16;
            if n == 0 {
                return;
            }
            // First, determine placements for all children (needed for intrinsic)
            let cols = self.cols.len() as u16;
            let rows = self.rows.len() as u16;
            let auto_flow = self.elements[self.grid_container_idx]
                .grid_auto_flow
                .unwrap_or(GridAutoFlow::Row);
            let mut placements: Vec<(u16, u16, u16, u16)> = Vec::new();
            let mut next_col: u16 = 0;
            let mut next_row: u16 = 0;
            let children = self.children[self.grid_container_idx].clone();
            for &child_idx in &children {
                if self.elements[child_idx].display == CssDisplay::None {
                    placements.push((0, 1, 0, 1));
                    continue;
                }
                let (cs, cspan, rs, rspan) = self.resolve_placement(
                    self.elements[child_idx].grid_column,
                    self.elements[child_idx].grid_row,
                    &mut next_col,
                    &mut next_row,
                    cols,
                    rows,
                    auto_flow,
                );
                placements.push((
                    cs.min(cols).max(0),
                    cspan.min(cols - cs.min(cols).max(0)),
                    rs.min(rows).max(0),
                    rspan.min(rows - rs.min(rows).max(0)),
                ));
            }
            // Compute track sizes with intrinsic for auto
            let col_sizes = self.compute_track_sizes_intrinsic(
                &self.cols.clone(),
                self.container_width,
                self.col_gap,
                &children,
                &placements,
                true,
            );
            let row_sizes = self.compute_track_sizes_intrinsic(
                &self.rows.clone(),
                self.container_height,
                self.row_gap,
                &children,
                &placements,
                false,
            );
            let cols = col_sizes.len() as u16;
            let rows = row_sizes.len() as u16;
            // Precompute track offsets
            let mut col_offsets = Vec::with_capacity(cols as usize);
            let mut acc: f32 = 0.0;
            for (i, &sz) in col_sizes.iter().enumerate() {
                col_offsets.push(acc);
                acc += sz
                    + if i + 1 < col_sizes.len() {
                        self.col_gap
                    } else {
                        0.0
                    };
            }
            let mut row_offsets = Vec::with_capacity(rows as usize);
            acc = 0.0;
            for (i, &sz) in row_sizes.iter().enumerate() {
                row_offsets.push(acc);
                acc += sz
                    + if i + 1 < row_sizes.len() {
                        self.row_gap
                    } else {
                        0.0
                    };
            }
            let container = &self.outputs[self.grid_container_idx];
            let content_x = container.x
                + self.computed_styles[self.grid_container_idx].padding[3]
                + self.computed_styles[self.grid_container_idx].border[3];
            let content_y = container.y
                + self.computed_styles[self.grid_container_idx].padding[0]
                + self.computed_styles[self.grid_container_idx].border[0];
            let auto_flow = self.elements[self.grid_container_idx]
                .grid_auto_flow
                .unwrap_or(GridAutoFlow::Row);
            let mut next_col: u16 = 0;
            let mut next_row: u16 = 0;
            let children = self.children[self.grid_container_idx].clone();
            for &child_idx in &children {
                if self.elements[child_idx].display == CssDisplay::None {
                    continue;
                }
                let placement_col = self.elements[child_idx].grid_column;
                let placement_row = self.elements[child_idx].grid_row;
                let (col_start, col_span, row_start, row_span) = self.resolve_placement(
                    placement_col,
                    placement_row,
                    &mut next_col,
                    &mut next_row,
                    cols,
                    rows,
                    auto_flow,
                );
                let col_start = col_start.min(cols).max(0);
                let row_start = row_start.min(rows).max(0);
                let col_span = col_span.min(cols - col_start);
                let row_span = row_span.min(rows - row_start);
                let x = content_x + col_offsets[col_start as usize];
                let y = content_y + row_offsets[row_start as usize];
                let w: f32 = col_sizes[col_start as usize..(col_start + col_span) as usize]
                    .iter()
                    .sum::<f32>()
                    + self.col_gap * (col_span as f32 - 1.0).max(0.0);
                let h: f32 = row_sizes[row_start as usize..(row_start + row_span) as usize]
                    .iter()
                    .sum::<f32>()
                    + self.row_gap * (row_span as f32 - 1.0).max(0.0);
                let style = &self.computed_styles[child_idx];
                let el = &self.elements[child_idx];
                let mut final_w =
                    w - style.padding[1] - style.padding[3] - style.border[1] - style.border[3];
                let mut final_h =
                    h - style.padding[0] - style.padding[2] - style.border[0] - style.border[2];
                if let Some(min_w) = el.min_width {
                    final_w = final_w.max(min_w);
                }
                if let Some(max_w) = el.max_width {
                    final_w = final_w.min(max_w);
                }
                if let Some(min_h) = el.min_height {
                    final_h = final_h.max(min_h);
                }
                if let Some(max_h) = el.max_height {
                    final_h = final_h.min(max_h);
                }
                self.outputs[child_idx].x =
                    x + style.margin[3] + style.border[3] + style.padding[3];
                self.outputs[child_idx].y =
                    y + style.margin[0] + style.border[0] + style.padding[0];
                self.outputs[child_idx].width = final_w.max(0.0);
                self.outputs[child_idx].height = final_h.max(0.0);
            }
        }

        fn compute_track_sizes(&self, tracks: &[GridTrack], available: f32, gap: f32) -> Vec<f32> {
            if tracks.is_empty() {
                return vec![available];
            }
            let mut fixed_sum: f32 = 0.0;
            let mut fr_sum: f32 = 0.0;
            let mut auto_count = 0;
            for t in tracks {
                match t {
                    GridTrack::Fixed(v) => fixed_sum += *v,
                    GridTrack::Fr(v) => fr_sum += *v,
                    GridTrack::Auto => auto_count += 1,
                }
            }
            let gaps = gap * (tracks.len() as f32 - 1.0).max(0.0);
            let mut remaining = (available - fixed_sum - gaps).max(0.0);
            // Auto tracks get equal share of remaining if no fr, else 0 for now (intrinsic)
            let auto_size = if auto_count > 0 && fr_sum == 0.0 {
                remaining / auto_count as f32
            } else if auto_count > 0 {
                100.0
            } else {
                0.0
            };
            if auto_count > 0 && fr_sum == 0.0 {
                remaining -= auto_size * auto_count as f32;
            } else if auto_count > 0 {
                // For mixed auto+fr, auto gets 100, remaining for fr
                remaining -= auto_size * auto_count as f32;
                remaining = remaining.max(0.0);
            }
            let mut sizes = Vec::with_capacity(tracks.len());
            for t in tracks {
                match t {
                    GridTrack::Fixed(v) => sizes.push(*v),
                    GridTrack::Auto => sizes.push(auto_size),
                    GridTrack::Fr(v) => {
                        let sz = if fr_sum > 0.0 {
                            remaining * (*v / fr_sum)
                        } else {
                            0.0
                        };
                        sizes.push(sz);
                    }
                }
            }
            sizes
        }

        fn compute_track_sizes_intrinsic(
            &self,
            tracks: &[GridTrack],
            available: f32,
            gap: f32,
            children: &[usize],
            placements: &[(u16, u16, u16, u16)],
            is_col: bool,
        ) -> Vec<f32> {
            if tracks.is_empty() {
                return vec![available];
            }
            let mut fixed_sum: f32 = 0.0;
            let mut fr_sum: f32 = 0.0;
            let mut auto_indices = Vec::new();
            for (i, t) in tracks.iter().enumerate() {
                match t {
                    GridTrack::Fixed(v) => fixed_sum += *v,
                    GridTrack::Fr(v) => fr_sum += *v,
                    GridTrack::Auto => auto_indices.push(i),
                }
            }
            let gaps = gap * (tracks.len() as f32 - 1.0).max(0.0);
            let mut remaining = (available - fixed_sum - gaps).max(0.0);
            // Compute intrinsic for each auto track
            let mut auto_sizes = vec![0.0; tracks.len()];
            for &idx in &auto_indices {
                let mut max_intrinsic: f32 = 0.0;
                for (child_pos, &child_idx) in children.iter().enumerate() {
                    if self.elements[child_idx].display == CssDisplay::None {
                        continue;
                    }
                    let (c_start, c_span, r_start, r_span) = placements[child_pos];
                    let in_track = if is_col {
                        c_start as usize <= idx && idx < (c_start + c_span) as usize
                    } else {
                        r_start as usize <= idx && idx < (r_start + r_span) as usize
                    };
                    if !in_track {
                        continue;
                    }
                    // Only consider non-spanning or spanning that includes this track; for spanning, distribute but for now just use max
                    if (is_col && c_span != 1) || (!is_col && r_span != 1) {
                        // For spanning items, distribute intrinsic equally for now
                        continue;
                    }
                    let intrinsic = self.intrinsic_for_item(child_idx, is_col);
                    max_intrinsic = max_intrinsic.max(intrinsic);
                }
                // Fallback to 100 if no items or intrinsic 0, but use at least 30 for row auto height
                if max_intrinsic <= 0.0 {
                    max_intrinsic = if is_col { 100.0 } else { 50.0 };
                }
                auto_sizes[idx] = max_intrinsic;
            }
            let auto_sum: f32 = auto_indices.iter().map(|&i| auto_sizes[i]).sum();
            remaining = (remaining - auto_sum).max(0.0);
            let mut sizes = Vec::with_capacity(tracks.len());
            for (i, t) in tracks.iter().enumerate() {
                match t {
                    GridTrack::Fixed(v) => sizes.push(*v),
                    GridTrack::Auto => sizes.push(auto_sizes[i]),
                    GridTrack::Fr(v) => {
                        let sz = if fr_sum > 0.0 {
                            remaining * (*v / fr_sum)
                        } else {
                            0.0
                        };
                        sizes.push(sz);
                    }
                }
            }
            sizes
        }

        fn resolve_placement(
            &self,
            col: Option<GridPlacement>,
            row: Option<GridPlacement>,
            next_col: &mut u16,
            next_row: &mut u16,
            cols: u16,
            rows: u16,
            flow: GridAutoFlow,
        ) -> (u16, u16, u16, u16) {
            let col_start = col
                .and_then(|p| p.start.map(|s| s - 1))
                .unwrap_or(*next_col);
            let col_span = col
                .and_then(|p| p.span)
                .or(col.and_then(|p| p.end.map(|e| e - 1 - col_start)))
                .unwrap_or(1);
            let row_start = row
                .and_then(|p| p.start.map(|s| s - 1))
                .unwrap_or(*next_row);
            let row_span = row
                .and_then(|p| p.span)
                .or(row.and_then(|p| p.end.map(|e| e - 1 - row_start)))
                .unwrap_or(1);
            let col_s = col_start.min(cols.saturating_sub(1));
            let row_s = row_start.min(rows.saturating_sub(1));
            // Advance auto cursor
            if col.is_none() && row.is_none() {
                match flow {
                    GridAutoFlow::Row | GridAutoFlow::Dense => {
                        *next_col += col_span;
                        if *next_col >= cols {
                            *next_col = 0;
                            *next_row += 1;
                        }
                    }
                    GridAutoFlow::Column => {
                        *next_row += row_span;
                        if *next_row >= rows {
                            *next_row = 0;
                            *next_col += 1;
                        }
                    }
                }
            } else if col.is_none() {
                *next_col = col_s + col_span;
                if *next_col >= cols {
                    *next_col = 0;
                    *next_row += 1;
                }
            } else if row.is_none() {
                *next_row = row_s + row_span;
            }
            (col_s, col_span, row_s, row_span)
        }
    }

    pub struct NativeLayoutEngine;

    impl NativeLayoutEngine {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for NativeLayoutEngine {
        fn default() -> Self {
            Self::new()
        }
    }

    impl LayoutEngine for NativeLayoutEngine {
        fn compute_layout(&self, input: &LayoutInput) -> LayoutOutput {
            let mut ctx = BlockLayoutContext::new(input);

            let roots = ctx.roots.clone();

            // Layout from roots
            for &root in &roots {
                ctx.layout_block(root, 0.0, 0.0, ctx.container_width);
            }

            LayoutOutput {
                elements: ctx.outputs,
            }
        }

        fn name(&self) -> &'static str {
            "native"
        }
    }
}

#[cfg(feature = "native-backend")]
pub use native_impl::NativeLayoutEngine;

#[cfg(feature = "taffy-backend")]
pub use taffy_impl::TaffyLayoutEngine;
