use std::collections::HashMap;

use iced::Color;
use iced::widget::image::Handle;

use crate::engine::dom::{Node, NodeType};
use crate::engine::js::js_bridge::FlatNode;
use crate::engine::stratus::{self, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent, Position, Stylesheet};
use crate::ui::style::C;

// ponytail: hard safety ceilings, not fidelity targets — normal pages must
// extract whole (PLAN A1); these only bound pathological inputs. Element
// ceiling doubles as the RAM guard; A3 slimming brought StyledElement from
// 800 to 544 bytes inline (guard test: styled_element_stays_slim). MAX_DEPTH
// also bounds native-recursion depth in the walk fns; an explicit-stack walk
// is the upgrade path if profiling ever pushes it higher.
pub const MAX_ELEMENTS: usize = 100_000;
pub(crate) const MAX_DEPTH: usize = 200;
const MAX_TEXT_LEN: usize = 64_000;

type SkipFn = Box<dyn Fn(&Node) -> bool>;

// ponytail: renderer only distinguishes bold today (numeric weights already
// collapse to normal); widen alongside canvas.rs when it needs more.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BoxSizing {
    #[default]
    ContentBox,
    BorderBox,
}

// CSS text-decoration is combinable ("underline line-through"), so a bitfield
// rather than a single enum. Bits mirror the three decorations the painter knows.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TextDecor(u8);

impl TextDecor {
    pub const UNDERLINE: TextDecor = TextDecor(1);
    pub const OVERLINE: TextDecor = TextDecor(2);
    pub const LINE_THROUGH: TextDecor = TextDecor(4);

    pub fn contains(self, other: TextDecor) -> bool {
        self.0 & other.0 != 0
    }

    fn from_css(value: Option<&str>) -> Self {
        let mut d = TextDecor(0);
        let Some(v) = value else { return d };
        if v.contains("underline") { d.0 |= TextDecor::UNDERLINE.0; }
        // "line-through" contains no other keyword's substring, but check the
        // longer names first regardless so ordering never matters.
        if v.contains("overline") { d.0 |= TextDecor::OVERLINE.0; }
        if v.contains("line-through") { d.0 |= TextDecor::LINE_THROUGH.0; }
        d
    }
}

#[derive(Debug, Clone)]
pub struct FullStyle {
    pub color: Color,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub background_color: Option<Color>,
    pub margin_top: f32,
    pub margin_right: Option<f32>,
    pub margin_bottom: f32,
    pub margin_left: Option<f32>,
    pub padding: [f32; 4],
    pub border_widths: [f32; 4],
    pub border_color: Option<Color>,
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub box_sizing: BoxSizing,
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
    pub text_decoration: TextDecor,
    pub border_radius: [f32; 4],
    pub position: Position,
    pub inset_top: f32,
    pub inset_right: f32,
    pub inset_bottom: f32,
    pub inset_left: f32,
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
    pub font_weight: FontWeight,
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
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub box_sizing: BoxSizing,
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
    pub text_decoration: TextDecor,
    pub border_radius: [f32; 4],
    pub input_type: String,
    pub input_value: String,
    pub input_placeholder: String,
    pub checked: bool,
    pub position: Position,
    pub inset_top: f32,
    pub inset_right: f32,
    pub inset_bottom: f32,
    pub inset_left: f32,
}

fn stratus_color(c: &stratus::Color) -> Color {
    Color::from_rgba(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0)
}

fn is_html_block_tag(tag: &str) -> bool {
    matches!(tag, "html" | "body" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
        | "p" | "ul" | "ol" | "li" | "table" | "form" | "section" | "article"
        | "nav" | "header" | "footer" | "main" | "aside" | "blockquote"
        | "figure" | "figcaption" | "details" | "summary" | "dialog"
        | "dd" | "dt" | "dl" | "pre" | "hr" | "fieldset" | "legend" | "address")
}

pub fn should_skip_tag(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "meta" | "link" | "head" | "title" | "svg" | "path" | "br" | "hr" | "template" | "iframe" | "option")
}

pub fn should_skip_content(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "template" | "svg" | "title" | "iframe")
}

fn should_recurse_skipped(tag: &str) -> bool {
    !should_skip_content(tag) && !matches!(tag, "head" | "meta" | "link")
}

fn uses_default_margins(tag: &str) -> bool {
    matches!(tag, "a" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "li" | "img" | "form" | "button" | "input" | "textarea" | "select")
}

fn default_top_margin(tag: &str) -> f32 {
    match tag {
        "h1" => 24.0, "h2" => 20.0, "h3" | "h4" | "h5" | "h6" => 16.0,
        "p" => 12.0, "li" => 8.0, "a" => 4.0, "img" => 4.0,
        "form" => 12.0, "button" | "input" | "textarea" | "select" => 4.0,
        _ => 0.0,
    }
}

fn apply_media_defaults(tag: &str, el: &mut StyledElement) {
    if tag == "img" {
        if el.css_width.is_none() { el.css_width = Some(200.0); }
        if el.css_height.is_none() { el.css_height = Some(150.0); }
    }
    if matches!(tag, "input" | "button" | "textarea" | "select") {
        if el.css_width.is_none() { el.css_width = Some(200.0); }
        if el.css_height.is_none() { el.css_height = Some(32.0); }
        if tag == "textarea" { el.css_height = Some(64.0); }
    }
}

// ponytail: single-pass entity decode, no recursion for &amp;lt; etc.
pub fn decode_html_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars();
    'outer: while let Some(c) = chars.next() {
        if c == '&' {
            let mut entity = String::new();
            let mut found_semicolon = false;
            for ch in &mut chars {
                if ch == ';' { found_semicolon = true; break; }
                entity.push(ch);
            }
            if !found_semicolon {
                result.push('&');
                result.push_str(&entity);
                continue 'outer;
            }
            let decoded = match entity.as_str() {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                "nbsp" => Some('\u{00A0}'),
                "copy" => Some('\u{00A9}'),
                "reg" => Some('\u{00AE}'),
                _ => {
                    if let Some(hex) = entity.strip_prefix("#x") {
                        u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                    } else if let Some(dec) = entity.strip_prefix('#') {
                        dec.parse::<u32>().ok().and_then(char::from_u32)
                    } else {
                        None
                    }
                }
            };
            match decoded {
                Some(d) => result.push(d),
                None => {
                    result.push('&');
                    result.push_str(&entity);
                    result.push(';');
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn get_all_text(node: &Node) -> String {
    // ponytail: build string in-place to avoid intermediate Vec allocation
    fn collect(node: &Node, out: &mut String) {
        match &node.node_type {
            NodeType::Text(t) => {
                let decoded = decode_html_entities(t.trim());
                if !decoded.is_empty() {
                    if !out.is_empty() { out.push(' '); }
                    out.push_str(&decoded);
                }
            }
            NodeType::Element(_) => {
                for child in &node.children { collect(child, out); }
            }
            _ => {}
        }
    }
    let mut out = String::new();
    collect(node, &mut out);
    out
}

fn compute_full_style(node: &Node, ss: &Stylesheet, vw: f32, vh: f32) -> FullStyle {
    let (tag, attrs) = match &node.node_type {
        NodeType::Element(e) => (e.tag_name.to_lowercase(), &e.attributes),
        _ => (String::new(), &HashMap::new()),
    };
    compute_full_style_inner(&tag, attrs, None, Some(node), ss, vw, vh)
}

fn compute_full_style_flat(node: &FlatNode, ss: &Stylesheet, vw: f32, vh: f32) -> FullStyle {
    compute_full_style_inner(&node.tag, &node.attrs, Some(&node.inline_styles), None::<&Node>, ss, vw, vh)
}

fn compute_full_style_inner(
    tag: &str,
    attrs: &HashMap<String, String>,
    inline_styles: Option<&HashMap<String, String>>,
    node: Option<&Node>,
    ss: &Stylesheet,
    vw: f32,
    vh: f32,
) -> FullStyle {
    let cs = match node {
        Some(n) => crate::engine::style::compute_style_vp(n, ss, vw, vh),
        None => {
            let merged = if let Some(inline) = inline_styles {
                let mut m = HashMap::with_capacity(attrs.len() + inline.len());
                m.extend(inline.iter().map(|(k, v)| (k.clone(), v.clone())));
                m.extend(attrs.iter().map(|(k, v)| (k.clone(), v.clone())));
                m
            } else {
                attrs.clone()
            };
            let element = crate::engine::stratus::ElementData::with_attributes_ref(tag.to_string(), &merged);
            crate::engine::stratus::resolve_style_vp(&element, ss, vw, vh)
        }
    };
    // Match (not unwrap_or): the palette read must stay lazy so elements with
    // an explicit CSS color never touch the global at all.
    let mut color = match cs.color.as_ref() {
        Some(c) => stratus_color(c),
        None => C::page_text(),
    };
    if tag == "a" && cs.color.is_none() {
        color = Color::from_rgb(0.0, 0.0, 0.93);
    }
    let font_size = cs.font_size.filter(|v| v.is_finite()).map(|v| v.clamp(6.0, 200.0)).unwrap_or(16.0);
    let font_weight = match cs.font_weight.as_deref() {
        Some("bold") => FontWeight::Bold,
        _ => FontWeight::Normal,
    };
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

    // ponytail: display:none elements still get a StyledElement but are skipped in layout
    let mut display = cs.display;
    if display == Display::Inline && is_html_block_tag(tag) {
        display = Display::Block;
    }

    let box_sizing = match cs.box_sizing.as_deref() {
        Some("border-box") => BoxSizing::BorderBox,
        _ => BoxSizing::ContentBox,
    };

    FullStyle {
        color, font_size, font_weight, background_color,
        margin_top, margin_right, margin_bottom, margin_left,
        padding: [pt, pr, pb, pl],
        border_widths: [bt, br, bb, bl],
        border_color,
        display,
        flex_direction: cs.flex.flex_direction,
        flex_wrap: cs.flex.flex_wrap,
        justify_content: cs.flex.justify_content,
        align_items: cs.flex.align_items,
        align_self: cs.flex.align_self,
        box_sizing,
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
        text_decoration: TextDecor::from_css(cs.text_decoration.as_deref()),
        border_radius: {
            let r = cs.border_radius.unwrap_or(0.0);
            [r, r, r, r]
        },
        position: cs.position,
        inset_top: cs.top.unwrap_or(0.0),
        inset_right: cs.right.unwrap_or(0.0),
        inset_bottom: cs.bottom.unwrap_or(0.0),
        inset_left: cs.left.unwrap_or(0.0),
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
        color: fs.color, font_size: fs.font_size, font_weight: fs.font_weight,
        background_color: fs.background_color,
        border_widths: fs.border_widths, border_color: fs.border_color,
        image_handle: None, image_url: None,
        margin_top: fs.margin_top, margin_bottom: fs.margin_bottom,
        margin_left: fs.margin_left, margin_right: fs.margin_right,
        padding: fs.padding,
        display: fs.display, flex_direction: fs.flex_direction,
        flex_wrap: fs.flex_wrap, justify_content: fs.justify_content,
        align_items: fs.align_items,
        align_self: fs.align_self,
        box_sizing: fs.box_sizing,
        flex_grow: fs.flex_grow, flex_shrink: fs.flex_shrink, flex_basis: fs.flex_basis,
        css_width: fs.css_width, css_height: fs.css_height,
        min_width: fs.min_width, max_width: fs.max_width,
        min_height: fs.min_height, max_height: fs.max_height,
        parent_index: parent_idx,
        x: 0.0, y: 0.0, width: 0.0, height: 0.0,
        line_height: fs.line_height,
        text_decoration: fs.text_decoration,
        border_radius: fs.border_radius,
        input_type: String::new(),
        input_value: String::new(),
        input_placeholder: String::new(),
        checked: false,
        position: fs.position,
        inset_top: fs.inset_top,
        inset_right: fs.inset_right,
        inset_bottom: fs.inset_bottom,
        inset_left: fs.inset_left,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn extract_elements(
    node: &Node,
    elements: &mut Vec<StyledElement>,
    depth: usize,
    ss: &Stylesheet,
    parent_style: Option<FullStyle>,
    parent_idx: Option<usize>,
    dom_path: Vec<usize>,
    viewport_w: f32,
    viewport_h: f32,
) {
    if depth > MAX_DEPTH || elements.len() >= MAX_ELEMENTS { return; }

    match &node.node_type {
        NodeType::Document => {
            for child in &node.children {
                extract_elements(child, elements, depth, ss, parent_style.clone(), parent_idx, dom_path.clone(), viewport_w, viewport_h);
            }
        }
        NodeType::Comment(_) => {}
        NodeType::Text(text) => {
            let txt = decode_html_entities(text.trim());
            if !txt.is_empty() && txt.len() < MAX_TEXT_LEN && !txt.chars().all(|c| c.is_whitespace()) {
                if let Some(ref ps) = parent_style {
                    let mut el = make_element("text", txt, ps, parent_idx, dom_path.clone());
                    el.background_color = None;
                    elements.push(el);
                } else {
                    let fs = compute_full_style(node, ss, viewport_w, viewport_h);
                    let mut el = make_element("text", txt, &fs, parent_idx, dom_path.clone());
                    el.background_color = None;
                    elements.push(el);
                }
            }
        }
        NodeType::Element(elem) => {
            let tag = elem.tag_name.to_lowercase();
            if should_skip_tag(&tag) {
                if should_recurse_skipped(&tag) {
                    let mut visible_idx = 0usize;
                    for child in &node.children {
                        if matches!(&child.node_type, NodeType::Comment(_)) {
                            continue;
                        }
                        let mut child_path = dom_path.clone();
                        child_path.push(visible_idx);
                        visible_idx += 1;
                        extract_elements(child, elements, depth + 1, ss, parent_style.clone(), parent_idx, child_path, viewport_w, viewport_h);
                    }
                }
                return;
            }

            let fs = compute_full_style(node, ss, viewport_w, viewport_h);

            let uses_default_margins = uses_default_margins(tag.as_str());

            let mut text_consumed = false;
            let mut extra_input_type = String::new();
            let mut extra_input_value = String::new();
            let mut extra_input_placeholder = String::new();
            let mut extra_checked = false;

            let (text_content, is_link, href, indent, tag_override, skip_element, recurse_into_children) = match tag.as_str() {
                "a" => {
                    let href = elem.attributes.get("href").map(|v| decode_html_entities(v));
                    let text = get_all_text(node);
                    if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                    else { text_consumed = true; (text, true, href, 0, "", false, false) }
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
                                let txt = decode_html_entities(t.trim());
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
                    let alt = elem.attributes.get("alt").map(|v| decode_html_entities(v)).unwrap_or_default();
                    (alt, false, None, 0, "img", false, false)
                }
                "button" => {
                    let text = get_all_text(node);
                    if text.is_empty() {
                        (String::new(), false, None, 0, "button", false, false)
                    } else {
                        text_consumed = true;
                        (text, false, None, 0, "button", false, false)
                    }
                }
                "input" => {
                    let input_type = elem.attributes.get("type").cloned().unwrap_or_else(|| "text".to_string());
                    let input_value = elem.attributes.get("value").cloned().unwrap_or_default();
                    let input_placeholder = elem.attributes.get("placeholder").cloned().unwrap_or_default();
                    let checked = elem.attributes.contains_key("checked");
                    let text = match input_type.as_str() {
                        "submit" | "button" | "reset" => {
                            if !input_value.is_empty() { input_value.clone() } else { input_type.clone() }
                        }
                        _ => input_placeholder.clone(),
                    };
                    extra_input_type = input_type;
                    extra_input_value = input_value;
                    extra_input_placeholder = input_placeholder;
                    extra_checked = checked;
                    (text, false, None, 0, "input", false, false)
                }
                "textarea" => {
                    let text = get_all_text(node);
                    extra_input_type = "textarea".to_string();
                    if text.is_empty() {
                        let placeholder = elem.attributes.get("placeholder").cloned().unwrap_or_default();
                        extra_input_placeholder = placeholder.clone();
                        (placeholder, false, None, 0, "textarea", false, false)
                    } else {
                        text_consumed = true;
                        extra_input_value = text.clone();
                        (text, false, None, 0, "textarea", false, false)
                    }
                }
                "select" => {
                    let selected_text = node.children.iter().find_map(|c| {
                        if let NodeType::Element(e) = &c.node_type {
                            if e.tag_name.to_lowercase() == "option" {
                                let txt = get_all_text(c);
                                if !txt.is_empty() { Some(txt) } else { None }
                            } else { None }
                        } else { None }
                    }).unwrap_or_default();
                    extra_input_type = "select".to_string();
                    if !selected_text.is_empty() {
                        text_consumed = true;
                    }
                    (selected_text, false, None, 0, "select", false, false)
                }
                "option" => {
                    let text = get_all_text(node);
                    if text.is_empty() {
                        (String::new(), false, None, 0, "", true, false)
                    } else {
                        text_consumed = true;
                        (text, false, None, 1, "option", false, false)
                    }
                }
                _ => {
                    if fs.display == Display::Inline && tag != "span" {
                        let text = get_all_text(node);
                        if text.is_empty() {
                        (String::new(), false, None, 0, "", false, true)
                        } else {
                            (text, false, None, 0, tag.as_str(), false, true)
                        }
                    } else {
                        (String::new(), false, None, 0, tag.as_str(), false, false)
                    }
                }
            };

            if !recurse_into_children && skip_element {
                return;
            }

            let needs_skip_children = !text_content.is_empty() && recurse_into_children && !skip_element;

            let this_idx = if !skip_element {
                let tag_name = if tag_override.is_empty() { &tag } else { tag_override };
                let mut el = make_element(tag_name, text_content, &fs, parent_idx, dom_path.clone());
                el.is_link = is_link;
                el.href = href;
                el.indent_level = indent;
                if tag == "img" {
                    el.image_url = elem.attributes.get("src").map(|v| decode_html_entities(v)).filter(|s| !s.is_empty());
                }
                apply_media_defaults(tag.as_str(), &mut el);
                if uses_default_margins {
                    if el.margin_top == 0.0 { el.margin_top = default_top_margin(tag.as_str()); }
                }
                if !extra_input_type.is_empty() {
                    el.input_type = extra_input_type;
                    el.input_value = extra_input_value;
                    el.input_placeholder = extra_input_placeholder;
                    el.checked = extra_checked;
                }
                if tag == "button" {
                    el.input_type = "button".to_string();
                }
                let idx = elements.len();
                elements.push(el);
                Some(idx)
            } else {
                parent_idx
            };

            let (new_parent, skip_fn): (Option<usize>, SkipFn) = match tag.as_str() {
                "img" | "input" | "button" => (parent_idx, Box::new(|_| true)),
                "a" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "textarea" => {
                    if text_consumed {
                        (parent_idx, Box::new(|c| matches!(c.node_type, NodeType::Text(_))))
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
                "select" => (parent_idx, Box::new(|_| true)),
                "form" => (this_idx.or(parent_idx), Box::new(|_| false)),
                _ => {
                    if needs_skip_children {
                        (this_idx.or(parent_idx), Box::new(|c| matches!(c.node_type, NodeType::Text(_))))
                    } else if skip_element {
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
                extract_elements(child, elements, depth + 1, ss, Some(fs.clone()), new_parent, child_path, viewport_w, viewport_h);
            }
        }
    }
}

// ── FlatNode-based extraction (avoids FlatNode → DOM round-trip) ──

fn get_all_text_flat(nodes: &[FlatNode], id: u32) -> String {
    fn collect(nodes: &[FlatNode], id: u32, out: &mut String) {
        let node = match nodes.get(id as usize) { Some(n) => n, None => return };
        if node.is_text {
            let decoded = decode_html_entities(&node.text);
            let trimmed = decoded.trim();
            if !trimmed.is_empty() {
                if !out.is_empty() { out.push(' '); }
                out.push_str(trimmed);
            }
        } else if !node.is_document {
            for &child in &node.children { collect(nodes, child, out); }
        }
    }
    let mut out = String::new();
    collect(nodes, id, &mut out);
    out
}

fn get_direct_text_flat(nodes: &[FlatNode], id: u32) -> String {
    let node = match nodes.get(id as usize) { Some(n) => n, None => return String::new() };
    let mut out = String::new();
    for &child_id in &node.children {
        if let Some(child) = nodes.get(child_id as usize) {
            if child.is_text {
                let decoded = decode_html_entities(&child.text);
                let trimmed = decoded.trim();
                if !trimmed.is_empty() {
                    if !out.is_empty() { out.push(' '); }
                    out.push_str(trimmed);
                }
            }
        }
    }
    out
}

// ponytail: mirrors extract_elements but takes &[FlatNode] to skip to_dom() serialization
pub(crate) fn extract_elements_flat(
    nodes: &[FlatNode],
    elements: &mut Vec<StyledElement>,
    ss: &Stylesheet,
    viewport_w: f32,
    viewport_h: f32,
) {
    #[allow(clippy::too_many_arguments)]
    fn walk(
        nodes: &[FlatNode],
        node_id: u32,
        elements: &mut Vec<StyledElement>,
        depth: usize,
        ss: &Stylesheet,
        parent_style: Option<FullStyle>,
        parent_idx: Option<usize>,
        dom_path: &mut Vec<usize>,
        vw: f32,
        vh: f32,
    ) {
        if depth > MAX_DEPTH || elements.len() >= MAX_ELEMENTS { return; }
        let node = match nodes.get(node_id as usize) { Some(n) => n, None => return };

        if node.is_document {
            for (ci, &child_id) in node.children.iter().enumerate() {
                dom_path.push(ci);
                walk(nodes, child_id, elements, depth + 1, ss, None, None, dom_path, vw, vh);
                dom_path.pop();
            }
            return;
        }

        if node.is_text {
            let txt = decode_html_entities(node.text.trim());
            if !txt.is_empty() && txt.len() < MAX_TEXT_LEN && !txt.chars().all(|c| c.is_whitespace()) {
                if let Some(ref ps) = parent_style {
                    let mut el = make_element("text", txt, ps, parent_idx, dom_path.clone());
                    el.background_color = None;
                    elements.push(el);
                } else {
                    let fs = compute_full_style_flat(node, ss, vw, vh);
                    let mut el = make_element("text", txt, &fs, parent_idx, dom_path.clone());
                    el.background_color = None;
                    elements.push(el);
                }
            }
            return;
        }

        let tag = node.tag.as_str();
        if should_skip_tag(tag) {
            if should_recurse_skipped(tag) {
                for (ci, &child_id) in node.children.iter().enumerate() {
                    dom_path.push(ci);
                    walk(nodes, child_id, elements, depth + 1, ss, parent_style.clone(), parent_idx, dom_path, vw, vh);
                    dom_path.pop();
                }
            }
            return;
        }

        let fs = compute_full_style_flat(node, ss, vw, vh);
        let uses_default_margins = uses_default_margins(tag);

        let (text_content, is_link, href, indent, tag_override, skip_element, recurse_into_children) = match tag {
            "a" => {
                let h = node.attrs.get("href").map(|v| decode_html_entities(v));
                let text = get_all_text_flat(nodes, node_id);
                if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                else { (text, true, h, 0, "", false, false) }
            }
            "h1" => {
                let text = get_all_text_flat(nodes, node_id);
                if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                else { (text, false, None, 0, "h1", false, false) }
            }
            "h2" => {
                let text = get_all_text_flat(nodes, node_id);
                if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                else { (text, false, None, 0, "h2", false, false) }
            }
            "h3" | "h4" | "h5" | "h6" => {
                let text = get_all_text_flat(nodes, node_id);
                if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                else { (text, false, None, 1, tag, false, false) }
            }
            "p" => {
                let direct_text = get_direct_text_flat(nodes, node_id);
                if direct_text.is_empty() { (String::new(), false, None, 0, "", true, true) }
                else { (direct_text, false, None, 0, "p", false, true) }
            }
            "li" => {
                let has_link = node.children.iter().any(|&c| {
                    nodes.get(c as usize).is_some_and(|n| !n.is_text && n.tag == "a")
                });
                if has_link { (String::new(), false, None, 0, "", true, false) }
                else {
                    let text = get_all_text_flat(nodes, node_id);
                    if text.is_empty() { (String::new(), false, None, 0, "", true, false) }
                    else { (format!("• {}", text), false, None, 1, "li", false, false) }
                }
            }
            "img" => {
                let alt = node.attrs.get("alt").map(|v| decode_html_entities(v)).unwrap_or_default();
                (alt, false, None, 0, "img", false, false)
            }
            "button" => {
                let text = get_all_text_flat(nodes, node_id);
                if text.is_empty() { (String::new(), false, None, 0, "button", false, false) }
                else { (text, false, None, 0, "button", false, false) }
            }
            "input" => {
                let value = node.attrs.get("value").cloned();
                let placeholder = node.attrs.get("placeholder").cloned();
                let input_type = node.attrs.get("type").cloned().unwrap_or_else(|| "text".to_string());
                let text = if input_type == "submit" || input_type == "button" || input_type == "reset" {
                    value.unwrap_or_else(|| input_type.to_string())
                } else {
                    value.or(placeholder).unwrap_or_default()
                };
                (text, false, None, 0, "input", false, false)
            }
            "textarea" => {
                let text = get_all_text_flat(nodes, node_id);
                if text.is_empty() {
                    let placeholder = node.attrs.get("placeholder").cloned().unwrap_or_default();
                    (placeholder, false, None, 0, "textarea", false, false)
                } else {
                    (text, false, None, 0, "textarea", false, false)
                }
            }
            "select" => (String::new(), false, None, 0, "select", false, true),
                _ => {
                if fs.display == Display::Inline && tag != "span" {
                    let text = get_all_text_flat(nodes, node_id);
                    if text.is_empty() {
                        (String::new(), false, None, 0, "", false, true)
                    } else {
                        (text, false, None, 0, tag, false, true)
                    }
                } else {
                    (String::new(), false, None, 0, tag, false, false)
                }
            }
        };

        if !recurse_into_children && skip_element { return; }

        let needs_skip_children = !text_content.is_empty() && recurse_into_children && !skip_element;

        let this_idx = if !skip_element {
            let tag_name = if tag_override.is_empty() { tag } else { tag_override };
            let mut el = make_element(tag_name, text_content, &fs, parent_idx, dom_path.clone());
            el.is_link = is_link;
            el.href = href;
            el.indent_level = indent;
            if tag == "img" {
                el.image_url = node.attrs.get("src").map(|v| decode_html_entities(v)).filter(|s| !s.is_empty());
            }
            apply_media_defaults(tag, &mut el);
            if uses_default_margins {
                if el.margin_top == 0.0 { el.margin_top = default_top_margin(tag); }
            }
            let idx = elements.len();
            elements.push(el);
            Some(idx)
        } else { parent_idx };

        let (new_parent, skip_text_children): (Option<usize>, bool) = match tag {
            "img" | "input" | "button" => (parent_idx, false),
            "a" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "textarea" => (parent_idx, true),
            "li" => (parent_idx, true),
            "p" => (this_idx.or(parent_idx), true),
            "select" | "form" => (this_idx.or(parent_idx), false),
            _ => {
                if skip_element { (parent_idx, false) }
                else if needs_skip_children { (this_idx.or(parent_idx), true) }
                else { (this_idx.or(parent_idx), false) }
            }
        };

        for (ci, &child_id) in node.children.iter().enumerate() {
            let child = match nodes.get(child_id as usize) { Some(c) => c, None => continue };
            if skip_text_children && child.is_text { continue; }
            dom_path.push(ci);
            walk(nodes, child_id, elements, depth + 1, ss, Some(fs.clone()), new_parent, dom_path, vw, vh);
            dom_path.pop();
        }
    }

    if nodes.is_empty() { return; }
    let mut path = Vec::new();
    walk(nodes, 0, elements, 0, ss, None, None, &mut path, viewport_w, viewport_h);
}

#[cfg(test)]
mod tests {
    use super::decode_html_entities;
    use crate::engine::stratus::Stylesheet;
    use crate::engine::parser::parse_html;

    #[test]
    fn test_decode_amp() { assert_eq!(decode_html_entities("&amp;"), "&"); }
    #[test]
    fn test_decode_lt() { assert_eq!(decode_html_entities("&lt;"), "<"); }
    #[test]
    fn test_decode_gt() { assert_eq!(decode_html_entities("&gt;"), ">"); }
    #[test]
    fn test_decode_quot() { assert_eq!(decode_html_entities("&quot;"), "\""); }
    #[test]
    fn test_decode_apos() { assert_eq!(decode_html_entities("&apos;"), "'"); }
    #[test]
    fn test_decode_decimal() { assert_eq!(decode_html_entities("&#123;"), "{"); }
    #[test]
    fn test_decode_hex_emoji() { assert_eq!(decode_html_entities("&#x1F600;"), "😀"); }
    #[test]
    fn test_decode_no_nested() { assert_eq!(decode_html_entities("&amp;lt;"), "&lt;"); }
    #[test]
    fn test_decode_no_entities() { assert_eq!(decode_html_entities("hello world"), "hello world"); }
    #[test]
    fn test_decode_unknown() { assert_eq!(decode_html_entities("&unknown;"), "&unknown;"); }
    #[test]
    fn test_decode_mixed() { assert_eq!(decode_html_entities("a &amp; b &lt; c"), "a & b < c"); }
    #[test]
    fn test_decode_nbsp() { assert_eq!(decode_html_entities("&nbsp;"), "\u{00A0}"); }
    #[test]
    fn test_decode_copy() { assert_eq!(decode_html_entities("&copy;"), "\u{00A9}"); }
    #[test]
    fn test_decode_reg() { assert_eq!(decode_html_entities("&reg;"), "\u{00AE}"); }

    #[test]
    fn test_extract_decodes_text() {
        let html = r#"<p>hello &amp; goodbye</p>"#;
        let dom = parse_html(html);
        let sheet = Stylesheet { rules: vec![] };
        let mut elements = Vec::new();
        crate::engine::pipeline::extractor::extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
        let p = elements.iter().find(|e| e.tag == "p").expect("should find <p>");
        assert_eq!(p.text, "hello & goodbye", "&amp; should decode to &");
    }

    #[test]
    fn test_extract_decodes_anchor_href() {
        let html = r#"<a href="https://x.com?a=1&amp;b=2">link</a>"#;
        let dom = parse_html(html);
        let sheet = Stylesheet { rules: vec![] };
        let mut elements = Vec::new();
        crate::engine::pipeline::extractor::extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
        let a = elements.iter().find(|e| e.tag == "a").expect("should find <a>");
        assert_eq!(a.href.as_deref(), Some("https://x.com?a=1&b=2"), "href &amp; should decode");
    }

    #[test]
    fn test_extract_decodes_img_alt() {
        let html = r#"<img src="x.png" alt="photo &amp; picture">"#;
        let dom = parse_html(html);
        let sheet = Stylesheet { rules: vec![] };
        let mut elements = Vec::new();
        crate::engine::pipeline::extractor::extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
        let img = elements.iter().find(|e| e.tag == "img").expect("should find <img>");
        assert_eq!(img.text, "photo & picture", "alt &amp; should decode");
    }

    // Flat extraction path must honor the same budget as the tree path.
    #[test]
    fn flat_path_passes_old_element_cap() {
        use crate::engine::js::js_bridge::FlatNode;
        let mut nodes = vec![FlatNode {
            parent: None,
            children: (1..=2500u32).collect(),
            tag: String::new(),
            attrs: std::collections::HashMap::new(),
            text: String::new(),
            is_text: false,
            is_document: true,
            inline_styles: std::collections::HashMap::new(),
        }];
        for _i in 1..=2500u32 {
            nodes.push(FlatNode {
                parent: Some(0),
                children: vec![],
                tag: "div".to_string(),
                attrs: std::collections::HashMap::new(),
                text: String::new(),
                is_text: false,
                is_document: false,
                inline_styles: std::collections::HashMap::new(),
            });
        }
        let sheet = Stylesheet { rules: vec![] };
        let mut elements = Vec::new();
        super::extract_elements_flat(&nodes, &mut elements, &sheet, 800.0, 600.0);
        assert!(
            elements.len() > 2000,
            "flat path should pass the old 2000-element cap, got {}",
            elements.len()
        );
    }

    // The budget is a bounded safety stop, not a fidelity target: extraction
    // must halt at MAX_DEPTH even on pathological nesting, while still
    // admitting depths the old cap of 50 rejected. Runs on a sized stack
    // because extract_elements recurses natively — debug frames at pathological
    // depths exceed the default libtest stack, which says nothing about the
    // budget semantics under test here.
    #[test]
    fn depth_budget_is_hard_stop_but_past_old_cap() {
        let result = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(|| {
                const NESTING: usize = 300;
                let mut html = String::from("<html><body>");
                for _ in 0..NESTING {
                    html.push_str("<div>");
                }
                html.push_str("<p>bottom</p>");
                for _ in 0..NESTING {
                    html.push_str("</div>");
                }
                html.push_str("</body></html>");

                let dom = parse_html(&html);
                let sheet = Stylesheet { rules: vec![] };
                let mut elements = Vec::new();
                super::extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
                (
                    !elements.is_empty(),
                    elements.iter().map(|e| e.dom_path.len()).max().unwrap_or(0),
                )
            })
            .expect("spawn stress thread")
            .join()
            .expect("stress thread panicked");
        let (has_content, deepest) = result;

        assert!(has_content, "shallow content still extracted");
        assert!(
            deepest > 50,
            "budget must admit depth beyond the old cap of 50 (deepest: {})",
            deepest
        );
        assert!(
            deepest <= super::MAX_DEPTH + 4,
            "extraction must stop at the depth budget (deepest: {}, budget: {})",
            deepest,
            super::MAX_DEPTH
        );
    }
}


#[cfg(test)]
mod size_probe {
    use super::StyledElement;

    // A3 guard: repeated per-element Strings became Copy enums / a u8 bitfield.
    // Measured 800 -> 544 bytes inline, plus one dropped heap allocation per
    // removed String (~10 per element). Crossing this bound means new weight
    // snuck back in — justify it against MAX_ELEMENTS (100k) or shrink elsewhere.
    #[test]
    fn styled_element_stays_slim() {
        let size = std::mem::size_of::<StyledElement>();
        assert!(
            size <= 544,
            "StyledElement grew to {} bytes; A3 slimming baseline is 544",
            size
        );
    }
}
