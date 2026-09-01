use crate::ui::style::*;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Color, Element, Length};

#[derive(Debug, Clone, PartialEq)]
pub enum DevToolsTab {
    Console,
    Elements,
    Network,
}

pub fn dev_console_overlay(screen: &super::BrowserScreen) -> Element<'_, super::BrowserMessage> {
    let tabs_container = dev_console_tabs(&screen.dev_tools_tab);
    let content: Element<'_, super::BrowserMessage> = match screen.dev_tools_tab {
        DevToolsTab::Console => {
            let errors: Vec<Element<'_, super::BrowserMessage>> = screen
                .js_errors
                .iter()
                .rev()
                .take(50)
                .map(|e| text(format!("> {}", e)).size(12).color(Color::WHITE).into())
                .collect();
            if errors.is_empty() {
                text("No console output")
                    .size(12)
                    .color(Color::from_rgba(1.0, 1.0, 1.0, 0.5))
                    .into()
            } else {
                scrollable(column(errors).spacing(1))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
        DevToolsTab::Elements => {
            let inspect_el = screen.inspect_element;
            let items: Vec<Element<'_, super::BrowserMessage>> = screen
                .styled_elements
                .iter()
                .enumerate()
                .take(200)
                .map(|(i, el)| {
                    let tag_display = if el.tag == "text" {
                        format!("#text \"{}\"", el.text.chars().take(30).collect::<String>())
                    } else {
                        format!("<{}>", el.tag)
                    };
                    let indent = "  ".repeat(el.indent_level);
                    let is_highlighted = inspect_el == Some(i);
                    let highlight = if is_highlighted {
                        C::accent()
                    } else {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.8)
                    };
                    let btn = button(
                        text(format!("{}{}", indent, tag_display))
                            .size(11)
                            .color(highlight),
                    )
                    .padding([2, 8])
                    .width(Length::Fill)
                    .style(move |_, _| iced::widget::button::Style {
                        background: if is_highlighted {
                            Some(iced::Background::Color(Color::from_rgba(
                                0.25, 0.5, 0.9, 0.2,
                            )))
                        } else {
                            None
                        },
                        text_color: highlight,
                        border: iced::Border {
                            radius: 2.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .on_press(super::BrowserMessage::InspectElement(i));
                    btn.into()
                })
                .collect();
            scrollable(column(items).spacing(0))
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        DevToolsTab::Network => {
            let items: Vec<Element<'_, super::BrowserMessage>> = screen
                .network_requests
                .iter()
                .rev()
                .take(100)
                .map(|req| {
                    text(format!("> {}", req))
                        .size(11)
                        .color(Color::from_rgba(1.0, 1.0, 1.0, 0.7))
                        .into()
                })
                .collect();
            let list: Element<'_, super::BrowserMessage> = if items.is_empty() {
                text("No network requests logged")
                    .size(12)
                    .color(Color::from_rgba(1.0, 1.0, 1.0, 0.5))
                    .into()
            } else {
                scrollable(column(items).spacing(1))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            };
            list
        }
    };
    column![tabs_container, content]
        .width(Length::Fill)
        .height(Length::Fixed(300.0))
        .into()
}

pub fn dev_console_tabs(current: &DevToolsTab) -> Element<'_, super::BrowserMessage> {
    let make = |label: &'static str, tab: DevToolsTab| {
        let active = *current == tab;
        let fg = if active {
            C::accent()
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.5)
        };
        button(text(label).size(12).color(fg))
            .padding([6, 14])
            .style(move |_, status| {
                let bg = if active {
                    Some(iced::Background::Color(Color::from_rgba(
                        0.25, 0.5, 0.9, 0.15,
                    )))
                } else {
                    match status {
                        iced::widget::button::Status::Hovered => Some(iced::Background::Color(
                            Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                        )),
                        _ => None,
                    }
                };
                iced::widget::button::Style {
                    background: bg,
                    text_color: fg,
                    border: iced::Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .on_press(super::BrowserMessage::DevToolsTabSelected(tab))
    };
    let tab_row = row![
        make("Console", DevToolsTab::Console),
        make("Elements", DevToolsTab::Elements),
        make("Network", DevToolsTab::Network),
        Space::with_width(Length::Fill),
        button(
            text("\u{00D7}")
                .size(14)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.5))
        )
        .padding([4, 8])
        .style(|_, _| iced::widget::button::Style {
            background: None,
            ..Default::default()
        })
        .on_press(super::BrowserMessage::ToggleConsole),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .padding([4, 8]);
    container(tab_row)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.0, 0.0, 0.0, 0.9,
            ))),
            ..Default::default()
        })
        .into()
}
