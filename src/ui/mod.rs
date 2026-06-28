
pub mod kor_renderer;
pub mod screens;
pub mod style;

use iced::{Element, Task, Subscription};
use screens::browser::{BrowserMessage, BrowserScreen};
use screens::palette::{PaletteMessage, PaletteScreen};
use screens::settings::{SettingsMessage, SettingsScreen};

#[derive(Debug, Clone)]
pub enum Message {
    Browser(BrowserMessage),
    Settings(SettingsMessage),
    Palette(PaletteMessage),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Browser,
    Settings,
    Palette,
}

pub struct AetherApp {
    current_screen: Screen,
    browser: BrowserScreen,
    settings: SettingsScreen,
    palette: PaletteScreen,
}

impl Default for AetherApp {
    fn default() -> Self {
        Self {
            current_screen: Screen::Browser,
            browser: BrowserScreen::new(),
            settings: SettingsScreen::new(),
            palette: PaletteScreen::new(),
        }
    }
}

impl AetherApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Browser(msg) => {
                match msg {
                    BrowserMessage::OpenSettings => {
                        self.current_screen = Screen::Settings;
                        Task::none()
                    }
                    BrowserMessage::OpenPalette => {
                        self.current_screen = Screen::Palette;
                        self.palette.reset();
                        Task::none()
                    }
                    other => {
                        self.browser.update(other).map(Message::Browser)
                    }
                }
            }
            Message::Settings(msg) => {
                match msg {
                    SettingsMessage::Back => {
                        self.current_screen = Screen::Browser;
                        Task::none()
                    },
                    other => self.settings.update(other).map(Message::Settings)
                }
            }
            Message::Palette(msg) => {
                match msg {
                    PaletteMessage::Close => {
                        self.current_screen = Screen::Browser;
                        Task::none()
                    }
                    other => {
                        self.palette.update(other).map(Message::Palette)
                    }
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.current_screen {
            Screen::Browser => self.browser.subscription().map(Message::Browser),
            _ => Subscription::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.current_screen {
            Screen::Browser => self.browser.view().map(Message::Browser),
            Screen::Settings => self.settings.view().map(Message::Settings),
            Screen::Palette => {
                use iced::widget::stack;
                stack![
                    self.browser.view().map(Message::Browser),
                    self.palette.view().map(Message::Palette),
                ]
                .into()
            }
        }
    }
}
