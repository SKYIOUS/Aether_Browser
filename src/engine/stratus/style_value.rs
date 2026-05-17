//! Stratus Style Value Types
//! Strongly-typed representation of CSS properties

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Display {
    #[default]
    Inline,
    Block,
    InlineBlock,
    Flex,
    InlineFlex,
    Grid,
    None,
}

impl Display {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "block" => Display::Block,
            "inline-block" => Display::InlineBlock,
            "flex" => Display::Flex,
            "inline-flex" => Display::InlineFlex,
            "grid" => Display::Grid,
            "none" => Display::None,
            _ => Display::Inline,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.trim_start_matches('#');
        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                (r, g, b)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                return Some(Color { r, g, b, a });
            }
            _ => return None,
        };
        Some(Color { r, g, b, a: 255 })
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_named(name: &str) -> Option<Color> {
        match name.to_lowercase().as_str() {
            "black" => Some(Color::BLACK),
            "white" => Some(Color::WHITE),
            "red" => Some(Color { r: 255, g: 0, b: 0, a: 255 }),
            "green" => Some(Color { r: 0, g: 128, b: 0, a: 255 }),
            "blue" => Some(Color { r: 0, g: 0, b: 255, a: 255 }),
            "yellow" => Some(Color { r: 255, g: 255, b: 0, a: 255 }),
            "cyan" => Some(Color { r: 0, g: 255, b: 255, a: 255 }),
            "magenta" => Some(Color { r: 255, g: 0, b: 255, a: 255 }),
            "gray" | "grey" => Some(Color { r: 128, g: 128, b: 128, a: 255 }),
            "orange" => Some(Color { r: 255, g: 165, b: 0, a: 255 }),
            "purple" => Some(Color { r: 128, g: 0, b: 128, a: 255 }),
            "pink" => Some(Color { r: 255, g: 192, b: 203, a: 255 }),
            "transparent" => Some(Color::TRANSPARENT),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
    Vw,
    Vh,
    Pt,
    Cm,
    Mm,
    In,
    Ch,
}

impl Unit {
    pub fn from_str(s: &str) -> Option<(f32, Unit)> {
        let s = s.trim();
        for (suffix, unit) in [("px", Unit::Px), ("em", Unit::Em), ("rem", Unit::Rem),
            ("%", Unit::Percent), ("vw", Unit::Vw), ("vh", Unit::Vh),
            ("pt", Unit::Pt), ("cm", Unit::Cm), ("mm", Unit::Mm),
            ("in", Unit::In), ("ch", Unit::Ch)] {
            if s.ends_with(suffix) {
                let value = s.trim_end_matches(suffix).parse::<f32>().ok()?;
                return Some((value, unit));
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LengthValue {
    pub value: f32,
    pub unit: Unit,
}

impl LengthValue {
    pub fn from_str(s: &str) -> Option<LengthValue> {
        Unit::from_str(s).map(|(value, unit)| LengthValue { value, unit })
    }

    pub fn px(value: f32) -> Self {
        LengthValue { value, unit: Unit::Px }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Position {
    #[default]
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "relative" => Position::Relative,
            "absolute" => Position::Absolute,
            "fixed" => Position::Fixed,
            "sticky" => Position::Sticky,
            _ => Position::Static,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "row-reverse" => FlexDirection::RowReverse,
            "column" => FlexDirection::Column,
            "column-reverse" => FlexDirection::ColumnReverse,
            _ => FlexDirection::Row,
        }
    }

    pub fn is_row(&self) -> bool {
        matches!(self, FlexDirection::Row | FlexDirection::RowReverse)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "wrap" => FlexWrap::Wrap,
            "wrap-reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum JustifyContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "flex-end" => JustifyContent::FlexEnd,
            "center" => JustifyContent::Center,
            "space-between" => JustifyContent::SpaceBetween,
            "space-around" => JustifyContent::SpaceAround,
            "space-evenly" => JustifyContent::SpaceEvenly,
            _ => JustifyContent::FlexStart,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AlignItems {
    #[default]
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
}

impl AlignItems {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "flex-start" => AlignItems::FlexStart,
            "flex-end" => AlignItems::FlexEnd,
            "center" => AlignItems::Center,
            "baseline" => AlignItems::Baseline,
            _ => AlignItems::Stretch,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AlignSelf {
    #[default]
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl AlignSelf {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "flex-start" => AlignSelf::FlexStart,
            "flex-end" => AlignSelf::FlexEnd,
            "center" => AlignSelf::Center,
            "baseline" => AlignSelf::Baseline,
            "stretch" => AlignSelf::Stretch,
            _ => AlignSelf::Auto,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FlexOptions {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transform {
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub rotate: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transition {
    pub property: String,
    pub duration: f32,
    pub timing_function: String,
    pub delay: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComputedStyle {
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_weight: Option<String>,
    pub font_family: Option<String>,
    pub text_align: Option<String>,
    pub display: Display,
    pub position: Position,
    pub overflow: Option<String>,
    pub visibility: Option<String>,
    pub opacity: Option<f32>,
    pub z_index: Option<i32>,
    pub margin_top: Option<f32>,
    pub margin_right: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_right: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,
    pub border_top_width: Option<f32>,
    pub border_right_width: Option<f32>,
    pub border_bottom_width: Option<f32>,
    pub border_left_width: Option<f32>,
    pub border_top_color: Option<Color>,
    pub border_right_color: Option<Color>,
    pub border_bottom_color: Option<Color>,
    pub border_left_color: Option<Color>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub flex: FlexOptions,
    pub transform: Option<Transform>,
    pub transition: Option<Transition>,
    pub box_sizing: Option<String>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        ComputedStyle {
            display: Display::default(),
            position: Position::default(),
            flex: FlexOptions::default(),
            transform: None,
            transition: None,
            ..Default::default()
        }
    }

    pub fn default_style() -> ComputedStyle {
        ComputedStyle {
            color: Some(Color::BLACK),
            background_color: None,
            font_size: Some(16.0),
            font_weight: Some("normal".to_string()),
            font_family: Some("sans-serif".to_string()),
            text_align: None,
            display: Display::Inline,
            position: Position::Static,
            overflow: None,
            visibility: Some("visible".to_string()),
            opacity: Some(1.0),
            z_index: None,
            margin_top: Some(0.0),
            margin_right: Some(0.0),
            margin_bottom: Some(0.0),
            margin_left: Some(0.0),
            padding_top: Some(0.0),
            padding_right: Some(0.0),
            padding_bottom: Some(0.0),
            padding_left: Some(0.0),
            border_top_width: Some(0.0),
            border_right_width: Some(0.0),
            border_bottom_width: Some(0.0),
            border_left_width: Some(0.0),
            border_top_color: None,
            border_right_color: None,
            border_bottom_color: None,
            border_left_color: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            flex: FlexOptions::default(),
            transform: None,
            transition: None,
            box_sizing: Some("content-box".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        assert_eq!(Color::from_hex("#fff"), Some(Color { r: 255, g: 255, b: 255, a: 255 }));
        assert_eq!(Color::from_hex("#ffffff"), Some(Color { r: 255, g: 255, b: 255, a: 255 }));
        assert_eq!(Color::from_hex("#00000080"), Some(Color { r: 0, g: 0, b: 0, a: 128 }));
    }

    #[test]
    fn test_color_from_named() {
        assert_eq!(Color::from_named("red"), Some(Color { r: 255, g: 0, b: 0, a: 255 }));
        assert_eq!(Color::from_named("transparent"), Some(Color::TRANSPARENT));
    }

    #[test]
    fn test_unit_parsing() {
        assert_eq!(Unit::from_str("10px"), Some((10.0, Unit::Px)));
        assert_eq!(Unit::from_str("1.5em"), Some((1.5, Unit::Em)));
        assert_eq!(Unit::from_str("50%"), Some((50.0, Unit::Percent)));
    }

    #[test]
    fn test_display_from_str() {
        assert_eq!(Display::from_str("flex"), Display::Flex);
        assert_eq!(Display::from_str("none"), Display::None);
        assert_eq!(Display::from_str("block"), Display::Block);
    }

    #[test]
    fn test_computed_style_default() {
        let style = ComputedStyle::default_style();
        assert_eq!(style.display, Display::Inline);
        assert_eq!(style.color, Some(Color::BLACK));
        assert_eq!(style.font_size, Some(16.0));
    }
}