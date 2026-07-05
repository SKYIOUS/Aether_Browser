use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::Padding;
use iced::{Alignment, Element, Length, Task};
use serde::{Serialize, Deserialize};
use crate::ui::style::*;
use crate::plog;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AetherSettings {
    pub home_page_url: String,
    pub default_search_engine: String,
    pub js_enabled: bool,
    pub cookies_enabled: bool,
}

impl Default for AetherSettings {
    fn default() -> Self {
        Self {
            home_page_url: "aether://design/spatial-minimalism".to_string(),
            default_search_engine: "duckduckgo".to_string(),
            js_enabled: true,
            cookies_enabled: true,
        }
    }
}

impl AetherSettings {
    pub fn load() -> Self {
        std::fs::read_to_string("aether_settings.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            if let Err(e) = std::fs::write("aether_settings.json", json) {
                plog!("settings", "Failed to save: {}", e);
            }
        }
    }

    pub fn search_url(&self, query: &str) -> String {
        let q = urlencoding(query);
        match self.default_search_engine.as_str() {
            "google" => format!("https://www.google.com/search?q={}", q),
            _ => format!("https://duckduckgo.com/?q={}", q),
        }
    }

    pub fn is_url(s: &str) -> bool {
        s.contains("://") || s.contains('.') || s.starts_with("about:") || s.starts_with("aether://")
    }
}

fn urlencoding(s: &str) -> String {
    s.chars().map(|c| match c {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
        ' ' => "+".to_string(),
        _ => format!("%{:02X}", c as u8),
    }).collect()
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Back,
    NavItem(usize),
    ToggleSilentFlow,
    ToggleLogging,
    AccentSelected(usize),
    HomePageChanged(String),
    SearchEngineChanged(String),
    ToggleJs,
    ToggleCookies,
}

const NAV_ITEMS: [(&str, &str); 5] = [
    ("◉", "Account"),
    ("⬡", "Privacy & Security"),
    ("◈", "Appearance"),
    ("⊞", "Extensions"),
    ("⚡", "High Performance"),
];

pub struct SettingsScreen {
    pub active_nav: usize,
    pub silent_flow: bool,
    pub logging_enabled: bool,
    pub accent_selected: usize,
    pub settings: AetherSettings,
    pub changed: bool,
}

impl SettingsScreen {
    pub fn new() -> Self {
        crate::logging::set_enabled(true);
        Self { active_nav: 0, silent_flow: true, logging_enabled: true, accent_selected: 0, settings: AetherSettings::load(), changed: false }
    }
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsScreen {
    pub fn update(&mut self, msg: SettingsMessage) -> Task<SettingsMessage> {
        match msg {
            SettingsMessage::NavItem(i) => self.active_nav = i,
            SettingsMessage::ToggleSilentFlow => self.silent_flow = !self.silent_flow,
            SettingsMessage::ToggleLogging => {
                self.logging_enabled = !self.logging_enabled;
                crate::logging::set_enabled(self.logging_enabled);
            }
            SettingsMessage::AccentSelected(i) => self.accent_selected = i,
            SettingsMessage::HomePageChanged(s) => { self.settings.home_page_url = s; self.changed = true; },
            SettingsMessage::SearchEngineChanged(s) => { self.settings.default_search_engine = s; self.changed = true; },
            SettingsMessage::ToggleJs => { self.settings.js_enabled = !self.settings.js_enabled; self.changed = true; },
            SettingsMessage::ToggleCookies => { self.settings.cookies_enabled = !self.settings.cookies_enabled; self.changed = true; },
            _ => {}
        }
        if self.changed { self.settings.save(); self.changed = false; }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, SettingsMessage> {
        row![self.nav_panel(), self.content_panel()]
            .height(Length::Fill)
            .into()
    }

    fn nav_panel(&self) -> Element<'_, SettingsMessage> {
        let header = column![
            text("Settings").size(20).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
            Space::with_height(4),
            text("Core Configuration").size(10).color(C::MUTED)
                .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        ];

        let nav = column(
            NAV_ITEMS.iter().enumerate().map(|(i, (icon, label))| {
                let active = i == self.active_nav;
                let row_content = row![
                    text(*icon).size(16).color(if active { C::ACCENT } else { C::MUTED }),
                    text(*label).size(13).color(if active { C::ACCENT } else { C::MUTED }),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                button(row_content)
                    .width(Length::Fill)
                    .padding([12, 16])
                    .style(sidebar_item_button_style(active))
                    .on_press(SettingsMessage::NavItem(i))
                    .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(4);

        let back_btn = button(
            row![
                text("←").size(14).color(C::MUTED),
                text("Back to Browser").size(12).color(C::MUTED),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([8, 0])
        .width(Length::Fill)
        .style(ghost_button_style())
        .on_press(SettingsMessage::Back);

        container(
            column![
                header,
                Space::with_height(32),
                nav,
                Space::with_height(Length::Fill),
                back_btn,
            ]
            .padding([32, 24])
            .height(Length::Fill),
        )
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .style(settings_nav_style())
        .into()
    }

    fn content_panel(&self) -> Element<'_, SettingsMessage> {
        // Browser section
        let section_browser = column![
            text("Browser").size(28).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(24),
            self.home_page_card(),
            Space::with_height(12),
            self.search_engine_card(),
            Space::with_height(12),
            self.js_card(),
            Space::with_height(12),
            self.cookies_card(),
        ];

        // Personalization section
        let section_1 = column![
            text("Personalization").size(28).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(24),
            self.silent_flow_card(),
            Space::with_height(12),
            self.accent_card(),
        ];

        // Developer section
        let section_dev = column![
            text("Developer").size(28).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(24),
            self.logging_card(),
        ];

        // Security section
        let section_2 = column![
            text("Security Context").size(28).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(24),
            self.security_card(),
        ];

        // Footer
        let footer = container(
            row![
                text("Aether Browser v0.1.0-alpha").size(11).color(C::DIM),
                Space::with_width(Length::Fill),
                text("Built with Rust & Iced").size(11).color(C::DIM),
            ]
        )
        .width(Length::Fill)
        .padding(Padding { top: 24.0, right: 0.0, bottom: 0.0, left: 0.0 })
        .style(|_| container::Style {
            border: iced::Border { color: C::BORDER, width: 1.0, radius: 0.0.into() },
            ..Default::default()
        });

        let body = scrollable(
            container(
                column![
                    section_browser,
                    Space::with_height(48),
                    section_1,
                    Space::with_height(48),
                    section_dev,
                    Space::with_height(48),
                    section_2,
                    Space::with_height(48),
                    footer,
                ]
                .padding([48, 56])
            )
            .width(Length::Fill)
        )
        .height(Length::Fill);

        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(main_area_style())
            .into()
    }

    fn home_page_card(&self) -> Element<'_, SettingsMessage> {
        let left = column![
            text("Home Page").size(14).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(4),
            text("Page shown on new tab or startup").size(12).color(C::MUTED),
        ];
        let input = text_input("URL", &self.settings.home_page_url)
            .on_input(SettingsMessage::HomePageChanged)
            .size(13)
            .padding(10);
        container(row![left, Space::with_width(Length::Fill), input.width(Length::Fixed(300.0))].align_y(Alignment::Center))
            .width(Length::Fill).padding(20).style(card_style()).into()
    }

    fn search_engine_card(&self) -> Element<'_, SettingsMessage> {
        let ddg_active = self.settings.default_search_engine == "duckduckgo";
        let google_active = self.settings.default_search_engine == "google";
        let left = column![
            text("Default Search Engine").size(14).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(4),
            text("Used when typing non-URLs in the address bar").size(12).color(C::MUTED),
        ];
        let ddg = button(text("DuckDuckGo").size(12).color(if ddg_active { C::ACCENT } else { C::MUTED }))
            .padding([6, 12]).style(pill_button_style(ddg_active)).on_press(SettingsMessage::SearchEngineChanged("duckduckgo".to_string()));
        let google = button(text("Google").size(12).color(if google_active { C::ACCENT } else { C::MUTED }))
            .padding([6, 12]).style(pill_button_style(google_active)).on_press(SettingsMessage::SearchEngineChanged("google".to_string()));
        container(row![left, Space::with_width(Length::Fill), ddg, google].spacing(8).align_y(Alignment::Center))
            .width(Length::Fill).padding(20).style(card_style()).into()
    }

    fn js_card(&self) -> Element<'_, SettingsMessage> {
        let on = self.settings.js_enabled;
        let toggle_color = if on { C::ACCENT } else { C::MUTED };
        let toggle_text = if on { "ON" } else { "OFF" };
        let left = column![
            text("JavaScript").size(14).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(4),
            text("Enable JavaScript execution on web pages").size(12).color(C::MUTED),
        ];
        let toggle = container(text(toggle_text).size(11).color(toggle_color)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }))
            .padding([6, 12]).style(|_| container::Style {
                background: Some(iced::Background::Color(C::ACCENT_DIM)),
                border: iced::Border { color: C::ACCENT_BORDER, width: 1.0, radius: 8.0.into() },
                ..Default::default()
            });
        button(row![left, Space::with_width(Length::Fill), toggle].align_y(Alignment::Center))
            .width(Length::Fill).padding(20).style(card_button_style()).on_press(SettingsMessage::ToggleJs).into()
    }

    fn cookies_card(&self) -> Element<'_, SettingsMessage> {
        let on = self.settings.cookies_enabled;
        let toggle_color = if on { C::ACCENT } else { C::MUTED };
        let toggle_text = if on { "ON" } else { "OFF" };
        let left = column![
            text("Cookies").size(14).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
            Space::with_height(4),
            text("Allow websites to store cookies on this device").size(12).color(C::MUTED),
        ];
        let toggle = container(text(toggle_text).size(11).color(toggle_color)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }))
            .padding([6, 12]).style(|_| container::Style {
                background: Some(iced::Background::Color(C::ACCENT_DIM)),
                border: iced::Border { color: C::ACCENT_BORDER, width: 1.0, radius: 8.0.into() },
                ..Default::default()
            });
        button(row![left, Space::with_width(Length::Fill), toggle].align_y(Alignment::Center))
            .width(Length::Fill).padding(20).style(card_button_style()).on_press(SettingsMessage::ToggleCookies).into()
    }

    fn silent_flow_card(&self) -> Element<'_, SettingsMessage> {
        let toggle_color = if self.silent_flow { C::ACCENT } else { C::MUTED };
        let toggle_text = if self.silent_flow { "ON" } else { "OFF" };

        let left = row![
            container(text("◉").size(22).color(C::ACCENT))
                .width(48).height(48)
                .center_x(Length::Fixed(48.0))
                .center_y(Length::Fixed(48.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(C::ACCENT_DIM)),
                    border: iced::Border { radius: 12.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            column![
                text("Enable Silent Flow").size(14).color(C::FG)
                    .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
                Space::with_height(4),
                text("Auto-hide UI chrome when focusing on content").size(12).color(C::MUTED),
            ]
            .spacing(0),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let toggle = container(
            text(toggle_text).size(11).color(toggle_color)
                .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        )
        .padding([6, 12])
        .style(|_| container::Style {
            background: Some(iced::Background::Color(C::ACCENT_DIM)),
            border: iced::Border { color: C::ACCENT_BORDER, width: 1.0, radius: 8.0.into() },
            ..Default::default()
        });

        button(
            row![left, Space::with_width(Length::Fill), toggle]
                .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(20)
        .style(|_, status| {
            let bg = match status {
                iced::widget::button::Status::Hovered => {
                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.04)
                }
                _ => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.02),
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border { color: C::BORDER, width: 1.0, radius: 16.0.into() },
                text_color: C::FG,
                ..Default::default()
            }
        })
        .on_press(SettingsMessage::ToggleSilentFlow)
        .into()
    }

    fn logging_card(&self) -> Element<'_, SettingsMessage> {
        let toggle_color = if self.logging_enabled { C::ACCENT } else { C::MUTED };
        let toggle_text = if self.logging_enabled { "ON" } else { "OFF" };

        let left = row![
            container(text("⚡").size(22).color(C::ACCENT))
                .width(48).height(48)
                .center_x(Length::Fixed(48.0))
                .center_y(Length::Fixed(48.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(C::ACCENT_DIM)),
                    border: iced::Border { radius: 12.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            column![
                text("Pipeline Logging").size(14).color(C::FG)
                    .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
                Space::with_height(4),
                text("Log all pipeline stages to logs/pipeline_*.log").size(12).color(C::MUTED),
            ]
            .spacing(0),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let toggle = container(
            text(toggle_text).size(11).color(toggle_color)
                .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        )
        .padding([6, 12])
        .style(|_| container::Style {
            background: Some(iced::Background::Color(C::ACCENT_DIM)),
            border: iced::Border { color: C::ACCENT_BORDER, width: 1.0, radius: 8.0.into() },
            ..Default::default()
        });

        button(
            row![left, Space::with_width(Length::Fill), toggle]
                .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(20)
        .style(|_, status| {
            let bg = match status {
                iced::widget::button::Status::Hovered => {
                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.04)
                }
                _ => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.02),
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border { color: C::BORDER, width: 1.0, radius: 16.0.into() },
                text_color: C::FG,
                ..Default::default()
            }
        })
        .on_press(SettingsMessage::ToggleLogging)
        .into()
    }

    fn accent_card(&self) -> Element<'_, SettingsMessage> {
        let accent_colors = [
            iced::Color::from_rgb(0.647, 0.788, 1.0),
            iced::Color::from_rgb(1.0, 0.706, 0.671),
            iced::Color::from_rgb(0.827, 0.737, 0.988),
        ];

        let swatches = row(
            accent_colors.iter().enumerate().map(|(i, &color)| {
                let selected = i == self.accent_selected;
                button(
                    container(Space::with_width(0.0))
                        .width(24).height(24)
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(color)),
                            border: iced::Border {
                                color: if selected {
                                    iced::Color::WHITE
                                } else {
                                    iced::Color::TRANSPARENT
                                },
                                width: if selected { 2.0 } else { 0.0 },
                                radius: 999.0.into(),
                            },
                            ..Default::default()
                        }),
                )
                    .padding(0)
                    .style(|_, _| iced::widget::button::Style::default())
                    .on_press(SettingsMessage::AccentSelected(i))
                    .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(8);

        let left = row![
            container(text("◈").size(22).color(C::MUTED))
                .width(48).height(48)
                .center_x(Length::Fixed(48.0))
                .center_y(Length::Fixed(48.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(
                        iced::Color::from_rgba(1.0, 1.0, 1.0, 0.04)
                    )),
                    border: iced::Border { radius: 12.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            column![
                text("Visual Foundation").size(14).color(C::FG)
                    .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
                Space::with_height(4),
                text("Currently using 'Modern Minimal' palette").size(12).color(C::MUTED),
            ]
            .spacing(0),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        container(
            row![left, Space::with_width(Length::Fill), swatches]
                .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(20)
        .style(card_style())
        .into()
    }

    fn security_card(&self) -> Element<'_, SettingsMessage> {
        container(
            column![
                row![
                    text("Hardened Sandbox").size(14).color(C::FG)
                        .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
                    Space::with_width(Length::Fill),
                    container(text("ACTIVE").size(10).color(C::ACCENT)
                        .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }))
                        .padding([4, 10])
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(C::ACCENT_DIM)),
                            border: iced::Border { radius: 6.0.into(), ..Default::default() },
                            ..Default::default()
                        }),
                ]
                .align_y(Alignment::Center),
                Space::with_height(12),
                text("Aether uses a multi-process architecture with strict memory isolation for every tab. Local data is encrypted using AES-256 by default.")
                    .size(13).color(C::MUTED),
                Space::with_height(20),
                text("Manage Security Keys →").size(12).color(C::ACCENT),
            ]
            .spacing(0)
        )
        .width(Length::Fill)
        .padding(24)
        .style(card_style())
        .into()
    }
}
