pub mod engine;
mod logging;
mod ui;

use iced::{window, Size};
use ui::get_app_theme;
use ui::VayuApp;

pub fn main() -> iced::Result {
    iced::application("Vayu Browser", VayuApp::update, VayuApp::view)
        .subscription(VayuApp::subscription)
        .window(window::Settings {
            size: Size::new(1440.0, 900.0),
            min_size: Some(Size::new(900.0, 600.0)),
            ..Default::default()
        })
        .theme(|_| get_app_theme())
        .run()
}
