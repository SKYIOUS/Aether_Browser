use crate::engine::css::{ComputedStyle, Display, Stylesheet};
use crate::engine::dom::Node;
use std::collections::HashMap;

pub struct ImageCache {
    images: HashMap<String, Vec<u8>>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self { images: HashMap::new() }
    }

    pub fn fetch_and_decode(&mut self, url: &str) -> Option<(u32, u32)> {
        let data = crate::engine::net::fetch_bytes(url);
        if data.is_empty() {
            return None;
        }
        let dims = crate::engine::image::get_image_dimensions(&data)?;
        self.images.insert(url.to_string(), data);
        Some(dims)
    }

    pub fn get(&self, url: &str) -> Option<&Vec<u8>> {
        self.images.get(url)
    }
}

#[derive(Debug, Clone)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub children: Vec<LayoutBox>,
    pub content: LayoutContent,
}

#[derive(Debug, Clone)]
pub enum LayoutContent {
    Text(String),
    Element {
        tag: String,
        style: ComputedStyle,
    },
    Image {
        src: String,
        alt: String,
        width: Option<f32>,
        height: Option<f32>,
        loaded: bool,
    },
    Empty,
}

impl LayoutContent {
    pub fn has_background(&self) -> bool {
        match self {
            LayoutContent::Element { style, .. } => style.background_color.is_some(),
            LayoutContent::Image { .. } => false,
            _ => false,
        }
    }

    pub fn has_foreground_color(&self) -> bool {
        match self {
            LayoutContent::Element { style, .. } => style.color.is_some(),
            LayoutContent::Text(_) => true,
            LayoutContent::Image { .. } => false,
            LayoutContent::Empty => false,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, LayoutContent::Image { .. })
    }
}

impl LayoutBox {
    pub fn new() -> Self {
        LayoutBox {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 0.0,
            children: Vec::new(),
            content: LayoutContent::Empty,
        }
    }

    pub fn with_content(content: LayoutContent) -> Self {
        LayoutBox {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 20.0,
            children: Vec::new(),
            content,
        }
    }
}

pub struct LayoutTree {
    pub root: LayoutBox,
    pub viewport_width: f32,
}

impl LayoutTree {
    pub fn new(viewport_width: f32) -> Self {
        LayoutTree {
            root: LayoutBox::new(),
            viewport_width,
        }
    }

    pub fn build(&mut self, node: &Node, stylesheet: &Stylesheet) {
        self.root = self.layout_node(node, stylesheet, 0.0);
    }

    fn layout_node(&self, node: &Node, stylesheet: &Stylesheet, y_offset: f32) -> LayoutBox {
        let style = crate::engine::css::compute_style(node, stylesheet);
        
        if style.display == Display::None {
            return LayoutBox::new();
        }

        let content = match &node.node_type {
            crate::engine::dom::NodeType::Text(t) => {
                LayoutContent::Text(t.clone())
            }
            crate::engine::dom::NodeType::Element(data) => {
                if data.tag_name.to_lowercase() == "img" {
                    let src = data.attributes.get("src").cloned().unwrap_or_default();
                    let alt = data.attributes.get("alt").cloned().unwrap_or_default();
                    let width = style.width.or_else(|| data.attributes.get("width").and_then(|w| w.parse().ok()));
                    let height = style.height.or_else(|| data.attributes.get("height").and_then(|h| h.parse().ok()));
                    LayoutContent::Image {
                        src,
                        alt,
                        width,
                        height,
                        loaded: false,
                    }
                } else {
                    LayoutContent::Element {
                        tag: data.tag_name.clone(),
                        style: style.clone(),
                    }
                }
            }
            _ => LayoutContent::Empty,
        };

        let mut box_ = if content.is_empty() && !node.children.is_empty() {
            LayoutBox::new()
        } else {
            LayoutBox::with_content(content)
        };

        box_.y = y_offset;

        // Layout children based on display type
        if style.display == Display::Flex {
            // Flexbox layout - horizontal by default
            let mut child_x = 0.0;
            let is_row = style.flex.flex_direction == crate::engine::css::FlexDirection::Row;
            
            for child in &node.children {
                let mut child_box = self.layout_node(child, stylesheet, y_offset);
                
                if is_row {
                    child_box.x = child_x;
                    child_box.y = y_offset;
                    child_x += child_box.width;
                    box_.children.push(child_box);
                } else {
                    child_box.x = 0.0;
                    child_box.y = child_x;
                    child_x += child_box.height;
                    box_.children.push(child_box);
                }
            }
            
            // Calculate total flex container size
            if is_row {
                box_.width = child_x;
                box_.height = box_.children.iter().map(|c| c.height).fold(0.0, f32::max);
            } else {
                box_.width = box_.children.iter().map(|c| c.width).fold(0.0, f32::max);
                box_.height = child_x;
            }
        } else {
            // Default block layout - vertical stacking
            let mut child_y = y_offset;
            let mut total_height = 0.0f32;
            for child in &node.children {
                let child_box = self.layout_node(child, stylesheet, child_y);
                child_y += child_box.height;
                total_height += child_box.height;
                box_.children.push(child_box);
            }
            if box_.children.is_empty() {
                box_.height = total_height;
            }
        }

        if box_.children.is_empty() && style.display != Display::Flex {
            box_.height = match &box_.content {
                LayoutContent::Text(t) => {
                    if t.is_empty() { 0.0 } else { 20.0 }
                }
                LayoutContent::Element { style, .. } => {
                    style.height.unwrap_or(20.0)
                }
                LayoutContent::Image { width: _, height, .. } => {
                    height.unwrap_or(150.0)
                }
                LayoutContent::Empty => 0.0,
            };
            if let LayoutContent::Image { width, .. } = &box_.content {
                box_.width = width.unwrap_or(200.0);
            }
        }

        box_
    }
}

impl LayoutContent {
    fn is_empty(&self) -> bool {
        match self {
            LayoutContent::Text(t) => t.is_empty(),
            LayoutContent::Element { .. } => false,
            LayoutContent::Image { src, .. } => src.is_empty(),
            LayoutContent::Empty => true,
        }
    }
}