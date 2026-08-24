use iced::widget::{container, row, text, Space};
use iced::{Alignment, Color, Element, Length};
use crate::ui::style::*;

pub fn sidebar(screen: &super::BrowserScreen) -> Element<'_, super::BrowserMessage> {
    {
        let mut vm = screen.sidebar_kor_vm.borrow_mut();
        vm.stack.clear();
        vm.execute(screen.sidebar_bytecode.clone());
    }
    {
        let mut vm = screen.sidebar_ws_kor_vm.borrow_mut();
        vm.stack.clear();
        vm.execute(screen.sidebar_ws_bytecode.clone());
    }
    let logo = row![
        container(text("\u{26A1}").size(18).color(C::accent()))
            .width(28).height(28)
            .center_x(Length::Fixed(28.0)).center_y(Length::Fixed(28.0))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.07))),
                border: iced::Border { radius: 8.0.into(), ..Default::default() },
                ..Default::default()
            }),
        text("AETHER").size(16).color(C::fg())
            .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
    ].spacing(10).align_y(Alignment::Center);
    let bottom = crate::ui::kor_renderer::render_kor_vm(&screen.sidebar_kor_vm.borrow());
    let ws_content = crate::ui::kor_renderer::render_kor_vm(&screen.sidebar_ws_kor_vm.borrow());
    let content = iced::widget::column![logo, Space::with_height(16), ws_content, Space::with_height(Length::Fill), bottom]
        .padding([32, 24]).spacing(0).height(Length::Fill);
    container(content).width(Length::Fixed(260.0)).height(Length::Fill).style(sidebar_style()).into()
}
