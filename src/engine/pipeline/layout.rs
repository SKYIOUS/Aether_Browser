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
        let content_w = el.width - el.padding[1] - el.padding[3] - el.border_widths[1] - el.border_widths[3];
        let available = if content_w.is_finite() && content_w > 0.0 { content_w } else { page_w };
        let lines = wrap_text(&el.text, available, fs);
        el.wrapped_lines = lines;
    }
}

fn el_to_caelum_style(el: &StyledElement) -> Option<Style> {
    if el.display == "none" { return None; }
    let cd = crate::bridge_gen::str_display_to_caelum(&el.display);
    let dim = |v: Option<f32>| v.map(Dimension::from_length).unwrap_or(Dimension::auto());
    let mm = |min: Option<f32>, max: Option<f32>| {
        (min.map(Dimension::from_length).unwrap_or(Dimension::auto()),
         max.map(Dimension::from_length).unwrap_or(Dimension::auto()))
    };
    
    let has_content = !el.text.is_empty() || !el.wrapped_lines.is_empty() || el.image_handle.is_some() || el.css_width.is_some() || el.css_height.is_some();
    
    let margin_left = el.margin_left.unwrap_or(0.0);
    let margin_right = el.margin_right.unwrap_or(0.0);
    let mut s = Style {
        display: cd,
        margin: Rect { top: LengthPercentageAuto::length(el.margin_top), right: LengthPercentageAuto::length(margin_right), bottom: LengthPercentageAuto::length(el.margin_bottom), left: LengthPercentageAuto::length(margin_left) },
        padding: Rect { top: LengthPercentage::length(el.padding[0]), right: LengthPercentage::length(el.padding[1]), bottom: LengthPercentage::length(el.padding[2]), left: LengthPercentage::length(el.padding[3]) },
        border: Rect { top: LengthPercentage::length(el.border_widths[0]), right: LengthPercentage::length(el.border_widths[1]), bottom: LengthPercentage::length(el.border_widths[2]), left: LengthPercentage::length(el.border_widths[3]) },
         size: Size { width: dim(el.css_width), height: dim(el.css_height) },
         ..Default::default()
     };
     
     s.position = crate::bridge_gen::str_position_to_caelum(&el.position);
     s.inset = Rect { top: LengthPercentageAuto::length(el.inset_top), right: LengthPercentageAuto::length(el.inset_right), bottom: LengthPercentageAuto::length(el.inset_bottom), left: LengthPercentageAuto::length(el.inset_left) };
    
    if cd == Display::Block && !has_content {
        s.min_size = Size { 
            width: Dimension::auto(), 
            height: Dimension::length(1.0)
        };
    }
    
    let (min_w, max_w) = mm(el.min_width, el.max_width);
    let (mut min_h, max_h) = mm(el.min_height, el.max_height);
    
    if cd == Display::Block && !has_content {
        if min_h.is_auto() { min_h = Dimension::length(1.0); }
    }
    
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
    s.box_sizing = match el.box_sizing.as_str() {
        "border-box" => BoxSizing::BorderBox,
        _ => BoxSizing::ContentBox,
    };
    if el.align_self != "auto" {
        s.align_self = Some(crate::bridge_gen::str_align_self_to_caelum(&el.align_self));
    }
    plog!("ELSTYLE", "tag={:15} cd={:?} size={:?} min_size={:?} max_size={:?} align_self={:?} align_items={:?} box_sizing={:?}", el.tag, cd, s.size, s.min_size, s.max_size, s.align_self, s.align_items, s.box_sizing);
    Some(s)
}

pub fn apply_caelum_layout(elements: &mut [StyledElement], container_width: f32, viewport_h: f32) {
    if elements.is_empty() { return; }

    // Fix 1: Pre-process to ensure proper parent-child relationships and filter invalid elements
    let valid_count = elements.iter().filter(|el| el.display != "none").count();
    if valid_count == 0 { return; }

    // ponytail: estimate heights for text elements so Caelum can stack block elements correctly
    for el in elements.iter_mut() {
        if el.css_height.is_none() && el.display != "none" && !el.text.is_empty() {
            let fs = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 };
            let pb = el.padding[1] + el.padding[3] + el.border_widths[1] + el.border_widths[3];
            let available_width = el.css_width.unwrap_or(container_width) - pb;
            let available = if available_width.is_finite() && available_width > 0.0 { available_width } else { container_width };
            let lines = wrap_text(&el.text, available, fs);
            el.css_height = Some(fs * el.line_height.max(1.0) * lines.len() as f32);
        }
        if el.display == "inline" && el.min_width.is_none() && !el.text.is_empty() && el.css_width.is_none() {
            let fs = el.font_size.clamp(6.0, 200.0);
            let cw = fs * CHAR_W_SCALE;
            let text_w = text_visual_width(&el.text) as f32 * cw;
            let pb = el.padding[1] + el.padding[3] + el.border_widths[1] + el.border_widths[3];
            el.min_width = Some(text_w + pb);
        }
    }

    let mut tree: CaelumTree = CaelumTree::new();

    // Fix 2: Root container should use flex column for proper stacking of block elements
    let root_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        size: Size { width: Dimension::from_length(container_width), height: Dimension::auto() },
        align_items: Some(crate::bridge_gen::str_align_items_to_caelum("stretch")),
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

    for i in 0..elements.len() {
        if elements[i].display == "none" { continue; }
        let child_id = match node_ids[i] { Some(id) => id, None => continue };

        let (parent_nid, parent_idx): (NodeId, Option<usize>) = match elements[i].parent_index {
            Some(pidx) => {
                if pidx < elements.len() && pidx != i {
                    match node_ids[pidx] { 
                        Some(id) => (id, Some(pidx)), 
                        None => (root_node, None),
                    }
                } else { 
                    (root_node, None)
                }
            }
            None => (root_node, None),
        };

        if parent_nid != child_id {
            if let Err(e) = tree.add_child(parent_nid, child_id) {
                plog!("CAELUM", "add_child failed for element {}: {:?}", i, e);
            }
        }
        if elements[i].parent_index != parent_idx {
            elements[i].parent_index = parent_idx;
        }
    }

    if elements.len() > 0 {
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
            abs_x[i] = if lx.is_finite() { lx } else { 0.0 };
            abs_y[i] = if ly.is_finite() { ly } else { 0.0 };
            widths[i] = if lw.is_finite() && lw >= 0.0 { lw } else { el.css_width.unwrap_or(container_width) };
            heights[i] = if lh.is_finite() && lh >= 0.0 { lh } else { el.css_height.unwrap_or(0.0) };
        }
    }
    // Caelum returns positions relative to each node's parent.
    // Accumulate full parent chain to produce absolute positions for rendering.
    let n = elements.len();
    for i in 0..n {
        let mut x = abs_x[i];
        let mut y = abs_y[i];
        let mut current = elements[i].parent_index;
        while let Some(pidx) = current {
            if pidx < n && pidx != i {
                x += abs_x[pidx];
                y += abs_y[pidx];
                current = elements[pidx].parent_index;
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
        if elements[i].display == "none" || elements[i].display == "inline" { continue; }
        let inline_children: Vec<usize> = (0..elements.len())
            .filter(|j| *j != i && elements[*j].parent_index == Some(i) && elements[*j].display == "inline")
            .collect();
        if inline_children.is_empty() { continue; }
        let fs = if elements[i].font_size.is_finite() { elements[i].font_size.clamp(6.0, 200.0) } else { 16.0 };
        let char_w = fs * CHAR_W_SCALE;
        let mut cumulative = text_visual_width(&elements[i].text) as f32 * char_w;
        for &child_idx in &inline_children {
            elements[child_idx].x = elements[i].x + cumulative;
            if elements[child_idx].css_width.is_none() {
                let child_text_w = text_visual_width(&elements[child_idx].text) as f32 * char_w;
                elements[child_idx].width = child_text_w.max(fs);
            }
            if elements[child_idx].css_height.is_none() {
                elements[child_idx].height = fs * elements[i].line_height.max(1.0);
            }
            cumulative += if elements[child_idx].css_width.is_some() {
                elements[child_idx].width
            } else {
                text_visual_width(&elements[child_idx].text) as f32 * char_w
            };
        }
    }

    apply_text_wrapping(elements, container_width);

    for (i, el) in elements.iter().enumerate().take(20) {
        let tag = if el.tag.len() > 15 { &el.tag[..15] } else { &el.tag };
        let text_preview: String = el.text.chars().take(30).collect();
        plog!("POS", "[{}] tag={:15} x={:>6.0} y={:>6.0} w={:>6.0} h={:>6.0} parent={:?} text=\"{}\"",
            i, tag, el.x, el.y, el.width, el.height, el.parent_index, text_preview);
    }
}
