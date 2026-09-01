use super::extractor::{BoxSizing as ElBoxSizing, StyledElement};
use crate::engine::stratus as css;
use crate::engine::text::measure_text_width;
use crate::plog;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use taffy::{
    AlignItems, AlignSelf, AvailableSpace, BoxSizing, Dimension, Display, FlexDirection, FlexWrap,
    JustifyContent, LengthPercentage, LengthPercentageAuto, NodeId, Position, Size as TaffySize,
    Style, TaffyTree,
};

// E1-B/C: Global constants for "M" and " " widths per font size
static CHAR_WIDTH_CACHE: OnceLock<RwLock<HashMap<u32, f32>>> = OnceLock::new();
static SPACE_WIDTH_CACHE: OnceLock<RwLock<HashMap<u32, f32>>> = OnceLock::new();

// E1-C: digit-width cache and profiling
static DIGIT_WIDTH_CACHE: OnceLock<RwLock<HashMap<(u32, char), f32>>> = OnceLock::new();

// E1-C: fast path profiling
thread_local! {
    static FAST_PATH_HITS: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static FAST_PATH_MISSES: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static FAST_PATH_FALLBACKS: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
}

fn get_char_width(font_size: f32) -> f32 {
    let fs_key = (font_size.clamp(6.0, 200.0) * 100.0) as u32;
    let cache = CHAR_WIDTH_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    let mut guard = cache.write().unwrap();
    *guard
        .entry(fs_key)
        .or_insert_with(|| measure_text_width("M", font_size))
}

fn get_space_width(font_size: f32) -> f32 {
    let fs_key = (font_size.clamp(6.0, 200.0) * 100.0) as u32;
    let cache = SPACE_WIDTH_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    let mut guard = cache.write().unwrap();
    *guard
        .entry(fs_key)
        .or_insert_with(|| measure_text_width(" ", font_size))
}

// E1-C: Get digit width with fast path for numeric strings
fn get_digit_width(font_size: f32, digit: char) -> f32 {
    let fs_key = (font_size.clamp(6.0, 200.0) * 100.0) as u32;
    let cache = DIGIT_WIDTH_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    let mut guard = cache.write().unwrap();
    *guard
        .entry((fs_key, digit))
        .or_insert_with(|| measure_text_width(&digit.to_string(), font_size))
}

// Check if a string is all ASCII digits
fn is_ascii_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

// Fast path: measure numeric string by summing digit widths + check if exact
fn measure_numeric_fast(text: &str, font_size: f32) -> Option<f32> {
    if !is_ascii_digits(text) {
        FAST_PATH_MISSES.with(|c| *c.borrow_mut() += 1);
        return None;
    }

    FAST_PATH_HITS.with(|c| *c.borrow_mut() += 1);

    // Sum digit widths
    let mut total = 0.0f32;
    for c in text.chars() {
        total += get_digit_width(font_size, c);
    }

    // Verify against actual shaping for this exact string
    // (cache will catch subsequent calls)
    let exact = measure_text_width(text, font_size);

    // If digit sum matches within 0.5px, trust the fast path
    // (floating point + shaping quirks can cause tiny differences)
    if (total - exact).abs() < 0.5 {
        Some(total)
    } else {
        FAST_PATH_FALLBACKS.with(|c| *c.borrow_mut() += 1);
        None // Fall back to exact measurement
    }
}

#[cfg(test)]
mod e1c_tests {
    use super::wrap_text;

    fn e1c_reset_counters() {
        super::FAST_PATH_HITS.with(|c| *c.borrow_mut() = 0);
        super::FAST_PATH_MISSES.with(|c| *c.borrow_mut() = 0);
        super::FAST_PATH_FALLBACKS.with(|c| *c.borrow_mut() = 0);
    }

    fn e1c_get_summary() -> (u64, u64, u64) {
        let hits = super::FAST_PATH_HITS.with(|c| *c.borrow());
        let misses = super::FAST_PATH_MISSES.with(|c| *c.borrow());
        let fallbacks = super::FAST_PATH_FALLBACKS.with(|c| *c.borrow());
        (hits, misses, fallbacks)
    }

    #[test]
    #[ignore]
    fn e1c_fast_path_stats() {
        println!("\n=== E1-C: Numeric fast path statistics ===");

        // Test with benchmark-like text: 2500 paragraphs
        let mut text = String::new();
        for i in 0..2500 {
            text.push_str(&format!("paragraph {} wraps across the line because this sentence is long enough to split\n", i));
        }

        e1c_reset_counters();
        let _ = wrap_text(&text, 800.0, 16.0);
        let (hits, misses, fallbacks) = e1c_get_summary();

        println!("Numeric fast path:");
        println!("  hits (digit strings):  {}", hits);
        println!("  misses (non-digits):   {}", misses);
        println!("  fallbacks (mismatch):  {}", fallbacks);
        println!(
            "  success rate:          {:.1}%",
            if hits > 0 {
                hits as f64 * 100.0 / (hits + fallbacks) as f64
            } else {
                0.0
            }
        );
    }
}

pub fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    if max_width <= 0.0 || font_size <= 0.0 || text.is_empty() {
        return vec![text.to_string()];
    }

    let char_w = get_char_width(font_size);
    let max_chars = (max_width / char_w).floor() as usize;
    if max_chars < 1 {
        return vec![text.to_string()];
    }

    let space_w = get_space_width(font_size);

    let mut lines: Vec<String> = vec![];
    let mut current = String::new();
    let mut current_w = 0.0f32;

    // Per-wrap memoization for word widths within this call
    let mut word_cache: HashMap<String, f32> = HashMap::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            if !current.is_empty() {
                lines.push(current.clone());
                current.clear();
                current_w = 0.0;
            }
            lines.push(String::new());
            continue;
        }
        for word in paragraph.split_whitespace() {
            let word_w = *word_cache.entry(word.to_string()).or_insert_with(|| {
                // E1-C: try numeric fast path first
                if let Some(fast_w) = measure_numeric_fast(word, font_size) {
                    fast_w
                } else {
                    measure_text_width(word, font_size)
                }
            });

            if current.is_empty() {
                current = word.to_string();
                current_w = word_w;
            } else {
                let candidate_w = current_w + space_w + word_w;
                if candidate_w <= max_width {
                    current.push(' ');
                    current.push_str(word);
                    current_w = candidate_w;
                } else {
                    lines.push(current.clone());
                    current = word.to_string();
                    current_w = word_w;
                }
            }
        }
        if !current.is_empty() {
            lines.push(current.clone());
            current.clear();
            current_w = 0.0;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(text.to_string());
    }
    lines
}

fn el_to_taffy_style(el: &StyledElement) -> Option<Style> {
    if el.display == css::Display::None {
        return None;
    }

    let display = match el.display {
        css::Display::Flex | css::Display::InlineFlex => Display::Flex,
        css::Display::Grid => Display::Grid,
        css::Display::None => Display::None,
        _ => Display::Block,
    };

    let position = match el.position {
        css::Position::Absolute | css::Position::Fixed => Position::Absolute,
        _ => Position::Relative,
    };

    let dim = |v: Option<f32>| v.map(Dimension::length).unwrap_or(Dimension::auto());
    let mm = |min: Option<f32>, max: Option<f32>| {
        (
            min.map(Dimension::length).unwrap_or(Dimension::auto()),
            max.map(Dimension::length).unwrap_or(Dimension::auto()),
        )
    };

    let has_content = !el.text.is_empty()
        || !el.wrapped_lines.is_empty()
        || el.image_handle.is_some()
        || el.css_width.is_some()
        || el.css_height.is_some();

    let margin_left = el.margin_left.unwrap_or(0.0);
    let margin_right = el.margin_right.unwrap_or(0.0);

    let mut s = Style {
        display,
        position,
        margin: taffy::Rect {
            top: LengthPercentageAuto::length(el.margin_top),
            right: LengthPercentageAuto::length(margin_right),
            bottom: LengthPercentageAuto::length(el.margin_bottom),
            left: LengthPercentageAuto::length(margin_left),
        },
        padding: taffy::Rect {
            top: LengthPercentage::length(el.padding[0]),
            right: LengthPercentage::length(el.padding[1]),
            bottom: LengthPercentage::length(el.padding[2]),
            left: LengthPercentage::length(el.padding[3]),
        },
        border: taffy::Rect {
            top: LengthPercentage::length(el.border_widths[0]),
            right: LengthPercentage::length(el.border_widths[1]),
            bottom: LengthPercentage::length(el.border_widths[2]),
            left: LengthPercentage::length(el.border_widths[3]),
        },
        size: TaffySize {
            width: dim(el.css_width),
            height: dim(el.css_height),
        },
        ..Default::default()
    };

    s.inset = taffy::Rect {
        top: LengthPercentageAuto::length(el.inset_top),
        right: LengthPercentageAuto::length(el.inset_right),
        bottom: LengthPercentageAuto::length(el.inset_bottom),
        left: LengthPercentageAuto::length(el.inset_left),
    };

    if display == Display::Block && !has_content {
        s.min_size = TaffySize {
            width: Dimension::auto(),
            height: Dimension::length(1.0),
        };
    }

    let (min_w, max_w) = mm(el.min_width, el.max_width);
    let (mut min_h, max_h) = mm(el.min_height, el.max_height);

    if matches!(display, Display::Block) && !has_content {
        min_h = if min_h == Dimension::auto() {
            Dimension::length(1.0)
        } else {
            min_h
        };
    }

    s.min_size = TaffySize {
        width: min_w,
        height: min_h,
    };
    s.max_size = TaffySize {
        width: max_w,
        height: max_h,
    };

    if display == Display::Flex {
        s.flex_direction = match el.flex_direction {
            css::FlexDirection::RowReverse => FlexDirection::RowReverse,
            css::FlexDirection::Column => FlexDirection::Column,
            css::FlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
            css::FlexDirection::Row => FlexDirection::Row,
        };
        s.flex_wrap = match el.flex_wrap {
            css::FlexWrap::WrapReverse => FlexWrap::WrapReverse,
            css::FlexWrap::Wrap => FlexWrap::Wrap,
            css::FlexWrap::NoWrap => FlexWrap::NoWrap,
        };
        s.justify_content = match el.justify_content {
            css::JustifyContent::Center => Some(JustifyContent::CENTER),
            css::JustifyContent::FlexEnd => Some(JustifyContent::FLEX_END),
            css::JustifyContent::SpaceBetween => Some(JustifyContent::SPACE_BETWEEN),
            css::JustifyContent::SpaceAround => Some(JustifyContent::SPACE_AROUND),
            css::JustifyContent::SpaceEvenly => Some(JustifyContent::SPACE_EVENLY),
            css::JustifyContent::FlexStart => Some(JustifyContent::FLEX_START),
        };
        s.align_items = match el.align_items {
            css::AlignItems::Center => Some(AlignItems::CENTER),
            css::AlignItems::FlexEnd => Some(AlignItems::FLEX_END),
            css::AlignItems::Baseline => Some(AlignItems::BASELINE),
            css::AlignItems::Stretch | css::AlignItems::FlexStart => Some(AlignItems::STRETCH),
        };
    }
    s.flex_grow = el.flex_grow;
    s.flex_shrink = el.flex_shrink;
    if let Some(basis) = el.flex_basis {
        s.flex_basis = Dimension::length(basis);
    }
    s.box_sizing = match el.box_sizing {
        ElBoxSizing::BorderBox => BoxSizing::BorderBox,
        ElBoxSizing::ContentBox => BoxSizing::ContentBox,
    };
    if el.align_self != css::AlignSelf::Auto {
        s.align_self = match el.align_self {
            css::AlignSelf::Center => Some(AlignSelf::CENTER),
            css::AlignSelf::FlexEnd => Some(AlignSelf::FLEX_END),
            css::AlignSelf::Baseline => Some(AlignSelf::BASELINE),
            css::AlignSelf::Stretch => Some(AlignSelf::STRETCH),
            css::AlignSelf::Auto | css::AlignSelf::FlexStart => Some(AlignSelf::FLEX_START),
        };
    }
    plog!("ELSTYLE", "tag={:15} cd={:?} size={:?} min_size={:?} max_size={:?} align_self={:?} align_items={:?} box_sizing={:?}", el.tag, display, s.size, s.min_size, s.max_size, s.align_self, s.align_items, s.box_sizing);
    Some(s)
}

pub fn apply_taffy_layout(elements: &mut [StyledElement], container_width: f32, viewport_h: f32) {
    if elements.is_empty() {
        return;
    }

    let valid_count = elements
        .iter()
        .filter(|el| el.display != css::Display::None)
        .count();
    if valid_count == 0 {
        return;
    }

    for el in elements.iter_mut() {
        if el.display == css::Display::None || el.text.is_empty() {
            continue;
        }
        let fs = if el.font_size.is_finite() {
            el.font_size.clamp(6.0, 200.0)
        } else {
            16.0
        };
        // ponytail: this is the ONLY wrap — its lines feed both taffy heights
        // and the painter, so they can never disagree. Narrowed flex items may
        // paint estimate-width lines; a second taffy pass is the measured
        // follow-up if that fidelity ever matters (PLAN A4).
        let pb = el.padding[1] + el.padding[3] + el.border_widths[1] + el.border_widths[3];
        let available_width = el.css_width.unwrap_or(container_width) - pb;
        let available = if available_width.is_finite() && available_width > 0.0 {
            available_width
        } else {
            container_width
        };
        let lines = wrap_text(&el.text, available, fs);
        if el.css_height.is_none() {
            el.css_height = Some(fs * el.line_height.max(1.0) * lines.len() as f32);
        }
        el.wrapped_lines = lines;
        if el.display == css::Display::Inline && el.min_width.is_none() && el.css_width.is_none() {
            let text_w = measure_text_width(&el.text, fs);
            el.min_width = Some(text_w + pb);
        }
    }

    let mut tree: TaffyTree<()> = TaffyTree::new();

    let root_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        size: TaffySize {
            width: Dimension::length(container_width),
            height: Dimension::auto(),
        },
        align_items: Some(AlignItems::STRETCH),
        ..Default::default()
    };
    let root_node = match tree.new_leaf(root_style) {
        Ok(n) => n,
        Err(_) => {
            plog!("TAFFY", "Failed to create root leaf");
            return;
        }
    };

    let mut node_ids: Vec<Option<NodeId>> = vec![None; elements.len()];

    for (i, el) in elements.iter().enumerate() {
        if let Some(style) = el_to_taffy_style(el) {
            if let Ok(nid) = tree.new_leaf(style) {
                node_ids[i] = Some(nid);
            }
        }
    }

    for i in 0..elements.len() {
        if elements[i].display == css::Display::None {
            continue;
        }
        let child_id = match node_ids[i] {
            Some(id) => id,
            None => continue,
        };

        let parent_id = match elements[i].parent_index {
            Some(pidx) => {
                if pidx < elements.len() && pidx != i {
                    match node_ids[pidx] {
                        Some(id) => id,
                        None => root_node,
                    }
                } else {
                    root_node
                }
            }
            None => root_node,
        };

        if parent_id != child_id {
            if let Err(e) = tree.add_child(parent_id, child_id) {
                plog!("TAFFY", "add_child failed for element {}: {:?}", i, e);
            }
        }
    }

    // Correct parent_index for elements whose original parent was invalid
    // (display=none, out of bounds, or has no Taffy node) to prevent
    // stale parent pointers from accumulating incorrect offsets.
    for i in 0..elements.len() {
        if elements[i].display == css::Display::None {
            continue;
        }
        if let Some(pidx) = elements[i].parent_index {
            if pidx >= elements.len()
                || pidx == i
                || elements[pidx].display == css::Display::None
                || node_ids[pidx].is_none()
            {
                elements[i].parent_index = None;
            }
        }
    }

    if !elements.is_empty() {
        if let Err(e) = tree.compute_layout(
            root_node,
            TaffySize {
                width: AvailableSpace::Definite(container_width),
                height: AvailableSpace::Definite(viewport_h),
            },
        ) {
            plog!("TAFFY", "compute_layout failed: {:?}", e);
        }
        plog!(
            "TAFFY",
            "Tree layout computed ({} nodes, viewport_h={})",
            node_ids.len(),
            viewport_h
        );
    }

    let mut abs_x: Vec<f32> = vec![0.0; elements.len()];
    let mut abs_y: Vec<f32> = vec![0.0; elements.len()];
    let mut widths: Vec<f32> = vec![0.0; elements.len()];
    let mut heights: Vec<f32> = vec![0.0; elements.len()];
    for (i, el) in elements.iter().enumerate() {
        let nid = match node_ids[i] {
            Some(id) => id,
            None => continue,
        };
        if let Ok(layout) = tree.layout(nid) {
            let lx = layout.location.x;
            let ly = layout.location.y;
            let lw = layout.size.width;
            let lh = layout.size.height;
            abs_x[i] = if lx.is_finite() { lx } else { 0.0 };
            abs_y[i] = if ly.is_finite() { ly } else { 0.0 };
            widths[i] = if lw.is_finite() && lw >= 0.0 {
                lw
            } else {
                el.css_width.unwrap_or(container_width)
            };
            heights[i] = if lh.is_finite() && lh >= 0.0 {
                lh
            } else {
                el.css_height.unwrap_or(0.0)
            };
        }
    }
    let n = elements.len();
    for i in 0..n {
        let mut x = abs_x[i];
        let mut y = abs_y[i];
        let mut current = elements[i].parent_index;
        let mut steps = 0;
        while let Some(pidx) = current {
            if pidx < n && pidx != i && steps < n {
                x += abs_x[pidx];
                y += abs_y[pidx];
                current = elements[pidx].parent_index;
                steps += 1;
            } else {
                break;
            }
        }
        elements[i].x = x;
        elements[i].y = y;
        elements[i].width = widths[i];
        elements[i].height = heights[i];
    }

    for i in 0..elements.len() {
        if elements[i].display == css::Display::None || elements[i].display == css::Display::Inline
        {
            continue;
        }
        let inline_children: Vec<usize> = (0..elements.len())
            .filter(|j| {
                *j != i
                    && elements[*j].parent_index == Some(i)
                    && elements[*j].display == css::Display::Inline
            })
            .collect();
        if inline_children.is_empty() {
            continue;
        }
        let fs = if elements[i].font_size.is_finite() {
            elements[i].font_size.clamp(6.0, 200.0)
        } else {
            16.0
        };
        let mut cumulative = measure_text_width(&elements[i].text, fs);
        for &child_idx in &inline_children {
            elements[child_idx].x = elements[i].x + cumulative;
            let child_w = if elements[child_idx].css_width.is_some() {
                elements[child_idx].width
            } else {
                let child_text_w = measure_text_width(&elements[child_idx].text, fs);
                elements[child_idx].width = child_text_w.max(fs);
                elements[child_idx].width
            };
            if elements[child_idx].css_height.is_none() {
                elements[child_idx].height = fs * elements[i].line_height.max(1.0);
            }
            cumulative += child_w;
        }
    }

    for (i, el) in elements.iter().enumerate().take(20) {
        let tag = if el.tag.len() > 15 {
            &el.tag[..15]
        } else {
            &el.tag
        };
        let text_preview: String = el.text.chars().take(30).collect();
        plog!(
            "POS",
            "[{}] tag={:15} x={:>6.0} y={:>6.0} w={:>6.0} h={:>6.0} parent={:?} text=\"{}\"",
            i,
            tag,
            el.x,
            el.y,
            el.width,
            el.height,
            el.parent_index,
            text_preview
        );
    }
}
