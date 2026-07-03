use iced::widget::{
    button, canvas, column, container, row, scrollable, text, Space,
};
use iced::widget::canvas::{Frame, Geometry, Image as CanvasImage, Program};
use iced::widget::image::Handle;
use iced::mouse;
use iced::{Alignment, Background, Color, Element, Length, Point, Rectangle, Size, Task};

use crate::ui::style::*;
use crate::plog;

use std::sync::{Arc, Mutex};

use korlang::vm::VirtualMachine;
use korlang::compile;
use crate::ui::kor_renderer::render_kor_vm;
use crate::engine::js::{JsBridge, JSEngine};
use crate::engine::pipeline::{fetch_page_content, StyledElement, normalize_nav_url, save_tabs, load_tabs, Tab};

// -- Messages

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
    Bookmark,
    LinkClicked(String),
    PageLoaded(String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>),
    TimerTick,
    ElementClicked(usize),
    TabSelected(usize),
    NewTab,
    CloseTab(usize),
    None,
}

// -- State

pub struct BrowserScreen {
    pub url: String,
    pub active_workspace: usize,
    pub content: String,
    pub styled_elements: Vec<StyledElement>,
    pub loading: bool,
    pub bridge: Option<Arc<Mutex<JsBridge>>>,
    pub js_engine: Option<JSEngine>,
    tab_history: Vec<(Vec<String>, usize)>,
    is_history_nav: bool,
    pub bounds: (f32, f32),
    pub kor_vm: VirtualMachine,
    pub navbar_kor_vm: VirtualMachine,
    pub sidebar_kor_vm: VirtualMachine,
    pub sidebar_ws_kor_vm: VirtualMachine,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    layout_gen: u64,
}
