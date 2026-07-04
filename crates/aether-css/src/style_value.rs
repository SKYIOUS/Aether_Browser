//! Stratus Style Value Types
//! Strongly-typed representation of CSS properties

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum Display {
    #[default]
    #[strum(serialize = "inline")]
    Inline,
    #[strum(serialize = "block")]
    Block,
    #[strum(serialize = "inline-block")]
    InlineBlock,
    #[strum(serialize = "flex")]
    Flex,
    #[strum(serialize = "inline-flex")]
    InlineFlex,
    #[strum(serialize = "grid")]
    Grid,
    #[strum(serialize = "none")]
    None,
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
    pub const CURRENT_COLOR: Color = Color { r: 0, g: 0, b: 1, a: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    pub fn is_current(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 1 && self.a == 0
    }

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
            "aliceblue" => Some(Color { r: 240, g: 248, b: 255, a: 255 }),
            "antiquewhite" => Some(Color { r: 250, g: 235, b: 215, a: 255 }),
            "aqua" => Some(Color { r: 0, g: 255, b: 255, a: 255 }),
            "aquamarine" => Some(Color { r: 127, g: 255, b: 212, a: 255 }),
            "azure" => Some(Color { r: 240, g: 255, b: 255, a: 255 }),
            "beige" => Some(Color { r: 245, g: 245, b: 220, a: 255 }),
            "bisque" => Some(Color { r: 255, g: 228, b: 196, a: 255 }),
            "black" => Some(Color::BLACK),
            "blanchedalmond" => Some(Color { r: 255, g: 235, b: 205, a: 255 }),
            "blue" => Some(Color { r: 0, g: 0, b: 255, a: 255 }),
            "blueviolet" => Some(Color { r: 138, g: 43, b: 226, a: 255 }),
            "brown" => Some(Color { r: 165, g: 42, b: 42, a: 255 }),
            "burlywood" => Some(Color { r: 222, g: 184, b: 135, a: 255 }),
            "cadetblue" => Some(Color { r: 95, g: 158, b: 160, a: 255 }),
            "chartreuse" => Some(Color { r: 127, g: 255, b: 0, a: 255 }),
            "chocolate" => Some(Color { r: 210, g: 105, b: 30, a: 255 }),
            "coral" => Some(Color { r: 255, g: 127, b: 80, a: 255 }),
            "cornflowerblue" => Some(Color { r: 100, g: 149, b: 237, a: 255 }),
            "cornsilk" => Some(Color { r: 255, g: 248, b: 220, a: 255 }),
            "crimson" => Some(Color { r: 220, g: 20, b: 60, a: 255 }),
            "currentcolor" => Some(Color::CURRENT_COLOR),
            "cyan" => Some(Color { r: 0, g: 255, b: 255, a: 255 }),
            "darkblue" => Some(Color { r: 0, g: 0, b: 139, a: 255 }),
            "darkcyan" => Some(Color { r: 0, g: 139, b: 139, a: 255 }),
            "darkgoldenrod" => Some(Color { r: 184, g: 134, b: 11, a: 255 }),
            "darkgray" => Some(Color { r: 169, g: 169, b: 169, a: 255 }),
            "darkgreen" => Some(Color { r: 0, g: 100, b: 0, a: 255 }),
            "darkgrey" => Some(Color { r: 169, g: 169, b: 169, a: 255 }),
            "darkkhaki" => Some(Color { r: 189, g: 183, b: 107, a: 255 }),
            "darkmagenta" => Some(Color { r: 139, g: 0, b: 139, a: 255 }),
            "darkolivegreen" => Some(Color { r: 85, g: 107, b: 47, a: 255 }),
            "darkorange" => Some(Color { r: 255, g: 140, b: 0, a: 255 }),
            "darkorchid" => Some(Color { r: 153, g: 50, b: 204, a: 255 }),
            "darkred" => Some(Color { r: 139, g: 0, b: 0, a: 255 }),
            "darksalmon" => Some(Color { r: 233, g: 150, b: 122, a: 255 }),
            "darkseagreen" => Some(Color { r: 143, g: 188, b: 143, a: 255 }),
            "darkslateblue" => Some(Color { r: 72, g: 61, b: 139, a: 255 }),
            "darkslategray" => Some(Color { r: 47, g: 79, b: 79, a: 255 }),
            "darkslategrey" => Some(Color { r: 47, g: 79, b: 79, a: 255 }),
            "darkturquoise" => Some(Color { r: 0, g: 206, b: 209, a: 255 }),
            "darkviolet" => Some(Color { r: 148, g: 0, b: 211, a: 255 }),
            "deeppink" => Some(Color { r: 255, g: 20, b: 147, a: 255 }),
            "deepskyblue" => Some(Color { r: 0, g: 191, b: 255, a: 255 }),
            "dimgray" => Some(Color { r: 105, g: 105, b: 105, a: 255 }),
            "dimgrey" => Some(Color { r: 105, g: 105, b: 105, a: 255 }),
            "dodgerblue" => Some(Color { r: 30, g: 144, b: 255, a: 255 }),
            "firebrick" => Some(Color { r: 178, g: 34, b: 34, a: 255 }),
            "floralwhite" => Some(Color { r: 255, g: 250, b: 240, a: 255 }),
            "forestgreen" => Some(Color { r: 34, g: 139, b: 34, a: 255 }),
            "fuchsia" => Some(Color { r: 255, g: 0, b: 255, a: 255 }),
            "gainsboro" => Some(Color { r: 220, g: 220, b: 220, a: 255 }),
            "ghostwhite" => Some(Color { r: 248, g: 248, b: 255, a: 255 }),
            "gold" => Some(Color { r: 255, g: 215, b: 0, a: 255 }),
            "goldenrod" => Some(Color { r: 218, g: 165, b: 32, a: 255 }),
            "gray" => Some(Color { r: 128, g: 128, b: 128, a: 255 }),
            "green" => Some(Color { r: 0, g: 128, b: 0, a: 255 }),
            "greenyellow" => Some(Color { r: 173, g: 255, b: 47, a: 255 }),
            "grey" => Some(Color { r: 128, g: 128, b: 128, a: 255 }),
            "honeydew" => Some(Color { r: 240, g: 255, b: 240, a: 255 }),
            "hotpink" => Some(Color { r: 255, g: 105, b: 180, a: 255 }),
            "indianred" => Some(Color { r: 205, g: 92, b: 92, a: 255 }),
            "indigo" => Some(Color { r: 75, g: 0, b: 130, a: 255 }),
            "ivory" => Some(Color { r: 255, g: 255, b: 240, a: 255 }),
            "khaki" => Some(Color { r: 240, g: 230, b: 140, a: 255 }),
            "lavender" => Some(Color { r: 230, g: 230, b: 250, a: 255 }),
            "lavenderblush" => Some(Color { r: 255, g: 240, b: 245, a: 255 }),
            "lawngreen" => Some(Color { r: 124, g: 252, b: 0, a: 255 }),
            "lemonchiffon" => Some(Color { r: 255, g: 250, b: 205, a: 255 }),
            "lightblue" => Some(Color { r: 173, g: 216, b: 230, a: 255 }),
            "lightcoral" => Some(Color { r: 240, g: 128, b: 128, a: 255 }),
            "lightcyan" => Some(Color { r: 224, g: 255, b: 255, a: 255 }),
            "lightgoldenrodyellow" => Some(Color { r: 250, g: 250, b: 210, a: 255 }),
            "lightgray" => Some(Color { r: 211, g: 211, b: 211, a: 255 }),
            "lightgreen" => Some(Color { r: 144, g: 238, b: 144, a: 255 }),
            "lightgrey" => Some(Color { r: 211, g: 211, b: 211, a: 255 }),
            "lightpink" => Some(Color { r: 255, g: 182, b: 193, a: 255 }),
            "lightsalmon" => Some(Color { r: 255, g: 160, b: 122, a: 255 }),
            "lightseagreen" => Some(Color { r: 32, g: 178, b: 170, a: 255 }),
            "lightskyblue" => Some(Color { r: 135, g: 206, b: 250, a: 255 }),
            "lightslategray" => Some(Color { r: 119, g: 136, b: 153, a: 255 }),
            "lightslategrey" => Some(Color { r: 119, g: 136, b: 153, a: 255 }),
            "lightsteelblue" => Some(Color { r: 176, g: 196, b: 222, a: 255 }),
            "lightyellow" => Some(Color { r: 255, g: 255, b: 224, a: 255 }),
            "lime" => Some(Color { r: 0, g: 255, b: 0, a: 255 }),
            "limegreen" => Some(Color { r: 50, g: 205, b: 50, a: 255 }),
            "linen" => Some(Color { r: 250, g: 240, b: 230, a: 255 }),
            "magenta" => Some(Color { r: 255, g: 0, b: 255, a: 255 }),
            "maroon" => Some(Color { r: 128, g: 0, b: 0, a: 255 }),
            "mediumaquamarine" => Some(Color { r: 102, g: 205, b: 170, a: 255 }),
            "mediumblue" => Some(Color { r: 0, g: 0, b: 205, a: 255 }),
            "mediumorchid" => Some(Color { r: 186, g: 85, b: 211, a: 255 }),
            "mediumpurple" => Some(Color { r: 147, g: 112, b: 219, a: 255 }),
            "mediumseagreen" => Some(Color { r: 60, g: 179, b: 113, a: 255 }),
            "mediumslateblue" => Some(Color { r: 123, g: 104, b: 238, a: 255 }),
            "mediumspringgreen" => Some(Color { r: 0, g: 250, b: 154, a: 255 }),
            "mediumturquoise" => Some(Color { r: 72, g: 209, b: 204, a: 255 }),
            "mediumvioletred" => Some(Color { r: 199, g: 21, b: 133, a: 255 }),
            "midnightblue" => Some(Color { r: 25, g: 25, b: 112, a: 255 }),
            "mintcream" => Some(Color { r: 245, g: 255, b: 250, a: 255 }),
            "mistyrose" => Some(Color { r: 255, g: 228, b: 225, a: 255 }),
            "moccasin" => Some(Color { r: 255, g: 228, b: 181, a: 255 }),
            "navajowhite" => Some(Color { r: 255, g: 222, b: 173, a: 255 }),
            "navy" => Some(Color { r: 0, g: 0, b: 128, a: 255 }),
            "oldlace" => Some(Color { r: 253, g: 245, b: 230, a: 255 }),
            "olive" => Some(Color { r: 128, g: 128, b: 0, a: 255 }),
            "olivedrab" => Some(Color { r: 107, g: 142, b: 35, a: 255 }),
            "orange" => Some(Color { r: 255, g: 165, b: 0, a: 255 }),
            "orangered" => Some(Color { r: 255, g: 69, b: 0, a: 255 }),
            "orchid" => Some(Color { r: 218, g: 112, b: 214, a: 255 }),
            "palegoldenrod" => Some(Color { r: 238, g: 232, b: 170, a: 255 }),
            "palegreen" => Some(Color { r: 152, g: 251, b: 152, a: 255 }),
            "paleturquoise" => Some(Color { r: 175, g: 238, b: 238, a: 255 }),
            "palevioletred" => Some(Color { r: 219, g: 112, b: 147, a: 255 }),
            "papayawhip" => Some(Color { r: 255, g: 239, b: 213, a: 255 }),
            "peachpuff" => Some(Color { r: 255, g: 218, b: 185, a: 255 }),
            "peru" => Some(Color { r: 205, g: 133, b: 63, a: 255 }),
            "pink" => Some(Color { r: 255, g: 192, b: 203, a: 255 }),
            "plum" => Some(Color { r: 221, g: 160, b: 221, a: 255 }),
            "powderblue" => Some(Color { r: 176, g: 224, b: 230, a: 255 }),
            "purple" => Some(Color { r: 128, g: 0, b: 128, a: 255 }),
            "rebeccapurple" => Some(Color { r: 102, g: 51, b: 153, a: 255 }),
            "red" => Some(Color { r: 255, g: 0, b: 0, a: 255 }),
            "rosybrown" => Some(Color { r: 188, g: 143, b: 143, a: 255 }),
            "royalblue" => Some(Color { r: 65, g: 105, b: 225, a: 255 }),
            "saddlebrown" => Some(Color { r: 139, g: 69, b: 19, a: 255 }),
            "salmon" => Some(Color { r: 250, g: 128, b: 114, a: 255 }),
            "sandybrown" => Some(Color { r: 244, g: 164, b: 96, a: 255 }),
            "seagreen" => Some(Color { r: 46, g: 139, b: 87, a: 255 }),
            "seashell" => Some(Color { r: 255, g: 245, b: 238, a: 255 }),
            "sienna" => Some(Color { r: 160, g: 82, b: 45, a: 255 }),
            "silver" => Some(Color { r: 192, g: 192, b: 192, a: 255 }),
            "skyblue" => Some(Color { r: 135, g: 206, b: 235, a: 255 }),
            "slateblue" => Some(Color { r: 106, g: 90, b: 205, a: 255 }),
            "slategray" => Some(Color { r: 112, g: 128, b: 144, a: 255 }),
            "slategrey" => Some(Color { r: 112, g: 128, b: 144, a: 255 }),
            "snow" => Some(Color { r: 255, g: 250, b: 250, a: 255 }),
            "springgreen" => Some(Color { r: 0, g: 255, b: 127, a: 255 }),
            "steelblue" => Some(Color { r: 70, g: 130, b: 180, a: 255 }),
            "tan" => Some(Color { r: 210, g: 180, b: 140, a: 255 }),
            "teal" => Some(Color { r: 0, g: 128, b: 128, a: 255 }),
            "thistle" => Some(Color { r: 216, g: 191, b: 216, a: 255 }),
            "tomato" => Some(Color { r: 255, g: 99, b: 71, a: 255 }),
            "transparent" => Some(Color::TRANSPARENT),
            "turquoise" => Some(Color { r: 64, g: 224, b: 208, a: 255 }),
            "violet" => Some(Color { r: 238, g: 130, b: 238, a: 255 }),
            "wheat" => Some(Color { r: 245, g: 222, b: 179, a: 255 }),
            "white" => Some(Color::WHITE),
            "whitesmoke" => Some(Color { r: 245, g: 245, b: 245, a: 255 }),
            "yellow" => Some(Color { r: 255, g: 255, b: 0, a: 255 }),
            "yellowgreen" => Some(Color { r: 154, g: 205, b: 50, a: 255 }),
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
    Vmin,
    Vmax,
    Pt,
    Pc,
    Cm,
    Mm,
    In,
    Ch,
    Ex,
}

impl Unit {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<(f32, Unit)> {
        let s = s.trim();
        for (suffix, unit) in [("px", Unit::Px), ("em", Unit::Em), ("rem", Unit::Rem),
            ("%", Unit::Percent), ("vw", Unit::Vw), ("vh", Unit::Vh),
            ("vmin", Unit::Vmin), ("vmax", Unit::Vmax),
            ("pt", Unit::Pt), ("pc", Unit::Pc),
            ("cm", Unit::Cm), ("mm", Unit::Mm),
            ("in", Unit::In), ("ch", Unit::Ch), ("ex", Unit::Ex)] {
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
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<LengthValue> {
        Unit::from_str(s).map(|(value, unit)| LengthValue { value, unit })
    }

    pub fn px(value: f32) -> Self {
        LengthValue { value, unit: Unit::Px }
    }
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum Position {
    #[default]
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "relative")]
    Relative,
    #[strum(serialize = "absolute")]
    Absolute,
    #[strum(serialize = "fixed")]
    Fixed,
    #[strum(serialize = "sticky")]
    Sticky,
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum FlexDirection {
    #[default]
    #[strum(serialize = "row")]
    Row,
    #[strum(serialize = "row-reverse")]
    RowReverse,
    #[strum(serialize = "column")]
    Column,
    #[strum(serialize = "column-reverse")]
    ColumnReverse,
}

impl FlexDirection {
    pub fn is_row(&self) -> bool {
        matches!(self, FlexDirection::Row | FlexDirection::RowReverse)
    }
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum FlexWrap {
    #[default]
    #[strum(serialize = "nowrap")]
    NoWrap,
    #[strum(serialize = "wrap")]
    Wrap,
    #[strum(serialize = "wrap-reverse")]
    WrapReverse,
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum JustifyContent {
    #[default]
    #[strum(serialize = "flex-start")]
    FlexStart,
    #[strum(serialize = "flex-end")]
    FlexEnd,
    #[strum(serialize = "center")]
    Center,
    #[strum(serialize = "space-between")]
    SpaceBetween,
    #[strum(serialize = "space-around")]
    SpaceAround,
    #[strum(serialize = "space-evenly")]
    SpaceEvenly,
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum AlignItems {
    #[default]
    #[strum(serialize = "stretch")]
    Stretch,
    #[strum(serialize = "flex-start")]
    FlexStart,
    #[strum(serialize = "flex-end")]
    FlexEnd,
    #[strum(serialize = "center")]
    Center,
    #[strum(serialize = "baseline")]
    Baseline,
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum AlignSelf {
    #[default]
    #[strum(serialize = "auto")]
    Auto,
    #[strum(serialize = "flex-start")]
    FlexStart,
    #[strum(serialize = "flex-end")]
    FlexEnd,
    #[strum(serialize = "center")]
    Center,
    #[strum(serialize = "baseline")]
    Baseline,
    #[strum(serialize = "stretch")]
    Stretch,
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
    pub line_height: Option<f32>,
    pub text_decoration: Option<String>,
    pub cursor: Option<String>,
    pub border_radius: Option<f32>,
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
            margin_top: None,
            margin_right: None,
            margin_bottom: None,
            margin_left: None,
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
            line_height: None,
            text_decoration: None,
            cursor: None,
            border_radius: None,
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
    fn test_color_current_color() {
        let c = Color::from_named("currentcolor").unwrap();
        assert!(c.is_current());
        assert_eq!(c, Color::CURRENT_COLOR);
    }

    #[test]
    fn test_unit_parsing() {
        assert_eq!(Unit::from_str("10px"), Some((10.0, Unit::Px)));
        assert_eq!(Unit::from_str("1.5em"), Some((1.5, Unit::Em)));
        assert_eq!(Unit::from_str("50%"), Some((50.0, Unit::Percent)));
    }

    #[test]
    fn test_display_from_str() {
        assert_eq!("flex".parse::<Display>().unwrap(), Display::Flex);
        assert_eq!("none".parse::<Display>().unwrap(), Display::None);
        assert_eq!("block".parse::<Display>().unwrap(), Display::Block);
    }

    #[test]
    fn test_computed_style_default() {
        let style = ComputedStyle::default_style();
        assert_eq!(style.display, Display::Inline);
        assert_eq!(style.color, Some(Color::BLACK));
        assert_eq!(style.font_size, Some(16.0));
    }
}


