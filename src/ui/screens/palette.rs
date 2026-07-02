use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length, Task};
use crate::ui::style::*;

#[derive(Debug, Clone)]
pub enum PaletteMessage {
    Close,
    QueryChanged(String),
    ActionSelected(usize),
    HistorySelected(usize),
}

const SUGGESTED: [(&str, &str, &str); 2] = [
    ("✦", "Generate Research Summary", "Synthesize current workspace with Aether AI"),
    ("◫", "Switch to 'Rust Backend' Workspace", "4 active tabs · Last seen 2h ago"),
];

const HISTORY: [&str; 2] = [
    "spatial design principles in browsers",
    "iced rust ui framework documentation",
];

pub struct PaletteScreen {
    pub query: String,
}

impl PaletteScreen {
    pub fn new() -> Self {
        Self { query: String::new() }
    }
}

impl Default for PaletteScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl PaletteScreen {
    pub fn reset(&mut self) {
        self.query.clear();
    }

    pub fn update(&mut self, msg: PaletteMessage) -> Task<PaletteMessage> {
        match msg {
            PaletteMessage::QueryChanged(s) => self.query = s,
            PaletteMessage::ActionSelected(_index) => {},
            PaletteMessage::HistorySelected(_index) => {},
            PaletteMessage::Close => {},
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, PaletteMessage> {
        // Dimmed backdrop
        let backdrop = container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(overlay_bg_style());

        // Palette panel
        let panel = container(self.palette_inner())
            .width(Length::Fixed(640.0))
            .style(palette_panel_style());

        // Center the panel near the top
        let overlay = container(
            column![
                Space::with_height(80),
                panel,
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center);

        // Stack backdrop + overlay panel
        iced::widget::stack![backdrop, overlay].into()
    }

    fn palette_inner(&self) -> Element<'_, PaletteMessage> {
        // Search row
        let search = container(
            row![
                text("⌕").size(24).color(C::ACCENT),
                text_input("What is the mission?", &self.query)
                    .on_input(PaletteMessage::QueryChanged)
                    .on_submit(PaletteMessage::ActionSelected(0))
                    .size(20)
                    .style(palette_input_style()),
                button(text("Esc").size(11).color(C::MUTED)
                    .font(iced::Font {
                        family: iced::font::Family::Monospace,
                        ..Default::default()
                    }))
                    .padding([6, 10])
                    .style(|_, _| iced::widget::button::Style {
                        background: Some(iced::Background::Color(
                            iced::Color::from_rgba(1.0, 1.0, 1.0, 0.05)
                        )),
                        border: iced::Border { color: C::BORDER_MID, width: 1.0, radius: 8.0.into() },
                        text_color: C::MUTED,
                        ..Default::default()
                    })
                    .on_press(PaletteMessage::Close),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding([20, 28])
        )
        .width(Length::Fill)
        .style(|_| container::Style {
            border: iced::Border { color: C::BORDER, width: 0.0, radius: 0.0.into() },
            ..Default::default()
        });

        // Divider
        let divider = container(Space::with_height(1.0))
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(C::BORDER)),
                ..Default::default()
            });

        // Suggested actions
        let suggested_label = text("Suggested Actions").size(10).color(C::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() });

        let suggested_items = column(
            SUGGESTED.iter().enumerate().map(|(i, (icon, title, subtitle))| {
                button(
                    row![
                        container(text(*icon).size(18).color(C::ACCENT))
                            .width(40).height(40)
                            .center_x(Length::Fixed(40.0))
                            .center_y(Length::Fixed(40.0))
                            .style(|_| container::Style {
                                background: Some(iced::Background::Color(
                                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.05)
                                )),
                                border: iced::Border { radius: 10.0.into(), ..Default::default() },
                                ..Default::default()
                            }),
                        column![
                            text(*title).size(13).color(C::FG)
                                .font(iced::Font { weight: iced::font::Weight::Medium, ..Default::default() }),
                            Space::with_height(2),
                            text(*subtitle).size(11).color(C::MUTED),
                        ]
                        .spacing(0),
                    ]
                    .spacing(16)
                    .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .padding([12, 16])
                .style(|_, status| {
                    let bg = match status {
                        iced::widget::button::Status::Hovered => {
                            Some(iced::Background::Color(C::ACCENT_DIM))
                        }
                        _ => None,
                    };
                    iced::widget::button::Style {
                        background: bg,
                        border: iced::Border { radius: 16.0.into(), ..Default::default() },
                        text_color: C::FG,
                        ..Default::default()
                    }
                })
                .on_press(PaletteMessage::ActionSelected(i))
                .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(2);

        // History divider
        let hist_divider = container(Space::with_height(1.0))
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(C::BORDER)),
                ..Default::default()
            });

        let history_label = text("History").size(10).color(C::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() });

        let history_items = column(
            HISTORY.iter().enumerate().map(|(i, entry)| {
                button(
                    row![
                        text("⏱").size(16).color(C::DIM),
                        text(*entry).size(13).color(C::MUTED),
                    ]
                    .spacing(16)
                    .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .padding([10, 16])
                .style(|_, status| {
                    let bg = match status {
                        iced::widget::button::Status::Hovered => {
                            Some(iced::Background::Color(C::ACCENT_DIM))
                        }
                        _ => None,
                    };
                    iced::widget::button::Style {
                        background: bg,
                        border: iced::Border { radius: 12.0.into(), ..Default::default() },
                        text_color: C::FG,
                        ..Default::default()
                    }
                })
                .on_press(PaletteMessage::HistorySelected(i))
                .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(2);

        let results = scrollable(
            container(
                column![
                    suggested_label,
                    Space::with_height(8),
                    suggested_items,
                    Space::with_height(16),
                    hist_divider,
                    Space::with_height(16),
                    history_label,
                    Space::with_height(8),
                    history_items,
                    Space::with_height(8),
                ]
                .padding([12, 16])
            )
            .width(Length::Fill)
        )
        .height(Length::Fixed(340.0));

        // Footer
        let footer = container(
            row![
                row![
                    text("↕").size(14).color(C::DIM),
                    text("Navigate").size(10).color(C::MUTED)
                        .font(iced::Font {
                            family: iced::font::Family::Monospace,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        }),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
                row![
                    text("↵").size(14).color(C::DIM),
                    text("Select").size(10).color(C::MUTED)
                        .font(iced::Font {
                            family: iced::font::Family::Monospace,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        }),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
                Space::with_width(Length::Fill),
                row![
                    container(Space::with_width(6.0))
                        .width(6).height(6)
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(C::ACCENT)),
                            border: iced::Border { radius: 999.0.into(), ..Default::default() },
                            ..Default::default()
                        }),
                    text("AI Agent Ready").size(10).color(C::MUTED)
                        .font(iced::Font {
                            family: iced::font::Family::Monospace,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        }),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .spacing(20)
            .align_y(Alignment::Center)
            .padding([14, 28])
        )
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(
                iced::Color::from_rgba(1.0, 1.0, 1.0, 0.02)
            )),
            border: iced::Border { color: C::BORDER, width: 1.0, radius: 0.0.into() },
            ..Default::default()
        });

        column![search, divider, results, footer].into()
    }
}
