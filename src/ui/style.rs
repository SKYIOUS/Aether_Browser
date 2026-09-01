#![allow(dead_code)]
use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color};
use std::sync::{OnceLock, RwLock, RwLockReadGuard};

use crate::ui::screens::settings::AccentColor;

// ── Runtime UI palette ──────────────────────────────────────────────────────
// ADR-0004: one process-global snapshot. Style closures cannot capture app
// state, so consumers copy single colors through short-lived C::*() reads;
// set_palette is the only writer (startup + explicit user actions).

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub bg: Color,
    pub sidebar: Color,
    pub surface: Color,
    pub card: Color,
    pub fg: Color,
    pub muted: Color,
    pub dim: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub accent_border: Color,
    pub page_bg: Color,
    pub page_text: Color,
    pub page_muted: Color,
    pub border: Color,
    pub border_mid: Color,
}

fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}
fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::from_rgba(r, g, b, a)
}

impl Palette {
    pub fn light(accent: &AccentColor) -> Self {
        Self {
            bg: rgb(0.93, 0.94, 0.96),
            sidebar: rgb(0.87, 0.88, 0.91),
            surface: rgb(0.97, 0.97, 0.99),
            card: rgb(1.00, 1.00, 1.00),
            fg: rgb(0.08, 0.08, 0.10),
            muted: rgb(0.42, 0.42, 0.46),
            dim: rgb(0.55, 0.55, 0.60),
            accent: rgb(accent.r, accent.g, accent.b),
            accent_dim: rgba(accent.r, accent.g, accent.b, 0.10),
            accent_border: rgba(accent.r, accent.g, accent.b, 0.25),
            page_bg: rgb(0.98, 0.98, 0.98),
            page_text: rgb(0.06, 0.06, 0.07),
            page_muted: rgb(0.35, 0.35, 0.38),
            border: rgba(0.0, 0.0, 0.0, 0.08),
            border_mid: rgba(0.0, 0.0, 0.0, 0.14),
        }
    }

    // Chrome inverts; web content (PAGE_*) is the page's business, not ours.
    pub fn dark(accent: &AccentColor) -> Self {
        Self {
            bg: rgb(0.11, 0.12, 0.15),
            sidebar: rgb(0.14, 0.15, 0.19),
            surface: rgb(0.18, 0.19, 0.24),
            card: rgb(0.22, 0.23, 0.28),
            fg: rgb(0.92, 0.93, 0.95),
            muted: rgb(0.68, 0.69, 0.73),
            dim: rgb(0.52, 0.53, 0.57),
            accent: rgb(accent.r, accent.g, accent.b),
            accent_dim: rgba(accent.r, accent.g, accent.b, 0.10),
            accent_border: rgba(accent.r, accent.g, accent.b, 0.25),
            page_bg: rgb(0.98, 0.98, 0.98),
            page_text: rgb(0.06, 0.06, 0.07),
            page_muted: rgb(0.35, 0.35, 0.38),
            border: rgba(1.0, 1.0, 1.0, 0.10),
            border_mid: rgba(1.0, 1.0, 1.0, 0.18),
        }
    }
}

static PALETTE: OnceLock<RwLock<Palette>> = OnceLock::new();

/// The only mutation path (ADR-0004). Call once at startup before first
/// render, then from explicit user actions paired with VayuSettings::save().
pub fn set_palette(dark: bool, accent: &AccentColor) {
    let next = if dark {
        Palette::dark(accent)
    } else {
        Palette::light(accent)
    };
    let lock = PALETTE.get_or_init(|| RwLock::new(next));
    *lock.write().unwrap_or_else(|e| e.into_inner()) = next;
}

fn read_palette() -> RwLockReadGuard<'static, Palette> {
    // Safety net for any consumer reached before startup wiring ran; the real
    // init happens eagerly in BrowserScreen::new.
    PALETTE
        .get_or_init(|| RwLock::new(Palette::light(&AccentColor::default())))
        .read()
        .unwrap_or_else(|e| e.into_inner())
}

// ── Color Palette accessors (was: Light Theme constants) ────────────────────
pub struct C;
impl C {
    pub fn bg() -> Color {
        read_palette().bg
    }
    pub fn sidebar() -> Color {
        read_palette().sidebar
    }
    pub fn surface() -> Color {
        read_palette().surface
    }
    pub fn card() -> Color {
        read_palette().card
    }

    pub fn fg() -> Color {
        read_palette().fg
    }
    pub fn muted() -> Color {
        read_palette().muted
    }
    pub fn dim() -> Color {
        read_palette().dim
    }

    pub fn accent() -> Color {
        read_palette().accent
    }
    pub fn accent_dim() -> Color {
        read_palette().accent_dim
    }
    pub fn accent_border() -> Color {
        read_palette().accent_border
    }

    pub fn page_bg() -> Color {
        read_palette().page_bg
    }
    pub fn page_text() -> Color {
        read_palette().page_text
    }
    pub fn page_muted() -> Color {
        read_palette().page_muted
    }

    pub fn border() -> Color {
        read_palette().border
    }
    pub fn border_mid() -> Color {
        read_palette().border_mid
    }

    // Misc - theme-independent by definition.
    pub const TRANSPARENT: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.0);
}

// ── Container Styles ─────────────────────────────────────────────────────────

pub fn sidebar_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::sidebar())),
        border: Border {
            color: C::border(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn main_area_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::bg())),
        ..Default::default()
    }
}

pub fn card_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::card())),
        border: Border {
            color: C::border(),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn status_bar_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::TRANSPARENT)),
        ..Default::default()
    }
}

pub fn overlay_bg_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.65))),
        ..Default::default()
    }
}

pub fn palette_panel_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::surface())),
        border: Border {
            color: C::border_mid(),
            width: 1.0,
            radius: 24.0.into(),
        },
        ..Default::default()
    }
}

pub fn settings_nav_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::sidebar())),
        border: Border {
            color: C::border(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

// ── Button Styles ─────────────────────────────────────────────────────────────

pub fn ghost_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
            }
            _ => None,
        };
        button::Style {
            background: bg,
            text_color: C::muted(),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn nav_icon_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
            }
            _ => None,
        };
        button::Style {
            background: bg,
            text_color: C::muted(),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn sidebar_item_button_style(
    active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_, status| {
        let bg = if active {
            Some(Background::Color(Color::from_rgba(0.25, 0.50, 0.90, 0.10)))
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04)))
                }
                _ => None,
            }
        };
        let text_color = if active { C::accent() } else { C::muted() };
        button::Style {
            background: bg,
            text_color,
            border: Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn pill_button_style(active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_, status| {
        let bg = if active {
            Some(Background::Color(Color::from_rgba(0.25, 0.50, 0.90, 0.10)))
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.25, 0.50, 0.90, 0.05)))
                }
                _ => None,
            }
        };
        let text_color = if active { C::accent() } else { C::muted() };
        let border_color = if active {
            C::accent_border()
        } else {
            Color::TRANSPARENT
        };
        button::Style {
            background: bg,
            text_color,
            border: Border {
                color: border_color,
                width: if active { 1.0 } else { 0.0 },
                radius: 10.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn card_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, status| {
        let bg = match status {
            button::Status::Hovered => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.04),
            _ => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.02),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            border: iced::Border {
                color: C::border(),
                width: 1.0,
                radius: 16.0.into(),
            },
            text_color: C::fg(),
            ..Default::default()
        }
    }
}

// ── Text Input Style ─────────────────────────────────────────────────────────

pub fn url_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    |_, _| text_input::Style {
        background: Background::Color(C::TRANSPARENT),
        border: Border {
            color: C::TRANSPARENT,
            width: 0.0,
            radius: 999.0.into(),
        },
        icon: C::muted(),
        placeholder: C::dim(),
        value: C::fg(),
        selection: C::accent_dim(),
    }
}

pub fn autocomplete_dropdown_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::surface())),
        border: iced::Border {
            color: C::border_mid(),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn palette_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    |_, _| text_input::Style {
        background: Background::Color(C::TRANSPARENT),
        border: Border {
            color: C::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: C::muted(),
        placeholder: C::dim(),
        value: C::fg(),
        selection: C::accent_dim(),
    }
}

// ?? B2 palette tests ????????????????????????????????????????????????????????
#[cfg(test)]
mod palette_tests {
    use super::{set_palette, Palette, C};
    use crate::ui::screens::settings::{AccentColor, VayuSettings};

    fn accent(r: f32, g: f32, b: f32) -> AccentColor {
        AccentColor { r, g, b }
    }
    fn rgb(r: f32, g: f32, b: f32) -> iced::Color {
        iced::Color::from_rgb(r, g, b)
    }
    fn rgba(r: f32, g: f32, b: f32, a: f32) -> iced::Color {
        iced::Color::from_rgba(r, g, b, a)
    }

    // No visual drift guard: every slot pinned to its pre-B2 literal.
    #[test]
    fn b2_light_matches_pre_b2_constants_exactly() {
        let p = Palette::light(&accent(0.25, 0.50, 0.90));
        assert_eq!(p.bg, rgb(0.93, 0.94, 0.96));
        assert_eq!(p.sidebar, rgb(0.87, 0.88, 0.91));
        assert_eq!(p.surface, rgb(0.97, 0.97, 0.99));
        assert_eq!(p.card, rgb(1.00, 1.00, 1.00));
        assert_eq!(p.fg, rgb(0.08, 0.08, 0.10));
        assert_eq!(p.muted, rgb(0.42, 0.42, 0.46));
        assert_eq!(p.dim, rgb(0.55, 0.55, 0.60));
        assert_eq!(p.accent, rgb(0.25, 0.50, 0.90));
        assert_eq!(p.accent_dim, rgba(0.25, 0.50, 0.90, 0.10));
        assert_eq!(p.accent_border, rgba(0.25, 0.50, 0.90, 0.25));
        assert_eq!(p.page_bg, rgb(0.98, 0.98, 0.98));
        assert_eq!(p.page_text, rgb(0.06, 0.06, 0.07));
        assert_eq!(p.page_muted, rgb(0.35, 0.35, 0.38));
        assert_eq!(p.border, rgba(0.0, 0.0, 0.0, 0.08));
        assert_eq!(p.border_mid, rgba(0.0, 0.0, 0.0, 0.14));
    }

    #[test]
    fn b2_dark_inverts_chrome_keeps_web_content_and_accent() {
        let a = accent(0.90, 0.40, 0.20);
        let light = Palette::light(&AccentColor::default());
        let d = Palette::dark(&a);
        assert_eq!(
            d.accent,
            rgb(0.90, 0.40, 0.20),
            "accent identical across modes"
        );
        assert_eq!(d.accent_dim, rgba(0.90, 0.40, 0.20, 0.10));
        assert_eq!(d.accent_border, rgba(0.90, 0.40, 0.20, 0.25));
        assert!(
            d.bg.r < 0.2 && d.sidebar.r < 0.3 && d.card.r < 0.3,
            "chrome goes dark"
        );
        assert!(d.fg.r > 0.8 && d.muted.r > 0.4, "text goes light");
        assert_eq!(
            d.page_bg, light.page_bg,
            "web content colors are not chrome"
        );
        assert_eq!(d.page_text, light.page_text);
        assert_eq!(d.page_muted, light.page_muted);
    }

    #[test]
    fn b2_set_then_accessors_roundtrip() {
        let a = accent(0.90, 0.40, 0.20);
        set_palette(true, &a);
        assert_eq!(C::bg(), Palette::dark(&a).bg);
        assert_eq!(C::accent(), rgb(0.90, 0.40, 0.20));
        set_palette(false, &AccentColor::default());
        assert_eq!(C::bg(), rgb(0.93, 0.94, 0.96));
    }

    #[test]
    fn b2_settings_fields_survive_serde() {
        let mut s = VayuSettings::default();
        s.dark_mode = true;
        s.accent_color = accent(0.10, 0.20, 0.30);
        let json = serde_json::to_string(&s).expect("serialize");
        let back: VayuSettings = serde_json::from_str(&json).expect("deserialize");
        assert!(back.dark_mode);
        assert_eq!(back.accent_color.r, 0.10);
        assert_eq!(back.accent_color.g, 0.20);
        assert_eq!(back.accent_color.b, 0.30);
    }
}
