use iced::widget::{
    button, canvas, column, container, row, scrollable, text, text_input, Space,
};
use iced::widget::canvas::{Frame, Geometry, Image as CanvasImage, Program};
use iced::widget::image::Handle;
use iced::mouse;
use iced::{Alignment, Background, Color, Element, Length, Point, Rectangle, Size, Task};

use crate::ui::style::*;
use crate::plog;

use crate::engine::media::MediaEngine;
use korlang::vm::VirtualMachine;
use crate::ui::kor_renderer::render_kor_vm;
use std::sync::{Arc, Mutex};

use crate::engine::dom::{ElementData, Node, NodeType};
use crate::engine::js::{JSEngine, JsBridge};
use crate::engine::net;
use crate::engine::parser::Parser;
use crate::engine::stratus::{self, ComputedStyle, Display, Stylesheet};

const CHAR_W_SCALE: f32 = 0.58;
const LINE_H_SCALE: f32 = 1.4;

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    UrlChanged(String),
    UrlSubmit,
    NavBack,
    NavForward,
    Refresh,
    WorkspaceSelected(usize),
    OpenSettings,
    OpenPalette,
    LinkClicked(String),
    PageLoaded(String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>),
    TimerTick,
    ElementClicked(usize),
    None,
}

pub struct BrowserScreen {
    pub url: String,
    pub active_workspace: usize,
    pub content: String,
    pub styled_elements: Vec<StyledElement>,
    pub loading: bool,
    pub kor_vm: Option<VirtualMachine>,
    pub media: MediaEngine,
    pub bridge: Option<Arc<Mutex<JsBridge>>>,
    pub js_engine: Option<JSEngine>,
    history: Vec<String>,
    history_index: usize,
    is_history_nav: bool,
}

#[derive(Debug, Clone)]
pub struct StyledElement {
    pub tag: String,
    pub text: String,
    pub wrapped_lines: Vec<String>,
    pub is_link: bool,
    pub href: Option<String>,
    pub indent_level: usize,
    pub color: Color,
    pub font_size: f32,
    pub font_weight: String,
    pub background_color: Option<Color>,
    pub border_widths: [f32; 4],
    pub border_color: Option<Color>,
    pub border_radius: f32,
    pub image_handle: Option<Handle>,
    pub image_url: Option<String>,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub padding: [f32; 4],
    pub display: String,
    pub flex_direction: String,
    pub flex_wrap: String,
    pub justify_content: String,
    pub align_items: String,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BrowserScreen {
    pub fn new() -> Self {
        let default_url = "aether://design/spatial-minimalism".to_string();
        let mut vm = VirtualMachine::new();
        let source = include_str!("../../../korlang/lib/browser_ui.kor");
        let bc = korlang::compile(source);
        vm.execute(bc);

        Self {
            url: default_url.clone(),
            active_workspace: 0,
            content: "Welcome".to_string(),
            styled_elements: vec![],
            loading: false,
            kor_vm: Some(vm),
            bridge: None,
            js_engine: None,
            history: vec![default_url.clone()],
            history_index: 0,
            is_history_nav: false,
            media: MediaEngine::new(),
        }
    }

    pub fn update(&mut self, msg: BrowserMessage) -> Task<BrowserMessage> {
        match msg {
            BrowserMessage::UrlChanged(s) => {
                self.url = s.clone();
                if let Some(ref mut vm) = self.kor_vm {
                    vm.update_state("url", korlang::Value::String(s));
                }
            }
            BrowserMessage::None => {}
            _ => {}
        }
        Task::none()
    }

    pub fn subscription(&self) -> iced::Subscription<BrowserMessage> {
        iced::Subscription::none()
    }

    pub fn view(&self) -> Element<'_, BrowserMessage> {
        let ui = if let Some(ref vm) = self.kor_vm { render_kor_vm(vm) } else { text("Init...").into() };
        let pg = PageCanvas { elements: &self.styled_elements };
        column![ui, scrollable(canvas(pg).width(Length::Fill).height(Length::Fixed(2000.0)))].into()
    }
}

struct PageCanvas<'a> { elements: &'a [StyledElement] }
impl Program<BrowserMessage> for PageCanvas<'_> {
    type State = ();
    fn draw(&self, _state: &(), renderer: &iced::Renderer, _theme: &iced::Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        for el in self.elements {
            if el.tag == "video" {
                frame.fill_rectangle(Point::new(el.x, el.y), Size::new(el.width, el.height), Color::BLACK);
                continue;
            }
            if let Some(ref h) = el.image_handle {
                frame.draw_image(Rectangle::new(Point::new(el.x, el.y), Size::new(el.width, el.height)), CanvasImage::new(h.clone()));
                continue;
            }
            if let Some(bg) = el.background_color { frame.fill_rectangle(Point::new(el.x, el.y), Size::new(el.width, el.height), bg); }
            frame.fill_text(iced::widget::canvas::Text { content: el.text.clone(), position: Point::new(el.x, el.y), color: el.color, size: iced::Pixels(el.font_size), ..Default::default() });
        }
        vec![frame.into_geometry()]
    }
}
