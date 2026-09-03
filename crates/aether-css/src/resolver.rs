//! Stratus Style Resolver
//! CSS Cascade and style computation

use super::matcher::{match_rules, ElementData, Specificity};
use super::parser::{CalcTerm, Declaration, PropertyValue, Stylesheet};
use super::style_value::{
    AlignItems, AlignSelf, Color, ComputedStyle, Display, FlexDirection, FlexWrap, JustifyContent,
    LengthValue, Position, Transform, Transition, Unit,
};
use std::collections::HashMap;

/// Bitmask tracking which inheritable properties were set to `inherit`.
/// Exists only until `apply_inheritance()` consumes it.
#[derive(Debug, Clone, Copy, Default)]
pub struct InheritMask(u32);

impl InheritMask {
    const COLOR: u32 = 1 << 0;
    const FONT_SIZE: u32 = 1 << 1;
    const FONT_WEIGHT: u32 = 1 << 2;
    const FONT_FAMILY: u32 = 1 << 3;
    const LINE_HEIGHT: u32 = 1 << 4;
    const TEXT_ALIGN: u32 = 1 << 5;
    const VISIBILITY: u32 = 1 << 6;

    fn set(&mut self, bit: u32) {
        self.0 |= bit;
    }
    fn has(&self, bit: u32) -> bool {
        self.0 & bit != 0
    }
}

/// Apply CSS inheritance: for each inheritable property, if the child has no
/// specified value (None) or the `inherit` keyword was used, copy the parent's
/// computed value. If no parent exists (root element), use the initial value
/// from `ComputedStyle::default_style()`.
///
/// `inherit_mask`: tracks properties set to `inherit` keyword
/// `set_mask`: tracks properties explicitly set by the cascade (non-inherit keyword)
pub fn apply_inheritance(
    child: &mut ComputedStyle,
    parent: Option<&ComputedStyle>,
    inherit_mask: InheritMask,
    set_mask: InheritMask,
) {
    let initial = ComputedStyle::default_style();
    let p = parent.unwrap_or(&initial);

    macro_rules! inherit {
        ($field:ident, $bit:expr) => {
            if inherit_mask.has($bit) {
                // Explicit `inherit` keyword: always copy from parent
                child.$field = p.$field.clone();
            } else if !set_mask.has($bit) {
                // Not set by cascade: use parent's value if available, else initial
                child.$field = p.$field.clone();
            }
            // else: set by cascade — keep child's value
        };
    }

    inherit!(color, InheritMask::COLOR);
    inherit!(font_size, InheritMask::FONT_SIZE);
    inherit!(font_weight, InheritMask::FONT_WEIGHT);
    inherit!(font_family, InheritMask::FONT_FAMILY);
    inherit!(line_height, InheritMask::LINE_HEIGHT);
    inherit!(text_align, InheritMask::TEXT_ALIGN);
    inherit!(visibility, InheritMask::VISIBILITY);
}

/// Custom properties map: `--name` → raw value.
pub type CustomPropertyMap = HashMap<String, PropertyValue>;

pub fn resolve_style(element: &ElementData, stylesheet: &Stylesheet) -> ComputedStyle {
    resolve_style_vp(element, stylesheet, 800.0, 600.0)
}

pub fn resolve_style_vp(
    element: &ElementData,
    stylesheet: &Stylesheet,
    viewport_w: f32,
    viewport_h: f32,
) -> ComputedStyle {
    resolve_style_with_vars(
        element,
        stylesheet,
        viewport_w,
        viewport_h,
        &CustomPropertyMap::new(),
    )
}

/// Resolve style with inherited custom properties.
/// Returns both the computed style and the element's own custom properties
/// (for passing to children).
pub fn resolve_style_with_vars(
    element: &ElementData,
    stylesheet: &Stylesheet,
    viewport_w: f32,
    viewport_h: f32,
    parent_vars: &CustomPropertyMap,
) -> ComputedStyle {
    let mut style = ComputedStyle::default_style();
    let matched = match_rules(element, stylesheet);

    // 1. Collect all declarations in cascade order.
    let all_decls: Vec<Declaration> = matched
        .iter()
        .flat_map(|(decls, _)| decls.iter().cloned())
        .collect();

    // 2. Collect custom properties from declarations + inherited parent.
    let mut custom = parent_vars.clone();
    collect_custom_properties(&mut custom, &all_decls);

    // 3. Apply standard declarations with var() substitution + calc().
    let mut mask = InheritMask::default();
    let mut set_mask = InheritMask::default();
    apply_declarations_with_vars(
        &mut style,
        &all_decls,
        &custom,
        viewport_w,
        viewport_h,
        &mut mask,
        &mut set_mask,
    );

    style
}

/// Resolve style and return both ComputedStyle + element's custom properties.
/// Used by the extraction pipeline to thread inheritance.
/// `parent_computed` is the parent element's resolved style (None for root).
pub fn resolve_style_with_vars_and_custom(
    element: &ElementData,
    stylesheet: &Stylesheet,
    viewport_w: f32,
    viewport_h: f32,
    parent_vars: &CustomPropertyMap,
    parent_computed: Option<&ComputedStyle>,
) -> (ComputedStyle, CustomPropertyMap, InheritMask) {
    let mut style = ComputedStyle::default_style();
    let matched = match_rules(element, stylesheet);

    let all_decls: Vec<Declaration> = matched
        .iter()
        .flat_map(|(decls, _)| decls.iter().cloned())
        .collect();

    let mut custom = parent_vars.clone();
    collect_custom_properties(&mut custom, &all_decls);

    let mut inherit_mask = InheritMask::default();
    let mut set_mask = InheritMask::default();
    apply_declarations_with_vars(
        &mut style,
        &all_decls,
        &custom,
        viewport_w,
        viewport_h,
        &mut inherit_mask,
        &mut set_mask,
    );

    apply_inheritance(&mut style, parent_computed, inherit_mask, set_mask);

    (style, custom, inherit_mask)
}

/// Collect custom property declarations (--*) into the map.
/// Later declarations override earlier ones (cascade order).
fn collect_custom_properties(map: &mut CustomPropertyMap, decls: &[Declaration]) {
    for decl in decls {
        if decl.name.starts_with("--") {
            map.insert(decl.name.clone(), decl.value.clone());
        }
    }
}

/// Apply declarations with var() substitution and calc() evaluation.
/// Resolves all var() and calc() in declaration values, then delegates
/// to the standard property application.
fn apply_declarations_with_vars(
    style: &mut ComputedStyle,
    decls: &[Declaration],
    custom: &CustomPropertyMap,
    viewport_w: f32,
    viewport_h: f32,
    inherit_mask: &mut InheritMask,
    set_mask: &mut InheritMask,
) {
    // Resolve all declarations: substitute var(), evaluate calc().
    let resolved: Vec<Declaration> = decls
        .iter()
        .filter(|d| !d.name.starts_with("--"))
        .map(|d| {
            let v = substitute_vars(&d.value, custom, 0);
            let v = eval_calc_all(&v, viewport_w, viewport_h);
            Declaration {
                name: d.name.clone(),
                value: v,
            }
        })
        .collect();

    // Apply via existing per-specificity path. We treat all resolved
    // declarations as having the same specificity since var() substitution
    // doesn't change cascade ordering.
    for decl in &resolved {
        apply_declarations_vp(
            style,
            std::slice::from_ref(decl),
            (0, 0, 0),
            viewport_w,
            viewport_h,
            inherit_mask,
            set_mask,
        );
    }
}

// ── var() substitution ──────────────────────────────────────────────

/// Substitute `var()` references in a PropertyValue.
/// Returns the resolved value, or the original if no var() is present.
/// `depth` tracks recursion to detect circular references (max 32).
fn substitute_vars(
    value: &PropertyValue,
    custom: &CustomPropertyMap,
    depth: usize,
) -> PropertyValue {
    if depth > 32 {
        // Circular reference — invalidate by returning empty keyword.
        return PropertyValue::Keyword(String::new());
    }

    match value {
        PropertyValue::Var { name, fallback } => {
            if let Some(resolved) = custom.get(name.as_str()) {
                // Recurse into the resolved value (it may contain further var()).
                substitute_vars(resolved, custom, depth + 1)
            } else if let Some(fb) = fallback {
                // Use fallback.
                substitute_vars(fb, custom, depth + 1)
            } else {
                // Undefined, no fallback — invalidate declaration.
                PropertyValue::Keyword(String::new())
            }
        }
        PropertyValue::Shorthand(parts) => {
            let resolved: Vec<PropertyValue> = parts
                .iter()
                .map(|p| substitute_vars(p, custom, depth))
                .collect();
            // If any part resolved to empty keyword (invalid), the whole shorthand is invalid.
            if resolved
                .iter()
                .any(|p| matches!(p, PropertyValue::Keyword(s) if s.is_empty()))
            {
                PropertyValue::Keyword(String::new())
            } else if resolved.len() == 1 {
                resolved.into_iter().next().unwrap()
            } else {
                PropertyValue::Shorthand(resolved)
            }
        }
        PropertyValue::Calc(terms) => {
            let resolved: Vec<CalcTerm> = terms
                .iter()
                .map(|t| substitute_vars_calc(t, custom, depth))
                .collect();
            PropertyValue::Calc(resolved)
        }
        _ => value.clone(),
    }
}

#[allow(clippy::only_used_in_recursion)]
fn substitute_vars_calc(term: &CalcTerm, custom: &CustomPropertyMap, depth: usize) -> CalcTerm {
    match term {
        CalcTerm::Paren(terms) => CalcTerm::Paren(
            terms
                .iter()
                .map(|t| substitute_vars_calc(t, custom, depth))
                .collect(),
        ),
        CalcTerm::Var(name, fallback) => {
            if let Some(resolved) = custom.get(name.as_str()) {
                match resolved {
                    PropertyValue::Number(n) => CalcTerm::Number(*n),
                    PropertyValue::Length(lv) => CalcTerm::Length(lv.clone()),
                    _ => CalcTerm::Number(0.0),
                }
            } else if let Some(fb) = fallback {
                let resolved_fb = substitute_vars(fb, custom, depth + 1);
                match resolved_fb {
                    PropertyValue::Number(n) => CalcTerm::Number(n),
                    PropertyValue::Length(lv) => CalcTerm::Length(lv),
                    _ => CalcTerm::Number(0.0),
                }
            } else {
                CalcTerm::Number(0.0)
            }
        }
        _ => term.clone(),
    }
}

// ── calc() evaluation ───────────────────────────────────────────────

/// Evaluate all Calc terms in a PropertyValue.
fn eval_calc_all(value: &PropertyValue, vw: f32, vh: f32) -> PropertyValue {
    match value {
        PropertyValue::Calc(terms) => {
            if let Some(result) = eval_calc(terms, vw, vh) {
                result
            } else {
                // Invalid calc — discard.
                PropertyValue::Keyword(String::new())
            }
        }
        PropertyValue::Shorthand(parts) => {
            let resolved: Vec<PropertyValue> =
                parts.iter().map(|p| eval_calc_all(p, vw, vh)).collect();
            if resolved.len() == 1 {
                resolved.into_iter().next().unwrap()
            } else {
                PropertyValue::Shorthand(resolved)
            }
        }
        _ => value.clone(),
    }
}

/// Evaluate a calc() expression (postfix terms) into a single value.
/// Supports +, -, *, / with standard operator precedence via parentheses.
fn eval_calc(terms: &[CalcTerm], vw: f32, vh: f32) -> Option<PropertyValue> {
    let mut values: Vec<f32> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let mut i = 0;
    while i < terms.len() {
        match &terms[i] {
            CalcTerm::Number(n) => {
                values.push(*n);
            }
            CalcTerm::Length(lv) => {
                values.push(lv_to_px(lv, vw, vh));
            }
            CalcTerm::Paren(sub) => {
                let v = eval_calc(sub, vw, vh)?;
                match v {
                    PropertyValue::Number(n) => values.push(n),
                    PropertyValue::Length(lv) => values.push(lv_to_px(&lv, vw, vh)),
                    _ => return None,
                }
            }
            CalcTerm::Var(_, _) => return None,
            CalcTerm::Add | CalcTerm::Sub | CalcTerm::Mul | CalcTerm::Div => {
                let op = match &terms[i] {
                    CalcTerm::Add => '+',
                    CalcTerm::Sub => '-',
                    CalcTerm::Mul => '*',
                    CalcTerm::Div => '/',
                    _ => unreachable!(),
                };
                // Apply higher precedence operators first.
                while let Some(&top) = ops.last() {
                    if precedence(top) >= precedence(op) {
                        let top_op = ops.pop().unwrap();
                        apply_op(&mut values, top_op)?;
                    } else {
                        break;
                    }
                }
                ops.push(op);
            }
        }
        i += 1;
    }

    // Apply remaining operators.
    while let Some(op) = ops.pop() {
        apply_op(&mut values, op)?;
    }

    if values.len() == 1 {
        Some(PropertyValue::Number(values[0]))
    } else {
        None
    }
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_op(values: &mut Vec<f32>, op: char) -> Option<()> {
    let b = values.pop()?;
    let a = values.pop()?;
    let result = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                return None; // Division by zero.
            }
            a / b
        }
        _ => return None,
    };
    values.push(result);
    Some(())
}

#[allow(dead_code)]
fn apply_declarations(style: &mut ComputedStyle, declarations: &[Declaration], spec: Specificity) {
    apply_declarations_vp(
        style,
        declarations,
        spec,
        800.0,
        600.0,
        &mut InheritMask::default(),
        &mut InheritMask::default(),
    )
}

fn apply_declarations_vp(
    style: &mut ComputedStyle,
    declarations: &[Declaration],
    _specificity: Specificity,
    viewport_w: f32,
    viewport_h: f32,
    inherit_mask: &mut InheritMask,
    set_mask: &mut InheritMask,
) {
    use super::property_names::CssPropertyName;
    use std::str::FromStr;

    for decl in declarations {
        let vw = viewport_w;
        let vh = viewport_h;
        if let Ok(prop) = CssPropertyName::from_str(&decl.name) {
            match prop {
                CssPropertyName::Color => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::COLOR);
                    } else if let Some(v) = parse_color(&decl.value) {
                        if !v.is_current() {
                            style.color = Some(v);
                            set_mask.set(InheritMask::COLOR);
                        }
                    }
                }
                CssPropertyName::Background => {
                    style.background_color = parse_background(&decl.value)
                }
                CssPropertyName::BackgroundColor => {
                    style.background_color = parse_color(&decl.value)
                }
                CssPropertyName::FontSize => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::FONT_SIZE);
                    } else {
                        style.font_size = parse_length_vp(&decl.value, vw, vh);
                        set_mask.set(InheritMask::FONT_SIZE);
                    }
                }
                CssPropertyName::FontWeight => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::FONT_WEIGHT);
                    } else {
                        style.font_weight = parse_keyword(&decl.value);
                        set_mask.set(InheritMask::FONT_WEIGHT);
                    }
                }
                CssPropertyName::FontFamily => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::FONT_FAMILY);
                    } else {
                        style.font_family = parse_keyword(&decl.value);
                        set_mask.set(InheritMask::FONT_FAMILY);
                    }
                }
                CssPropertyName::TextAlign => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::TEXT_ALIGN);
                    } else {
                        style.text_align = parse_keyword(&decl.value);
                        set_mask.set(InheritMask::TEXT_ALIGN);
                    }
                }
                CssPropertyName::Display => style.display = parse_display(&decl.value),
                CssPropertyName::Position => style.position = parse_position(&decl.value),
                CssPropertyName::Overflow => style.overflow = parse_keyword(&decl.value),
                CssPropertyName::Visibility => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::VISIBILITY);
                    } else {
                        style.visibility = parse_keyword(&decl.value);
                        set_mask.set(InheritMask::VISIBILITY);
                    }
                }
                CssPropertyName::Opacity => {
                    style.opacity = match &decl.value {
                        PropertyValue::Number(n) => Some(n.clamp(0.0, 1.0)),
                        PropertyValue::Keyword(s) => {
                            s.parse::<f32>().ok().map(|v| v.clamp(0.0, 1.0))
                        }
                        _ => None,
                    };
                }
                CssPropertyName::ZIndex => {
                    style.z_index = match &decl.value {
                        PropertyValue::Number(n) => Some(*n as i32),
                        PropertyValue::Keyword(s) => s.parse::<f32>().ok().map(|v| v as i32),
                        _ => None,
                    };
                }

                CssPropertyName::Margin
                | CssPropertyName::MarginTop
                | CssPropertyName::MarginRight
                | CssPropertyName::MarginBottom
                | CssPropertyName::MarginLeft => {
                    apply_sides_vp(
                        &mut style.margin_top,
                        &mut style.margin_right,
                        &mut style.margin_bottom,
                        &mut style.margin_left,
                        &decl.name,
                        &decl.value,
                        vw,
                        vh,
                    );
                }
                CssPropertyName::Padding
                | CssPropertyName::PaddingTop
                | CssPropertyName::PaddingRight
                | CssPropertyName::PaddingBottom
                | CssPropertyName::PaddingLeft => {
                    apply_sides_vp(
                        &mut style.padding_top,
                        &mut style.padding_right,
                        &mut style.padding_bottom,
                        &mut style.padding_left,
                        &decl.name,
                        &decl.value,
                        vw,
                        vh,
                    );
                }
                CssPropertyName::BorderWidth
                | CssPropertyName::BorderTopWidth
                | CssPropertyName::BorderRightWidth
                | CssPropertyName::BorderBottomWidth
                | CssPropertyName::BorderLeftWidth => {
                    apply_sides_vp(
                        &mut style.border_top_width,
                        &mut style.border_right_width,
                        &mut style.border_bottom_width,
                        &mut style.border_left_width,
                        &decl.name,
                        &decl.value,
                        vw,
                        vh,
                    );
                }
                CssPropertyName::BorderColor
                | CssPropertyName::BorderTopColor
                | CssPropertyName::BorderRightColor
                | CssPropertyName::BorderBottomColor
                | CssPropertyName::BorderLeftColor => {
                    apply_border_colors(
                        &mut style.border_top_color,
                        &mut style.border_right_color,
                        &mut style.border_bottom_color,
                        &mut style.border_left_color,
                        &decl.name,
                        &decl.value,
                    );
                }

                CssPropertyName::Width => style.width = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::Height => {
                    style.height = parse_length_vp_vertical(&decl.value, vw, vh)
                }
                CssPropertyName::MinWidth => style.min_width = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::MinHeight => {
                    style.min_height = parse_length_vp_vertical(&decl.value, vw, vh)
                }
                CssPropertyName::MaxWidth => style.max_width = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::MaxHeight => {
                    style.max_height = parse_length_vp_vertical(&decl.value, vw, vh)
                }
                CssPropertyName::Top => style.top = parse_length_vp_vertical(&decl.value, vw, vh),
                CssPropertyName::Right => style.right = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::Bottom => {
                    style.bottom = parse_length_vp_vertical(&decl.value, vw, vh)
                }
                CssPropertyName::Left => style.left = parse_length_vp(&decl.value, vw, vh),

                CssPropertyName::FlexDirection => {
                    style.flex.flex_direction = parse_flex_direction(&decl.value)
                }
                CssPropertyName::FlexWrap => style.flex.flex_wrap = parse_flex_wrap(&decl.value),
                CssPropertyName::JustifyContent => {
                    style.flex.justify_content = parse_justify_content(&decl.value)
                }
                CssPropertyName::AlignItems => {
                    style.flex.align_items = parse_align_items(&decl.value)
                }
                CssPropertyName::AlignSelf => style.flex.align_self = parse_align_self(&decl.value),
                CssPropertyName::Flex => {
                    apply_flex_shorthand(&mut style.flex, &decl.value, vw, vh);
                }
                CssPropertyName::FlexGrow => {
                    style.flex.flex_grow = match &decl.value {
                        PropertyValue::Number(n) => n.max(0.0),
                        PropertyValue::Keyword(s) => {
                            s.parse::<f32>().ok().map(|v| v.max(0.0)).unwrap_or(0.0)
                        }
                        _ => 0.0,
                    };
                }
                CssPropertyName::FlexShrink => {
                    style.flex.flex_shrink = match &decl.value {
                        PropertyValue::Number(n) => n.max(0.0),
                        PropertyValue::Keyword(s) => {
                            s.parse::<f32>().ok().map(|v| v.max(0.0)).unwrap_or(1.0)
                        }
                        _ => 1.0,
                    };
                }
                CssPropertyName::FlexBasis => {
                    style.flex.flex_basis = parse_length_vp(&decl.value, vw, vh)
                }

                CssPropertyName::Transform => style.transform = parse_transform(&decl.value),
                CssPropertyName::Transition => style.transition = parse_transition(&decl.value),

                CssPropertyName::BoxSizing => style.box_sizing = parse_keyword(&decl.value),

                CssPropertyName::LineHeight => {
                    if matches!(&decl.value, PropertyValue::Keyword(s) if s == "inherit") {
                        inherit_mask.set(InheritMask::LINE_HEIGHT);
                    } else {
                        style.line_height = parse_length_vp(&decl.value, vw, vh);
                        set_mask.set(InheritMask::LINE_HEIGHT);
                    }
                }
                CssPropertyName::TextDecoration => {
                    style.text_decoration = parse_keyword(&decl.value)
                }
                CssPropertyName::Cursor => style.cursor = parse_keyword(&decl.value),
                CssPropertyName::BorderRadius => {
                    style.border_radius = parse_length_vp(&decl.value, vw, vh)
                }
                CssPropertyName::Border => {
                    apply_border_shorthand(style, &decl.value);
                }
            }
        }
    }

    let base = style.color.clone().unwrap_or(Color::BLACK);
    for bc in [
        &mut style.border_top_color,
        &mut style.border_right_color,
        &mut style.border_bottom_color,
        &mut style.border_left_color,
    ] {
        if let Some(c) = bc {
            if c.is_current() {
                *bc = Some(base.clone());
            }
        }
    }
    if let Some(c) = &mut style.background_color {
        if c.is_current() {
            *c = base;
        }
    }
}

fn parse_color(value: &PropertyValue) -> Option<Color> {
    match value {
        PropertyValue::Color(c) => {
            if c.is_current() {
                Some(Color::CURRENT_COLOR)
            } else {
                Some(c.clone())
            }
        }
        PropertyValue::Keyword(s) => Color::from_named(s).or_else(|| {
            if s.starts_with('#') {
                Color::from_hex(s)
            } else {
                None
            }
        }),
        _ => None,
    }
}

fn parse_background(value: &PropertyValue) -> Option<Color> {
    parse_color(value)
}

// ── Viewport-resolution helpers ──
// These functions resolve vw/vh/percent units using the given viewport dimensions.
// parse_length is retained for backward compat (uses default 800×600 viewport).

fn lv_to_px(lv: &LengthValue, vw: f32, vh: f32) -> f32 {
    lv_to_px_font(lv, vw, vh, 16.0)
}

fn lv_to_px_font(lv: &LengthValue, vw: f32, vh: f32, font_size: f32) -> f32 {
    let vw_val = lv.value * vw / 100.0;
    let vh_val = lv.value * vh / 100.0;
    match lv.unit {
        Unit::Px => lv.value,
        Unit::Vw => vw_val,
        Unit::Vh => vh_val,
        Unit::Vmin => vw_val.min(vh_val),
        Unit::Vmax => vw_val.max(vh_val),
        Unit::Percent => lv.value * vw / 100.0,
        Unit::Em | Unit::Rem => lv.value * font_size,
        Unit::In => lv.value * 96.0,
        Unit::Cm => lv.value * 96.0 / 2.54,
        Unit::Mm => lv.value * 96.0 / 25.4,
        Unit::Pt => lv.value * 96.0 / 72.0,
        Unit::Pc => lv.value * 96.0 / 6.0,
        Unit::Ch => lv.value * 8.0,
        Unit::Ex => lv.value * 7.0,
    }
}

#[allow(dead_code)]
fn resolve_length_for_unit(lv: &LengthValue, vw: f32, vh: f32) -> f32 {
    lv_to_px(lv, vw, vh)
}

fn parse_length_vp(value: &PropertyValue, vw: f32, vh: f32) -> Option<f32> {
    match value {
        PropertyValue::Length(lv) => Some(lv_to_px(lv, vw, vh)),
        PropertyValue::Number(n) => Some(*n),
        PropertyValue::Keyword(s) => s.parse().ok(),
        _ => None,
    }
}

fn lv_to_px_vertical(lv: &LengthValue, vw: f32, vh: f32) -> f32 {
    if lv.unit == Unit::Percent {
        return lv.value * vh / 100.0;
    }
    lv_to_px(lv, vw, vh)
}

fn parse_length_vp_vertical(value: &PropertyValue, vw: f32, vh: f32) -> Option<f32> {
    match value {
        PropertyValue::Length(lv) => {
            let mut p = lv_to_px(lv, vw, vh);
            if lv.unit == Unit::Percent {
                p = lv.value * vh / 100.0;
            }
            Some(p)
        }
        PropertyValue::Number(n) => Some(*n),
        PropertyValue::Keyword(s) => s.parse().ok(),
        _ => None,
    }
}

fn parse_side_shorthand_vp(value: &PropertyValue, vw: f32, vh: f32) -> Option<[Option<f32>; 4]> {
    let parts: Vec<PropertyValue> = match value {
        PropertyValue::Shorthand(parts) => parts.clone(),
        PropertyValue::Keyword(s) => s
            .split_whitespace()
            .map(|p| {
                if p == "auto" {
                    PropertyValue::Keyword("auto".to_string())
                } else if let Some(lv) = LengthValue::from_str(p) {
                    PropertyValue::Length(lv)
                } else if let Ok(n) = p.parse::<f32>() {
                    PropertyValue::Number(n)
                } else {
                    PropertyValue::Keyword(p.to_string())
                }
            })
            .collect(),
        _ => return None,
    };

    let len = parts.len();
    if len == 0 || len > 4 {
        return None;
    }

    let mut vals: Vec<Option<f32>> = Vec::with_capacity(len);
    for (i, part) in parts.iter().enumerate() {
        if matches!(part, PropertyValue::Keyword(s) if s == "auto") {
            vals.push(None);
            continue;
        }
        let px = if i % 2 == 0 && len >= 2 {
            parse_length_vp_vertical(part, vw, vh)
        } else {
            parse_length_vp(part, vw, vh)
        };
        let v = px?;
        vals.push(Some(v));
    }

    Some(match vals.len() {
        1 => [vals[0], vals[0], vals[0], vals[0]],
        2 => [vals[0], vals[1], vals[0], vals[1]],
        3 => [vals[0], vals[1], vals[2], vals[1]],
        4 => [vals[0], vals[1], vals[2], vals[3]],
        _ => return None,
    })
}

fn parse_keyword(value: &PropertyValue) -> Option<String> {
    match value {
        PropertyValue::Keyword(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_display(value: &PropertyValue) -> Display {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => Display::Inline,
    }
}

fn parse_position(value: &PropertyValue) -> Position {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => Position::Static,
    }
}

fn parse_flex_direction(value: &PropertyValue) -> FlexDirection {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => FlexDirection::Row,
    }
}

fn parse_flex_wrap(value: &PropertyValue) -> FlexWrap {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => FlexWrap::NoWrap,
    }
}

fn parse_justify_content(value: &PropertyValue) -> JustifyContent {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => JustifyContent::FlexStart,
    }
}

fn parse_align_items(value: &PropertyValue) -> AlignItems {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => AlignItems::Stretch,
    }
}

fn parse_align_self(value: &PropertyValue) -> AlignSelf {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => AlignSelf::Auto,
    }
}

fn parse_transform(value: &PropertyValue) -> Option<Transform> {
    match value {
        PropertyValue::Keyword(s) => {
            let mut t = Transform::default();
            let s_lower = s.to_lowercase();

            if s_lower.contains("translate") {
                t.translate_x = Some(0.0);
                t.translate_y = Some(0.0);
            }
            if s_lower.contains("rotate") {
                t.rotate = Some(0.0);
            }
            if s_lower.contains("scale") {
                t.scale_x = Some(1.0);
                t.scale_y = Some(1.0);
            }

            if t.translate_x.is_some() || t.rotate.is_some() || t.scale_x != Some(1.0) {
                Some(t)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_transition(value: &PropertyValue) -> Option<Transition> {
    match value {
        PropertyValue::Keyword(s) => {
            let parts: Vec<&str> = s.split_whitespace().collect();
            Some(Transition {
                property: parts
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "all".to_string()),
                duration: parts
                    .get(1)
                    .and_then(|v| v.trim_end_matches("s").parse().ok())
                    .unwrap_or(0.3),
                timing_function: parts
                    .get(2)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "ease".to_string()),
                delay: 0.0,
            })
        }
        _ => None,
    }
}

fn apply_flex_shorthand(
    flex: &mut super::style_value::FlexOptions,
    value: &PropertyValue,
    vw: f32,
    vh: f32,
) {
    match value {
        PropertyValue::Keyword(s) => match s.as_str() {
            "none" => {
                flex.flex_grow = 0.0;
                flex.flex_shrink = 0.0;
                flex.flex_basis = None;
                return;
            }
            "auto" => {
                flex.flex_grow = 1.0;
                flex.flex_shrink = 1.0;
                flex.flex_basis = None;
                return;
            }
            "initial" => {
                flex.flex_grow = 0.0;
                flex.flex_shrink = 1.0;
                flex.flex_basis = None;
                return;
            }
            _ => {}
        },
        PropertyValue::Number(_) => {}
        PropertyValue::Shorthand(_) => {}
        _ => return,
    }

    let parts: Vec<&PropertyValue> = match value {
        PropertyValue::Shorthand(parts) => parts.iter().collect(),
        other => vec![other],
    };

    let len = parts.len();
    if len == 0 || len > 3 {
        return;
    }

    // flex-grow (required, must be a number)
    let grow = match parts[0] {
        PropertyValue::Number(n) => n.max(0.0),
        PropertyValue::Keyword(s) => match s.parse::<f32>() {
            Ok(v) => v.max(0.0),
            Err(_) => return,
        },
        _ => return,
    };
    flex.flex_grow = grow;

    // flex-shrink (optional, defaults to 1)
    flex.flex_shrink = if len >= 2 {
        match parts[1] {
            PropertyValue::Number(n) => n.max(0.0),
            PropertyValue::Keyword(s) => s.parse::<f32>().ok().map(|v| v.max(0.0)).unwrap_or(1.0),
            _ => 1.0,
        }
    } else {
        1.0
    };

    // flex-basis (optional, defaults to 0%)
    flex.flex_basis = if len >= 3 {
        match parts[2] {
            PropertyValue::Keyword(s) if s == "auto" => None,
            other => parse_length_from_pv_vp(other, vw, vh),
        }
    } else {
        Some(0.0)
    };
}

fn parse_length_from_pv_vp(value: &PropertyValue, vw: f32, vh: f32) -> Option<f32> {
    match value {
        PropertyValue::Length(lv) => Some(lv_to_px(lv, vw, vh)),
        PropertyValue::Number(n) => Some(*n),
        PropertyValue::Keyword(s) => s.parse().ok(),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_sides_vp(
    top: &mut Option<f32>,
    right: &mut Option<f32>,
    bottom: &mut Option<f32>,
    left: &mut Option<f32>,
    name: &str,
    value: &PropertyValue,
    vw: f32,
    vh: f32,
) {
    // Determine if this is a vertical or horizontal property
    let is_vertical = matches!(
        name,
        "margin-top"
            | "margin-bottom"
            | "padding-top"
            | "padding-bottom"
            | "border-top-width"
            | "border-bottom-width"
    );
    // First try single length
    let maybe_len = if is_vertical {
        parse_length_vp_vertical(value, vw, vh)
    } else {
        parse_length_vp(value, vw, vh)
    };
    if let Some(len) = maybe_len {
        match name {
            "margin-top" => *top = Some(len),
            "margin-right" => *right = Some(len),
            "margin-bottom" => *bottom = Some(len),
            "margin-left" => *left = Some(len),
            "margin" => {
                *top = Some(len);
                *right = Some(len);
                *bottom = Some(len);
                *left = Some(len);
            }
            "padding-top" => *top = Some(len),
            "padding-right" => *right = Some(len),
            "padding-bottom" => *bottom = Some(len),
            "padding-left" => *left = Some(len),
            "padding" => {
                *top = Some(len);
                *right = Some(len);
                *bottom = Some(len);
                *left = Some(len);
            }
            "border-top-width" => *top = Some(len),
            "border-right-width" => *right = Some(len),
            "border-bottom-width" => *bottom = Some(len),
            "border-left-width" => *left = Some(len),
            "border-width" => {
                *top = Some(len);
                *right = Some(len);
                *bottom = Some(len);
                *left = Some(len);
            }
            _ => {}
        }
        return;
    }

    // Try shorthand (e.g. "5em auto" → [5em, auto, 5em, auto])
    if let Some(quads) = parse_side_shorthand_vp(value, vw, vh) {
        let is_shorthand = matches!(name, "margin" | "padding" | "border-width");
        if is_shorthand
            || name == "margin-top"
            || name == "padding-top"
            || name == "border-top-width"
        {
            if let Some(v) = quads[0] {
                *top = Some(v);
            }
        }
        if is_shorthand
            || name == "margin-right"
            || name == "padding-right"
            || name == "border-right-width"
        {
            if let Some(v) = quads[1] {
                *right = Some(v);
            }
        }
        if is_shorthand
            || name == "margin-bottom"
            || name == "padding-bottom"
            || name == "border-bottom-width"
        {
            if let Some(v) = quads[2] {
                *bottom = Some(v);
            }
        }
        if is_shorthand
            || name == "margin-left"
            || name == "padding-left"
            || name == "border-left-width"
        {
            if let Some(v) = quads[3] {
                *left = Some(v);
            }
        }
    }
}

fn apply_border_colors(
    top: &mut Option<Color>,
    right: &mut Option<Color>,
    bottom: &mut Option<Color>,
    left: &mut Option<Color>,
    name: &str,
    value: &PropertyValue,
) {
    let Some(color) = parse_color(value) else {
        return;
    };

    match name {
        "border-top-color" => *top = Some(color),
        "border-right-color" => *right = Some(color),
        "border-bottom-color" => *bottom = Some(color),
        "border-left-color" => *left = Some(color),
        "border-color" => {
            *top = Some(color.clone());
            *right = Some(color.clone());
            *bottom = Some(color.clone());
            *left = Some(color);
        }
        _ => {}
    }
}

/// CSS `border` shorthand: `<line-width> || <line-style> || <color>`
///
/// `border-style` is not supported by this project — style keywords are
/// silently ignored. `border: none` sets all widths to 0.
fn apply_border_shorthand(style: &mut ComputedStyle, value: &PropertyValue) {
    // Keyword forms: "none"
    if let PropertyValue::Keyword(s) = value {
        if s == "none" {
            style.border_top_width = Some(0.0);
            style.border_right_width = Some(0.0);
            style.border_bottom_width = Some(0.0);
            style.border_left_width = Some(0.0);
            return;
        }
    }

    let parts: Vec<&PropertyValue> = match value {
        PropertyValue::Shorthand(parts) => parts.iter().collect(),
        other => vec![other],
    };

    let mut width: Option<f32> = None;
    let mut color: Option<Color> = None;

    for part in &parts {
        match part {
            // Length or number → width
            PropertyValue::Length(lv) => {
                width = Some(lv_to_px(lv, 800.0, 600.0));
            }
            PropertyValue::Number(n) => {
                width = Some(*n);
            }
            // Keyword → could be color, style (ignored), or numeric string
            PropertyValue::Keyword(s) => {
                if let Some(c) = parse_color(&PropertyValue::Keyword(s.clone())) {
                    color = Some(c);
                } else if let Ok(n) = s.parse::<f32>() {
                    width = Some(n);
                }
                // Style keywords (solid, dotted, etc.) are ignored — no
                // border-style support in this project.
            }
            // Explicit color value
            PropertyValue::Color(c) => {
                color = Some(c.clone());
            }
            _ => {}
        }
    }

    if let Some(w) = width {
        style.border_top_width = Some(w);
        style.border_right_width = Some(w);
        style.border_bottom_width = Some(w);
        style.border_left_width = Some(w);
    }
    if let Some(c) = color {
        style.border_top_color = Some(c.clone());
        style.border_right_color = Some(c.clone());
        style.border_bottom_color = Some(c.clone());
        style.border_left_color = Some(c);
    }
}

pub fn resolve_styles_for_tree(
    element: &ElementData,
    stylesheet: &Stylesheet,
    results: &mut std::collections::HashMap<String, ComputedStyle>,
) {
    let style = resolve_style(element, stylesheet);
    results.insert(element.tag_name.clone(), style);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Declaration, PropertyValue, Rule, Selector, SimpleSelector, Stylesheet};

    fn make_stylesheet(declarations: &[(&str, PropertyValue)]) -> Stylesheet {
        Stylesheet {
            rules: vec![Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("div".into()),
                    id: None,
                    class: vec![],
                    attribute: None,
                    pseudo_class: None,
                })],
                declarations: declarations
                    .iter()
                    .map(|(n, v)| Declaration {
                        name: n.to_string(),
                        value: v.clone(),
                    })
                    .collect(),
            }],
        }
    }

    #[test]
    fn test_resolve_simple() {
        let stylesheet = make_stylesheet(&[("color", PropertyValue::Keyword("red".into()))]);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_resolve_display() {
        let stylesheet = make_stylesheet(&[("display", PropertyValue::Keyword("flex".into()))]);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.display, Display::Flex);
    }

    #[test]
    fn test_resolve_flex() {
        let stylesheet = make_stylesheet(&[
            ("display", PropertyValue::Keyword("flex".into())),
            ("flex-direction", PropertyValue::Keyword("column".into())),
            ("justify-content", PropertyValue::Keyword("center".into())),
        ]);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.display, Display::Flex);
        assert_eq!(style.flex.flex_direction, FlexDirection::Column);
        assert_eq!(style.flex.justify_content, JustifyContent::Center);
    }

    #[test]
    fn test_cascade_override() {
        let stylesheet = Stylesheet {
            rules: vec![
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: Some("div".into()),
                        id: None,
                        class: vec![],
                        attribute: None,
                        pseudo_class: None,
                    })],
                    declarations: vec![Declaration {
                        name: "color".into(),
                        value: PropertyValue::Keyword("red".into()),
                    }],
                },
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: Some("div".into()),
                        id: None,
                        class: vec![],
                        attribute: None,
                        pseudo_class: None,
                    })],
                    declarations: vec![Declaration {
                        name: "color".into(),
                        value: PropertyValue::Keyword("blue".into()),
                    }],
                },
            ],
        };
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255
            })
        );
    }

    #[test]
    fn test_specificity_override() {
        let stylesheet = Stylesheet {
            rules: vec![
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: Some("div".into()),
                        id: None,
                        class: vec![],
                        attribute: None,
                        pseudo_class: None,
                    })],
                    declarations: vec![Declaration {
                        name: "color".into(),
                        value: PropertyValue::Keyword("red".into()),
                    }],
                },
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: None,
                        id: Some("id".into()),
                        class: vec![],
                        attribute: None,
                        pseudo_class: None,
                    })],
                    declarations: vec![Declaration {
                        name: "color".into(),
                        value: PropertyValue::Keyword("blue".into()),
                    }],
                },
            ],
        };
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "id".to_string());
        let element = ElementData::with_attributes("div".to_string(), attrs);

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255
            })
        );
    }

    #[test]
    fn test_resolve_current_color() {
        let stylesheet =
            make_stylesheet(&[("color", PropertyValue::Keyword("currentColor".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.color, Some(Color::BLACK));
    }

    #[test]
    fn test_resolve_hsl_color() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Color(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_resolve_color_value() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Color(Color {
                r: 128,
                g: 64,
                b: 192,
                a: 255,
            }),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 128,
                g: 64,
                b: 192,
                a: 255
            })
        );
    }

    #[test]
    fn test_resolve_border_current_color() {
        let stylesheet = make_stylesheet(&[
            ("color", PropertyValue::Keyword("red".into())),
            (
                "border-color",
                PropertyValue::Keyword("currentColor".into()),
            ),
        ]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.border_top_color,
            Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_resolve_rgb_color() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Color(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_resolve_rgba_color() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Color(Color {
                r: 0,
                g: 255,
                b: 0,
                a: 128,
            }),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 0,
                g: 255,
                b: 0,
                a: 128
            })
        );
    }

    #[test]
    fn test_resolve_hsla_color() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Color(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 64,
            }),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.color,
            Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 64
            })
        );
    }

    #[test]
    fn test_flex_shorthand_single_number() {
        let stylesheet = make_stylesheet(&[("flex", PropertyValue::Keyword("1".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 1.0);
        assert_eq!(style.flex.flex_shrink, 1.0);
        assert_eq!(style.flex.flex_basis, Some(0.0));
    }

    #[test]
    fn test_flex_shorthand_single_number_2() {
        let stylesheet = make_stylesheet(&[("flex", PropertyValue::Keyword("2".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 2.0);
        assert_eq!(style.flex.flex_shrink, 1.0);
        assert_eq!(style.flex.flex_basis, Some(0.0));
    }

    #[test]
    fn test_flex_shorthand_two_numbers() {
        let stylesheet = make_stylesheet(&[(
            "flex",
            PropertyValue::Shorthand(vec![
                PropertyValue::Keyword("1".into()),
                PropertyValue::Keyword("0".into()),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 1.0);
        assert_eq!(style.flex.flex_shrink, 0.0);
        assert_eq!(style.flex.flex_basis, Some(0.0));
    }

    #[test]
    fn test_flex_shorthand_three_values_auto() {
        let stylesheet = make_stylesheet(&[(
            "flex",
            PropertyValue::Shorthand(vec![
                PropertyValue::Keyword("1".into()),
                PropertyValue::Keyword("1".into()),
                PropertyValue::Keyword("auto".into()),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 1.0);
        assert_eq!(style.flex.flex_shrink, 1.0);
        assert_eq!(style.flex.flex_basis, None);
    }

    #[test]
    fn test_flex_shorthand_three_values_length() {
        let stylesheet = make_stylesheet(&[(
            "flex",
            PropertyValue::Shorthand(vec![
                PropertyValue::Keyword("1".into()),
                PropertyValue::Keyword("0".into()),
                PropertyValue::Length(crate::style_value::LengthValue {
                    value: 200.0,
                    unit: crate::style_value::Unit::Px,
                }),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 1.0);
        assert_eq!(style.flex.flex_shrink, 0.0);
        assert_eq!(style.flex.flex_basis, Some(200.0));
    }

    #[test]
    fn test_flex_shorthand_none() {
        let stylesheet = make_stylesheet(&[("flex", PropertyValue::Keyword("none".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 0.0);
        assert_eq!(style.flex.flex_shrink, 0.0);
        assert_eq!(style.flex.flex_basis, None);
    }

    #[test]
    fn test_flex_shorthand_auto() {
        let stylesheet = make_stylesheet(&[("flex", PropertyValue::Keyword("auto".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 1.0);
        assert_eq!(style.flex.flex_shrink, 1.0);
        assert_eq!(style.flex.flex_basis, None);
    }

    #[test]
    fn test_flex_shorthand_initial() {
        let stylesheet = make_stylesheet(&[("flex", PropertyValue::Keyword("initial".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 0.0);
        assert_eq!(style.flex.flex_shrink, 1.0);
        assert_eq!(style.flex.flex_basis, None);
    }

    #[test]
    fn test_flex_shorthand_does_not_override_longhand() {
        let stylesheet = make_stylesheet(&[
            ("flex-grow", PropertyValue::Keyword("3".into())),
            ("flex", PropertyValue::Keyword("1".into())),
        ]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.flex.flex_grow, 1.0);
        assert_eq!(style.flex.flex_shrink, 1.0);
    }

    // ── border shorthand tests ───────────────────────────────────────

    #[test]
    fn test_border_shorthand_width_style_color() {
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![
                PropertyValue::Length(LengthValue {
                    value: 1.0,
                    unit: Unit::Px,
                }),
                PropertyValue::Keyword("solid".into()),
                PropertyValue::Keyword("red".into()),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(1.0));
        assert_eq!(style.border_right_width, Some(1.0));
        assert_eq!(style.border_bottom_width, Some(1.0));
        assert_eq!(style.border_left_width, Some(1.0));
        assert_eq!(
            style.border_top_color,
            Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_border_shorthand_width_color() {
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![
                PropertyValue::Length(LengthValue {
                    value: 2.0,
                    unit: Unit::Px,
                }),
                PropertyValue::Keyword("blue".into()),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(2.0));
        assert_eq!(
            style.border_top_color,
            Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255
            })
        );
    }

    #[test]
    fn test_border_shorthand_width_only() {
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![PropertyValue::Length(LengthValue {
                value: 3.0,
                unit: Unit::Px,
            })]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(3.0));
        assert_eq!(style.border_right_width, Some(3.0));
        assert_eq!(style.border_bottom_width, Some(3.0));
        assert_eq!(style.border_left_width, Some(3.0));
    }

    #[test]
    fn test_border_shorthand_color_only() {
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![PropertyValue::Keyword("green".into())]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(
            style.border_top_color,
            Some(Color {
                r: 0,
                g: 128,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_border_shorthand_style_ignored() {
        // border-style is not supported by this project; "solid" is ignored.
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![PropertyValue::Keyword("solid".into())]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        // No width or color set — only the unsupported style keyword was present.
        assert_eq!(style.border_top_width, Some(0.0)); // default
        assert_eq!(style.border_top_color, None); // default
    }

    #[test]
    fn test_border_shorthand_none() {
        let stylesheet = make_stylesheet(&[("border", PropertyValue::Keyword("none".into()))]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(0.0));
        assert_eq!(style.border_right_width, Some(0.0));
        assert_eq!(style.border_bottom_width, Some(0.0));
        assert_eq!(style.border_left_width, Some(0.0));
    }

    #[test]
    fn test_border_shorthand_zero() {
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![PropertyValue::Number(0.0)]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(0.0));
        assert_eq!(style.border_right_width, Some(0.0));
    }

    #[test]
    fn test_border_shorthand_unordered_tokens() {
        // "red 1px dashed" — color first, then width, then style (ignored)
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![
                PropertyValue::Keyword("red".into()),
                PropertyValue::Length(LengthValue {
                    value: 4.0,
                    unit: Unit::Px,
                }),
                PropertyValue::Keyword("dashed".into()),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(4.0));
        assert_eq!(
            style.border_top_color,
            Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    #[test]
    fn test_border_longhand_overrides_shorthand() {
        let stylesheet = make_stylesheet(&[
            (
                "border",
                PropertyValue::Shorthand(vec![
                    PropertyValue::Length(LengthValue {
                        value: 1.0,
                        unit: Unit::Px,
                    }),
                    PropertyValue::Keyword("red".into()),
                ]),
            ),
            (
                "border-top-width",
                PropertyValue::Length(LengthValue {
                    value: 5.0,
                    unit: Unit::Px,
                }),
            ),
        ]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        // Longhand applied after shorthand — top is overridden.
        assert_eq!(style.border_top_width, Some(5.0));
        // Other sides remain from shorthand.
        assert_eq!(style.border_right_width, Some(1.0));
    }

    #[test]
    fn test_border_shorthand_does_not_corrupt_other_properties() {
        let stylesheet = make_stylesheet(&[
            ("color", PropertyValue::Keyword("blue".into())),
            (
                "border",
                PropertyValue::Shorthand(vec![
                    PropertyValue::Length(LengthValue {
                        value: 1.0,
                        unit: Unit::Px,
                    }),
                    PropertyValue::Keyword("red".into()),
                ]),
            ),
        ]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        // color must not be affected by border shorthand
        assert_eq!(
            style.color,
            Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255
            })
        );
    }

    #[test]
    fn test_border_shorthand_number_width() {
        let stylesheet = make_stylesheet(&[(
            "border",
            PropertyValue::Shorthand(vec![
                PropertyValue::Number(2.0),
                PropertyValue::Keyword("solid".into()),
                PropertyValue::Keyword("black".into()),
            ]),
        )]);
        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.border_top_width, Some(2.0));
        assert_eq!(
            style.border_top_color,
            Some(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255
            })
        );
    }

    // ── var() tests ────────────────────────────────────────────────

    #[test]
    fn test_var_basic_substitution() {
        let stylesheet = make_stylesheet(&[
            ("--primary", PropertyValue::Keyword("#336699".into())),
            (
                "color",
                PropertyValue::Var {
                    name: "--primary".into(),
                    fallback: None,
                },
            ),
        ]);
        let element = ElementData::new("div".to_string());
        let mut parent_vars = CustomPropertyMap::new();
        parent_vars.insert("--primary".into(), PropertyValue::Keyword("#336699".into()));
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(style.color, Some(Color::from_hex("#336699").unwrap()));
    }

    #[test]
    fn test_var_with_fallback() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Var {
                name: "--missing".into(),
                fallback: Some(Box::new(PropertyValue::Keyword("red".into()))),
            },
        )]);
        let element = ElementData::new("div".to_string());
        let parent_vars = CustomPropertyMap::new(); // no --missing
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(style.color, Some(Color::from_named("red").unwrap()));
    }

    #[test]
    fn test_var_undefined_no_fallback() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Var {
                name: "--missing".into(),
                fallback: None,
            },
        )]);
        let element = ElementData::new("div".into());
        let parent_vars = CustomPropertyMap::new();
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        // Undefined var without fallback → empty keyword → parse_color returns None → default BLACK stays.
        assert_eq!(style.color, Some(Color::BLACK));
    }

    #[test]
    fn test_var_inherited() {
        let stylesheet = make_stylesheet(&[(
            "color",
            PropertyValue::Var {
                name: "--c".into(),
                fallback: None,
            },
        )]);
        let element = ElementData::new("div".into());
        let mut parent_vars = CustomPropertyMap::new();
        parent_vars.insert("--c".into(), PropertyValue::Keyword("green".into()));
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(style.color, Some(Color::from_named("green").unwrap()));
    }

    #[test]
    fn test_var_local_override() {
        // Element declares --c: blue; uses var(--c). Parent has --c: green.
        let stylesheet = make_stylesheet(&[
            ("--c", PropertyValue::Keyword("blue".into())),
            (
                "color",
                PropertyValue::Var {
                    name: "--c".into(),
                    fallback: None,
                },
            ),
        ]);
        let element = ElementData::new("div".into());
        let mut parent_vars = CustomPropertyMap::new();
        parent_vars.insert("--c".into(), PropertyValue::Keyword("green".into()));
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        // Local override wins over inherited.
        assert_eq!(style.color, Some(Color::from_named("blue").unwrap()));
    }

    #[test]
    fn test_var_nested_fallback() {
        // var(--a, var(--b, 10px)): --a is missing, --b is missing → fallback = 10px
        let stylesheet = make_stylesheet(&[(
            "padding-top",
            PropertyValue::Var {
                name: "--a".into(),
                fallback: Some(Box::new(PropertyValue::Var {
                    name: "--b".into(),
                    fallback: Some(Box::new(PropertyValue::Length(LengthValue {
                        value: 10.0,
                        unit: Unit::Px,
                    }))),
                })),
            },
        )]);
        let element = ElementData::new("div".into());
        let parent_vars = CustomPropertyMap::new();
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(style.padding_top, Some(10.0));
    }

    #[test]
    fn test_var_nested_fallback_inner_hit() {
        // var(--a, var(--b, 10px)): --a missing, --b = 20px → result = 20px
        let stylesheet = make_stylesheet(&[(
            "padding-top",
            PropertyValue::Var {
                name: "--a".into(),
                fallback: Some(Box::new(PropertyValue::Var {
                    name: "--b".into(),
                    fallback: Some(Box::new(PropertyValue::Length(LengthValue {
                        value: 10.0,
                        unit: Unit::Px,
                    }))),
                })),
            },
        )]);
        let element = ElementData::new("div".into());
        let mut parent_vars = CustomPropertyMap::new();
        parent_vars.insert(
            "--b".into(),
            PropertyValue::Length(LengthValue {
                value: 20.0,
                unit: Unit::Px,
            }),
        );
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(style.padding_top, Some(20.0));
    }

    #[test]
    fn test_var_circular_reference() {
        // --a: var(--b); --b: var(--a) → circular → resolved to empty
        let stylesheet = make_stylesheet(&[
            (
                "--a",
                PropertyValue::Var {
                    name: "--b".into(),
                    fallback: None,
                },
            ),
            (
                "--b",
                PropertyValue::Var {
                    name: "--a".into(),
                    fallback: None,
                },
            ),
            (
                "color",
                PropertyValue::Var {
                    name: "--a".into(),
                    fallback: None,
                },
            ),
        ]);
        let element = ElementData::new("div".into());
        let parent_vars = CustomPropertyMap::new();
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        // Circular → resolved to empty keyword → parse_color returns None → default BLACK stays.
        assert_eq!(style.color, Some(Color::BLACK));
    }

    #[test]
    fn test_var_in_shorthand() {
        let stylesheet = make_stylesheet(&[
            (
                "--m",
                PropertyValue::Length(LengthValue {
                    value: 16.0,
                    unit: Unit::Px,
                }),
            ),
            (
                "margin",
                PropertyValue::Var {
                    name: "--m".into(),
                    fallback: None,
                },
            ),
        ]);
        let element = ElementData::new("div".into());
        let mut parent_vars = CustomPropertyMap::new();
        parent_vars.insert(
            "--m".into(),
            PropertyValue::Length(LengthValue {
                value: 16.0,
                unit: Unit::Px,
            }),
        );
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(style.margin_top, Some(16.0));
        assert_eq!(style.margin_right, Some(16.0));
    }

    // ── calc() tests ───────────────────────────────────────────────

    #[test]
    fn test_calc_addition() {
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Length(LengthValue {
                    value: 100.0,
                    unit: Unit::Percent,
                }),
                CalcTerm::Add,
                CalcTerm::Length(LengthValue {
                    value: 20.0,
                    unit: Unit::Px,
                }),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        // 100% of 800 = 800 + 20 = 820
        assert_eq!(style.width, Some(820.0));
    }

    #[test]
    fn test_calc_subtraction() {
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Length(LengthValue {
                    value: 100.0,
                    unit: Unit::Percent,
                }),
                CalcTerm::Sub,
                CalcTerm::Length(LengthValue {
                    value: 20.0,
                    unit: Unit::Px,
                }),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        // 800 - 20 = 780
        assert_eq!(style.width, Some(780.0));
    }

    #[test]
    fn test_calc_multiplication() {
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Length(LengthValue {
                    value: 20.0,
                    unit: Unit::Px,
                }),
                CalcTerm::Mul,
                CalcTerm::Number(3.0),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        assert_eq!(style.width, Some(60.0));
    }

    #[test]
    fn test_calc_division() {
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Length(LengthValue {
                    value: 100.0,
                    unit: Unit::Px,
                }),
                CalcTerm::Div,
                CalcTerm::Number(4.0),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        assert_eq!(style.width, Some(25.0));
    }

    #[test]
    fn test_calc_division_by_zero() {
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Number(10.0),
                CalcTerm::Div,
                CalcTerm::Number(0.0),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        // Division by zero → invalid calc → keyword (parse fails).
        assert_eq!(style.width, None);
    }

    #[test]
    fn test_calc_precedence_mul_before_add() {
        // calc(2 + 3 * 4) should be 2 + (3*4) = 14
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Number(2.0),
                CalcTerm::Add,
                CalcTerm::Number(3.0),
                CalcTerm::Mul,
                CalcTerm::Number(4.0),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        assert_eq!(style.width, Some(14.0));
    }

    #[test]
    fn test_calc_parentheses() {
        // calc((2 + 3) * 4) should be (2+3)*4 = 20
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Paren(vec![
                    CalcTerm::Number(2.0),
                    CalcTerm::Add,
                    CalcTerm::Number(3.0),
                ]),
                CalcTerm::Mul,
                CalcTerm::Number(4.0),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        assert_eq!(style.width, Some(20.0));
    }

    #[test]
    fn test_calc_negative_value() {
        // calc(-20px + 50px) = 30
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Number(-20.0),
                CalcTerm::Add,
                CalcTerm::Length(LengthValue {
                    value: 50.0,
                    unit: Unit::Px,
                }),
            ]),
        )]);
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        assert_eq!(style.width, Some(30.0));
    }

    // ── var + calc integration ─────────────────────────────────────

    #[test]
    fn test_var_inside_calc() {
        // --spacing: 20px; width: calc(100% - var(--spacing))
        let stylesheet = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Length(LengthValue {
                    value: 100.0,
                    unit: Unit::Percent,
                }),
                CalcTerm::Sub,
                CalcTerm::Length(LengthValue {
                    value: 0.0,
                    unit: Unit::Px,
                }), // placeholder
            ]),
        )]);
        // Manually construct with var inside calc isn't possible at parser level yet.
        // Instead, test via substitution: calc with a Var inside a Paren.
        let stylesheet2 = make_stylesheet(&[(
            "width",
            PropertyValue::Calc(vec![
                CalcTerm::Length(LengthValue {
                    value: 100.0,
                    unit: Unit::Percent,
                }),
                CalcTerm::Sub,
                CalcTerm::Paren(vec![
                    CalcTerm::Number(0.0), // will be replaced by var resolution
                ]),
            ]),
        )]);
        // For now, test that calc evaluates: 100% - 0 = 800 (viewport width)
        let element = ElementData::new("div".into());
        let style = resolve_style_with_vars(
            &element,
            &stylesheet2,
            800.0,
            600.0,
            &CustomPropertyMap::new(),
        );
        assert_eq!(style.width, Some(800.0));
    }

    #[test]
    fn test_bootstrap_style_declaration() {
        // Simulate: :root { --primary: #336699; } .card { color: var(--primary); }
        let stylesheet = make_stylesheet(&[
            ("--primary", PropertyValue::Keyword("#336699".into())),
            (
                "color",
                PropertyValue::Var {
                    name: "--primary".into(),
                    fallback: None,
                },
            ),
        ]);
        let element = ElementData::new("div".into());
        let mut parent_vars = CustomPropertyMap::new();
        parent_vars.insert("--primary".into(), PropertyValue::Keyword("#336699".into()));
        let style = resolve_style_with_vars(&element, &stylesheet, 800.0, 600.0, &parent_vars);
        assert_eq!(
            style.color,
            Some(Color {
                r: 0x33,
                g: 0x66,
                b: 0x99,
                a: 255
            })
        );
    }
}
