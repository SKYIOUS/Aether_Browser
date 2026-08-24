use iced::widget::{button, container, row, text};
use iced::{Alignment, Background, Element, Length};
use crate::ui::style::*;

pub fn tab_bar(screen: &super::BrowserScreen) -> Element<'_, super::BrowserMessage> {
    let tabs: Vec<Element<'_, super::BrowserMessage>> = screen.tabs.iter().enumerate().map(|(i, tab)| {
        let is_active = i == screen.active_tab;
        let is_hovered = tab.is_hovered;
        let bg = if is_active { 
            Background::Color(C::PAGE_BG) 
        } else if is_hovered { 
            Background::Color(C::SURFACE) 
        } else { 
            Background::Color(C::SURFACE) 
        };
        let title_color = if is_active { C::ACCENT } else if is_hovered { C::FG } else { C::MUTED };
        let title = text(&tab.title).size(12).color(title_color);
        let tab_elem: Element<'_, super::BrowserMessage> = if screen.tabs.len() > 1 {
            let close = button(text("\u{00D7}").size(12).color(if is_hovered { C::ACCENT } else { C::DIM }))
                .padding([2, 6])
                .style(move |_, _| button::Style {
                    background: Some(Background::Color(if is_hovered { C::ACCENT } else { C::TRANSPARENT })),
                    border: iced::Border { radius: 3.0.into(), ..Default::default() },
                    ..Default::default()
                })
                .on_press(super::BrowserMessage::CloseTab(i));
            let content = row![title, close].spacing(6).align_y(Alignment::Center);
            button(content)
                .padding([6, 12])
                .style(move |_, _| button::Style { 
                    background: Some(bg), 
                    border: iced::Border { radius: 4.0.into(), ..Default::default() }, 
                    ..Default::default() 
                })
                .on_press(super::BrowserMessage::TabSelected(i))
                .into()
        } else {
            button(title)
                .padding([6, 12])
                .style(move |_, _| button::Style { 
                    background: Some(bg), 
                    border: iced::Border { radius: 4.0.into(), ..Default::default() }, 
                    ..Default::default() 
                })
                .on_press(super::BrowserMessage::TabSelected(i))
                .into()
        };
        tab_elem
    }).collect();

    let active = screen.active_tab;
    // ASCII labels only: this codebase has a history of glyph-font trouble.
    row![
        container(row(tabs).spacing(2)).width(Length::Fill),
        button(text("+").size(14).color(C::ACCENT))
            .padding([6, 10])
            .style(ghost_button_style())
            .on_press(super::BrowserMessage::NewTab),
        button(text("Dup").size(11).color(C::MUTED))
            .padding([5, 7])
            .style(ghost_button_style())
            .on_press(super::BrowserMessage::DuplicateTab(active)),
        button(text("Close others").size(11).color(C::MUTED))
            .padding([5, 7])
            .style(ghost_button_style())
            .on_press(super::BrowserMessage::CloseOtherTabs(active)),
    ].align_y(Alignment::Center).into()
}
