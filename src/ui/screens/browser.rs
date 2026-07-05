use iced::widget::{
    button, canvas, column, container, row, scrollable, text, text_input, Space,
};
use iced::widget::canvas::{Cache, Geometry, Image as CanvasImage};
use iced::keyboard;
use iced::mouse;
use iced::{Alignment, Background, Color, Element, Length, Point, Rectangle, Size, Task};

use crate::ui::style::*;
use crate::ui::screens::settings::AetherSettings;
use crate::plog;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;

use korlang::vm::{VirtualMachine, OpCode};
use korlang::compile;
use crate::engine::korlang::register_default_callbacks;
use crate::ui::kor_renderer::render_kor_vm;
use crate::engine::js::{JsBridge, JSEngine};
use crate::engine::pipeline::{fetch_page_content, StyledElement, normalize_nav_url, save_tabs, load_tabs, Tab};

// -- Messages

#[derive(Debug, Clone, PartialEq)]
pub enum DevToolsTab { Console, Elements, Network }

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
    ToggleConsole,
    DevToolsTabSelected(DevToolsTab),
    ToggleInspect,
    InspectElement(usize),
    UrlInputChanged(String),
    UrlSubmitted,
    AutocompleteSelected(usize),
    AutocompleteDismiss,
    FormElementClicked(usize),
    FormInputKeyPressed(char),
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
    pub kor_vm: RefCell<VirtualMachine>,
    pub sidebar_kor_vm: RefCell<VirtualMachine>,
    pub sidebar_ws_kor_vm: RefCell<VirtualMachine>,
    status_bytecode: Vec<OpCode>,
    sidebar_bytecode: Vec<OpCode>,
    sidebar_ws_bytecode: Vec<OpCode>,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    layout_gen: u64,
    page_canvas: Option<PageCanvas>,
    js_errors: Vec<String>,
    show_dev_console: bool,
    pub url_input: String,
    pub url_history: Vec<String>,
    pub show_autocomplete: bool,
    pub autocomplete_index: usize,
    pub dev_tools_tab: DevToolsTab,
    pub network_requests: Vec<String>,
    pub inspect_mode: bool,
    pub inspect_element: Option<usize>,
    pub form_inputs: HashMap<usize, String>,
    pub active_form_element: Option<usize>,
    pub settings: AetherSettings,
}

struct PageCanvas {
    elements: Vec<StyledElement>,
    cache: Cache,
}

impl PageCanvas {
    fn new(elements: Vec<StyledElement>) -> Self {
        Self { elements, cache: Cache::new() }
    }
}

impl iced::widget::canvas::Program<BrowserMessage> for PageCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let size = bounds.size();
        vec![self.cache.draw(renderer, size, |frame| {
            plog!("DRAW", "Rendering {} elements into {:?}", self.elements.len(), size);
            frame.fill_rectangle(Point::new(0.0, 0.0), size, iced::Color::WHITE);
            for el in &self.elements {
                if let Some(ref handle) = el.image_handle {
                    let iw = if el.width.is_finite() { el.width.max(50.0) } else { 50.0 };
                    let ih = if el.height.is_finite() { el.height.max(50.0) } else { 50.0 };
                    let ix = el.x.max(0.0);
                    let iy = el.y.max(0.0);
                    if ix.is_finite() && iy.is_finite() && iw.is_finite() && ih.is_finite() {
                        frame.draw_image(Rectangle::new(Point::new(ix, iy), Size::new(iw, ih)), CanvasImage::new(handle.clone()));
                    }
                } else if matches!(el.tag.as_str(), "input" | "textarea" | "select" | "button") {
                    let ex = if el.x.is_finite() { el.x.max(0.0) } else { 0.0 };
                    let ey = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
                    let ew = if el.width.is_finite() { el.width.max(60.0) } else { 200.0 };
                    let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { 32.0 };
                    let border_col = el.border_color.unwrap_or(iced::Color::from_rgb(0.7, 0.7, 0.7));
                    let bg_col = if el.tag == "button" { iced::Color::from_rgb(0.92, 0.92, 0.92) } else { iced::Color::WHITE };
                    frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, eh), bg_col);
                    frame.stroke_rectangle(Point::new(ex, ey), Size::new(ew, eh), iced::widget::canvas::Stroke::default().with_color(border_col).with_width(1.0));
                    let fs = if el.font_size.is_finite() { el.font_size.clamp(8.0, 64.0) } else { 14.0 };
                    let label = if el.text.is_empty() {
                        if el.tag == "button" { "Button".to_string() }
                        else if el.tag == "select" { "▾ Select".to_string() }
                        else { "".to_string() }
                    } else { el.text.clone() };
                    if !label.is_empty() {
                        frame.fill_text(iced::widget::canvas::Text {
                            content: label,
                            position: Point::new(ex + 4.0, ey + (eh - fs) / 2.0),
                            color: el.color,
                            size: iced::Pixels(fs),
                            ..Default::default()
                        });
                    }
                } else {
                    // ponytail: skip backgrounds for structural elements to avoid gray bars
                    let bg = if matches!(el.tag.as_str(), "body" | "html") { None } else { el.background_color };
                    let bw = el.border_widths;
                    let bc = el.border_color;
                    let ex = if el.x.is_finite() { el.x.max(0.0) } else { 0.0 };
                    let ey = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
                    let ew = if el.width.is_finite() { el.width.max(1.0) } else { 1.0 };
                    let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { let f = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 }; f * el.line_height.max(1.0) };
                    if bg.is_some() || bc.is_some() {
                        let fill = bg.unwrap_or(iced::Color::TRANSPARENT);
                        frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, eh), fill);
                    }
                    if let Some(color) = bc {
                        if bw[0] > 0.0 { frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, bw[0]), color); }
                        if bw[2] > 0.0 { frame.fill_rectangle(Point::new(ex, ey + eh - bw[2]), Size::new(ew, bw[2]), color); }
                        if bw[3] > 0.0 { frame.fill_rectangle(Point::new(ex, ey), Size::new(bw[3], eh), color); }
                        if bw[1] > 0.0 { frame.fill_rectangle(Point::new(ex + ew - bw[1], ey), Size::new(bw[1], eh), color); }
                    }
                    let weight = if el.font_weight == "bold" { iced::font::Weight::Bold } else { iced::font::Weight::Normal };
                    let fs = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 };
                    let line_h = fs * el.line_height.max(1.0);
                    let px0 = el.x.max(0.0) + bw[3];
                    let py0 = el.y.max(0.0) + bw[0];
                    let lines: Vec<&str> = if el.wrapped_lines.is_empty() { vec![&el.text] } else { el.wrapped_lines.iter().map(|s| s.as_str()).collect() };
                    for (li, line) in lines.iter().enumerate() {
                        let py = py0 + li as f32 * line_h;
                        if fs.is_finite() && px0.is_finite() && py.is_finite() && !line.is_empty() {
                            frame.fill_text(iced::widget::canvas::Text {
                                content: line.to_string(),
                                position: Point::new(px0, py),
                                color: el.color,
                                size: iced::Pixels(fs),
                                font: iced::Font { weight, ..Default::default() },
                                shaping: iced::widget::text::Shaping::Advanced,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        })]
    }

    fn update(
        &self,
        _state: &mut (),
        event: iced::widget::canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<BrowserMessage>) {
        use iced::widget::canvas::event;
        if let iced::widget::canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                plog!("CLICK", "Click at pos=({:.0},{:.0})", pos.x, pos.y);
                for (i, el) in self.elements.iter().enumerate() {
                    if el.is_link {
                        let text_w = el.text.len() as f32 * el.font_size * 0.55;
                        let hit = Rectangle::new(Point::new(el.x, el.y), Size::new(text_w, el.font_size + 4.0));
                        if hit.contains(pos) {
                            plog!("CLICK", "Link hit at element {} href={:?}", i, el.href);
                            if let Some(ref href) = el.href {
                                return (event::Status::Captured, Some(BrowserMessage::LinkClicked(href.clone())));
                            }
                        }
                    }
                    let ex = el.x.max(0.0);
                    let ey = el.y.max(0.0);
                    let ew = if el.width.is_finite() { el.width.max(1.0) } else { 200.0 };
                    let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { 30.0 };
                    let hit = Rectangle::new(Point::new(ex, ey), Size::new(ew, eh));
                    if hit.contains(pos) {
                        plog!("CLICK", "Element {} hit at [{:.0},{:.0} {:.0}x{:.0}] tag={}", i, ex, ey, ew, eh, el.tag);
                        let msg = if matches!(el.tag.as_str(), "input" | "textarea" | "select" | "button") {
                            BrowserMessage::FormElementClicked(i)
                        } else {
                            BrowserMessage::ElementClicked(i)
                        };
                        return (event::Status::Captured, Some(msg));
                    }
                }
            }
        }
        (event::Status::Ignored, None)
    }
}

impl BrowserScreen {
    pub fn new() -> Self {
        let default_url = "aether://design/spatial-minimalism".to_string();
        let mut kor_vm = VirtualMachine::new();
        register_default_callbacks(&mut kor_vm);
        kor_vm.set_builtin("status_left", korlang::vm::Value::String("Aether Ready".to_string()));
        kor_vm.set_builtin("status_mid", korlang::vm::Value::String("Idle".to_string()));
        kor_vm.set_builtin("status_right", korlang::vm::Value::String("Local shell".to_string()));
        let status_src = r#"
Component StatusBar {
    Row(spacing: 8) {
        Text(size: 10, text: status_left)
        Text(size: 10, text: " · ")
        Text(size: 10, text: status_mid)
        Text(size: 10, text: " · ")
        Text(size: 10, text: status_right)
    }
}
"#;
        let status_bytecode = compile(status_src);
        kor_vm.execute(status_bytecode.clone());

        let mut sidebar_kor_vm = VirtualMachine::new();
        register_default_callbacks(&mut sidebar_kor_vm);
        let sidebar_src = r#"
Component SidebarBottom {
    Column(spacing: 8) {
        Button(text: "⏱ History", on_click: "back")
        Button(text: "⚙ Settings", on_click: "settings")
    }
}
"#;
        let sidebar_bytecode = compile(sidebar_src);
        sidebar_kor_vm.execute(sidebar_bytecode.clone());

        let mut sidebar_ws_kor_vm = VirtualMachine::new();
        register_default_callbacks(&mut sidebar_ws_kor_vm);
        let sidebar_ws_src = r#"
Component SidebarWS {
    Column(spacing: 8) {
        Text(text: "WORKSPACES", size: 11)
        Button(text: "⬡ Design Studio", on_click: "ws0")
        Button(text: "⬡ Research Lab", on_click: "ws1")
        Button(text: "⬡ Deep Work", on_click: "ws2")
        Text(text: "COLLECTIONS", size: 11)
        Button(text: "▤ Aether UI", on_click: "ws0")
        Button(text: "▤ Rust / Iced Docs", on_click: "ws1")
    }
}
"#;
        let sidebar_ws_bytecode = compile(sidebar_ws_src);
        sidebar_ws_kor_vm.execute(sidebar_ws_bytecode.clone());
        let loaded_tabs = load_tabs();
        let url_history: Vec<String> = loaded_tabs.iter().map(|t| t.url.clone()).collect();
        let (tabs, tab_history, url_val, content_val) = if loaded_tabs.is_empty() {
            (vec![Tab { title: "New Tab".to_string(), url: default_url.clone() }],
             vec![(vec![default_url.clone()], 0)],
             default_url.clone(),
             "Welcome to Aether Browser".to_string())
        } else {
            let count = loaded_tabs.len();
            let history: Vec<(Vec<String>, usize)> = loaded_tabs.iter().map(|t| (vec![t.url.clone()], 0)).collect();
            let url = loaded_tabs[0].url.clone();
            (loaded_tabs, history, url, format!("Restored {} tabs", count))
        };
        let settings = AetherSettings::load();
        crate::engine::pipeline::set_js_enabled(settings.js_enabled);
        Self {
            url: url_val.clone(),
            active_workspace: 0,
            content: content_val,
            styled_elements: vec![],
            loading: false,
            bridge: None,
            js_engine: None,
            tab_history,
            is_history_nav: false,
            bounds: (1440.0, 900.0),
            kor_vm: RefCell::new(kor_vm),
            sidebar_kor_vm: RefCell::new(sidebar_kor_vm),
            sidebar_ws_kor_vm: RefCell::new(sidebar_ws_kor_vm),
            status_bytecode,
            sidebar_bytecode,
            sidebar_ws_bytecode,
            tabs,
            active_tab: 0,
            layout_gen: 0,
            page_canvas: None,
            js_errors: vec![],
            show_dev_console: false,
            url_input: url_val.clone(),
            url_history,
            show_autocomplete: false,
            autocomplete_index: 0,
            dev_tools_tab: DevToolsTab::Console,
            network_requests: vec![],
            inspect_mode: false,
            inspect_element: None,
            form_inputs: HashMap::new(),
            active_form_element: None,
            settings,
        }
    }


    pub fn update(&mut self, msg: BrowserMessage) -> Task<BrowserMessage> {
        // Handle pending korlang side effects before processing messages
        if let Some(url) = crate::engine::korlang::take_navigation_url() {
            return self.navigate_to(&url);
        }
        if let Some(title) = crate::engine::korlang::take_window_title() {
            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                tab.title = title;
            }
        }
        match msg {
            BrowserMessage::UrlChanged(s) => {
                self.url = s.clone();
                Task::none()
            }
            BrowserMessage::UrlSubmit => {
                let url = self.url.clone();
                plog!("NAV", "UrlSubmit: {}", url);
                self.navigate_to(&url)
            }
            BrowserMessage::UrlInputChanged(s) => {
                self.url_input = s.clone();
                self.show_autocomplete = !s.is_empty() && self.url_history.iter().any(|h| h.contains(&s));
                self.autocomplete_index = 0;
                Task::none()
            }
            BrowserMessage::UrlSubmitted => {
                let input = self.url_input.trim().to_string();
                self.show_autocomplete = false;
                if input.is_empty() { return Task::none(); }
                if !self.url_history.contains(&input) {
                    self.url_history.push(input.clone());
                }
                self.navigate_to(&input)
            }
            BrowserMessage::AutocompleteSelected(idx) => {
                self.show_autocomplete = false;
                let url = self.url_history.get(idx).cloned();
                if let Some(item) = url {
                    self.url_input = item.clone();
                    self.navigate_to(&item)
                } else { Task::none() }
            }
            BrowserMessage::AutocompleteDismiss => {
                self.show_autocomplete = false;
                Task::none()
            }
            BrowserMessage::LinkClicked(url) => {
                plog!("NAV", "LinkClicked: {}", url);
                self.navigate_to(&url)
            }
            BrowserMessage::NavBack => {
                let result = {
                    let (hist, idx) = &mut self.tab_history[self.active_tab];
                    if *idx > 0 {
                        *idx -= 1;
                        Some((hist[*idx].clone(), *idx))
                    } else {
                        None
                    }
                };
                if let Some((url, _)) = result {
                    plog!("NAV", "NavBack to {}", url);
                    self.is_history_nav = true;
                    self.loading = true;
                    self.bridge = None;
                    let (bw, bh) = self.bounds;
                    return Task::perform(fetch_page_content(url, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                }
                Task::none()
            }
            BrowserMessage::NavForward => {
                let result = {
                    let (hist, idx) = &mut self.tab_history[self.active_tab];
                    if *idx + 1 < hist.len() {
                        *idx += 1;
                        Some((hist[*idx].clone(), *idx))
                    } else {
                        None
                    }
                };
                if let Some((url, _)) = result {
                    plog!("NAV", "NavForward to {}", url);
                    self.is_history_nav = true;
                    self.loading = true;
                    self.bridge = None;
                    let (bw, bh) = self.bounds;
                    return Task::perform(fetch_page_content(url, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                }
                Task::none()
            }
            BrowserMessage::Refresh => {
                let url = self.url.clone();
                plog!("NAV", "Refresh: {}", url);
                self.navigate_to(&url)
            }
            BrowserMessage::PageLoaded(page_url, elements, bridge_opt) => {
                self.loading = false;
                let count = elements.len();
                plog!("PAGE", "PageLoaded: URL={} elements={}", page_url, count);
                self.url = page_url.clone();
                self.url_input = page_url.clone();
                self.show_autocomplete = false;
                if !self.is_history_nav {
                    let (ref mut hist, ref mut idx) = self.tab_history[self.active_tab];
                    hist.truncate(*idx + 1);
                    hist.push(page_url.clone());
                    *idx = hist.len() - 1;
                }
                if !self.url_history.contains(&page_url) && !page_url.starts_with("aether://") {
                    self.url_history.push(page_url.clone());
                }
                self.is_history_nav = false;
                self.styled_elements = elements;
                self.layout_gen += 1;
                self.page_canvas = Some(PageCanvas::new(self.styled_elements.clone()));
                let page_title = bridge_opt.as_ref().and_then(|b| {
                    b.lock()
                        .ok()
                        .map(|guard| guard.doc_title.trim().to_string())
                        .filter(|title| !title.is_empty())
                }).unwrap_or_else(|| {
                    page_url
                        .split("://")
                        .nth(1)
                        .and_then(|rest| rest.split('/').next())
                        .filter(|s| !s.is_empty())
                        .unwrap_or(&page_url)
                        .to_string()
                });
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    tab.title = page_title;
                }
                self.bridge = bridge_opt;
                self.js_engine = Some(JSEngine::new());
                self.js_errors = self.bridge.as_ref().map(|b| {
                    b.lock().unwrap_or_else(|e| e.into_inner()).js_errors.clone()
                }).unwrap_or_default();
                self.kor_vm.borrow_mut().update_state("status_mid", korlang::vm::Value::String("Loaded".to_string()));
                self.kor_vm.borrow_mut().update_state("status_right", korlang::vm::Value::String(format!("{} elements", count)));
                self.content = format!("Loaded ({} elements)", count);
                Task::none()
            }
            BrowserMessage::TimerTick => {
                if let Some(ref bridge) = self.bridge {
                    let ready = {
                        let mut b = bridge.lock().unwrap_or_else(|e| e.into_inner());
                        b.poll_timers()
                    };
                    if !ready.is_empty() {
                        if let Some(ref mut js) = self.js_engine {
                            for (_timer_id, source) in ready {
                                if let Err(e) = js.execute_source(&source, bridge) {
                                    if let Ok(mut b) = bridge.lock() {
                                        b.report_js_error(format!("Timer: {}", e));
                                    }
                                }
                            }
                            js.process_pending_js_work();
                        }
                    }
                    let nav = {
                        let mut b = bridge.lock().unwrap_or_else(|e| e.into_inner());
                        b.pending_navigation.take()
                    };
                    if let Some(url) = nav {
                        self.url = url;
                        self.loading = true;
                        self.bridge = None;
                        let (bw, bh) = self.bounds;
                        return Task::perform(fetch_page_content(self.url.clone(), bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                    }
                    let hist_delta = {
                        let mut b = bridge.lock().unwrap_or_else(|e| e.into_inner());
                        b.pending_history_delta.take()
                    };
                    if let Some(delta) = hist_delta {
                        let url = {
                            let (hist, idx) = &mut self.tab_history[self.active_tab];
                            let new_idx = (*idx as i32 + delta).clamp(0, hist.len() as i32 - 1) as usize;
                            if new_idx < hist.len() && new_idx != *idx {
                                *idx = new_idx;
                                Some(hist[new_idx].clone())
                            } else {
                                None
                            }
                        };
                        if let Some(url) = url {
                            self.url = url.clone();
                            self.is_history_nav = true;
                            self.loading = true;
                            self.kor_vm.borrow_mut().update_state("status_mid", korlang::vm::Value::String("Loading".to_string()));
                            self.kor_vm.borrow_mut().update_state("status_right", korlang::vm::Value::String(url.clone()));
                            self.bridge = None;
                            let (bw, bh) = self.bounds;
                            return Task::perform(fetch_page_content(url, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                        }
                    }
                }
                Task::none()
            }
            BrowserMessage::ElementClicked(idx) => {
                if self.inspect_mode {
                    self.inspect_element = Some(idx);
                    self.dev_tools_tab = DevToolsTab::Elements;
                    self.show_dev_console = true;
                    return Task::none();
                }
                if let Some(ref bridge) = self.bridge {
                    let el = &self.styled_elements[idx];
                    let listeners = {
                        let b = bridge.lock().unwrap_or_else(|e| e.into_inner());
                        let mut all = vec![];
                        if let Some(node_id) = b.find_node_by_path(&el.dom_path) {
                            all.extend(b.get_event_listeners_bubbling(node_id, "click"));
                        }
                        all
                    };
                    if !listeners.is_empty() {
                        if let Some(ref mut js) = self.js_engine {
                            for (source, _node_id) in listeners {
                                if let Err(e) = js.execute_source(&source, bridge) {
                                    if let Ok(mut b) = bridge.lock() {
                                        b.report_js_error(format!("Event: {}", e));
                                    }
                                }
                            }
                            js.process_pending_js_work();
                        }
                    }
                }
                Task::none()
            }
            BrowserMessage::Bookmark => Task::none(),
            BrowserMessage::WorkspaceSelected(i) => { self.active_workspace = i; Task::none() }
            BrowserMessage::TabSelected(i) => {
                if i < self.tabs.len() { self.active_tab = i; }
                Task::none()
            }
            BrowserMessage::NewTab => {
                let title = format!("Tab {}", self.tabs.len() + 1);
                self.tabs.push(Tab { title, url: "about:blank".to_string() });
                self.active_tab = self.tabs.len() - 1;
                self.url = "about:blank".to_string();
                self.content = "New tab".to_string();
                self.styled_elements = vec![];
                self.loading = false;
                self.bridge = None;
                self.tab_history.push((vec!["about:blank".to_string()], 0));
                save_tabs(&self.tabs);
                Task::none()
            }
            BrowserMessage::CloseTab(i) => {
                if self.tabs.len() > 1 && i < self.tabs.len() {
                    self.tabs.remove(i);
                    self.tab_history.remove(i);
                    if self.active_tab >= self.tabs.len() { self.active_tab = self.tabs.len() - 1; }
                    save_tabs(&self.tabs);
                }
                Task::none()
            }
            BrowserMessage::ToggleConsole => {
                self.show_dev_console = !self.show_dev_console;
                Task::none()
            }
            BrowserMessage::DevToolsTabSelected(tab) => {
                self.dev_tools_tab = tab;
                Task::none()
            }
            BrowserMessage::ToggleInspect => {
                self.inspect_mode = !self.inspect_mode;
                self.inspect_element = None;
                Task::none()
            }
            BrowserMessage::InspectElement(idx) => {
                self.inspect_element = Some(idx);
                Task::none()
            }
            BrowserMessage::FormElementClicked(idx) => {
                self.active_form_element = Some(idx);
                Task::none()
            }
            BrowserMessage::FormInputKeyPressed(ch) => {
                if let Some(idx) = self.active_form_element {
                    if ch == '\x08' {
                        let val = self.form_inputs.entry(idx).or_default();
                        val.pop();
                    } else {
                        let val = self.form_inputs.entry(idx).or_default();
                        val.push(ch);
                    }
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<BrowserMessage> {
        use iced::keyboard::key;
        let has_timers = self.bridge.as_ref().is_some_and(|b| b.lock().unwrap_or_else(|e| e.into_inner()).has_pending_timers());
        let timer_sub = if has_timers {
            iced::time::every(std::time::Duration::from_millis(100)).map(|_| BrowserMessage::TimerTick)
        } else {
            iced::Subscription::none()
        };
        let key_sub = keyboard::on_key_press(|k, _m| {
            match k {
                key::Key::Named(key::Named::F12) => Some(BrowserMessage::ToggleConsole),
                key::Key::Named(key::Named::Escape) => Some(BrowserMessage::AutocompleteDismiss),
                key::Key::Character(ref c) if c.chars().next().map_or(false, |ch| !ch.is_control()) => {
                    c.chars().next().map(BrowserMessage::FormInputKeyPressed)
                }
                key::Key::Named(key::Named::Backspace) => Some(BrowserMessage::FormInputKeyPressed('\x08')),
                _ => None,
            }
        });
        iced::Subscription::batch(vec![timer_sub, key_sub])
    }

    pub fn view(&self) -> Element<'_, BrowserMessage> {
        let sidebar = self.sidebar();
        let main = self.main_area();
        let content = if self.show_dev_console {
            let console = self.dev_console_overlay();
            column![row![sidebar, main], console].into()
        } else {
            row![sidebar, main].into()
        };
        // ponytail: autocomplete rendered inside top_bar, no overlay needed
        content
    }

    fn dev_console_overlay(&self) -> Element<'_, BrowserMessage> {
        let tabs_container = self.dev_console_tabs();
        let content: Element<'_, BrowserMessage> = match self.dev_tools_tab {
            DevToolsTab::Console => {
                let errors: Vec<Element<'_, BrowserMessage>> = self.js_errors.iter().rev().take(50).map(|e| {
                    text(format!("> {}", e)).size(12).color(iced::Color::WHITE).into()
                }).collect();
                if errors.is_empty() {
                    text("No console output").size(12).color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.5)).into()
                } else {
                    scrollable(column(errors).spacing(1)).width(Length::Fill).height(Length::Fill).into()
                }
            }
            DevToolsTab::Elements => {
                let inspect_el = self.inspect_element;
                let items: Vec<Element<'_, BrowserMessage>> = self.styled_elements.iter().enumerate().take(200).map(|(i, el)| {
                    let tag_display = if el.tag == "text" { format!("#text \"{}\"", el.text.chars().take(30).collect::<String>()) }
                        else { format!("<{}>", el.tag) };
                    let indent = "  ".repeat(el.indent_level);
                    let is_highlighted = inspect_el == Some(i);
                    let highlight = if is_highlighted { C::ACCENT } else { iced::Color::from_rgba(1.0, 1.0, 1.0, 0.8) };
                    let btn = button(text(format!("{}{}", indent, tag_display)).size(11).color(highlight))
                        .padding([2, 8]).width(Length::Fill).style(move |_, _| iced::widget::button::Style {
                            background: if is_highlighted { Some(Background::Color(Color::from_rgba(0.25, 0.5, 0.9, 0.2))) } else { None },
                            text_color: highlight,
                            border: iced::Border { radius: 2.0.into(), ..Default::default() },
                            ..Default::default()
                        }).on_press(BrowserMessage::InspectElement(i));
                    btn.into()
                }).collect();
                scrollable(column(items).spacing(0)).width(Length::Fill).height(Length::Fill).into()
            }
            DevToolsTab::Network => {
                let items: Vec<Element<'_, BrowserMessage>> = self.network_requests.iter().rev().take(100).map(|req| {
                    text(format!("> {}", req)).size(11).color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.7)).into()
                }).collect();
                let list: Element<'_, BrowserMessage> = if items.is_empty() {
                    text("No network requests logged").size(12).color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.5)).into()
                } else {
                    scrollable(column(items).spacing(1)).width(Length::Fill).height(Length::Fill).into()
                };
                list
            }
        };
        column![tabs_container, content]
            .width(Length::Fill).height(Length::Fixed(300.0))
            .into()
    }

    fn dev_console_tabs(&self) -> Element<'_, BrowserMessage> {
        let current = &self.dev_tools_tab;
        let make = |label: &'static str, tab: DevToolsTab| {
            let active = *current == tab;
            let fg = if active { C::ACCENT } else { iced::Color::from_rgba(1.0, 1.0, 1.0, 0.5) };
            button(text(label).size(12).color(fg))
                .padding([6, 14])
                .style(move |_, status| {
                    let bg = if active {
                        Some(Background::Color(Color::from_rgba(0.25, 0.5, 0.9, 0.15)))
                    } else {
                        match status {
                            iced::widget::button::Status::Hovered => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
                            _ => None,
                        }
                    };
                    iced::widget::button::Style { background: bg, text_color: fg, border: iced::Border { radius: 6.0.into(), ..Default::default() }, ..Default::default() }
                })
                .on_press(BrowserMessage::DevToolsTabSelected(tab))
        };
        let tab_row = row![
            make("Console", DevToolsTab::Console),
            make("Elements", DevToolsTab::Elements),
            make("Network", DevToolsTab::Network),
            Space::with_width(Length::Fill),
            button(text("\u{00D7}").size(14).color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.5)))
                .padding([4, 8]).style(|_, _| iced::widget::button::Style { background: None, ..Default::default() })
                .on_press(BrowserMessage::ToggleConsole),
        ].spacing(4).align_y(Alignment::Center).padding([4, 8]);
        container(tab_row)
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.9))),
                ..Default::default()
            })
            .into()
    }

    fn main_area(&self) -> Element<'_, BrowserMessage> {
        let top = self.top_bar();
        let status = self.status_bar();
        let body: Element<'_, BrowserMessage> = if self.loading {
            container(
                column![
                    text("Loading...").size(20).color(C::PAGE_MUTED),
                    text("Fetching page content").size(13).color(C::DIM),
                ]
                .align_x(Alignment::Center).spacing(8)
            )
            .width(Length::Fill).height(Length::Fill)
            .center_x(Length::Fill).center_y(Length::Fill)
            .style(|_| container::Style { background: Some(Background::Color(C::PAGE_BG)), ..Default::default() })
            .into()
        } else if self.page_canvas.is_some() {
            let pc = self.page_canvas.as_ref().unwrap();
            let total_h = pc.elements.iter()
                .map(|el| { let ey = if el.y.is_finite() { el.y } else { 0.0 }; ey + el.height.max(el.font_size.clamp(6.0, 200.0)) + 40.0 })
                .fold(0.0, f32::max);
            let total_h = if total_h.is_finite() { total_h.max(100.0) } else { 800.0 };
            container(
                scrollable(canvas(pc).width(Length::Fixed(self.bounds.0)).height(Length::Fixed(total_h)))
                    .width(Length::Fill).height(Length::Fill)
            )
            .width(Length::Fill).height(Length::Fill)
            .style(|_| container::Style { background: Some(Background::Color(C::PAGE_BG)), ..Default::default() })
            .into()
        } else {
            container(
                scrollable(
                    column(vec![text(&self.content).size(14).color(C::PAGE_TEXT).into()]).padding(40).max_width(800)
                )
                .width(Length::Fill).height(Length::Fill)
            )
            .width(Length::Fill).height(Length::Fill)
            .style(|_| container::Style { background: Some(Background::Color(C::PAGE_BG)), ..Default::default() })
            .into()
        };
        let tabs = self.tab_bar();
        container(column![tabs, top, body, status])
            .width(Length::Fill).height(Length::Fill).style(main_area_style()).into()
    }

    fn sidebar(&self) -> Element<'_, BrowserMessage> {
        {
            let mut vm = self.sidebar_kor_vm.borrow_mut();
            vm.stack.clear();
            vm.execute(self.sidebar_bytecode.clone());
        }
        {
            let mut vm = self.sidebar_ws_kor_vm.borrow_mut();
            vm.stack.clear();
            vm.execute(self.sidebar_ws_bytecode.clone());
        }
        let logo = row![
            container(text("⬡").size(18).color(C::ACCENT))
                .width(28).height(28)
                .center_x(Length::Fixed(28.0)).center_y(Length::Fixed(28.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.07))),
                    border: iced::Border { radius: 8.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            text("AETHER").size(16).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
        ].spacing(10).align_y(Alignment::Center);
        let bottom = render_kor_vm(&*self.sidebar_kor_vm.borrow());
        let ws_content = render_kor_vm(&*self.sidebar_ws_kor_vm.borrow());
        let content = column![logo, Space::with_height(16), ws_content, Space::with_height(Length::Fill), bottom]
            .padding([32, 24]).spacing(0).height(Length::Fill);
        container(content).width(Length::Fixed(260.0)).height(Length::Fill).style(sidebar_style()).into()
    }

    fn tab_bar(&self) -> Element<'_, BrowserMessage> {
        let tabs: Vec<Element<'_, BrowserMessage>> = self.tabs.iter().enumerate().map(|(i, tab)| {
            let is_active = i == self.active_tab;
            let bg = if is_active { Background::Color(C::PAGE_BG) } else { Background::Color(C::SURFACE) };
            let title = text(&tab.title).size(12).color(if is_active { C::ACCENT } else { C::MUTED });
            let tab_elem: Element<'_, BrowserMessage> = if self.tabs.len() > 1 {
                let close = button(text("×").size(12).color(C::DIM)).padding([2, 6]).on_press(BrowserMessage::CloseTab(i));
                let content = row![title, close].spacing(6).align_y(Alignment::Center);
                button(content).padding([6, 12])
                    .style(move |_, _| button::Style { background: Some(bg), border: iced::Border { radius: 4.0.into(), ..Default::default() }, ..Default::default() })
                    .on_press(BrowserMessage::TabSelected(i)).into()
            } else {
                button(title).padding([6, 12])
                    .style(move |_, _| button::Style { background: Some(bg), border: iced::Border { radius: 4.0.into(), ..Default::default() }, ..Default::default() })
                    .on_press(BrowserMessage::TabSelected(i)).into()
            };
            tab_elem
        }).collect();

        row![
            container(row(tabs).spacing(2)).width(Length::Fill),
            button(text("+").size(14).color(C::ACCENT)).padding([6, 10]).style(ghost_button_style()).on_press(BrowserMessage::NewTab),
        ].align_y(Alignment::Center).into()
    }

    fn top_bar(&self) -> Element<'_, BrowserMessage> {
        let can_go_back = self.tab_history.get(self.active_tab).map(|(_h, i)| *i > 0).unwrap_or(false);
        let can_go_forward = self.tab_history.get(self.active_tab).map(|(h, i)| *i + 1 < h.len()).unwrap_or(false);
        let secure_icon = text(secure_indicator(&self.url)).size(14);

        let back_btn: Element<'_, BrowserMessage> = if can_go_back {
            button(text("\u{2190}").size(18).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style()).on_press(BrowserMessage::NavBack).into()
        } else {
            button(text("\u{2190}").size(18).color(C::DIM))
                .padding([6, 8]).style(nav_icon_button_style()).into()
        };
        let fwd_btn: Element<'_, BrowserMessage> = if can_go_forward {
            button(text("\u{2192}").size(18).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style()).on_press(BrowserMessage::NavForward).into()
        } else {
            button(text("\u{2192}").size(18).color(C::DIM))
                .padding([6, 8]).style(nav_icon_button_style()).into()
        };
        let refresh_btn = button(text("\u{21BB}").size(18).color(C::MUTED))
            .padding([6, 8]).style(nav_icon_button_style()).on_press(BrowserMessage::Refresh);
        let url_input_widget = text_input("Search or navigate", &self.url_input)
            .on_input(BrowserMessage::UrlInputChanged)
            .on_submit(BrowserMessage::UrlSubmitted)
            .size(14).padding(10)
            .style(url_input_style())
            .width(Length::Fill);

        let url_bar = container(
            row![secure_icon, url_input_widget]
                .spacing(8).align_y(Alignment::Center).padding([0, 12])
        ).style(|_| container::Style {
            background: Some(Background::Color(C::SURFACE)),
            border: iced::Border { color: C::BORDER, width: 1.0, radius: 999.0.into() },
            ..Default::default()
        }).width(Length::Fill);

        let bookmark_btn = button(text("\u{2606}").size(16).color(C::MUTED))
            .padding([6, 8]).style(nav_icon_button_style()).on_press(BrowserMessage::Bookmark);
        let palette_btn = button(text("\u{229E}").size(16).color(C::MUTED))
            .padding([6, 8]).style(nav_icon_button_style()).on_press(BrowserMessage::OpenPalette);
        let inspect_icon = if self.inspect_mode { "\u{25C9}" } else { "\u{25CB}" };
        let inspect_btn = button(text(inspect_icon).size(14).color(if self.inspect_mode { C::ACCENT } else { C::MUTED }))
            .padding([6, 8]).style(nav_icon_button_style()).on_press(BrowserMessage::ToggleInspect);

        // Autocomplete dropdown
        let matches: Vec<&String> = if self.show_autocomplete && !self.url_input.is_empty() {
            self.url_history.iter().filter(|h| h.contains(&self.url_input)).take(8).collect()
        } else { vec![] };

        let matched_index = self.autocomplete_index;
        let input_with_dropdown: Element<'_, BrowserMessage> = if matches.is_empty() {
            url_bar.into()
        } else {
            let items: Vec<Element<'_, BrowserMessage>> = matches.iter().enumerate().map(|(i, h)| {
                let selected = i == matched_index;
                let bg_color = if selected { C::ACCENT_DIM } else { Color::TRANSPARENT };
                let item = container(text(h.as_str()).size(12).color(C::FG))
                    .width(Length::Fill).padding([6, 12])
                    .style(move |_| container::Style {
                        background: Some(Background::Color(bg_color)),
                        border: iced::Border { radius: 4.0.into(), ..Default::default() },
                        ..Default::default()
                    });
                button(item).width(Length::Fill).padding(0)
                    .style(|_, _| iced::widget::button::Style { background: None, text_color: C::FG, border: iced::Border { radius: 4.0.into(), ..Default::default() }, ..Default::default() })
                    .on_press(BrowserMessage::AutocompleteSelected(i)).into()
            }).collect();

            column![
                url_bar,
                container(column(items).spacing(0).padding(4).max_width(600.0))
                    .style(autocomplete_dropdown_style())
                    .max_width(600.0),
            ].spacing(0).into()
        };

        let bar = container(
            row![
                back_btn, fwd_btn, refresh_btn,
                Space::with_width(8),
                input_with_dropdown,
                Space::with_width(8),
                inspect_btn, bookmark_btn, palette_btn,
            ].spacing(4).align_y(Alignment::Center).padding([0, 16])
        ).height(Length::Fixed(56.0)).width(Length::Fill).center_y(Length::Fixed(56.0))
        .style(|_| container::Style { background: None, ..Default::default() });

        container(column![
            bar,
            container(Space::with_height(1.0)).width(Length::Fill)
                .style(|_| container::Style { background: Some(Background::Color(C::BORDER)), ..Default::default() }),
        ]).width(Length::Fill).into()
    }

    fn status_bar(&self) -> Element<'_, BrowserMessage> {
        {
            let mut vm = self.kor_vm.borrow_mut();
            vm.stack.clear();
            vm.execute(self.status_bytecode.clone());
        }
        container(render_kor_vm(&*self.kor_vm.borrow()))
            .height(Length::Fixed(40.0)).width(Length::Fill)
            .center_x(Length::Fill).center_y(Length::Fixed(40.0))
            .style(status_bar_style()).into()
    }

    fn navigate_to(&mut self, input: &str) -> Task<BrowserMessage> {
        let input = input.trim();
        if input.is_empty() { return Task::none(); }

        // Check if it's a search query (not a URL)
        let target = if AetherSettings::is_url(input) {
            normalize_nav_url(input)
        } else {
            self.settings.search_url(input)
        };

        plog!("NAV", "Navigating to: {}", target);
        self.url = target.clone();
        self.url_input = target.clone();
        self.show_autocomplete = false;
        self.loading = true;
        self.bridge = None;
        self.is_history_nav = false;
        let (bw, bh) = self.bounds;
        Task::perform(fetch_page_content(target, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
    }

}

fn secure_indicator(url: &str) -> String {
    if url.starts_with("https://") { "\u{1F512}".to_string() }
    else if url.starts_with("http://") { "\u{26A0}".to_string() }
    else { String::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Color;
    use crate::engine::pipeline::{apply_caelum_layout, normalize_nav_url};

    fn make_test(tag: &str, text: &str, display: &str, parent: Option<usize>) -> StyledElement {
        StyledElement {
            tag: tag.to_string(), text: text.to_string(), wrapped_lines: vec![],
            dom_path: vec![],
            is_link: false, href: None, indent_level: 0,
            color: Color::BLACK, font_size: 16.0, font_weight: "normal".to_string(),
            background_color: None, border_widths: [0.0; 4], border_color: None,
            image_handle: None, image_url: None,
            margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None,
            padding: [0.0; 4], display: display.to_string(),
            flex_direction: "row".to_string(), flex_wrap: "nowrap".to_string(),
            justify_content: "flex-start".to_string(), align_items: "stretch".to_string(),
            flex_grow: 0.0, flex_shrink: 1.0, flex_basis: None,
            css_width: None, css_height: None, parent_index: parent,
            min_width: None, max_width: None, min_height: None, max_height: None,
            x: 0.0, y: 0.0, width: 0.0, height: 0.0,
            line_height: 1.4, text_decoration: String::new(),
            text_transform: String::new(), border_radius: [0.0; 4],
        }
    }

    #[test]
    fn test_ifc_simple_inline_siblings() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hello", "inline", Some(0)),
            make_test("span", "World", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.x >= 0.0, "x={}", el.x); }
        assert!(elements[2].x >= elements[1].x, "span1 x={} < span0 x={}", elements[2].x, elements[1].x);
    }

    #[test]
    fn test_ifc_single_inline_in_block() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hi", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0, 6000.0);
        assert!(elements[1].x.is_finite() && elements[1].x >= 0.0);
        assert!(elements[1].width.is_finite() && elements[1].width > 0.0);
    }

    #[test]
    fn test_ifc_inline_wraps_when_exceeds_container() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "ABCDEFGH", "inline", Some(0)),
            make_test("span", "IJKLMNOP", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.y.is_finite()); }
    }

    #[test]
    fn test_ifc_mixed_inline_and_block() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hello", "inline", Some(0)),
            make_test("p", "Block", "block", Some(0)),
            make_test("span", "World", "inline", Some(0)),
        ];
        apply_caelum_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.y.is_finite()); }
    }

    #[test]
    fn test_ifc_nested_inline() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Outer ", "inline", Some(0)),
            make_test("span", "Inner", "inline", Some(1)),
        ];
        apply_caelum_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.y.is_finite()); }
    }

    #[test]
    fn test_ifc_inline_block_margin_contrib() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            StyledElement {
                margin_top: 10.0, margin_bottom: 10.0,
                css_width: Some(100.0), css_height: Some(50.0),
                width: 100.0, height: 50.0,
                ..make_test("div", "", "inline-block", Some(0))
            },
        ];
        apply_caelum_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.y.is_finite()); }
    }

    #[test]
    fn test_stratus_roundtrip() {
        let css = r#"
            body { margin: 0; padding: 0; }
            .box { display: block; color: #ff0000; font-size: 18px; }
        "#;
        let stylesheet = crate::engine::stratus::parse(css);
        assert_eq!(stylesheet.rules.len(), 2);
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("class".to_string(), "box".to_string());
        let ed = crate::engine::stratus::ElementData::with_attributes("div".to_string(), attrs);
        let style = crate::engine::stratus::resolve_style(&ed, &stylesheet);
        assert_eq!(style.display, crate::engine::stratus::Display::Block);
        assert!(style.color.is_some(), "color should be resolved");
        let c = style.color.unwrap_or(crate::engine::stratus::Color { r: 0, g: 0, b: 0, a: 255 });
        assert_eq!(c.r, 255, "r={}", c.r);
    }

    #[test]
    fn test_js_bridge_init() {
        use crate::engine::js::JsBridge;
        let bridge = JsBridge::new();
        let dom = bridge.to_dom();
        assert!(dom.is_document());
    }

    #[test]
    fn test_nav_url_normalization() {
        let cases = [
            ("https://example.com", "https://example.com"),
            ("//example.com", "https://example.com"),
            ("example.com", "https://example.com"),
            ("http://example.com", "http://example.com"),
            ("aether://home", "aether://home"),
        ];
        for (input, expected) in &cases {
            assert_eq!(&normalize_nav_url(input), expected, "input={}", input);
        }
    }
}
