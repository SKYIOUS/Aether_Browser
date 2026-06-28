use iced::widget::{
    button, canvas, column, container, row, scrollable, text, text_input, Space,
};
use iced::widget::canvas::{Frame, Geometry, Image as CanvasImage, Program};
use iced::widget::image::Handle;
use iced::mouse;
use iced::{Alignment, Background, Color, Element, Length, Point, Rectangle, Size, Task};

use crate::ui::style::*;
use crate::plog;

use std::sync::{Arc, Mutex};

const CHAR_W_SCALE: f32 = 0.58;
const LINE_H_SCALE: f32 = 1.4;

fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    if max_width <= 0.0 || font_size <= 0.0 || text.is_empty() {
        return vec![text.to_string()];
    }
    let char_w = font_size * CHAR_W_SCALE;
    let max_chars = (max_width / char_w).floor() as usize;
    if max_chars < 1 { return vec![text.to_string()]; }

    let mut lines: Vec<String> = vec![];
    let mut current = String::new();
    for word in text.split_inclusive(|c: char| c.is_whitespace()) {
        let trimmed = word.trim_end_matches(|c: char| c.is_whitespace());
        let sep = &word[trimmed.len()..];
        if current.is_empty() {
            current = trimmed.to_string();
        } else {
            let candidate = format!("{} {}", current, trimmed);
            if candidate.chars().count() <= max_chars {
                current = candidate;
            } else {
                lines.push(current.clone());
                current = trimmed.to_string();
            }
        }
        if !sep.is_empty() && !current.is_empty() {
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

fn apply_text_wrapping(elements: &mut Vec<StyledElement>, container_width: f32) {
    let page_w = container_width;
    for el in elements.iter_mut() {
        let fs = if el.font_size.is_finite() { el.font_size.max(6.0).min(200.0) } else { 16.0 };
        let available = if el.width.is_finite() && el.width > 0.0 { el.width } else { page_w };
        let lines = wrap_text(&el.text, available, fs);
        el.wrapped_lines = lines;
    }
}

fn stratus_color(c: &crate::engine::stratus::Color) -> Color {
    Color::from_rgba(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0)
}

// ── Module-level helpers ─────────────────────────────────────────────

fn extract_styles(node: &crate::engine::dom::Node, styles: &mut Vec<String>) {
    match &node.node_type {
        NodeType::Element(elem) => {
            if elem.tag_name.to_lowercase() == "style" {
                for child in &node.children {
                    if let NodeType::Text(text) = &child.node_type {
                        styles.push(text.clone());
                    }
                }
            }
            for child in &node.children {
                extract_styles(child, styles);
            }
        }
        _ => {}
    }
}

fn extract_links(node: &crate::engine::dom::Node, links: &mut Vec<String>) {
    match &node.node_type {
        NodeType::Element(elem) => {
            if elem.tag_name.to_lowercase() == "link" {
                if let Some(rel) = elem.attributes.get("rel") {
                    if rel.contains("stylesheet") {
                        if let Some(href) = elem.attributes.get("href") {
                            links.push(href.clone());
                        }
                    }
                }
            }
            for child in &node.children {
                extract_links(child, links);
            }
        }
        _ => {}
    }
}

fn should_skip_tag(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "meta" | "link" | "head" | "title" | "svg" | "path" | "br" | "hr" | "input" | "button" | "iframe" | "textarea" | "select" | "option" | "form" | "template")
}

fn should_skip_content(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "template" | "svg" | "title")
}

fn get_all_text(node: &crate::engine::dom::Node) -> String {
    match &node.node_type {
        NodeType::Text(t) => t.trim().to_string(),
        NodeType::Element(_) => {
            node.children.iter()
                .map(get_all_text)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        }
        NodeType::Document | NodeType::Comment(_) => String::new(),
    }
}

#[derive(Debug, Clone)]
struct FullStyle {
    color: Color,
    font_size: f32,
    font_weight: String,
    background_color: Option<Color>,
    margin_top: f32,
    margin_right: Option<f32>,
    margin_bottom: f32,
    margin_left: Option<f32>,
    padding: [f32; 4],
    border_widths: [f32; 4],
    border_color: Option<Color>,
    display: String,
    flex_direction: String,
    flex_wrap: String,
    justify_content: String,
    align_items: String,
    align_self: String,
    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: Option<f32>,
    css_width: Option<f32>,
    css_height: Option<f32>,
    min_width: Option<f32>,
    max_width: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    text_align: Option<String>,
}

fn is_html_block_tag(tag: &str) -> bool {
    matches!(tag, "html" | "body" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
        | "p" | "ul" | "ol" | "li" | "table" | "form" | "section" | "article"
        | "nav" | "header" | "footer" | "main" | "aside" | "blockquote"
        | "figure" | "figcaption" | "details" | "summary" | "dialog"
        | "dd" | "dt" | "dl" | "pre" | "hr" | "fieldset" | "legend" | "address" | "canvas")
}

fn compute_full_style(node: &crate::engine::dom::Node, ss: &crate::engine::css::Stylesheet) -> FullStyle {
    let tag = match &node.node_type {
        crate::engine::dom::NodeType::Element(e) => e.tag_name.to_lowercase(),
        _ => String::new(),
    };
    let cs = crate::engine::css::compute_style(node, ss);
    let color = cs.color.as_ref().map(stratus_color).unwrap_or(C::PAGE_TEXT);
    let font_size = cs.font_size.filter(|v| v.is_finite()).map(|v| v.max(6.0).min(200.0)).unwrap_or(16.0);
    let font_weight = cs.font_weight.unwrap_or_else(|| "normal".to_string());
    let background_color = cs.background_color.as_ref().map(stratus_color);

    let margin_top = cs.margin_top.filter(|v| v.is_finite()).map(|v| v.max(0.0)).unwrap_or(0.0);
    let margin_right = cs.margin_right.filter(|v| v.is_finite()).map(|v| v.max(0.0));
    let margin_bottom = cs.margin_bottom.filter(|v| v.is_finite()).map(|v| v.max(0.0)).unwrap_or(0.0);
    let margin_left = cs.margin_left.filter(|v| v.is_finite()).map(|v| v.max(0.0));

    let pt = cs.padding_top.unwrap_or(0.0).max(0.0);
    let pr = cs.padding_right.unwrap_or(0.0).max(0.0);
    let pb = cs.padding_bottom.unwrap_or(0.0).max(0.0);
    let pl = cs.padding_left.unwrap_or(0.0).max(0.0);

    let bt = cs.border_top_width.filter(|v| v.is_finite()).unwrap_or(0.0).max(0.0);
    let br = cs.border_right_width.filter(|v| v.is_finite()).unwrap_or(0.0).max(0.0);
    let bb = cs.border_bottom_width.filter(|v| v.is_finite()).unwrap_or(0.0).max(0.0);
    let bl = cs.border_left_width.filter(|v| v.is_finite()).unwrap_or(0.0).max(0.0);
    let border_color = cs.border_top_color.or(cs.border_right_color).or(cs.border_bottom_color).or(cs.border_left_color).map(|c| stratus_color(&c));

    let mut display = match cs.display {
        crate::engine::stratus::Display::Block => "block",
        crate::engine::stratus::Display::Flex | crate::engine::stratus::Display::InlineFlex => "flex",
        crate::engine::stratus::Display::Inline | crate::engine::stratus::Display::InlineBlock => "inline",
        crate::engine::stratus::Display::Grid => "grid",
        crate::engine::stratus::Display::None => "none",
    }.to_string();

    // HTML UA default: block-level elements default to display:block
    if display == "inline" && is_html_block_tag(&tag) {
        display = "block".to_string();
    }

    let flex_direction = match cs.flex.flex_direction {
        crate::engine::stratus::FlexDirection::Row => "row",
        crate::engine::stratus::FlexDirection::RowReverse => "row-reverse",
        crate::engine::stratus::FlexDirection::Column => "column",
        crate::engine::stratus::FlexDirection::ColumnReverse => "column-reverse",
    }.to_string();

    let flex_wrap = match cs.flex.flex_wrap {
        crate::engine::stratus::FlexWrap::NoWrap => "nowrap",
        crate::engine::stratus::FlexWrap::Wrap => "wrap",
        crate::engine::stratus::FlexWrap::WrapReverse => "wrap-reverse",
    }.to_string();

    let justify_content = match cs.flex.justify_content {
        crate::engine::stratus::JustifyContent::FlexStart => "flex-start",
        crate::engine::stratus::JustifyContent::FlexEnd => "flex-end",
        crate::engine::stratus::JustifyContent::Center => "center",
        crate::engine::stratus::JustifyContent::SpaceBetween => "space-between",
        crate::engine::stratus::JustifyContent::SpaceAround => "space-around",
        crate::engine::stratus::JustifyContent::SpaceEvenly => "space-evenly",
    }.to_string();

    let align_items = match cs.flex.align_items {
        crate::engine::stratus::AlignItems::Stretch => "stretch",
        crate::engine::stratus::AlignItems::FlexStart => "flex-start",
        crate::engine::stratus::AlignItems::FlexEnd => "flex-end",
        crate::engine::stratus::AlignItems::Center => "center",
        crate::engine::stratus::AlignItems::Baseline => "baseline",
    }.to_string();

    let align_self = match cs.flex.align_self {
        crate::engine::stratus::AlignSelf::Auto => "auto",
        crate::engine::stratus::AlignSelf::Stretch => "stretch",
        crate::engine::stratus::AlignSelf::FlexStart => "flex-start",
        crate::engine::stratus::AlignSelf::FlexEnd => "flex-end",
        crate::engine::stratus::AlignSelf::Center => "center",
        crate::engine::stratus::AlignSelf::Baseline => "baseline",
    }.to_string();

    FullStyle {
        color, font_size, font_weight, background_color,
        margin_top, margin_right, margin_bottom, margin_left,
        padding: [pt, pr, pb, pl],
        border_widths: [bt, br, bb, bl],
        border_color,
        display, flex_direction, flex_wrap,
        justify_content, align_items, align_self,
        flex_grow: cs.flex.flex_grow,
        flex_shrink: cs.flex.flex_shrink,
        flex_basis: cs.flex.flex_basis,
        css_width: cs.width.filter(|v| v.is_finite()),
        css_height: cs.height.filter(|v| v.is_finite()),
        min_width: cs.min_width.filter(|v| v.is_finite()),
        max_width: cs.max_width.filter(|v| v.is_finite()),
        min_height: cs.min_height.filter(|v| v.is_finite()),
        max_height: cs.max_height.filter(|v| v.is_finite()),
        text_align: cs.text_align,
    }
}

enum ScriptSource {
    Inline(String),
    External(String),
}

fn extract_scripts(node: &crate::engine::dom::Node, scripts: &mut Vec<ScriptSource>) {
    if let NodeType::Element(elem) = &node.node_type {
        let tag = elem.tag_name.to_lowercase();
        if tag == "script" {
            let src = elem.attributes.get("src").cloned();
            if let Some(url) = src {
                if !url.is_empty() {
                    scripts.push(ScriptSource::External(url));
                    return;
                }
            }
            let text: String = node.children.iter()
                .filter_map(|c| {
                    if let NodeType::Text(t) = &c.node_type {
                        let s = t.trim().to_string();
                        if !s.is_empty() { Some(s) } else { None }
                    } else { None }
                })
                .collect::<Vec<_>>()
                .join("\n");
            if !text.is_empty() {
                scripts.push(ScriptSource::Inline(text));
            }
            return;
        }
    }
    for child in &node.children {
        extract_scripts(child, scripts);
    }
}

fn make_element(tag: &str, text: String, fs: &FullStyle, parent_idx: Option<usize>) -> StyledElement {
    StyledElement {
        tag: tag.to_string(), text, wrapped_lines: vec![],
        is_link: false, href: None, indent_level: 0,
        color: fs.color, font_size: fs.font_size, font_weight: fs.font_weight.clone(),
        background_color: fs.background_color,
        border_widths: fs.border_widths, border_color: fs.border_color,
        image_handle: None, image_url: None,
        margin_top: fs.margin_top, margin_bottom: fs.margin_bottom,
        margin_left: fs.margin_left, margin_right: fs.margin_right,
        padding: fs.padding,
        display: fs.display.clone(), flex_direction: fs.flex_direction.clone(),
        flex_wrap: fs.flex_wrap.clone(), justify_content: fs.justify_content.clone(),
        align_items: fs.align_items.clone(),
        flex_grow: fs.flex_grow, flex_shrink: fs.flex_shrink, flex_basis: fs.flex_basis,
        css_width: fs.css_width, css_height: fs.css_height,
        min_width: fs.min_width, max_width: fs.max_width,
        min_height: fs.min_height, max_height: fs.max_height,
        parent_index: parent_idx,
        x: 0.0, y: 0.0, width: 0.0, height: 0.0,
    }
}

fn extract_elements(
    node: &crate::engine::dom::Node,
    elements: &mut Vec<StyledElement>,
    depth: usize,
    ss: &crate::engine::css::Stylesheet,
    parent_style: Option<FullStyle>,
    parent_idx: Option<usize>,
) {
    if depth > 30 || elements.len() >= 300 { return; }

    match &node.node_type {
        NodeType::Document | NodeType::Comment(_) => { return; }
        NodeType::Text(text) => {
            let txt = text.trim();
            if !txt.is_empty() && txt.len() < 1000 && !txt.chars().all(|c| c.is_whitespace()) {
                if let Some(ref ps) = parent_style {
                    let mut el = make_element("text", txt.to_string(), ps, parent_idx);
                    el.background_color = None;
                    elements.push(el);
                } else {
                    let fs = compute_full_style(node, ss);
                    let mut el = make_element("text", txt.to_string(), &fs, parent_idx);
                    el.background_color = None;
                    elements.push(el);
                }
            }
        }
        NodeType::Element(elem) => {
            let tag = elem.tag_name.to_lowercase();
            if should_skip_tag(&tag) {
                if !should_skip_content(&tag) && tag != "head" && tag != "meta" && tag != "link" {
                    for child in &node.children {
                        extract_elements(child, elements, depth + 1, ss, parent_style.clone(), parent_idx);
                    }
                }
                return;
            }

            let fs = compute_full_style(node, ss);

            let uses_default_margins = matches!(tag.as_str(), "a" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "li" | "img");

            let mut text_consumed = false;

            let (text_content, is_link, href, indent, tag_override, skip_element, recurse_into_children) = match tag.as_str() {
                "a" => {
                    let href = elem.attributes.get("href").cloned();
                    let text = get_all_text(node);
                    if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                    else { text_consumed = true; (text, true, href, 0, "a", false, false) }
                }
                "h1" => {
                    let text = get_all_text(node);
                    if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                    else { text_consumed = true; (text, false, None, 0, "h1", false, false) }
                }
                "h2" => {
                    let text = get_all_text(node);
                    if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                    else { text_consumed = true; (text, false, None, 0, "h2", false, false) }
                }
                "h3" | "h4" | "h5" | "h6" => {
                    let text = get_all_text(node);
                    if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                    else { text_consumed = true; (text, false, None, 1, tag.as_str(), false, false) }
                }
                "p" => {
                    let direct_text: String = node.children.iter()
                        .filter_map(|c| {
                            if let NodeType::Text(t) = &c.node_type {
                                let txt = t.trim().to_string();
                                if !txt.is_empty() { Some(txt) } else { None }
                            } else { None }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    if direct_text.is_empty() { (String::new(), false, None, 0, "", true, true) }
                    else { text_consumed = true; (direct_text, false, None, 0, "p", false, true) }
                }
                "li" => {
                    let has_link = node.children.iter().any(|c| {
                        if let NodeType::Element(e) = &c.node_type {
                            e.tag_name.to_lowercase() == "a"
                        } else { false }
                    });
                    if has_link { (String::new(), false, None, 0, "", true, false) }
                    else {
                        let text = get_all_text(node);
                        if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                        else { text_consumed = true; (format!("• {}", text), false, None, 1, "li", false, false) }
                    }
                }
                "img" => {
                    let alt = elem.attributes.get("alt").cloned().unwrap_or_default();
                    (alt, false, None, 0, "img", false, false)
                }
                _ => {
                    if fs.display == "inline" && tag != "span" {
                        (String::new(), false, None, 0, "", true, true)
                    } else {
                        (String::new(), false, None, 0, tag.as_str(), false, false)
                    }
                }
            };

            if !recurse_into_children && skip_element {
                return;
            }

            let this_idx = if !skip_element {
                let mut el = make_element(tag_override, text_content, &fs, parent_idx);
                el.is_link = is_link;
                el.href = href;
                el.indent_level = indent;
                if tag == "img" {
                    el.image_url = elem.attributes.get("src").cloned().filter(|s| !s.is_empty());
                    if el.css_width.is_none() { el.css_width = Some(200.0); }
                    if el.css_height.is_none() { el.css_height = Some(150.0); }
                }
                if uses_default_margins {
                    let def_mt: f32 = match tag.as_str() {
                        "h1" => 24.0, "h2" => 20.0, "h3" | "h4" | "h5" | "h6" => 16.0,
                        "p" => 12.0, "li" => 8.0, "a" => 4.0, "img" => 4.0, _ => 0.0,
                    };
                    if el.margin_top == 0.0 { el.margin_top = def_mt; }
                }
                let idx = elements.len();
                elements.push(el);
                Some(idx)
            } else {
                parent_idx
            };

            // Determine parent for children and which children to recurse into
            let (new_parent, skip_fn): (Option<usize>, Box<dyn Fn(&crate::engine::dom::Node) -> bool>) = match tag.as_str() {
                "img" => (parent_idx, Box::new(|_| true)), // no children
                "a" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    // Text already captured via get_all_text, skip all children
                    if text_consumed {
                        (parent_idx, Box::new(|_| true))
                    } else {
                        (parent_idx, Box::new(|_| false))
                    }
                }
                "li" => {
                    // get_all_text already captured children
                    if text_consumed {
                        (parent_idx, Box::new(|_| true))
                    } else {
                        (parent_idx, Box::new(|_| false))
                    }
                }
                "p" => {
                    let new_p = this_idx.or(parent_idx);
                    (new_p, Box::new(|c: &crate::engine::dom::Node| matches!(c.node_type, NodeType::Text(_))))
                }
                _ => {
                    if skip_element {
                        (parent_idx, Box::new(|_| false))
                    } else {
                        (this_idx.or(parent_idx), Box::new(|_| false))
                    }
                }
            };

            for child in &node.children {
                if skip_fn(child) { continue; }
                extract_elements(child, elements, depth + 1, ss, Some(fs.clone()), new_parent);
            }
        }
    }
}

fn apply_caelum_layout(elements: &mut Vec<StyledElement>, container_width: f32) {
    if elements.is_empty() { return; }
    use crate::engine::caelum::prelude::*;

    fn str_display_to_caelum(d: &str) -> Display {
        match d {
            "flex" => Display::Flex,
            "block" => Display::Block,
            "inline" | "grid" => Display::Block,
            "none" => Display::None,
            _ => Display::Block,
        }
    }

    fn str_flex_dir(d: &str) -> FlexDirection {
        match d {
            "row" => FlexDirection::Row,
            "row-reverse" => FlexDirection::RowReverse,
            "column" => FlexDirection::Column,
            "column-reverse" => FlexDirection::ColumnReverse,
            _ => FlexDirection::Row,
        }
    }

    fn str_flex_wrap(w: &str) -> FlexWrap {
        match w {
            "nowrap" => FlexWrap::NoWrap,
            "wrap" => FlexWrap::Wrap,
            "wrap-reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        }
    }

    fn str_justify(j: &str) -> JustifyContent {
        match j {
            "flex-start" => JustifyContent::FlexStart,
            "flex-end" => JustifyContent::FlexEnd,
            "center" => JustifyContent::Center,
            "space-between" => JustifyContent::SpaceBetween,
            "space-around" => JustifyContent::SpaceAround,
            "space-evenly" => JustifyContent::SpaceEvenly,
            _ => JustifyContent::FlexStart,
        }
    }

    fn str_align_items(a: &str) -> AlignItems {
        match a {
            "stretch" => AlignItems::Stretch,
            "flex-start" => AlignItems::FlexStart,
            "flex-end" => AlignItems::FlexEnd,
            "center" => AlignItems::Center,
            "baseline" => AlignItems::Baseline,
            _ => AlignItems::Stretch,
        }
    }

    fn dim(v: Option<f32>) -> Dimension {
        v.map(Dimension::from_length).unwrap_or(Dimension::auto())
    }

    fn dim_min_max(min_v: Option<f32>, max_v: Option<f32>) -> (Dimension, Dimension) {
        let min = min_v.map(Dimension::from_length).unwrap_or(Dimension::auto());
        let max = max_v.map(Dimension::from_length).unwrap_or(Dimension::auto());
        (min, max)
    }

    let mut tree: CaelumTree = CaelumTree::new();

    let root_style = Style {
        display: Display::Block,
        size: Size { width: Dimension::from_length(container_width), height: Dimension::auto() },
        ..Default::default()
    };
    let root_node = tree.new_leaf(root_style).unwrap();

    let mut node_ids: Vec<Option<NodeId>> = vec![None; elements.len()];

    // First pass: create Caelum nodes for all elements
    for (i, el) in elements.iter().enumerate() {
        if el.display == "none" { continue; }

        let caelum_display = str_display_to_caelum(&el.display);

        let auto_margin = |v: Option<f32>| v.map(LengthPercentageAuto::length).unwrap_or(LengthPercentageAuto::auto());
        let mut style = Style {
            display: caelum_display,
            margin: Rect {
                top: LengthPercentageAuto::length(el.margin_top),
                right: auto_margin(el.margin_right),
                bottom: LengthPercentageAuto::length(el.margin_bottom),
                left: auto_margin(el.margin_left),
            },
            padding: Rect {
                top: LengthPercentage::length(el.padding[0]),
                right: LengthPercentage::length(el.padding[1]),
                bottom: LengthPercentage::length(el.padding[2]),
                left: LengthPercentage::length(el.padding[3]),
            },
            border: Rect {
                top: LengthPercentage::length(el.border_widths[0]),
                right: LengthPercentage::length(el.border_widths[1]),
                bottom: LengthPercentage::length(el.border_widths[2]),
                left: LengthPercentage::length(el.border_widths[3]),
            },
            size: Size { width: dim(el.css_width), height: dim(el.css_height) },
            ..Default::default()
        };

        let (min_w, max_w) = dim_min_max(el.min_width, el.max_width);
        let (min_h, max_h) = dim_min_max(el.min_height, el.max_height);
        style.min_size = Size { width: min_w, height: min_h };
        if el.min_height.is_some() {
            eprintln!("[CAE] elem[{}] min_h={:.0} → {:?}", i, el.min_height.unwrap(), min_h);
        }
        style.max_size = Size { width: max_w, height: max_h };

        if caelum_display == Display::Flex {
            style.flex_direction = str_flex_dir(&el.flex_direction);
            style.flex_wrap = str_flex_wrap(&el.flex_wrap);
            style.justify_content = Some(str_justify(&el.justify_content));
            style.align_items = Some(str_align_items(&el.align_items));
        }

        style.flex_grow = el.flex_grow;
        style.flex_shrink = el.flex_shrink;
        if let Some(basis) = el.flex_basis {
            style.flex_basis = Dimension::from_length(basis);
        }

        match tree.new_leaf(style) {
            Ok(nid) => node_ids[i] = Some(nid),
            _ => {}
        }
    }

    // Second pass: build parent-child tree
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

        let _ = tree.add_child(parent_nid, child_id);
    }

    if elements.len() > 1 {
        let viewport_h = 6000.0;
        let _ = tree.compute_layout(root_node, Size {
            width: AvailableSpace::Definite(container_width),
            height: AvailableSpace::Definite(viewport_h),
        });
        plog!("CAELUM", "Tree layout computed ({} nodes, viewport_h={})", node_ids.len(), viewport_h);
    }

    // Read Caelum positions (relative to parent, X is used for centering, Y is overridden)
    let mut abs_x: Vec<f32> = vec![0.0; elements.len()];
    let mut widths: Vec<f32> = vec![0.0; elements.len()];
    let mut heights: Vec<f32> = vec![0.0; elements.len()];
    for (i, el) in elements.iter().enumerate() {
        let nid = match node_ids[i] { Some(id) => id, None => continue };
        if let Ok(layout) = tree.layout(nid) {
            let lx = layout.location.x;
            let lw = layout.size.width;
            let lh = layout.size.height;
            abs_x[i] = if lx.is_finite() { lx.max(0.0) } else { 0.0 };
            widths[i] = if lw.is_finite() && lw > 0.0 { lw } else { el.css_width.unwrap_or(container_width) };
            heights[i] = if lh.is_finite() && lh > 0.0 { lh } else { el.css_height.unwrap_or(0.0) };
        }
    }
    // Accumulate parent X positions
    for i in 0..elements.len() {
        if let Some(pidx) = elements[i].parent_index {
            if pidx < elements.len() {
                abs_x[i] += abs_x[pidx];
            }
        }
    }
    // Write back X from Caelum; Y will be computed by vertical flow below
    for (i, el) in elements.iter_mut().enumerate() {
        el.x = abs_x[i];
        el.width = widths[i];
        el.height = heights[i];
    }
    // Override widths for inline elements (a, span, etc.) based on text content
    for el in elements.iter_mut() {
        if el.display == "inline" && !el.text.is_empty() {
            let fs = el.font_size.max(6.0).min(200.0);
            let text_w = el.text.chars().count() as f32 * fs * CHAR_W_SCALE;
            if text_w > 0.0 {
                el.width = text_w.min(container_width);
            }
        }
    }

    // ── Manual vertical flow with Inline Formatting Context ──
    const LHS: f32 = 1.4;
    let n = elements.len();

    // Helper: pack consecutive inline children into horizontal line boxes.
    // Returns total height consumed (y-bottom relative to origin_y).
    fn layout_inline_run(
        run: &[usize],
        elements: &mut [StyledElement],
        h: &[f32],
        origin_x: f32,
        origin_y: f32,
        max_width: f32,
    ) -> f32 {
        if run.is_empty() { return 0.0; }
        let right_edge = origin_x + max_width;
        let mut cx = origin_x;
        let mut cy = origin_y;
        let mut line_h = 0.0f32;
        for &ci in run {
            let ml = elements[ci].margin_left.unwrap_or(0.0);
            let mr = elements[ci].margin_right.unwrap_or(0.0);
            let total_w = elements[ci].width + ml + mr;
            let item_h = if elements[ci].display == "inline-block" {
                h[ci] + elements[ci].margin_top + elements[ci].margin_bottom
            } else {
                h[ci]
            };
            if cx + total_w > right_edge && cx > origin_x {
                cx = origin_x;
                cy += line_h;
                line_h = 0.0;
            }
            elements[ci].x = cx + ml;
            elements[ci].y = cy;
            cx += total_w;
            if item_h > line_h { line_h = item_h; }
        }
        (cy + line_h) - origin_y
    }

    // Bottom-up pass: compute heights
    let mut h = vec![0.0; n];
    let mut is_inline = vec![false; n];
    for i in (0..n).rev() {
        let own_height = if elements[i].height > 0.0 {
            elements[i].height
        } else if !elements[i].text.is_empty() {
            let fs = elements[i].font_size.max(6.0).min(200.0);
            fs * LHS
        } else {
            0.0
        };
        if own_height > h[i] { h[i] = own_height; }
        let inl = elements[i].display == "inline" || elements[i].display == "inline-block";
        is_inline[i] = inl;
        // Skip accumulation only when child is inline AND parent is also inline
        let skip_parent_acc = inl && elements[i].parent_index
            .and_then(|pidx| if pidx < n { elements.get(pidx) } else { None })
            .map(|p| p.display == "inline" || p.display == "inline-block")
            .unwrap_or(false);
        if !skip_parent_acc {
            if let Some(pidx) = elements[i].parent_index {
                if pidx < n {
                    h[pidx] += h[i] + elements[i].margin_top + elements[i].margin_bottom;
                }
            }
        }
    }

    // Build children-per-parent lists (document-order traversal)
    let mut children_of: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        if let Some(pidx) = elements[i].parent_index {
            if pidx < n {
                children_of[pidx].push(i);
            }
        }
    }

    // Top-down pass: assign Y positions with IFC line-box packing
    let mut used = vec![0.0; n];
    for pidx in 0..n {
        let ch = &children_of[pidx];
        if ch.is_empty() { continue; }
        let origin_x = elements[pidx].x + elements[pidx].padding[3];
        let max_w = (elements[pidx].width - elements[pidx].padding[1] - elements[pidx].padding[3]).max(1.0);

        let mut i = 0;
        while i < ch.len() {
            let ci = ch[i];
            if is_inline[ci] {
                let run_start = i;
                let mut run_end = i + 1;
                while run_end < ch.len() && is_inline[ch[run_end]] {
                    run_end += 1;
                }
                let total_h = layout_inline_run(
                    &ch[run_start..run_end], elements, &h,
                    origin_x, used[pidx], max_w,
                );
                used[pidx] += total_h;
                i = run_end;
            } else if elements[ci].display != "none" {
                elements[ci].y = used[pidx] + elements[ci].margin_top;
                used[pidx] += h[ci] + elements[ci].margin_top + elements[ci].margin_bottom;
                i += 1;
            } else {
                i += 1;
            }
        }
    }

    // Accumulate parent Y offsets (content area top, including padding)
    for i in 0..n {
        if let Some(pidx) = elements[i].parent_index {
            if pidx < n {
                elements[i].y += elements[pidx].y + elements[pidx].padding[0];
            }
        }
    }
    // Apply computed heights — prefer manual flow over Caelum
    for (i, el) in elements.iter_mut().enumerate() {
        if h[i] > 0.0 {
            el.height = h[i];
        }
    }
    // Expand parents to contain all children (including inline runs)
    for i in 0..n {
        if let Some(pidx) = elements[i].parent_index {
            if pidx < n {
                let child_end = elements[i].y + h[i];
                let parent_end = elements[pidx].y + elements[pidx].height;
                if child_end > parent_end {
                    elements[pidx].height = child_end - elements[pidx].y;
                }
            }
        }
    }

    // Log final positions for first 20 elements
    for (i, el) in elements.iter().enumerate().take(20) {
        let tag = if el.tag.len() > 15 { &el.tag[..15] } else { &el.tag };
        let text_preview: String = el.text.chars().take(30).collect();
        plog!("POS", "[{}] tag={:15} x={:>6.0} y={:>6.0} w={:>6.0} h={:>6.0} parent={:?} text=\"{}\"",
            i, tag, el.x, el.y, el.width, el.height, el.parent_index, text_preview);
    }
}

fn inject_js_output(dom: &mut crate::engine::dom::Node, text: &str) {
    if text.is_empty() { return; }
    fn is_body(node: &crate::engine::dom::Node) -> bool {
        matches!(&node.node_type, crate::engine::dom::NodeType::Element(data) if data.tag_name.to_lowercase() == "body")
    }
    fn find_body_mut<'a>(node: &'a mut crate::engine::dom::Node) -> Option<&'a mut crate::engine::dom::Node> {
        if is_body(node) { return Some(node); }
        for child in &mut node.children {
            if let Some(found) = find_body_mut(child) {
                return Some(found);
            }
        }
        None
    }
    if let Some(body) = find_body_mut(dom) {
        body.children.push(crate::engine::dom::Node::new_text(text.to_string()));
    } else {
        dom.children.push(crate::engine::dom::Node::new_text(text.to_string()));
    }
}

use crate::engine::js::{JsBridge, JSEngine};

async fn fetch_page_content(url: String) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    use crate::engine::net::fetch;
    use crate::engine::parser::Parser;

    plog!("FETCH", "URL={}", url);
    let html = fetch(&url);
    plog!("FETCH", "Status=OK len={}", html.len());

    if html.starts_with("Error") || html.is_empty() {
        plog!("FETCH", "Error or empty response");
        return (url, vec![], None);
    }

    let max_html = 500_000;
    let html = if html.len() > max_html {
        plog!("FETCH", "Truncated from {} to {}", html.len(), max_html);
        html[..max_html].to_string()
    } else {
        html
    };

    let mut parser = Parser::new(html);
    let mut dom_node = parser.parse_node();
    plog!("PARSE", "DOM root has {} children", dom_node.children.len());

    let mut styles = Vec::new();
    extract_styles(&dom_node, &mut styles);
    plog!("STYLE", "Found {} style blocks", styles.len());

    let mut stylesheet = crate::engine::css::Stylesheet { rules: Vec::new() };
    let style_limit = styles.len().min(20);
    plog!("STYLE", "Processing up to {} style blocks", style_limit);
    for (si, style_content) in styles.iter().take(style_limit).enumerate() {
        let max_css_len = 100_000;
        let trimmed = if style_content.len() > max_css_len {
            plog!("CSS", "Truncated inline style {} from {} to {}", si, style_content.len(), max_css_len);
            &style_content[..max_css_len]
        } else {
            style_content.as_str()
        };
        let mut parser = crate::engine::css::Parser::new(trimmed.to_string());
        stylesheet.rules.extend(parser.parse_rules());
    }
    plog!("CSS", "Parsed {} rules from inline styles", stylesheet.rules.len());

    let mut link_urls = Vec::new();
    extract_links(&dom_node, &mut link_urls);
    let link_limit = link_urls.len().min(20);
    plog!("CSS", "Found {} external CSS links, processing {}", link_urls.len(), link_limit);
    for link_url in link_urls.iter().take(link_limit) {
        let resolved = crate::engine::net::resolve_url(link_url, &url);
        plog!("CSS", "Fetching external CSS from {}", resolved);
        let css_content = crate::engine::net::fetch(&resolved);
        if !css_content.starts_with("Error") && !css_content.is_empty() {
            let max_css_len = 100_000;
            let trimmed = if css_content.len() > max_css_len {
                plog!("CSS", "Truncated external CSS from {} to {}", css_content.len(), max_css_len);
                css_content[..max_css_len].to_string()
            } else {
                css_content
            };
            let mut parser = crate::engine::css::Parser::new(trimmed);
            let rules = parser.parse_rules();
            let count = rules.len();
            stylesheet.rules.extend(rules);
            plog!("CSS", "Parsed {} rules from external CSS", count);
        }
    }
    plog!("CSS", "Total stylesheet rules: {}", stylesheet.rules.len());

    let mut scripts = Vec::new();
    extract_scripts(&dom_node, &mut scripts);
    plog!("JS", "Found {} script blocks", scripts.len());
    let bridge = Arc::new(Mutex::new(JsBridge::load_dom(&dom_node, &url)));
    for (si, script) in scripts.iter().enumerate() {
        let code = match script {
            ScriptSource::Inline(s) => {
                plog!("JS", "Executing inline script {}", si);
                s.clone()
            }
            ScriptSource::External(src) => {
                let resolved = crate::engine::net::resolve_url(src, &url);
                plog!("JS", "Fetching external script from {}", resolved);
                let fetched = crate::engine::net::fetch(&resolved);
                if fetched.starts_with("Error") || fetched.is_empty() { continue; }
                fetched
            }
        };
        let mut js = JSEngine::new();
        let _ = js.execute_with_bridge(&code, &bridge);
    }
    let (modified_dom, js_output) = {
        let mut guard = bridge.lock().unwrap();
        let dom = guard.to_dom();
        let output = guard.take_output();
        (dom, output)
    };
    dom_node = modified_dom;
    if !js_output.is_empty() {
        plog!("JS", "Injecting JS output ({} chars)", js_output.len());
        inject_js_output(&mut dom_node, &js_output);
    }

    let mut elements = Vec::new();
    extract_elements(&dom_node, &mut elements, 0, &stylesheet, None, None);
    plog!("EXTRACT", "Extracted {} elements", elements.len());
    elements.truncate(300);

    let mut img_count = 0;
    for el in elements.iter_mut() {
        if let Some(ref img_src) = el.image_url.clone() {
            img_count += 1;
            let resolved = crate::engine::net::resolve_url(img_src, &url);
            let bytes = crate::engine::net::fetch_bytes(&resolved);
            if !bytes.is_empty() && bytes.len() < 5_000_000 {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    let rgba = img.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    let max_dim = 800.0;
                    let scale = if (w as f32).max(h as f32) > max_dim {
                        max_dim / (w as f32).max(h as f32)
                    } else {
                        1.0
                    };
                    if scale < 1.0 {
                        let resized = image::imageops::resize(&rgba, (w as f32 * scale) as u32, (h as f32 * scale) as u32, image::imageops::FilterType::Lanczos3);
                        let (rw, rh) = resized.dimensions();
                        el.width = rw as f32;
                        el.height = rh as f32;
                        el.image_handle = Some(Handle::from_rgba(rw, rh, resized.into_raw()));
                    } else {
                        el.width = w as f32;
                        el.height = h as f32;
                        el.image_handle = Some(Handle::from_rgba(w, h, rgba.into_raw()));
                    }
                }
            }
        }
    }
    plog!("IMAGES", "Loaded {} images", img_count);

    apply_caelum_layout(&mut elements, 800.0);
    plog!("CAELUM", "Layout computed for {} elements", elements.len());

    apply_text_wrapping(&mut elements, 800.0);
    plog!("FINAL", "Done. URL={} elements={}", url, elements.len());

    (url, elements, Some(bridge))
}

// ── Messages ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    UrlChanged(String),
    UrlSubmit,
    NavBack,
    NavForward,
    Refresh,
    WorkspaceSelected(usize),
    OpenSettings,
    OpenPalette,
    LinkClicked(String),
    PageLoaded(String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>),
    TimerTick,
    ElementClicked(usize),
    CanvasClick(Point),
    TogglePrivate,
    ToggleDevTools,
    None,
}

// ── Workspace / Collection data ───────────────────────────────────────────────

const WORKSPACES: [(&str, &str); 3] = [
    ("⬡", "Design Studio"),
    ("⬡", "Research Lab"),
    ("⬡", "Deep Work"),
];

const COLLECTIONS: [(&str, &str); 2] = [
    ("▤", "Aether UI"),
    ("▤", "Rust / Iced Docs"),
];

use crate::engine::dom::NodeType;

fn normalize_nav_url(url: &str) -> String {
    let s = url.trim();
    if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("aether://") || s.starts_with("about:") {
        s.to_string()
    } else if s.starts_with("//") {
        let stripped = s.trim_start_matches('/');
        format!("https://{}", stripped)
    } else {
        format!("https://{}", s)
    }
}

// ── State ─────────────────────────────────────────────────────────────────────

pub struct BrowserScreen {
    pub url: String,
    pub active_workspace: usize,
    pub content: String,
    pub styled_elements: Vec<StyledElement>,
    pub loading: bool,
    pub bridge: Option<Arc<Mutex<JsBridge>>>,
    pub js_engine: Option<JSEngine>,
    pub private_mode: bool,
    pub devtools_open: bool,
    history: Vec<String>,
    history_index: usize,
    is_history_nav: bool,
}

#[derive(Debug, Clone)]
pub struct StyledElement {
    pub tag: String,
    pub text: String,
    pub wrapped_lines: Vec<String>,
    pub is_link: bool,
    pub href: Option<String>,
    pub indent_level: usize,
    pub color: Color,
    pub font_size: f32,
    pub font_weight: String,
    pub background_color: Option<Color>,
    pub border_widths: [f32; 4],
    pub border_color: Option<Color>,
    pub image_handle: Option<Handle>,
    pub image_url: Option<String>,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub padding: [f32; 4],
    pub display: String,
    pub flex_direction: String,
    pub flex_wrap: String,
    pub justify_content: String,
    pub align_items: String,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub css_width: Option<f32>,
    pub css_height: Option<f32>,
    pub parent_index: Option<usize>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

struct PageCanvas<'a> {
    elements: &'a [StyledElement],
}

impl Program<BrowserMessage> for PageCanvas<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        plog!("DRAW", "Rendering {} elements into {:?}", self.elements.len(), bounds.size());
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(Point::new(0.0, 0.0), bounds.size(), iced::Color::WHITE);
        for el in self.elements {
            if let Some(ref handle) = el.image_handle {
                let iw = if el.width.is_finite() { el.width.max(50.0) } else { 50.0 };
                let ih = if el.height.is_finite() { el.height.max(50.0) } else { 50.0 };
                let ix = el.x.max(0.0);
                let iy = el.y.max(0.0);
                if ix.is_finite() && iy.is_finite() && iw.is_finite() && ih.is_finite() {
                    frame.draw_image(Rectangle::new(Point::new(ix, iy), Size::new(iw, ih)), CanvasImage::new(handle.clone()));
                }
                continue;
            }

            let bg = el.background_color;
            let bw = el.border_widths;
            let bc = el.border_color;
            let ex = if el.x.is_finite() { el.x.max(0.0) } else { 0.0 };
            let ey = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
            let ew = if el.width.is_finite() { el.width.max(1.0) } else { 1.0 };
            let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { let f = if el.font_size.is_finite() { el.font_size.max(6.0).min(200.0) } else { 16.0 }; f * 1.4 };

            if bg.is_some() || bc.is_some() {
                let fill = bg.unwrap_or(iced::Color::TRANSPARENT);
                frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, eh), fill);
            }

            if let Some(color) = bc {
                if bw[0] > 0.0 {
                    frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, bw[0]), color);
                }
                if bw[2] > 0.0 {
                    frame.fill_rectangle(Point::new(ex, ey + eh - bw[2]), Size::new(ew, bw[2]), color);
                }
                if bw[3] > 0.0 {
                    frame.fill_rectangle(Point::new(ex, ey), Size::new(bw[3], eh), color);
                }
                if bw[1] > 0.0 {
                    frame.fill_rectangle(Point::new(ex + ew - bw[1], ey), Size::new(bw[1], eh), color);
                }
            }

            let weight = if el.font_weight == "bold" { iced::font::Weight::Bold } else { iced::font::Weight::Normal };
            let fs = if el.font_size.is_finite() { el.font_size.max(6.0).min(200.0) } else { 16.0 };
            let line_h = fs * LINE_H_SCALE;
            let px0 = el.x.max(0.0) + bw[3];
            let py0 = el.y.max(0.0) + bw[0];
            let lines: Vec<&str> = if el.wrapped_lines.is_empty() {
                vec![&el.text]
            } else {
                el.wrapped_lines.iter().map(|s| s.as_str()).collect()
            };

            for (li, line) in lines.iter().enumerate() {
                let py = py0 + li as f32 * line_h;
                if fs.is_finite() && px0.is_finite() && py.is_finite() && !line.is_empty() {
                    frame.fill_text(iced::widget::canvas::Text {
                        content: line.to_string(),
                        position: Point::new(px0, py),
                        color: el.color,
                        size: iced::Pixels(fs),
                        font: iced::Font { weight, ..Default::default() },
                        ..Default::default()
                    });
                }
            }
        }
        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut (),
        event: iced::widget::canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<BrowserMessage>) {
        use iced::widget::canvas::event;
        if let iced::widget::canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                plog!("CLICK", "Click at pos=({:.0},{:.0})", pos.x, pos.y);
                for (i, el) in self.elements.iter().enumerate() {
                    if el.is_link {
                        let text_w = el.text.len() as f32 * el.font_size * 0.55;
                        let hit = Rectangle::new(Point::new(el.x, el.y), Size::new(text_w, el.font_size + 4.0));
                        if hit.contains(pos) {
                            plog!("CLICK", "Link hit at element {} href={:?}", i, el.href);
                            if let Some(ref href) = el.href {
                                return (event::Status::Captured, Some(BrowserMessage::LinkClicked(href.clone())));
                            }
                        }
                    }
                    let ex = el.x.max(0.0);
                    let ey = el.y.max(0.0);
                    let ew = if el.width.is_finite() { el.width.max(1.0) } else { 200.0 };
                    let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { 30.0 };
                    let hit = Rectangle::new(Point::new(ex, ey), Size::new(ew, eh));
                    if hit.contains(pos) {
                        plog!("CLICK", "Element {} hit at [{:.0},{:.0} {:.0}x{:.0}] tag={}", i, ex, ey, ew, eh, el.tag);
                        return (event::Status::Captured, Some(BrowserMessage::ElementClicked(i)));
                    }
                }
            }
        }
        (event::Status::Ignored, None)
    }
}

impl BrowserScreen {
    pub fn new() -> Self {
        let default_url = "aether://design/spatial-minimalism".to_string();
        Self {
            url: default_url.clone(),
            active_workspace: 0,
            content: "Welcome to Aether Browser".to_string(),
            styled_elements: vec![],
            loading: false,
            bridge: None,
            js_engine: None,
            private_mode: false,
            devtools_open: false,
            history: vec![default_url],
            history_index: 0,
            is_history_nav: false,
        }
    }

    pub fn update(&mut self, msg: BrowserMessage) -> Task<BrowserMessage> {
        match msg {
            BrowserMessage::UrlChanged(s) => { self.url = s; Task::none() }
            BrowserMessage::UrlSubmit => {
                let target = normalize_nav_url(&self.url);
                self.url = target.clone();
                self.loading = true;
                self.bridge = None;
                self.is_history_nav = false;
                Task::perform(fetch_page_content(target), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
            }
            BrowserMessage::LinkClicked(url) => {
                let target = normalize_nav_url(&url);
                self.url = target.clone();
                self.loading = true;
                self.bridge = None;
                self.is_history_nav = false;
                Task::perform(fetch_page_content(target), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
            }
            BrowserMessage::NavBack => {
                if self.history_index > 0 {
                    self.history_index -= 1;
                    let url = self.history[self.history_index].clone();
                    self.url = url.clone();
                    self.is_history_nav = true;
                    self.loading = true;
                    self.bridge = None;
                    return Task::perform(fetch_page_content(url), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                }
                Task::none()
            }
            BrowserMessage::NavForward => {
                if self.history_index + 1 < self.history.len() {
                    self.history_index += 1;
                    let url = self.history[self.history_index].clone();
                    self.url = url.clone();
                    self.is_history_nav = true;
                    self.loading = true;
                    self.bridge = None;
                    return Task::perform(fetch_page_content(url), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                }
                Task::none()
            }
            BrowserMessage::Refresh => {
                self.loading = true;
                self.bridge = None;
                self.is_history_nav = false;
                Task::perform(fetch_page_content(self.url.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
            }
            BrowserMessage::PageLoaded(page_url, elements, bridge_opt) => {
                self.loading = false;
                self.url = page_url.clone();
                if !self.is_history_nav {
                    self.history.truncate(self.history_index + 1);
                    self.history.push(page_url);
                    self.history_index = self.history.len() - 1;
                }
                self.is_history_nav = false;
                self.styled_elements = elements;
                self.bridge = bridge_opt;
                self.js_engine = Some(JSEngine::new());
                self.content = format!("Loaded ({} elements)", self.styled_elements.len());
                Task::none()
            }
            BrowserMessage::ElementClicked(idx) => {
                if let Some(el) = self.styled_elements.get(idx) {
                    if let Some(ref bridge_arc) = self.bridge {
                        let listeners = {
                            let b = bridge_arc.lock().unwrap();
                            // Attempt to find node by index or tag (simplified matching)
                            // We use bubbling to handle the event correctly
                            let node_id = idx as u32; // Assuming direct mapping for now
                            b.get_event_listeners_bubbling(node_id, "click")
                        };
                        if !listeners.is_empty() {
                            let mut js = JSEngine::new();
                            for (source, _node_id) in listeners {
                                let _ = js.execute_source(&source, bridge_arc);
                            }
                        }
                    }
                }
                Task::none()
            }
            BrowserMessage::CanvasClick(_) => Task::none(),
            BrowserMessage::TogglePrivate => {
                self.private_mode = !self.private_mode;
                Task::none()
            }
            BrowserMessage::ToggleDevTools => {
                self.devtools_open = !self.devtools_open;
                Task::none()
            }
            BrowserMessage::TimerTick => {
                if let Some(ref bridge_arc) = self.bridge {
                    let ready = {
                        let mut b = bridge_arc.lock().unwrap();
                        b.poll_timers()
                    };
                    if !ready.is_empty() {
                        let mut js = JSEngine::new();
                        for (_timer_id, source) in ready {
                            let _ = js.execute_source(&source, bridge_arc);
                        }
                    }
                    let nav = {
                        let mut b = bridge_arc.lock().unwrap();
                        b.pending_navigation.take()
                    };
                    if let Some(url) = nav {
                        self.url = url;
                        self.loading = true;
                        self.bridge = None;
                        return Task::perform(fetch_page_content(self.url.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                    }
                }
                Task::none()
            }
            BrowserMessage::WorkspaceSelected(i) => { self.active_workspace = i; Task::none() }
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<BrowserMessage> {
        if let Some(ref bridge_arc) = self.bridge {
             let has_timers = bridge_arc.lock().unwrap().has_pending_timers();
             if has_timers {
                 return iced::time::every(std::time::Duration::from_millis(100)).map(|_| BrowserMessage::TimerTick);
             }
        }
        iced::Subscription::none()
    }

    pub fn view(&self) -> Element<'_, BrowserMessage> {
        let sidebar = self.sidebar();
        let main = self.main_area();

        row![sidebar, main].into()
    }

    // ── Main Area ─────────────────────────────────────────────────────────────

    fn main_area(&self) -> Element<'_, BrowserMessage> {
        let top = self.top_bar();
        let status = self.status_bar();

        let body: Element<'_, BrowserMessage> = if self.loading {
            container(
                column![
                    text("Loading...").size(20).color(C::PAGE_MUTED),
                    text("Fetching page content").size(13).color(C::DIM),
                ]
                .align_x(Alignment::Center)
                .spacing(8)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::PAGE_BG)),
                ..Default::default()
            })
            .into()
        } else if !self.styled_elements.is_empty() {
            let total_h = self.styled_elements.iter()
                .map(|el| {
                    let ey = if el.y.is_finite() { el.y } else { 0.0 };
                    ey + el.height.max(el.font_size.max(6.0).min(200.0)) + 40.0
                })
                .fold(0.0, f32::max);
            let total_h = if total_h.is_finite() { total_h.max(100.0) } else { 800.0 };

            let pg = PageCanvas { elements: &self.styled_elements };

            container(
                scrollable(canvas(pg).width(Length::Fixed(800.0)).height(Length::Fixed(total_h)))
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::PAGE_BG)),
                ..Default::default()
            })
            .into()
        } else {
            container(
                scrollable(
                    column(
                        vec![
                            text(&self.content).size(14).color(C::PAGE_TEXT).into()
                        ]
                    )
                    .padding(40)
                    .max_width(800)
                )
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::PAGE_BG)),
                ..Default::default()
            })
            .into()
        };

        container(column![top, body, status])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(main_area_style())
            .into()
    }

    // ── Sidebar ───────────────────────────────────────────────────────────────

    fn sidebar(&self) -> Element<'_, BrowserMessage> {
        let logo = row![
            container(
                text("⬡").size(18).color(C::ACCENT)
            )
            .width(28).height(28)
            .center_x(Length::Fixed(28.0))
            .center_y(Length::Fixed(28.0))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(
                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.07)
                )),
                border: iced::Border { radius: 8.0.into(), ..Default::default() },
                ..Default::default()
            }),
            text("AETHER").size(16).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let ws_label = text("WORKSPACES")
            .size(10)
            .color(C::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() });

        let workspaces = column(
            WORKSPACES.iter().enumerate().map(|(i, (icon, name))| {
                let active = i == self.active_workspace;
                let label = row![
                    text(*icon).size(16).color(if active { C::ACCENT } else { C::MUTED }),
                    text(*name).size(13).color(if active { C::ACCENT } else { C::MUTED }),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                button(label)
                    .padding([10, 16])
                    .width(Length::Fill)
                    .style(sidebar_item_button_style(active))
                    .on_press(BrowserMessage::WorkspaceSelected(i))
                    .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(2);

        let col_label = text("COLLECTIONS")
            .size(10)
            .color(C::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() });

        let collections = column(
            COLLECTIONS.iter().map(|(icon, name)| {
                let label = row![
                    text(*icon).size(15).color(C::MUTED),
                    text(*name).size(13).color(C::MUTED),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                button(label)
                    .padding([8, 16])
                    .width(Length::Fill)
                    .style(sidebar_item_button_style(false))
                    .on_press(BrowserMessage::NavBack)
                    .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(2);

        let bottom = column![
            button(
                row![text("⏱").size(16).color(C::MUTED), text("History").size(13).color(C::MUTED)]
                    .spacing(12).align_y(Alignment::Center)
            )
            .padding([8, 0])
            .width(Length::Fill)
            .style(ghost_button_style())
            .on_press(BrowserMessage::NavBack),

            button(
                row![text("⚙").size(16).color(C::MUTED), text("Settings").size(13).color(C::MUTED)]
                    .spacing(12).align_y(Alignment::Center)
            )
            .padding([8, 0])
            .width(Length::Fill)
            .style(ghost_button_style())
            .on_press(BrowserMessage::OpenSettings),
        ]
        .spacing(8);

        let content = column![
            logo,
            Space::with_height(16),
            ws_label,
            Space::with_height(8),
            workspaces,
            Space::with_height(24),
            col_label,
            Space::with_height(8),
            collections,
            Space::with_height(Length::Fill),
            bottom,
        ]
        .padding([32, 24])
        .spacing(0)
        .height(Length::Fill);

        container(content)
            .width(Length::Fixed(260.0))
            .height(Length::Fill)
            .style(sidebar_style())
            .into()
    }

    fn top_bar(&self) -> Element<'_, BrowserMessage> {
        let nav = row![
            button(text("←").size(16).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::NavBack),
            button(text("→").size(16).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::NavForward),
            button(text("↻").size(16).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::Refresh),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let url_bar = container(
            row![
                text("⌕").size(16).color(C::MUTED),
                text_input("Search or navigate", &self.url)
                    .on_input(BrowserMessage::UrlChanged)
                    .on_submit(BrowserMessage::UrlSubmit)
                    .size(13)
                    .style(url_input_style()),
                text("☆").size(14).color(C::DIM),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .padding([0, 16])
        )
        .height(Length::Fixed(40.0))
        .width(Length::Fill)
        .max_width(600.0)
        .center_y(Length::Fixed(40.0))
        .style(url_bar_style());

        let right_icons = row![
            button(text("⊞").size(18).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::OpenPalette),
            container(text("A").size(13).color(C::ACCENT))
                .width(32).height(32)
                .center_x(Length::Fixed(32.0))
                .center_y(Length::Fixed(32.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(
                        iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08)
                    )),
                    border: iced::Border { radius: 999.0.into(), color: C::BORDER_MID, width: 1.0 },
                    ..Default::default()
                }),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let bar = container(
            row![nav, url_bar, right_icons]
                .spacing(16)
                .align_y(Alignment::Center)
                .padding([0, 40])
        )
        .height(Length::Fixed(64.0))
        .width(Length::Fill)
        .center_y(Length::Fixed(64.0))
        .style(|_| container::Style {
            background: None,
            border: iced::Border { color: C::BORDER, width: 0.0, radius: 0.0.into() },
            ..Default::default()
        });

        container(column![
            bar,
            container(Space::with_height(1.0))
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(C::BORDER)),
                    ..Default::default()
                }),
        ])
        .width(Length::Fill)
        .into()
    }

    fn status_bar(&self) -> Element<'_, BrowserMessage> {
        let dot = text(" · ").size(10).color(C::DIM);

        container(
            row![
                text("Secure Core").size(10).color(C::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                dot,
                text("Flow Active").size(10).color(C::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                text(" · ").size(10).color(C::DIM),
                text("1.2ms Latency").size(10).color(C::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        )
        .height(Length::Fixed(40.0))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fixed(40.0))
        .style(status_bar_style())
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Color;

    fn make_test(tag: &str, text: &str, display: &str, parent: Option<usize>) -> StyledElement {
        StyledElement {
            tag: tag.to_string(),
            text: text.to_string(),
            wrapped_lines: vec![],
            is_link: false,
            href: None,
            indent_level: 0,
            color: Color::BLACK,
            font_size: 16.0,
            font_weight: "normal".to_string(),
            background_color: None,
            border_widths: [0.0; 4],
            border_color: None,
            image_handle: None,
            image_url: None,
            margin_top: 0.0,
            margin_bottom: 0.0,
            margin_left: None,
            margin_right: None,
            padding: [0.0; 4],
            display: display.to_string(),
            flex_direction: "row".to_string(),
            flex_wrap: "nowrap".to_string(),
            justify_content: "flex-start".to_string(),
            align_items: "stretch".to_string(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            css_width: None,
            css_height: None,
            parent_index: parent,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }

    const EPS: f32 = 0.01;

    #[test]
    fn test_ifc_simple_inline_siblings() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hello", "inline", Some(0)),
            make_test("span", "World", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0);
        assert!((elements[1].x - 0.0).abs() < EPS, "span0 x={}", elements[1].x);
        let expected_x = 5.0 * 16.0 * CHAR_W_SCALE;
        assert!((elements[2].x - expected_x).abs() < EPS, "span1 x={} expected={}", elements[2].x, expected_x);
        assert!((elements[1].y - 0.0).abs() < EPS, "span0 y={}", elements[1].y);
        assert!((elements[2].y - 0.0).abs() < EPS, "span1 y={}", elements[2].y);
    }

    #[test]
    fn test_ifc_single_inline_in_block() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hi", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0);
        let text_w = 2.0 * 16.0 * CHAR_W_SCALE;
        assert!((elements[1].x - 0.0).abs() < EPS, "x={}", elements[1].x);
        assert!((elements[1].width - text_w).abs() < EPS, "width={} expected={}", elements[1].width, text_w);
    }

    #[test]
    fn test_ifc_inline_wraps_when_exceeds_container() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "ABCDEFGH", "inline", Some(0)),
            make_test("span", "IJKLMNOP", "inline", Some(0)),
        ];
        // container width is 800, each span text width ≈ 8*16*0.58=74.24
        // both fit on one line (74.24+74.24=148.48 < 800), so no wrap expected
        apply_caelum_layout(&mut elements, 800.0);
        let text_w = 8.0 * 16.0 * CHAR_W_SCALE;
        assert!((elements[1].x - 0.0).abs() < EPS, "span0 x={}", elements[1].x);
        assert!((elements[2].x - text_w).abs() < EPS, "span1 x={} expected={}", elements[2].x, text_w);
        assert!((elements[1].y - 0.0).abs() < EPS, "span0 y={}", elements[1].y);
        assert!((elements[2].y - 0.0).abs() < EPS, "span1 y={}", elements[2].y);
    }

    #[test]
    fn test_ifc_mixed_inline_and_block() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hello", "inline", Some(0)),
            make_test("p", "Block", "block", Some(0)),
            make_test("span", "World", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0);
        let text_w = 5.0 * 16.0 * CHAR_W_SCALE;
        // span0 on line 1
        assert!((elements[1].x - 0.0).abs() < EPS, "span0 x={}", elements[1].x);
        assert!((elements[1].y - 0.0).abs() < EPS, "span0 y={}", elements[1].y);
        // block below span0
        assert!(elements[3].y > elements[1].y, "block y={} should be > span0 y={}", elements[3].y, elements[1].y);
        // span1 on a new line below block
        assert!(elements[3].y > 0.0);
        assert!((elements[3].x - 0.0).abs() < EPS, "span1 x={}", elements[3].x);
    }

    #[test]
    fn test_ifc_nested_inline() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Outer ", "inline", Some(0)),
            make_test("span", "Inner", "inline", Some(1)),
        ];
        apply_caelum_layout(&mut elements, 800.0);
        // Outer span width = 6 * 16 * 0.58
        // Inner span width = 5 * 16 * 0.58
        // Both should be on same line (nested spans flow inline)
        assert!((elements[1].y - 0.0).abs() < EPS, "outer y={}", elements[1].y);
        assert!((elements[2].y - 0.0).abs() < EPS, "inner y={}", elements[2].y);
    }

    #[test]
    fn test_ifc_inline_block_margin_contrib() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            StyledElement {
                margin_top: 10.0,
                margin_bottom: 10.0,
                css_width: Some(100.0),
                css_height: Some(50.0),
                width: 100.0,
                height: 50.0,
                ..make_test("div", "", "inline-block", Some(0))
            },
        ];
        apply_caelum_layout(&mut elements, 800.0);
        // inline-block height = 50 + margins (10+10) = 70 contributed to line height
        assert!((elements[1].y - 0.0).abs() < EPS, "ib y={}", elements[1].y);
        let line_h = 50.0 + 10.0 + 10.0;
        assert!((elements[0].height - line_h).abs() < EPS, "parent h={} expected={}", elements[0].height, line_h);
    }
}
