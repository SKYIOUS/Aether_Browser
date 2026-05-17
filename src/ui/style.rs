use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color};

// ── Color Palette (Light Theme) ─────────────────────────────────────────────
pub struct C;
impl C {
    // Backgrounds
    pub const BG: Color = Color::from_rgb(0.93, 0.94, 0.96);
    pub const SIDEBAR: Color = Color::from_rgb(0.87, 0.88, 0.91);
    pub const SURFACE: Color = Color::from_rgb(0.97, 0.97, 0.99);
    pub const PANEL: Color = Color::from_rgb(0.95, 0.95, 0.97);
    pub const PANEL_HEADER: Color = Color::from_rgb(0.92, 0.93, 0.95);
    pub const CONTROL_PANEL: Color = Color::from_rgb(0.94, 0.94, 0.96);
    pub const CARD: Color = Color::from_rgb(1.0, 1.0, 1.0);

    // Text
    pub const FG: Color = Color::from_rgb(0.08, 0.08, 0.10);
    pub const MUTED: Color = Color::from_rgb(0.42, 0.42, 0.46);
    pub const DIM: Color = Color::from_rgb(0.55, 0.55, 0.60);

    // Accent / Brand
    pub const ACCENT: Color = Color::from_rgb(0.25, 0.50, 0.90);
    pub const ACCENT_DIM: Color = Color::from_rgba(0.25, 0.50, 0.90, 0.10);
    pub const ACCENT_BORDER: Color = Color::from_rgba(0.25, 0.50, 0.90, 0.25);

    // Page content (light background, dark text — normal web rendering)
    pub const PAGE_BG: Color = Color::from_rgb(0.98, 0.98, 0.98);
    pub const PAGE_TEXT: Color = Color::from_rgb(0.06, 0.06, 0.07);
    pub const PAGE_LINK: Color = Color::from_rgb(0.02, 0.33, 0.75);
    pub const PAGE_LINK_HOVER: Color = Color::from_rgb(0.01, 0.25, 0.60);
    pub const PAGE_MUTED: Color = Color::from_rgb(0.35, 0.35, 0.38);
    pub const PAGE_HEADING: Color = Color::from_rgb(0.04, 0.04, 0.05);

    // Borders / Dividers
    pub const BORDER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.08);
    pub const BORDER_MID: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.14);

    // Misc
    pub const GREEN: Color = Color::from_rgb(0.18, 0.70, 0.38);
    pub const TRANSPARENT: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.0);
}

// ── Container Styles ─────────────────────────────────────────────────────────

pub fn sidebar_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::SIDEBAR)),
        border: Border {
            color: C::BORDER,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn topbar_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::TRANSPARENT)),
        border: Border {
            color: C::BORDER,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn main_area_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::BG)),
        ..Default::default()
    }
}

pub fn panel_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::PANEL)),
        border: Border {
            color: C::BORDER,
            width: 1.0,
            radius: 16.0.into(),
        },
        ..Default::default()
    }
}

pub fn panel_header_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::PANEL_HEADER)),
        border: Border {
            color: C::BORDER,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn control_panel_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::CONTROL_PANEL)),
        border: Border {
            color: C::BORDER,
            width: 1.0,
            radius: 16.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::CARD)),
        border: Border {
            color: C::BORDER,
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn pill_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::CARD)),
        border: Border {
            color: C::BORDER_MID,
            width: 1.0,
            radius: 20.0.into(),
        },
        ..Default::default()
    }
}

pub fn url_bar_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04))),
        border: Border {
            color: C::BORDER,
            width: 1.0,
            radius: 999.0.into(),
        },
        ..Default::default()
    }
}

pub fn active_workspace_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.25, 0.50, 0.90, 0.08))),
        border: Border {
            color: C::ACCENT,
            width: 0.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn workspace_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::TRANSPARENT)),
        border: Border {
            color: C::TRANSPARENT,
            width: 0.0,
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

pub fn graph_area_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
        border: Border {
            color: C::BORDER,
            width: 1.0,
            radius: 16.0.into(),
        },
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
        background: Some(Background::Color(C::SURFACE)),
        border: Border {
            color: C::BORDER_MID,
            width: 1.0,
            radius: 24.0.into(),
        },
        ..Default::default()
    }
}

pub fn settings_nav_style() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(C::SIDEBAR)),
        border: Border {
            color: C::BORDER,
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
            text_color: C::MUTED,
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn accent_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => {
                Color::from_rgba(0.25, 0.50, 0.90, 0.20)
            }
            _ => Color::from_rgba(0.25, 0.50, 0.90, 0.10),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: C::ACCENT,
            border: Border {
                color: C::ACCENT_BORDER,
                width: 1.0,
                radius: 12.0.into(),
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
            text_color: C::MUTED,
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn sidebar_item_button_style(active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
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
        let text_color = if active { C::ACCENT } else { C::MUTED };
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
        let text_color = if active { C::ACCENT } else { C::MUTED };
        let border_color = if active { C::ACCENT_BORDER } else { Color::TRANSPARENT };
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

// ── Text Input Style ─────────────────────────────────────────────────────────

pub fn url_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    |_, _| text_input::Style {
        background: Background::Color(C::TRANSPARENT),
        border: Border {
            color: C::TRANSPARENT,
            width: 0.0,
            radius: 999.0.into(),
        },
        icon: C::MUTED,
        placeholder: C::DIM,
        value: C::FG,
        selection: C::ACCENT_DIM,
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
        icon: C::MUTED,
        placeholder: C::DIM,
        value: C::FG,
        selection: C::ACCENT_DIM,
    }
}

pub fn link_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, status| {
        let bg = match status {
            button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.647, 0.788, 1.0, 0.05))),
            button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.647, 0.788, 1.0, 0.10))),
            _ => None,
        };
        button::Style {
            background: bg,
            text_color: C::ACCENT,
            border: Border::default(),
            ..Default::default()
        }
    }
}
