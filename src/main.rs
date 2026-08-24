mod ui;
pub mod engine;
mod logging;

use ui::VayuApp;
use ui::get_app_theme;
use iced::{window, Size};

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
