use iced::Color;
use iced::widget::image::Handle;

use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::Stylesheet;
use crate::engine::stratus;
use crate::ui::style::C;

#[derive(Debug, Clone)]
pub struct FullStyle {
    pub color: Color,
    pub font_size: f32,
    pub font_weight: String,
    pub background_color: Option<Color>,
    pub margin_top: f32,
    pub margin_right: Option<f32>,
    pub margin_bottom: f32,
    pub margin_left: Option<f32>,
    pub padding: [f32; 4],
    pub border_widths: [f32; 4],
    pub border_color: Option<Color>,
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
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub line_height: f32,
    pub text_decoration: String,
    pub text_transform: String,
    pub border_radius: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct StyledElement {
    pub tag: String,
    pub text: String,
    pub wrapped_lines: Vec<String>,
    pub dom_path: Vec<usize>,
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
    pub line_height: f32,
    pub text_decoration: String,
    pub text_transform: String,
    pub border_radius: [f32; 4],
}

fn stratus_color(c: &stratus::Color) -> Color {
    Color::from_rgba(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0)
}

fn is_html_block_tag(tag: &str) -> bool {
    matches!(tag, "html" | "body" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
        | "p" | "ul" | "ol" | "li" | "table" | "form" | "section" | "article"
        | "nav" | "header" | "footer" | "main" | "aside" | "blockquote"
        | "figure" | "figcaption" | "details" | "summary" | "dialog"
        | "dd" | "dt" | "dl" | "pre" | "hr" | "fieldset" | "legend" | "address" | "canvas")
}

pub fn should_skip_tag(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "meta" | "link" | "head" | "title" | "svg" | "path" | "br" | "hr" | "input" | "button" | "iframe" | "textarea" | "select" | "option" | "form" | "template")
}

pub fn should_skip_content(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "template" | "svg" | "title")
}

fn get_all_text(node: &Node) -> String {
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

fn compute_full_style(node: &Node, ss: &Stylesheet) -> FullStyle {
    let tag = match &node.node_type {
        NodeType::Element(e) => e.tag_name.to_lowercase(),
        _ => String::new(),
    };
    let cs = crate::engine::style::compute_style(node, ss);
    let color = cs.color.as_ref().map(stratus_color).unwrap_or(C::PAGE_TEXT);
    let font_size = cs.font_size.filter(|v| v.is_finite()).map(|v| v.clamp(6.0, 200.0)).unwrap_or(16.0);
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

            let mut display = crate::bridge_gen::display_to_string(&cs.display).to_string();
    // ponytail: display:none elements still get a StyledElement but are skipped in layout
    if display == "inline" && is_html_block_tag(&tag) {
        display = "block".to_string();
    }

    let flex_direction = crate::bridge_gen::flex_direction_to_string(&cs.flex.flex_direction).to_string();
    let flex_wrap = crate::bridge_gen::flex_wrap_to_string(&cs.flex.flex_wrap).to_string();
    let justify_content = crate::bridge_gen::justify_content_to_string(&cs.flex.justify_content).to_string();
    let align_items = crate::bridge_gen::align_items_to_string(&cs.flex.align_items).to_string();

    FullStyle {
        color, font_size, font_weight, background_color,
        margin_top, margin_right, margin_bottom, margin_left,
        padding: [pt, pr, pb, pl],
        border_widths: [bt, br, bb, bl],
        border_color,
        display, flex_direction, flex_wrap,
        justify_content, align_items,
        flex_grow: cs.flex.flex_grow,
        flex_shrink: cs.flex.flex_shrink,
        flex_basis: cs.flex.flex_basis,
        css_width: cs.width.filter(|v| v.is_finite()),
        css_height: cs.height.filter(|v| v.is_finite()),
        min_width: cs.min_width.filter(|v| v.is_finite()),
        max_width: cs.max_width.filter(|v| v.is_finite()),
        min_height: cs.min_height.filter(|v| v.is_finite()),
        max_height: cs.max_height.filter(|v| v.is_finite()),
        line_height: cs.line_height.unwrap_or(1.4),
        text_decoration: cs.text_decoration.unwrap_or_default(),
        text_transform: String::new(),
        border_radius: {
            let r = cs.border_radius.unwrap_or(0.0);
            [r, r, r, r]
        },
    }
}

fn make_element(
    tag: &str,
    text: String,
    fs: &FullStyle,
    parent_idx: Option<usize>,
    dom_path: Vec<usize>,
) -> StyledElement {
    StyledElement {
        tag: tag.to_string(), text, wrapped_lines: vec![],
        dom_path,
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
        line_height: fs.line_height,
        text_decoration: fs.text_decoration.clone(),
        text_transform: fs.text_transform.clone(),
        border_radius: fs.border_radius,
    }
}

pub fn extract_elements(
    node: &Node,
    elements: &mut Vec<StyledElement>,
    depth: usize,
    ss: &Stylesheet,
    parent_style: Option<FullStyle>,
    parent_idx: Option<usize>,
    dom_path: Vec<usize>,
) {
    if depth > 50 || elements.len() >= 2000 { return; }

    match &node.node_type {
        NodeType::Document | NodeType::Comment(_) => {}
        NodeType::Text(text) => {
            let txt = text.trim();
            if !txt.is_empty() && txt.len() < 5000 && !txt.chars().all(|c| c.is_whitespace()) {
                if let Some(ref ps) = parent_style {
                    let mut el = make_element("text", txt.to_string(), ps, parent_idx, dom_path.clone());
                    el.background_color = None;
                    elements.push(el);
                } else {
                    let fs = compute_full_style(node, ss);
                    let mut el = make_element("text", txt.to_string(), &fs, parent_idx, dom_path.clone());
                    el.background_color = None;
                    elements.push(el);
                }
            }
        }
        NodeType::Element(elem) => {
            let tag = elem.tag_name.to_lowercase();
            if should_skip_tag(&tag) {
                if !should_skip_content(&tag) && tag != "head" && tag != "meta" && tag != "link" {
                    let mut visible_idx = 0usize;
                    for child in &node.children {
                        if matches!(&child.node_type, NodeType::Comment(_)) {
                            continue;
                        }
                        let mut child_path = dom_path.clone();
                        child_path.push(visible_idx);
                        visible_idx += 1;
                        extract_elements(child, elements, depth + 1, ss, parent_style.clone(), parent_idx, child_path);
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
                let mut el = make_element(tag_override, text_content, &fs, parent_idx, dom_path.clone());
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

            let (new_parent, skip_fn): (Option<usize>, Box<dyn Fn(&Node) -> bool>) = match tag.as_str() {
                "img" => (parent_idx, Box::new(|_| true)),
                "a" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    if text_consumed {
                        (parent_idx, Box::new(|_| true))
                    } else {
                        (parent_idx, Box::new(|_| false))
                    }
                }
                "li" => {
                    if text_consumed {
                        (parent_idx, Box::new(|_| true))
                    } else {
                        (parent_idx, Box::new(|_| false))
                    }
                }
                "p" => {
                    let new_p = this_idx.or(parent_idx);
                    (new_p, Box::new(|c: &Node| matches!(c.node_type, NodeType::Text(_))))
                }
                _ => {
                    if skip_element {
                        (parent_idx, Box::new(|_| false))
                    } else {
                        (this_idx.or(parent_idx), Box::new(|_| false))
                    }
                }
            };

            let mut visible_idx = 0usize;
            for child in &node.children {
                if matches!(&child.node_type, NodeType::Comment(_)) {
                    continue;
                }
                let mut child_path = dom_path.clone();
                child_path.push(visible_idx);
                visible_idx += 1;
                if skip_fn(child) { continue; }
                extract_elements(child, elements, depth + 1, ss, Some(fs.clone()), new_parent, child_path);
            }
        }
    }
}
