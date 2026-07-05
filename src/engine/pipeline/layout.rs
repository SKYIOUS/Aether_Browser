use unicode_width::UnicodeWidthStr;

use super::extractor::StyledElement;
use crate::engine::caelum::prelude::*;
use crate::plog;

const CHAR_W_SCALE: f32 = 0.58;

fn text_visual_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    if max_width <= 0.0 || font_size <= 0.0 || text.is_empty() {
        return vec![text.to_string()];
    }
    let char_w = font_size * CHAR_W_SCALE;
    let max_chars = (max_width / char_w).floor() as usize;
    if max_chars < 1 { return vec![text.to_string()]; }

    let mut lines: Vec<String> = vec![];
    let mut current = String::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            if !current.is_empty() {
                lines.push(current.clone());
                current.clear();
            }
            lines.push(String::new());
            continue;
        }
        for word in paragraph.split_whitespace() {
            if current.is_empty() {
                current = word.to_string();
            } else {
                let candidate = format!("{} {}", current, word);
                if text_visual_width(&candidate) <= max_chars {
                    current = candidate;
                } else {
                    lines.push(current.clone());
                    current = word.to_string();
                }
            }
        }
        if !current.is_empty() {
            lines.push(current.clone());
            current.clear();
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

fn apply_text_wrapping(elements: &mut [StyledElement], container_width: f32) {
    let page_w = container_width;
    for el in elements.iter_mut() {
        let fs = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 };
        let available = if el.width.is_finite() && el.width > 0.0 { el.width } else { page_w };
        let lines = wrap_text(&el.text, available, fs);
        el.wrapped_lines = lines;
    }
}

fn el_to_caelum_style(el: &StyledElement) -> Option<Style> {
    if el.display == "none" { return None; }
    let cd = crate::bridge_gen::str_display_to_caelum(&el.display);
    let auto = |v: Option<f32>| v.map(LengthPercentageAuto::length).unwrap_or(LengthPercentageAuto::auto());
    let dim = |v: Option<f32>| v.map(Dimension::from_length).unwrap_or(Dimension::auto());
    let mm = |min: Option<f32>, max: Option<f32>| {
        (min.map(Dimension::from_length).unwrap_or(Dimension::auto()),
         max.map(Dimension::from_length).unwrap_or(Dimension::auto()))
    };
    let mut s = Style {
        display: cd,
        margin: Rect { top: LengthPercentageAuto::length(el.margin_top), right: auto(el.margin_right), bottom: LengthPercentageAuto::length(el.margin_bottom), left: auto(el.margin_left) },
        padding: Rect { top: LengthPercentage::length(el.padding[0]), right: LengthPercentage::length(el.padding[1]), bottom: LengthPercentage::length(el.padding[2]), left: LengthPercentage::length(el.padding[3]) },
        border: Rect { top: LengthPercentage::length(el.border_widths[0]), right: LengthPercentage::length(el.border_widths[1]), bottom: LengthPercentage::length(el.border_widths[2]), left: LengthPercentage::length(el.border_widths[3]) },
        size: Size { width: dim(el.css_width), height: dim(el.css_height) },
        ..Default::default()
    };
    let (min_w, max_w) = mm(el.min_width, el.max_width);
    let (min_h, max_h) = mm(el.min_height, el.max_height);
    s.min_size = Size { width: min_w, height: min_h };
    s.max_size = Size { width: max_w, height: max_h };
    if cd == Display::Flex {
        s.flex_direction = crate::bridge_gen::str_flex_direction_to_caelum(&el.flex_direction);
        s.flex_wrap = crate::bridge_gen::str_flex_wrap_to_caelum(&el.flex_wrap);
        s.justify_content = Some(crate::bridge_gen::str_justify_content_to_caelum(&el.justify_content));
        s.align_items = Some(crate::bridge_gen::str_align_items_to_caelum(&el.align_items));
    }
    s.flex_grow = el.flex_grow;
    s.flex_shrink = el.flex_shrink;
    if let Some(basis) = el.flex_basis { s.flex_basis = Dimension::from_length(basis); }
    Some(s)
}

pub fn apply_caelum_layout(elements: &mut [StyledElement], container_width: f32, viewport_h: f32) {
    if elements.is_empty() { return; }

    // ponytail: estimate heights for text elements so Caelum can stack block elements correctly
    for el in elements.iter_mut() {
        if el.css_height.is_none() && el.display != "none" && !el.text.is_empty() {
            let fs = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 };
            let text_w = text_visual_width(&el.text) as f32 * fs * CHAR_W_SCALE;
            let available_width = el.css_width.unwrap_or(container_width);
            let line_count = if available_width > 0.0 && text_w > available_width {
                (text_w / available_width).ceil().max(1.0)
            } else { 1.0 };
            el.css_height = Some(fs * el.line_height.max(1.0) * line_count);
        }
    }

    let mut tree: CaelumTree = CaelumTree::new();

    let root_style = Style {
        display: Display::Block,
        size: Size { width: Dimension::from_length(container_width), height: Dimension::auto() },
        ..Default::default()
    };
    let root_node = match tree.new_leaf(root_style) {
        Ok(n) => n,
        Err(_) => { plog!("CAELUM", "Failed to create root leaf"); return; }
    };

    let mut node_ids: Vec<Option<NodeId>> = vec![None; elements.len()];

    for (i, el) in elements.iter().enumerate() {
        if let Some(style) = el_to_caelum_style(el) {
            if let Ok(nid) = tree.new_leaf(style) {
                node_ids[i] = Some(nid);
            }
        }
    }

    for (i, el) in elements.iter().enumerate() {
        if el.display == "none" { continue; }
        let child_id = match node_ids[i] { Some(id) => id, None => continue };

        let parent_nid = match el.parent_index {
            Some(pidx) => {
                if pidx < elements.len() {
                    match node_ids[pidx] { Some(id) => id, None => root_node }
                } else { root_node }
            }
            None => root_node,
        };

        if let Err(e) = tree.add_child(parent_nid, child_id) {
            plog!("CAELUM", "add_child failed: {:?}", e);
        }
    }

    if elements.len() > 1 {
        if let Err(e) = tree.compute_layout(root_node, Size {
            width: AvailableSpace::Definite(container_width),
            height: AvailableSpace::Definite(viewport_h),
        }) {
            plog!("CAELUM", "compute_layout failed: {:?}", e);
        }
        plog!("CAELUM", "Tree layout computed ({} nodes, viewport_h={})", node_ids.len(), viewport_h);
    }

    let mut abs_x: Vec<f32> = vec![0.0; elements.len()];
    let mut abs_y: Vec<f32> = vec![0.0; elements.len()];
    let mut widths: Vec<f32> = vec![0.0; elements.len()];
    let mut heights: Vec<f32> = vec![0.0; elements.len()];
    for (i, el) in elements.iter().enumerate() {
        let nid = match node_ids[i] { Some(id) => id, None => continue };
        if let Ok(layout) = tree.layout(nid) {
            let lx = layout.location.x;
            let ly = layout.location.y;
            let lw = layout.size.width;
            let lh = layout.size.height;
            abs_x[i] = if lx.is_finite() { lx.max(0.0) } else { 0.0 };
            abs_y[i] = if ly.is_finite() { ly.max(0.0) } else { 0.0 };
            widths[i] = if lw.is_finite() && lw > 0.0 { lw } else { el.css_width.unwrap_or(container_width) };
            heights[i] = if lh.is_finite() && lh > 0.0 { lh } else { el.css_height.unwrap_or(0.0) };
        }
    }
    for i in 0..elements.len() {
        if let Some(pidx) = elements[i].parent_index {
            if pidx < elements.len() {
                abs_x[i] += abs_x[pidx];
                abs_y[i] += abs_y[pidx];
            }
        }
    }
    for (i, el) in elements.iter_mut().enumerate() {
        el.x = abs_x[i];
        el.y = abs_y[i];
        el.width = widths[i];
        el.height = heights[i];
    }
    for el in elements.iter_mut() {
        if el.display == "inline" && !el.text.is_empty() {
            let fs = el.font_size.clamp(6.0, 200.0);
            let cw = fs * CHAR_W_SCALE;
            let max_line_w = if el.wrapped_lines.is_empty() {
                text_visual_width(&el.text) as f32 * cw
            } else {
                el.wrapped_lines.iter()
                    .map(|l| text_visual_width(l) as f32 * cw)
                    .fold(0.0f32, f32::max)
            };
            if max_line_w > 0.0 {
                el.width = max_line_w.min(container_width);
            }
        }
    }

    apply_text_wrapping(elements, container_width);

    let n = elements.len();
    for i in (0..n).rev() {
        if let Some(pidx) = elements[i].parent_index {
            if pidx < n {
                let child_bottom = elements[i].y + elements[i].height + elements[i].margin_bottom;
                let parent_bottom = elements[pidx].y + elements[pidx].height;
                if child_bottom > parent_bottom {
                    elements[pidx].height = child_bottom - elements[pidx].y;
                }
            }
        }
    }

    for (i, el) in elements.iter().enumerate().take(20) {
        let tag = if el.tag.len() > 15 { &el.tag[..15] } else { &el.tag };
        let text_preview: String = el.text.chars().take(30).collect();
        plog!("POS", "[{}] tag={:15} x={:>6.0} y={:>6.0} w={:>6.0} h={:>6.0} parent={:?} text=\"{}\"",
            i, tag, el.x, el.y, el.width, el.height, el.parent_index, text_preview);
    }
}
