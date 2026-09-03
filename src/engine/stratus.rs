//! Stratus CSS Engine — cssparser-based wrapper
//!
//! Uses cssparser for robust CSS tokenization and parsing.
//! Re-exports style types from aether_css and provides a new cssparser-based `parse` function.

pub use aether_css::{
    apply_inheritance, match_element, match_rules, resolve_style, resolve_style_vp,
    resolve_style_with_vars, resolve_style_with_vars_and_custom, resolve_styles_for_tree,
    AlignContent, AlignItems, AlignSelf, CalcTerm, Color, ComputedStyle, CssPropertyName,
    CustomPropertyMap, Declaration, Display, ElementData, FlexDirection, FlexOptions, FlexWrap,
    InheritMask, JustifyContent, LengthValue, Position, PropertyValue, Rule, Selector,
    SimpleSelector, Specificity, Stylesheet, Transform, Transition, Unit,
};

use cssparser::{BasicParseErrorKind, Delimiter, ParseError, Parser, ParserInput, Token};

pub fn parse(css: &str) -> Stylesheet {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    let mut stylesheet = Stylesheet::new();

    // Every branch below consumes at least one byte on failure, so the loop
    // always makes progress and terminates with the parser. Input-size caps
    // live upstream (fetcher per-source/cumulative budgets), not here.
    while !parser.is_exhausted() {
        parser.skip_whitespace();
        if parser.is_exhausted() {
            break;
        }

        match parser.try_parse(parse_rule) {
            Ok(rule) => stylesheet.rules.push(rule),
            Err(_) => match parser.try_parse(parse_at_rule) {
                Ok(rules) => stylesheet.rules.extend(rules),
                Err(_) => {
                    let _ = parser.next();
                }
            },
        }
    }

    stylesheet
}

fn parse_rule<'i, 't>(p: &mut Parser<'i, 't>) -> Result<Rule, ParseError<'i, ()>> {
    p.skip_whitespace();

    let mut selectors = Vec::new();
    let mut has_selectors = false;

    loop {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }

        match parse_simple_selector(p) {
            Ok(sel) => {
                if sel.tag_name.is_some() || sel.id.is_some() || !sel.class.is_empty() {
                    selectors.push(Selector::Simple(sel));
                    has_selectors = true;
                }
            }
            Err(_) => break,
        }

        p.skip_whitespace();
        let token = p.next()?.clone();
        match &token {
            Token::Comma => continue,
            Token::CurlyBracketBlock => {
                let declarations = p.parse_nested_block(parse_declarations)?;
                if has_selectors {
                    return Ok(Rule {
                        selectors,
                        declarations,
                    });
                } else {
                    return Err(p.new_error(BasicParseErrorKind::EndOfInput));
                }
            }
            other => {
                let e = ParseError {
                    kind: cssparser::ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(
                        other.clone(),
                    )),
                    location: p.current_source_location(),
                };
                return Err(e);
            }
        }
    }

    if has_selectors {
        Ok(Rule {
            selectors,
            declarations: Vec::new(),
        })
    } else {
        Err(p.new_error(BasicParseErrorKind::EndOfInput))
    }
}

fn parse_simple_selector<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<SimpleSelector, ParseError<'i, ()>> {
    let mut selector = SimpleSelector::new();
    let mut found = false;

    loop {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }

        let parsed = p.try_parse(|p| -> Result<FoundComponent, ParseError<'i, ()>> {
            let token = p.next()?.clone();
            Ok(match &token {
                Token::Ident(name) => FoundComponent::Tag(name.to_string()),
                Token::IDHash(name) => FoundComponent::Id(name.to_string()),
                Token::Hash(name) => FoundComponent::Id(name.to_string()),
                Token::Delim('.') => {
                    let nxt = p.next()?.clone();
                    if let Token::Ident(name) = &nxt {
                        FoundComponent::Class(name.to_string())
                    } else {
                        return Err(ParseError {
                            kind: cssparser::ParseErrorKind::Basic(
                                BasicParseErrorKind::UnexpectedToken(nxt),
                            ),
                            location: p.current_source_location(),
                        });
                    }
                }
                Token::Colon => {
                    let nxt = p.next()?.clone();
                    if let Token::Ident(name) = &nxt {
                        FoundComponent::Pseudo(name.to_string())
                    } else {
                        return Err(ParseError {
                            kind: cssparser::ParseErrorKind::Basic(
                                BasicParseErrorKind::UnexpectedToken(nxt),
                            ),
                            location: p.current_source_location(),
                        });
                    }
                }
                Token::Delim('*') => FoundComponent::Universal,
                _ => {
                    return Err(ParseError {
                        kind: cssparser::ParseErrorKind::Basic(
                            BasicParseErrorKind::UnexpectedToken(token),
                        ),
                        location: p.current_source_location(),
                    });
                }
            })
        });

        match parsed {
            Ok(FoundComponent::Tag(name)) => {
                found = true;
                if selector.tag_name.is_none() {
                    selector.tag_name = Some(name);
                }
            }
            Ok(FoundComponent::Id(name)) => {
                found = true;
                selector.id = Some(name);
            }
            Ok(FoundComponent::Class(name)) => {
                found = true;
                selector.class.push(name);
            }
            Ok(FoundComponent::Pseudo(name)) => {
                found = true;
                selector.pseudo_class = Some(name);
            }
            Ok(FoundComponent::Universal) => {
                found = true;
            }
            Err(_) => break,
        }
    }

    if !found {
        Err(p.new_error(BasicParseErrorKind::EndOfInput))
    } else {
        Ok(selector)
    }
}

enum FoundComponent {
    Tag(String),
    Id(String),
    Class(String),
    Pseudo(String),
    Universal,
}

fn parse_at_rule<'i, 't>(p: &mut Parser<'i, 't>) -> Result<Vec<Rule>, ParseError<'i, ()>> {
    let token = p.next()?.clone();
    match &token {
        Token::AtKeyword(name) if name.eq_ignore_ascii_case("media") => {
            let mut found_block = false;
            while !p.is_exhausted() {
                let t = p.next()?.clone();
                if matches!(t, Token::CurlyBracketBlock) {
                    found_block = true;
                    break;
                }
            }
            if !found_block {
                return Ok(Vec::new());
            }
            p.parse_nested_block(parse_rules_in_block)
        }
        _ => loop {
            let t = p.next()?.clone();
            match &t {
                Token::Semicolon => return Ok(Vec::new()),
                Token::CurlyBracketBlock => {
                    return p.parse_nested_block(|p| -> Result<Vec<Rule>, ParseError<'_, ()>> {
                        while !p.is_exhausted() {
                            p.skip_whitespace();
                            if p.is_exhausted() {
                                break;
                            }
                            let _ = p.next()?;
                        }
                        Ok(Vec::new())
                    });
                }
                _ => continue,
            }
        },
    }
}

fn parse_rules_in_block<'i, 't>(p: &mut Parser<'i, 't>) -> Result<Vec<Rule>, ParseError<'i, ()>> {
    let mut rules = Vec::new();
    while !p.is_exhausted() {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }

        match p.try_parse(parse_rule) {
            Ok(rule) => rules.push(rule),
            Err(_) => match p.try_parse(parse_at_rule) {
                Ok(more) => rules.extend(more),
                Err(_) => {
                    let _ = p.next();
                }
            },
        }
    }
    Ok(rules)
}

fn parse_declarations<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<Vec<Declaration>, ParseError<'i, ()>> {
    let mut declarations = Vec::new();

    while !p.is_exhausted() {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }

        let parsed = p.try_parse(|p| -> Result<Declaration, ParseError<'i, ()>> {
            let name = parse_property_name(p)?;
            p.skip_whitespace();
            p.expect_colon()?;
            p.skip_whitespace();

            // Custom properties (--*) store raw value tokens; do not parse through
            // normal value pipeline.
            let value = if name.starts_with("--") {
                parse_raw_value(p)?
            } else {
                parse_value(p)?
            };

            Ok(Declaration { name, value })
        });

        match parsed {
            Ok(decl) => {
                declarations.push(decl);
            }
            Err(_) => {
                let _ = p.next();
            }
        }
    }

    Ok(declarations)
}

/// Parse a property name: either a standard identifier or a dashed
/// identifier (custom property `--*`).
fn parse_property_name<'i, 't>(p: &mut Parser<'i, 't>) -> Result<String, ParseError<'i, ()>> {
    let name = p.expect_ident_cloned()?;
    Ok(name.to_string())
}

/// Parse a custom property value as raw tokens (no type-level parsing).
/// Custom properties store arbitrary CSS token sequences.
fn parse_raw_value<'i, 't>(p: &mut Parser<'i, 't>) -> Result<PropertyValue, ParseError<'i, ()>> {
    let mut tokens = Vec::new();
    p.parse_until_before(Delimiter::Semicolon, |p| {
        while !p.is_exhausted() {
            p.skip_whitespace();
            if p.is_exhausted() {
                break;
            }
            let tok = p.next()?.clone();
            tokens.push(tok_to_raw_value(p, &tok)?);
        }
        Ok(())
    })?;

    if tokens.len() == 1 {
        Ok(tokens.into_iter().next().unwrap())
    } else if tokens.is_empty() {
        Ok(PropertyValue::Keyword(String::new()))
    } else {
        Ok(PropertyValue::Shorthand(tokens))
    }
}

/// Convert a cssparser token to a raw PropertyValue for custom property storage.
fn tok_to_raw_value<'i, 't>(
    p: &mut Parser<'i, 't>,
    token: &Token<'i>,
) -> Result<PropertyValue, ParseError<'i, ()>> {
    match token {
        Token::Number { value, .. } => Ok(PropertyValue::Number(*value)),
        Token::Dimension { value, unit, .. } => Ok(PropertyValue::Length(LengthValue {
            value: *value,
            unit: unit_to_enum(unit.as_ref()),
        })),
        Token::Percentage { unit_value, .. } => Ok(PropertyValue::Length(LengthValue {
            value: *unit_value * 100.0,
            unit: Unit::Percent,
        })),
        Token::Ident(name) => Ok(PropertyValue::Keyword(name.to_string())),
        Token::Hash(name) | Token::IDHash(name) => Ok(PropertyValue::Keyword(format!("#{}", name))),
        Token::Function(name) => {
            if name.eq_ignore_ascii_case("var") {
                return parse_var_function(p);
            }
            let _ = p.parse_nested_block(|p| -> Result<(), ParseError<'_, ()>> {
                while !p.is_exhausted() {
                    let _ = p.next()?;
                }
                Ok(())
            });
            Ok(PropertyValue::Keyword(name.to_string()))
        }
        Token::Delim(c) => Ok(PropertyValue::Keyword(c.to_string())),
        _ => Ok(PropertyValue::Keyword(format!("{:?}", token))),
    }
}

fn parse_value<'i, 't>(p: &mut Parser<'i, 't>) -> Result<PropertyValue, ParseError<'i, ()>> {
    let values: Vec<PropertyValue> =
        p.parse_until_before(Delimiter::Semicolon, parse_value_tokens)?;

    if values.len() == 1 {
        Ok(values.into_iter().next().unwrap())
    } else if values.is_empty() {
        Ok(PropertyValue::Keyword(String::new()))
    } else {
        Ok(PropertyValue::Shorthand(values))
    }
}

fn parse_value_tokens<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<Vec<PropertyValue>, ParseError<'i, ()>> {
    let mut values = Vec::new();
    while !p.is_exhausted() {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }

        if let Ok(color) = p.try_parse(try_parse_color) {
            values.push(PropertyValue::Color(color));
            continue;
        }

        let token = p.next()?.clone();
        match token {
            Token::Function(name) if name.eq_ignore_ascii_case("color-mix") => {
                let color = p.parse_nested_block(try_parse_color_mix)?;
                values.push(PropertyValue::Color(color));
                continue;
            }
            Token::Function(name) if name.eq_ignore_ascii_case("var") => {
                if let Ok(var_pv) = p.try_parse(|p| p.parse_nested_block(parse_var_function_inner))
                {
                    values.push(var_pv);
                } else {
                    // Consume the nested block to avoid re-parsing its tokens.
                    let _ = p.parse_nested_block(|p| -> Result<(), ParseError<'_, ()>> {
                        while !p.is_exhausted() {
                            let _ = p.next()?;
                        }
                        Ok(())
                    });
                }
                continue;
            }
            Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
                if let Ok(calc_pv) =
                    p.try_parse(|p| p.parse_nested_block(parse_calc_function_inner))
                {
                    values.push(calc_pv);
                } else {
                    // Consume the nested block to avoid re-parsing its tokens.
                    let _ = p.parse_nested_block(|p| -> Result<(), ParseError<'_, ()>> {
                        while !p.is_exhausted() {
                            let _ = p.next()?;
                        }
                        Ok(())
                    });
                }
                continue;
            }
            _ => {
                let pv = convert_token_to_value(p, &token)?;
                values.push(pv);
            }
        }
    }
    Ok(values)
}

fn convert_token_to_value<'i, 't>(
    p: &mut Parser<'i, 't>,
    token: &Token<'i>,
) -> Result<PropertyValue, ParseError<'i, ()>> {
    match token {
        Token::Number { value, .. } => Ok(PropertyValue::Number(*value)),
        Token::Dimension { value, unit, .. } => Ok(PropertyValue::Length(LengthValue {
            value: *value,
            unit: unit_to_enum(unit.as_ref()),
        })),
        Token::Percentage { unit_value, .. } => Ok(PropertyValue::Length(LengthValue {
            value: *unit_value * 100.0,
            unit: Unit::Percent,
        })),
        Token::Ident(name) => {
            if let Some(color) = Color::from_named(name) {
                Ok(PropertyValue::Color(color))
            } else if let Some(lv) = LengthValue::from_str(name) {
                Ok(PropertyValue::Length(lv))
            } else {
                Ok(PropertyValue::Keyword(name.to_string()))
            }
        }
        Token::Hash(name) | Token::IDHash(name) => {
            let hex = format!("#{}", name);
            Ok(Color::from_hex(&hex)
                .map(PropertyValue::Color)
                .unwrap_or_else(|| PropertyValue::Keyword(name.to_string())))
        }
        Token::Function(name) => {
            let _ = p.parse_nested_block(|p| -> Result<(), ParseError<'_, ()>> {
                while !p.is_exhausted() {
                    let _ = p.next()?;
                }
                Ok(())
            });
            Ok(PropertyValue::Keyword(name.to_string()))
        }
        Token::Delim(c) => Ok(PropertyValue::Keyword(c.to_string())),
        _ => Ok(PropertyValue::Keyword(format!("{:?}", token))),
    }
}

/// Parse `var(--name)` or `var(--name, fallback)` from the nested block.
/// Called after `Token::Function("var")` has been consumed.
fn parse_var_function_inner<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<PropertyValue, ParseError<'i, ()>> {
    p.skip_whitespace();
    let name = parse_dashed_ident_arg(p)?;
    p.skip_whitespace();

    let fallback = if p.try_parse(|p| p.expect_comma()).is_ok() {
        p.skip_whitespace();
        let fallback_val = parse_value(p)?;
        Some(Box::new(fallback_val))
    } else {
        None
    };

    Ok(PropertyValue::Var { name, fallback })
}

/// Parse a dashed ident argument (e.g. `--primary`) inside var().
fn parse_dashed_ident_arg<'i, 't>(p: &mut Parser<'i, 't>) -> Result<String, ParseError<'i, ()>> {
    let tok = p.next()?.clone();
    match tok {
        Token::Ident(ref name) if name.starts_with("--") => Ok(name.to_string()),
        _ => Err(p.new_custom_error(())),
    }
}

/// Parse `calc(...)` from the nested block.
/// Called after `Token::Function("calc")` has been consumed.
fn parse_calc_function_inner<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<PropertyValue, ParseError<'i, ()>> {
    let terms = parse_calc_expression(p)?;
    Ok(PropertyValue::Calc(terms))
}

/// Parse a calc() expression with operator precedence.
fn parse_calc_expression<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<Vec<CalcTerm>, ParseError<'i, ()>> {
    let mut terms = Vec::new();
    parse_calc_additive(p, &mut terms)?;
    Ok(terms)
}

fn parse_calc_additive<'i, 't>(
    p: &mut Parser<'i, 't>,
    terms: &mut Vec<CalcTerm>,
) -> Result<(), ParseError<'i, ()>> {
    parse_calc_multiplicative(p, terms)?;
    loop {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }
        let ok = p
            .try_parse(|p| {
                let tok = p.next()?.clone();
                match tok {
                    Token::Delim('+') => {
                        p.skip_whitespace();
                        parse_calc_multiplicative(p, terms)?;
                        terms.push(CalcTerm::Add);
                        Ok(())
                    }
                    Token::Delim('-') => {
                        p.skip_whitespace();
                        parse_calc_multiplicative(p, terms)?;
                        terms.push(CalcTerm::Sub);
                        Ok(())
                    }
                    _ => Err(p.new_custom_error(())),
                }
            })
            .is_ok();
        if !ok {
            break;
        }
    }
    Ok(())
}

fn parse_calc_multiplicative<'i, 't>(
    p: &mut Parser<'i, 't>,
    terms: &mut Vec<CalcTerm>,
) -> Result<(), ParseError<'i, ()>> {
    parse_calc_unary(p, terms)?;
    loop {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }
        let ok = p
            .try_parse(|p| {
                let tok = p.next()?.clone();
                match tok {
                    Token::Delim('*') => {
                        p.skip_whitespace();
                        parse_calc_unary(p, terms)?;
                        terms.push(CalcTerm::Mul);
                        Ok(())
                    }
                    Token::Delim('/') => {
                        p.skip_whitespace();
                        parse_calc_unary(p, terms)?;
                        terms.push(CalcTerm::Div);
                        Ok(())
                    }
                    _ => Err(p.new_custom_error(())),
                }
            })
            .is_ok();
        if !ok {
            break;
        }
    }
    Ok(())
}

fn parse_calc_unary<'i, 't>(
    p: &mut Parser<'i, 't>,
    terms: &mut Vec<CalcTerm>,
) -> Result<(), ParseError<'i, ()>> {
    p.skip_whitespace();
    if p.is_exhausted() {
        return Ok(());
    }
    let tok = p.next()?.clone();
    match tok {
        Token::Delim('-') => {
            p.skip_whitespace();
            parse_calc_primary(p, terms)?;
            negate_last_number(terms);
        }
        Token::Delim('+') => {}
        Token::Number { value, .. } => {
            terms.push(CalcTerm::Number(value));
        }
        Token::Dimension { value, unit, .. } => {
            terms.push(CalcTerm::Length(LengthValue {
                value,
                unit: unit_to_enum(unit.as_ref()),
            }));
        }
        Token::Percentage { unit_value, .. } => {
            terms.push(CalcTerm::Length(LengthValue {
                value: unit_value * 100.0,
                unit: Unit::Percent,
            }));
        }
        Token::ParenthesisBlock => {
            let sub = p.parse_nested_block(parse_calc_expression)?;
            terms.push(CalcTerm::Paren(sub));
        }
        Token::Function(ref name) if name.eq_ignore_ascii_case("var") => {
            let var_pv = p.parse_nested_block(parse_var_function_inner)?;
            if let PropertyValue::Var {
                name: var_name,
                fallback,
            } = var_pv
            {
                terms.push(CalcTerm::Var(var_name, fallback));
            } else {
                return Err(p.new_custom_error(()));
            }
        }
        _ => return Err(p.new_custom_error(())),
    }
    Ok(())
}

fn negate_last_number(terms: &mut [CalcTerm]) {
    if let Some(CalcTerm::Number(n)) = terms.last_mut() {
        *n = -*n;
    }
}

fn parse_calc_primary<'i, 't>(
    p: &mut Parser<'i, 't>,
    terms: &mut Vec<CalcTerm>,
) -> Result<(), ParseError<'i, ()>> {
    let tok = p.next()?.clone();
    match tok {
        Token::Number { value, .. } => {
            terms.push(CalcTerm::Number(value));
        }
        Token::Dimension { value, unit, .. } => {
            terms.push(CalcTerm::Length(LengthValue {
                value,
                unit: unit_to_enum(unit.as_ref()),
            }));
        }
        Token::Percentage { unit_value, .. } => {
            terms.push(CalcTerm::Length(LengthValue {
                value: unit_value * 100.0,
                unit: Unit::Percent,
            }));
        }
        Token::ParenthesisBlock => {
            let sub = p.parse_nested_block(parse_calc_expression)?;
            terms.push(CalcTerm::Paren(sub));
        }
        Token::Function(ref name) if name.eq_ignore_ascii_case("var") => {
            let var_pv = p.parse_nested_block(parse_var_function_inner)?;
            if let PropertyValue::Var {
                name: var_name,
                fallback,
            } = var_pv
            {
                terms.push(CalcTerm::Var(var_name, fallback));
            } else {
                return Err(p.new_custom_error(()));
            }
        }
        _ => return Err(p.new_custom_error(())),
    }
    Ok(())
}

/// Parse `var(--name)` from raw value context (custom property values).
fn parse_var_function<'i, 't>(p: &mut Parser<'i, 't>) -> Result<PropertyValue, ParseError<'i, ()>> {
    p.parse_nested_block(parse_var_function_inner)
}

fn unit_to_enum(s: &str) -> Unit {
    if s.eq_ignore_ascii_case("px") {
        Unit::Px
    } else if s.eq_ignore_ascii_case("em") {
        Unit::Em
    } else if s.eq_ignore_ascii_case("rem") {
        Unit::Rem
    } else if s.eq_ignore_ascii_case("vw") {
        Unit::Vw
    } else if s.eq_ignore_ascii_case("vh") {
        Unit::Vh
    } else if s.eq_ignore_ascii_case("vmin") {
        Unit::Vmin
    } else if s.eq_ignore_ascii_case("vmax") {
        Unit::Vmax
    } else if s.eq_ignore_ascii_case("pt") {
        Unit::Pt
    } else if s.eq_ignore_ascii_case("pc") {
        Unit::Pc
    } else if s.eq_ignore_ascii_case("cm") {
        Unit::Cm
    } else if s.eq_ignore_ascii_case("mm") {
        Unit::Mm
    } else if s.eq_ignore_ascii_case("in") {
        Unit::In
    } else if s.eq_ignore_ascii_case("ch") {
        Unit::Ch
    } else if s.eq_ignore_ascii_case("ex") {
        Unit::Ex
    } else {
        Unit::Px
    }
}

#[derive(Copy, Clone)]
enum ColorArg {
    Num(f32),
    Pct(f32),
}

fn try_parse_color<'i, 't>(p: &mut Parser<'i, 't>) -> Result<Color, ()> {
    let token = p.next().map_err(|_| ())?;
    match token {
        Token::Hash(name) | Token::IDHash(name) => {
            let hex = format!("#{}", name);
            Color::from_hex(&hex).ok_or(())
        }
        Token::Ident(name) => {
            let name_lower = name.to_lowercase();
            Color::from_named(&name_lower).ok_or(())
        }
        Token::Function(name) => {
            let name_lower = name.to_lowercase();
            if matches!(name_lower.as_str(), "rgb" | "rgba" | "hsl" | "hsla") {
                p.parse_nested_block(|p| parse_color_function(p, &name_lower))
                    .map_err(|_| ())
            } else {
                Err(())
            }
        }
        _ => Err(()),
    }
}

fn try_parse_color_mix<'i, 't>(p: &mut Parser<'i, 't>) -> Result<Color, ParseError<'i, ()>> {
    p.skip_whitespace();
    let _ = p.try_parse(|p| {
        let token = p.next()?;
        match token {
            Token::Ident(name) if name.eq_ignore_ascii_case("in") => {
                p.skip_whitespace();
                let _t2 = p.next()?;
                p.skip_whitespace();
                match p.next() {
                    Ok(Token::Comma) => {
                        eprintln!("try_parse_color_mix: found comma");
                    }
                    Ok(other) => {
                        eprintln!("try_parse_color_mix: unexpected token={:?}", other);
                    }
                    Err(_) => {
                        eprintln!("try_parse_color_mix: error after color-space");
                    }
                }
                p.skip_whitespace();
                Ok(())
            }
            _ => Err(p.new_error::<()>(BasicParseErrorKind::EndOfInput)),
        }
    });

    let first = p.next()?;
    let color = match first {
        Token::Hash(name) | Token::IDHash(name) => {
            let hex = format!("#{}", name);
            Color::from_hex(&hex)
                .ok_or_else(|| p.new_error::<()>(BasicParseErrorKind::EndOfInput))?
        }
        Token::Ident(name) => Color::from_named(name.as_ref())
            .ok_or_else(|| p.new_error::<()>(BasicParseErrorKind::EndOfInput))?,
        _ => return Err(p.new_error::<()>(BasicParseErrorKind::EndOfInput)),
    };
    while !p.is_exhausted() {
        let _ = p.next();
    }
    Ok(color)
}

fn parse_color_function<'i, 't>(
    p: &mut Parser<'i, 't>,
    name: &str,
) -> Result<Color, ParseError<'i, ()>> {
    let mut args: Vec<ColorArg> = Vec::new();
    loop {
        p.skip_whitespace();
        if p.is_exhausted() {
            break;
        }
        let tok = p.next()?;
        match tok {
            Token::Comma | Token::Delim('/') => continue,
            Token::Number { value, .. } => args.push(ColorArg::Num(*value)),
            Token::Percentage { unit_value, .. } => args.push(ColorArg::Pct(*unit_value)),
            Token::Dimension { value, .. } => args.push(ColorArg::Num(*value)),
            _ => break,
        }
    }

    match name {
        "rgb" | "rgba" => {
            if args.len() < 3 {
                return Err(p.new_error(BasicParseErrorKind::EndOfInput));
            }
            let r = arg_to_rgb(args[0]);
            let g = arg_to_rgb(args[1]);
            let b = arg_to_rgb(args[2]);
            let a = if args.len() >= 4 {
                arg_to_alpha(args[3])
            } else {
                255
            };
            Ok(Color { r, g, b, a })
        }
        "hsl" | "hsla" => {
            if args.len() < 3 {
                return Err(p.new_error(BasicParseErrorKind::EndOfInput));
            }
            let h = arg_to_f32(args[0]);
            let s = arg_to_pct(args[1]);
            let l = arg_to_pct(args[2]);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            let a = if args.len() >= 4 {
                arg_to_alpha(args[3])
            } else {
                255
            };
            Ok(Color { r, g, b, a })
        }
        _ => Err(p.new_error(BasicParseErrorKind::EndOfInput)),
    }
}

fn arg_to_rgb(arg: ColorArg) -> u8 {
    let v = match arg {
        ColorArg::Num(n) => n,
        ColorArg::Pct(p) => p * 255.0,
    };
    (v.round().clamp(0.0, 255.0)) as u8
}

fn arg_to_pct(arg: ColorArg) -> f32 {
    match arg {
        ColorArg::Num(n) => n,
        ColorArg::Pct(p) => p * 100.0,
    }
}

fn arg_to_f32(arg: ColorArg) -> f32 {
    match arg {
        ColorArg::Num(n) => n,
        ColorArg::Pct(p) => p * 100.0,
    }
}

fn arg_to_alpha(arg: ColorArg) -> u8 {
    let v = match arg {
        ColorArg::Num(n) => n,
        ColorArg::Pct(p) => p,
    };
    (v * 255.0).round().clamp(0.0, 255.0) as u8
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h = h / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;
    if s == 0.0 {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }
    let hue_to_rgb = |p: f32, q: f32, mut t: f32| -> f32 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0 / 2.0 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}
