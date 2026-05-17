mod ui;
pub mod engine;

use ui::AetherApp;
use iced::{window, Size};

pub fn main() -> iced::Result {
    iced::application("Aether Browser", AetherApp::update, AetherApp::view)
        .window(window::Settings {
            size: Size::new(1440.0, 900.0),
            min_size: Some(Size::new(900.0, 600.0)),
            ..Default::default()
        })
        .theme(|_| iced::Theme::Light)
        .run()
}
