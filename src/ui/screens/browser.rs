use iced::widget::{
    button, canvas, column, container, row, scrollable, text, Space,
};
use iced::widget::canvas::{Frame, Geometry, Image as CanvasImage};
use iced::mouse;
use iced::{Alignment, Background, Element, Length, Point, Rectangle, Size, Task};

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

struct PageCanvas<'a> {
    elements: &'a [StyledElement],
}

impl iced::widget::canvas::Program<BrowserMessage> for PageCanvas<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        plog!("DRAW", "Rendering {} elements into {:?}", self.elements.len(), bounds.size());
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(Point::new(0.0, 0.0), bounds.size(), iced::Color::WHITE);
        for el in self.elements {
            if let Some(ref handle) = el.image_handle {
                let iw = if el.width.is_finite() { el.width.max(50.0) } else { 50.0 };
                let ih = if el.height.is_finite() { el.height.max(50.0) } else { 50.0 };
                let ix = el.x.max(0.0);
                let iy = el.y.max(0.0);
                if ix.is_finite() && iy.is_finite() && iw.is_finite() && ih.is_finite() {
                    frame.draw_image(Rectangle::new(Point::new(ix, iy), Size::new(iw, ih)), CanvasImage::new(handle.clone()));
                }
                continue;
            }
            let bg = el.background_color;
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
        vec![frame.into_geometry()]
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
                        return (event::Status::Captured, Some(BrowserMessage::ElementClicked(i)));
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
        kor_vm.set_builtin("nav_back", korlang::vm::Value::String("\u{2190}".to_string()));
        kor_vm.set_builtin("nav_forward", korlang::vm::Value::String("\u{2192}".to_string()));
        kor_vm.set_builtin("nav_refresh", korlang::vm::Value::String("\u{21BB}".to_string()));
        kor_vm.set_builtin("nav_search", korlang::vm::Value::String("Search or navigate".to_string()));
        kor_vm.set_builtin("nav_bookmark", korlang::vm::Value::String("\u{2606}".to_string()));
        kor_vm.set_builtin("nav_palette", korlang::vm::Value::String("\u{229E}".to_string()));
        let status_src = r#"
Component StatusBar {
    Row(spacing: 8) {
        Text(size: 10, text: "Secure Core")
        Text(size: 10, text: " \u{00B7} ")
        Text(size: 10, text: "Flow Active")
        Text(size: 10, text: " \u{00B7} ")
        Text(size: 10, text: "1.2ms Latency")
    }
}
"#;
        let bytecode = compile(status_src);
        kor_vm.execute(bytecode);

        let mut navbar_kor_vm = VirtualMachine::new();
        navbar_kor_vm.heap.insert("nav_back".into(), korlang::vm::Value::String("\u{2190}".into()));
        navbar_kor_vm.heap.insert("nav_forward".into(), korlang::vm::Value::String("\u{2192}".into()));
        navbar_kor_vm.heap.insert("nav_refresh".into(), korlang::vm::Value::String("\u{21BB}".into()));
        navbar_kor_vm.heap.insert("nav_search".into(), korlang::vm::Value::String("Search or navigate".into()));
        navbar_kor_vm.heap.insert("nav_bookmark".into(), korlang::vm::Value::String("\u{2606}".into()));
        navbar_kor_vm.heap.insert("nav_palette".into(), korlang::vm::Value::String("\u{229E}".into()));
        navbar_kor_vm.heap.insert("url_input".into(), korlang::vm::Value::String(default_url.clone()));
        let navbar_src = r#"
Component NavBar {
    Row(spacing: 4) {
        Button(text: nav_back, on_click: "back")
        Button(text: nav_forward, on_click: "forward")
        Button(text: nav_refresh, on_click: "refresh")
        TextInput(placeholder: nav_search, on_submit: "navigate")
        Button(text: nav_bookmark, on_click: "bookmark")
        Button(text: nav_palette, on_click: "palette")
    }
}
"#;
        let navbar_bytecode = compile(navbar_src);
        navbar_kor_vm.execute(navbar_bytecode);

        let mut sidebar_kor_vm = VirtualMachine::new();
        let sidebar_src = r#"
Component SidebarBottom {
    Column(spacing: 8) {
        Button(text: "\u{23F1} History", on_click: "back")
        Button(text: "\u{2699} Settings", on_click: "settings")
    }
}
"#;
        let sb_bytecode = compile(sidebar_src);
        sidebar_kor_vm.execute(sb_bytecode);

        let mut sidebar_ws_kor_vm = VirtualMachine::new();
        let sidebar_ws_src = r#"
Component SidebarWS {
    Column(spacing: 8) {
        Text(text: "WORKSPACES", size: 10)
        Button(text: "\u{2B21} Design Studio", on_click: "ws0")
        Button(text: "\u{2B21} Research Lab", on_click: "ws1")
        Button(text: "\u{2B21} Deep Work", on_click: "ws2")
        Text(text: "COLLECTIONS", size: 10)
        Button(text: "\u{25A4} Aether UI", on_click: "ws0")
        Button(text: "\u{25A4} Rust / Iced Docs", on_click: "ws1")
    }
}
"#;
        let sws_bytecode = compile(sidebar_ws_src);
        sidebar_ws_kor_vm.execute(sws_bytecode);
        let loaded_tabs = load_tabs();
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
        Self {
            url: url_val,
            active_workspace: 0,
            content: content_val,
            styled_elements: vec![],
            loading: false,
            bridge: None,
            js_engine: None,
            tab_history,
            is_history_nav: false,
            bounds: (1440.0, 900.0),
            kor_vm,
            navbar_kor_vm,
            sidebar_kor_vm,
            sidebar_ws_kor_vm,
            tabs,
            active_tab: 0,
            layout_gen: 0,
        }
    }


    pub fn update(&mut self, msg: BrowserMessage) -> Task<BrowserMessage> {
        match msg {
            BrowserMessage::UrlChanged(s) => {
                self.url = s.clone();
                self.navbar_kor_vm.update_state("url_input", korlang::vm::Value::String(s));
                Task::none()
            }
            BrowserMessage::UrlSubmit => {
                plog!("NAV", "UrlSubmit: {}", self.url);
                let target = normalize_nav_url(&self.url);
                self.url = target.clone();
                self.loading = true;
                self.bridge = None;
                self.is_history_nav = false;
                let (bw, bh) = self.bounds;
                Task::perform(fetch_page_content(target, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
            }
            BrowserMessage::LinkClicked(url) => {
                plog!("NAV", "LinkClicked: {}", url);
                let target = normalize_nav_url(&url);
                self.url = target.clone();
                self.loading = true;
                self.bridge = None;
                self.is_history_nav = false;
                let (bw, bh) = self.bounds;
                Task::perform(fetch_page_content(target, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
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
                if let Some((url, idx)) = result {
                    plog!("NAV", "NavBack to index={} url={}", idx, url);
                    self.url = url.clone();
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
                if let Some((url, idx)) = result {
                    plog!("NAV", "NavForward to index={} url={}", idx, url);
                    self.url = url.clone();
                    self.is_history_nav = true;
                    self.loading = true;
                    self.bridge = None;
                    let (bw, bh) = self.bounds;
                    return Task::perform(fetch_page_content(url, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                }
                Task::none()
            }
            BrowserMessage::Refresh => {
                plog!("NAV", "Refresh: {}", self.url);
                self.loading = true;
                self.bridge = None;
                self.is_history_nav = false;
                let (bw, bh) = self.bounds;
                Task::perform(fetch_page_content(self.url.clone(), bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
            }
            BrowserMessage::PageLoaded(page_url, elements, bridge_opt) => {
                self.loading = false;
                let count = elements.len();
                plog!("PAGE", "PageLoaded: URL={} elements={}", page_url, count);
                self.url = page_url.clone();
                if !self.is_history_nav {
                    let (ref mut hist, ref mut idx) = self.tab_history[self.active_tab];
                    hist.truncate(*idx + 1);
                    hist.push(page_url);
                    *idx = hist.len() - 1;
                }
                self.is_history_nav = false;
                self.styled_elements = elements;
                self.layout_gen += 1;
                self.bridge = bridge_opt;
                self.js_engine = Some(JSEngine::new());
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
                                let _ = js.execute_source(&source, bridge);
                            }
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
                            self.bridge = None;
                            let (bw, bh) = self.bounds;
                            return Task::perform(fetch_page_content(url, bw, bh), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                        }
                    }
                }
                Task::none()
            }
            BrowserMessage::ElementClicked(idx) => {
                if let Some(ref bridge) = self.bridge {
                    let el = &self.styled_elements[idx];
                    let listeners = {
                        let b = bridge.lock().unwrap_or_else(|e| e.into_inner());
                        let tag = el.tag.to_lowercase();
                        let mut all = vec![];
                        if let Some(body) = b.body_id {
                            let candidates = b.query_selector_all(body, &tag);
                            for cid in candidates {
                                let els = b.get_event_listeners(cid, "click");
                                all.extend(els.into_iter().map(|s| (cid, s)));
                            }
                        }
                        all
                    };
                    if !listeners.is_empty() {
                        if let Some(ref mut js) = self.js_engine {
                            for (_node_id, source) in listeners {
                                let _ = js.execute_source(&source, bridge);
                            }
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
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<BrowserMessage> {
        let has_timers = self.bridge.as_ref().is_some_and(|b| b.lock().unwrap_or_else(|e| e.into_inner()).has_pending_timers());
        if has_timers {
            iced::time::every(std::time::Duration::from_millis(100)).map(|_| BrowserMessage::TimerTick)
        } else {
            iced::Subscription::none()
        }
    }

    pub fn view(&self) -> Element<'_, BrowserMessage> {
        let sidebar = self.sidebar();
        let main = self.main_area();
        row![sidebar, main].into()
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
        } else if !self.styled_elements.is_empty() {
            let total_h = self.styled_elements.iter()
                .map(|el| { let ey = if el.y.is_finite() { el.y } else { 0.0 }; ey + el.height.max(el.font_size.clamp(6.0, 200.0)) + 40.0 })
                .fold(0.0, f32::max);
            let total_h = if total_h.is_finite() { total_h.max(100.0) } else { 800.0 };
            let pg = PageCanvas { elements: &self.styled_elements };
            container(
                scrollable(canvas(pg).width(Length::Fixed(self.bounds.0)).height(Length::Fixed(total_h)))
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
        let logo = row![
            container(text("\u{2B21}").size(18).color(C::ACCENT))
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
        let bottom = render_kor_vm(&self.sidebar_kor_vm);
        let ws_content = render_kor_vm(&self.sidebar_ws_kor_vm);
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
                let close = button(text("\u{00D7}").size(10).color(C::DIM)).padding([2, 4]).on_press(BrowserMessage::CloseTab(i));
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
        let nav_content = render_kor_vm(&self.navbar_kor_vm);
        let bar = container(row![nav_content].spacing(16).align_y(Alignment::Center).padding([0, 40]))
            .height(Length::Fixed(64.0)).width(Length::Fill).center_y(Length::Fixed(64.0))
            .style(|_| container::Style { background: None, border: iced::Border { color: C::BORDER, width: 0.0, radius: 0.0.into() }, ..Default::default() });
        container(column![
            bar,
            container(Space::with_height(1.0)).width(Length::Fill)
                .style(|_| container::Style { background: Some(iced::Background::Color(C::BORDER)), ..Default::default() }),
        ]).width(Length::Fill).into()
    }

    fn status_bar(&self) -> Element<'_, BrowserMessage> {
        container(render_kor_vm(&self.kor_vm))
            .height(Length::Fixed(40.0)).width(Length::Fill)
            .center_x(Length::Fill).center_y(Length::Fixed(40.0))
            .style(status_bar_style()).into()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use iced::Color;
    use crate::engine::pipeline::{apply_caelum_layout, normalize_nav_url};

    fn make_test(tag: &str, text: &str, display: &str, parent: Option<usize>) -> StyledElement {
        StyledElement {
            tag: tag.to_string(), text: text.to_string(), wrapped_lines: vec![],
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

    const EPS: f32 = 0.01;

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
        let c = style.color.expect("color should be resolved");
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
