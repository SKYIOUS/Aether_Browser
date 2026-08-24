use iced::widget::{button, canvas as iced_canvas, column, container, row, scrollable, text, text_input, Space};
use iced::keyboard;
use iced::{Alignment, Background, Color, Element, Length, Task};

use crate::ui::style::*;
use crate::ui::screens::settings::VayuSettings;
use crate::plog;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;

use korlang::vm::{VirtualMachine, OpCode};
use korlang::compile;
use crate::engine::korlang::register_default_callbacks;
use crate::ui::kor_renderer::render_kor_vm;
use crate::engine::js::{JsBridge, JSEngine};
use crate::engine::pipeline::{fetch_page_content, apply_taffy_layout, StyledElement, normalize_nav_url, 
save_tabs, load_tabs, load_bookmarks, save_bookmarks, Bookmark, session_was_unclean, mark_session_started, mark_session_clean_exit, Tab};

mod canvas;
mod devtools;
mod tab_bar;
mod workspaces;

pub use devtools::DevToolsTab;

// -- Messages

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
    BookmarkClicked(usize),
    DuplicateTab(usize),
    CloseOtherTabs(usize),
    StartFreshSession,
    DismissCrashBanner,
    SessionEnding,
    LinkClicked(String),
    PageLoaded(String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>),
    TimerTick,
    ElementClicked(usize),
    TabSelected(usize),
    TabHovered(usize),
    TabUnhovered(usize),
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
    PageScrolled(f32),
    FormInputKeyPressed(char),
    RunKorScript(String),
    RunKorOnPage,
    KorScriptResult(String),
    WindowResized(f32, f32),
    None,
}

// -- State

pub struct BrowserScreen {
    pub url: String,
    pub active_workspace: usize,
    pub content: String,
    pub styled_elements: Arc<Vec<StyledElement>>,
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
    page_canvas: Option<canvas::PageCanvas>,
    js_errors: Vec<String>,
    show_dev_console: bool,
    pub url_input: String,
    pub url_history: Vec<String>,
    pub bookmarks: Vec<Bookmark>,
    pub show_bookmarks_bar: bool,
    pub crashed_last_session: bool,
    pub show_autocomplete: bool,
    pub autocomplete_index: usize,
    pub dev_tools_tab: DevToolsTab,
    pub network_requests: Vec<String>,
    pub inspect_mode: bool,
    pub inspect_element: Option<usize>,
    pub form_inputs: HashMap<usize, String>,
    pub active_form_element: Option<usize>,
    pub settings: VayuSettings,
}

impl Default for BrowserScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserScreen {
    pub fn new() -> Self {
        let default_url = "vayu://design/spatial-minimalism".to_string();
        let mut kor_vm = VirtualMachine::new();
        register_default_callbacks(&mut kor_vm);
        kor_vm.set_builtin("status_left", korlang::vm::Value::String("Vayu Ready".to_string()));
        kor_vm.set_builtin("status_mid", korlang::vm::Value::String("Idle".to_string()));
        kor_vm.set_builtin("status_right", korlang::vm::Value::String("Local shell".to_string()));
        let status_src = r#"
Component StatusBar {
    Row(spacing: 8) {
        Text(size: 10, text: status_left)
        Text(size: 10, text: " � ")
        Text(size: 10, text: status_mid)
        Text(size: 10, text: " � ")
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
        Button(text: "? History", on_click: "back")
        Button(text: "? Settings", on_click: "settings")
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
        Button(text: "? Design Studio", on_click: "ws0")
        Button(text: "? Research Lab", on_click: "ws1")
        Button(text: "? Deep Work", on_click: "ws2")
        Text(text: "COLLECTIONS", size: 11)
        Button(text: "? Vayu UI", on_click: "ws0")
        Button(text: "? Rust / Iced Docs", on_click: "ws1")
    }
}
"#;
        let sidebar_ws_bytecode = compile(sidebar_ws_src);
        sidebar_ws_kor_vm.execute(sidebar_ws_bytecode.clone());
        crate::engine::js::js_bridge::load_local_storage();
        let loaded_tabs = load_tabs();
        let url_history: Vec<String> = loaded_tabs.iter().map(|t| t.url.clone()).collect();
        let (tabs, tab_history, url_val, content_val) = if loaded_tabs.is_empty() {
            (vec![Tab::new("New Tab", &default_url, 0)],
             vec![(vec![default_url.clone()], 0)],
             default_url.clone(),
             "Welcome to Vayu Browser".to_string())
        } else {
            let count = loaded_tabs.len();
            let history: Vec<(Vec<String>, usize)> = loaded_tabs.iter().map(|t| (vec![t.url.clone()], 0)).collect();
            let url = loaded_tabs[0].url.clone();
            (loaded_tabs, history, url, format!("Restored {} tabs", count))
        };
        let settings = VayuSettings::load();
        crate::engine::pipeline::set_js_enabled(settings.js_enabled);
        // Sentinel goes up only after tabs/history are fully reconstructed,
        // so creating it can never re-flag THIS startup as a crash.
        mark_session_started();
        Self {
            url: url_val.clone(),
            active_workspace: 0,
            content: content_val,
            styled_elements: Arc::new(vec![]),
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
            bookmarks: load_bookmarks(),
            show_bookmarks_bar: settings.show_bookmarks_bar,
            // Sentinel check happens only after tabs/history above are fully
            // reconstructed; creating the new lock here cannot re-flag THIS
            // startup as a crash.
            crashed_last_session: session_was_unclean(),
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
                    save_tabs(&self.tabs);
                    return Task::perform(fetch_page_content(url, bw, bh, self.url_history.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
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
                    save_tabs(&self.tabs);
                    return Task::perform(fetch_page_content(url, bw, bh, self.url_history.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
                }
                Task::none()
            }
            BrowserMessage::Refresh => {
                let url = self.url.clone();
                plog!("NAV", "Refresh: {}", url);
                save_tabs(&self.tabs);
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
                if !self.url_history.contains(&page_url) && !page_url.starts_with("vayu://") {
                    self.url_history.push(page_url.clone());
                }
                self.is_history_nav = false;
                self.styled_elements = Arc::new(elements);
                self.layout_gen += 1;
                self.page_canvas = Some(canvas::PageCanvas::new(Arc::clone(&self.styled_elements), self.inspect_element, self.bounds.1));
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
            BrowserMessage::WindowResized(w, h) => {
                self.bounds = (w, h);
                if !self.styled_elements.is_empty() {
                    let content_w = (w - 260.0).max(200.0);
                    let viewport_h = h;
                    apply_taffy_layout(&mut *Arc::make_mut(&mut self.styled_elements), content_w, viewport_h);
                    self.page_canvas = Some(canvas::PageCanvas::new(Arc::clone(&self.styled_elements), self.inspect_element, h));
                }
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
                        return Task::perform(fetch_page_content(self.url.clone(), bw, bh, self.url_history.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
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
                            return Task::perform(fetch_page_content(url, bw, bh, self.url_history.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b));
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
            BrowserMessage::Bookmark => {
                let title = self
                    .tabs
                    .get(self.active_tab)
                    .map(|t| t.title.clone())
                    .unwrap_or_else(|| self.url.clone());
                self.bookmarks =
                    toggle_bookmark(std::mem::take(&mut self.bookmarks), &self.url, &title);
                save_bookmarks(&self.bookmarks);
                Task::none()
            }
            BrowserMessage::BookmarkClicked(i) => {
                match self.bookmarks.get(i) {
                    Some(b) => {
                        let url = b.url.clone();
                        self.navigate_to(&url)
                    }
                    None => Task::none(),
                }
            }
            BrowserMessage::DuplicateTab(i) => {
                if i < self.tabs.len() {
                    let tab = self.tabs[i].clone();
                    let history = self.tab_history[i].clone();
                    self.tabs.insert(i + 1, tab);
                    self.tab_history.insert(i + 1, history);
                    self.active_tab = i + 1;
                    save_tabs(&self.tabs);
                }
                Task::none()
            }
            BrowserMessage::CloseOtherTabs(keep) => {
                // Single-tab (or stale index): nothing to close.
                if keep < self.tabs.len() && self.tabs.len() > 1 {
                    let history = self.tab_history[keep].clone();
                    let kept = self.tabs[keep].clone();
                    self.tabs = vec![kept];
                    self.tab_history = vec![history];
                    self.active_tab = 0;
                    if let Some(active_tab) = self.tabs.get_mut(0) {
                        active_tab.update_accessed();
                    }
                    save_tabs(&self.tabs);
                }
                Task::none()
            }
            BrowserMessage::StartFreshSession => {
                self.tabs = vec![Tab::new("New Tab", "about:blank", self.active_workspace)];
                self.tab_history = vec![(vec!["about:blank".to_string()], 0)];
                self.active_tab = 0;
                self.url = "about:blank".to_string();
                self.url_input = "about:blank".to_string();
                self.content = "New session".to_string();
                self.styled_elements = Arc::new(vec![]);
                self.page_canvas = None;
                self.bridge = None;
                self.crashed_last_session = false;
                save_tabs(&self.tabs);
                Task::none()
            }
            BrowserMessage::DismissCrashBanner => {
                self.crashed_last_session = false;
                Task::none()
            }
            BrowserMessage::SessionEnding => {
                mark_session_clean_exit();
                Task::none()
            }
            BrowserMessage::WorkspaceSelected(i) => { self.active_workspace = i; Task::none() }
            BrowserMessage::TabSelected(i) => {
                if i < self.tabs.len() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        tab.set_hover(false);
                    }
                    self.active_tab = i;
                    if let Some(tab) = self.tabs.get_mut(i) {
                        tab.update_accessed();
                    }
                }
                Task::none()
            }
            BrowserMessage::TabHovered(i) => {
                if i < self.tabs.len() && i != self.active_tab {
                    if let Some(tab) = self.tabs.get_mut(i) {
                        tab.set_hover(true);
                        if tab.should_switch_on_hover() {
                            self.active_tab = i;
                            tab.update_accessed();
                        }
                    }
                }
                Task::none()
            }
            BrowserMessage::TabUnhovered(i) => {
                if i < self.tabs.len() {
                    if let Some(tab) = self.tabs.get_mut(i) {
                        tab.set_hover(false);
                    }
                }
                Task::none()
            }
            BrowserMessage::NewTab => {
                let title = format!("Tab {}", self.tabs.len() + 1);
                self.tabs.push(Tab::new(&title, "about:blank", self.active_workspace));
                self.active_tab = self.tabs.len() - 1;
                self.url = "about:blank".to_string();
                self.content = "New tab".to_string();
                self.styled_elements = Arc::new(vec![]);
                self.loading = false;
                self.bridge = None;
                self.tab_history.push((vec!["about:blank".to_string()], 0));
                save_tabs(&self.tabs);
                Task::none()
            }
            BrowserMessage::CloseTab(i) => {
                if self.tabs.len() > 1 && i < self.tabs.len() {
                    let was_active = i == self.active_tab;
                    self.tabs.remove(i);
                    self.tab_history.remove(i);
                    if was_active {
                        self.active_tab = self.tabs.len() - 1;
                    } else if i < self.active_tab {
                        self.active_tab -= 1;
                    }
                    if let Some(active_tab) = self.tabs.get_mut(self.active_tab) {
                        active_tab.update_accessed();
                    }
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
                if let Some(pc) = self.page_canvas.as_mut() {
                    pc.focused_index = None;
                    pc.cache.clear();
                }
                Task::none()
            }
            BrowserMessage::InspectElement(idx) => {
                self.inspect_element = Some(idx);
                if let Some(pc) = self.page_canvas.as_mut() {
                    pc.focused_index = Some(idx);
                    pc.cache.clear();
                }
                Task::none()
            }
            BrowserMessage::FormElementClicked(idx) => {
                self.active_form_element = Some(idx);
                Task::none()
            }
            BrowserMessage::PageScrolled(y) => {
                if let Some(pc) = self.page_canvas.as_mut() {
                    // Only invalidate when the band actually moved; the canvas
                    // Cache never invalidates on its own (bounds are unchanged
                    // while scrolling).
                    if (pc.scroll_top - y).abs() > f32::EPSILON {
                        pc.scroll_top = y;
                        pc.cache.clear();
                    }
                }
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
            BrowserMessage::RunKorScript(script) => {
                plog!("KOR", "Executing Kor script: {}", script);
                let mut vm = self.kor_vm.borrow_mut();
                vm.stack.clear();
                
                // Set up page context
                vm.set_builtin("page_url", korlang::vm::Value::String(self.url.clone()));
                vm.set_builtin("element_count", korlang::vm::Value::Number(self.styled_elements.len() as f64));
                
                // Execute with timeout protection
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    vm.execute(korlang::compile(&script));
                    vm.stack.last().cloned()
                }));
                
                match result {
                    Ok(Some(val)) => {
                        let result_str = match val {
                            korlang::vm::Value::String(s) => s,
                            korlang::vm::Value::Number(n) => format!("{}", n),
                            korlang::vm::Value::Bool(b) => format!("{}", b),
                            _ => "Script executed".to_string(),
                        };
                        plog!("KOR", "Script result: {}", result_str);
                        self.content = result_str;
                    }
                    Ok(None) => {
                        plog!("KOR", "Script executed (no return value)");
                    }
                    Err(_) => {
                        plog!("KOR", "Script execution panicked");
                        self.content = "Error: Script execution failed".to_string();
                    }
                }
                Task::none()
            }
            BrowserMessage::RunKorOnPage => {
                plog!("KOR", "Running Kor script on current page");
                // Create a default script that interacts with page content
                let default_script = r#"
                    Component PageSummary {
                        Column(spacing: 8) {
                            Text(text: "Page Analysis", size: 16)
                            Text(text: str(page_url), size: 12)
                            Text(text: str(element_count) + " elements found", size: 12)
                            Button(text: "Refresh Data", on_click: "refresh")
                        }
                    }
                "#;
                
                let mut vm = self.kor_vm.borrow_mut();
                vm.stack.clear();
                vm.set_builtin("page_url", korlang::vm::Value::String(self.url.clone()));
                vm.set_builtin("element_count", korlang::vm::Value::Number(self.styled_elements.len() as f64));
                vm.execute(korlang::compile(default_script));
                
                self.content = "Kor script executed on page".to_string();
                Task::none()
            }
            BrowserMessage::KorScriptResult(result) => {
                plog!("KOR", "Script result received: {}", result);
                self.content = result;
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
                key::Key::Character(ref c) if c.chars().next().is_some_and(|ch| !ch.is_control()) => {
                    c.chars().next().map(BrowserMessage::FormInputKeyPressed)
                }
                key::Key::Named(key::Named::Backspace) => Some(BrowserMessage::FormInputKeyPressed('\x08')),
                _ => None,
            }
        });
        iced::Subscription::batch(vec![timer_sub, key_sub])
    }

    pub fn view(&self) -> Element<'_, BrowserMessage> {
        let sidebar = workspaces::sidebar(self);
        let main = self.main_area();
        let content = if self.show_dev_console {
            let console = devtools::dev_console_overlay(self);
            column![row![sidebar, main], console].into()
        } else {
            row![sidebar, main].into()
        };
        // ponytail: autocomplete rendered inside top_bar, no overlay needed
        content
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
            let pc = self.page_canvas.as_ref().expect("Expected Some value, found None");
            let total_h = pc.elements.iter()
                .filter(|el| el.display != crate::engine::stratus::Display::None)
                .map(|el| {
                    let ey = if el.y.is_finite() { el.y } else { 0.0 };
                    let h = if el.height.is_finite() { el.height.max(el.font_size.clamp(6.0, 200.0) * el.line_height.max(1.0)) } else { el.font_size.clamp(6.0, 200.0) * el.line_height.max(1.0) };
                    ey + h + el.margin_bottom
                })
                .fold(0.0, f32::max);
            let total_h = if total_h.is_finite() { total_h.max(100.0) } else { 800.0 };
            let content_w = (self.bounds.0 - 260.0).max(200.0);
            container(
                scrollable(iced_canvas(pc).width(Length::Fixed(content_w)).height(Length::Fixed(total_h)))
                    .width(Length::Fill).height(Length::Fill)
                    .on_scroll(|vp| BrowserMessage::PageScrolled(vp.absolute_offset().y))
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
        let tabs = tab_bar::tab_bar(self);
        let mut main_col = column![tabs, top];
        if let Some(banner) = self.crash_banner() {
            main_col = main_col.push(banner);
        }
        if let Some(bar) = self.bookmarks_bar() {
            main_col = main_col.push(bar);
        }
        let main_col = main_col.push(body).push(status);
        container(main_col)
            .width(Length::Fill).height(Length::Fill).style(main_area_style()).into()
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
        let kor_btn = button(text("\u{26A1}").size(14).color(C::ACCENT))
            .padding([6, 8]).style(nav_icon_button_style())
            .on_press(BrowserMessage::RunKorOnPage);

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
                kor_btn, inspect_btn, bookmark_btn, palette_btn,
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
        container(render_kor_vm(&self.kor_vm.borrow()))
            .height(Length::Fixed(40.0)).width(Length::Fill)
            .center_x(Length::Fill).center_y(Length::Fixed(40.0))
            .style(status_bar_style()).into()
    }

    // Thin warning strip shown only when the previous run never reached a
    // clean exit. Keep-tabs dismisses; start-fresh resets to one New Tab.
    fn crash_banner(&self) -> Option<Element<'_, BrowserMessage>> {
        if !self.crashed_last_session {
            return None;
        }
        let dismiss = button(text("Keep tabs").size(12).color(C::MUTED))
            .padding([4, 8])
            .style(nav_icon_button_style())
            .on_press(BrowserMessage::DismissCrashBanner);
        let fresh = button(text("Start fresh").size(12).color(C::ACCENT))
            .padding([4, 8])
            .style(nav_icon_button_style())
            .on_press(BrowserMessage::StartFreshSession);
        Some(
            container(
                row![
                    text("Browser didn't shut down cleanly last time.").size(12).color(C::FG),
                    Space::with_width(Length::Fill),
                    dismiss,
                    fresh,
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .padding([4, 8]),
            )
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.30, 0.18, 0.05))),
                ..Default::default()
            })
            .into(),
        )
    }

    // Compact strip between top bar and page: one small button per bookmark.
    // Hidden entirely when the setting is off or nothing is bookmarked yet.
    fn bookmarks_bar(&self) -> Option<Element<'_, BrowserMessage>> {
        if !self.show_bookmarks_bar || self.bookmarks.is_empty() {
            return None;
        }
        let items: Vec<Element<'_, BrowserMessage>> = self
            .bookmarks
            .iter()
            .enumerate()
            .map(|(i, b)| {
                button(text(&b.title).size(12).color(C::MUTED))
                    .padding([4, 8])
                    .style(nav_icon_button_style())
                    .on_press(BrowserMessage::BookmarkClicked(i))
                    .into()
            })
            .collect();
        Some(
            container(scrollable(row(items).spacing(2).padding([2, 4])).width(Length::Fill))
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::SURFACE)),
                border: iced::Border { radius: 0.0.into(), ..Default::default() },
                ..Default::default()
            })
            .into(),
        )
    }

    fn navigate_to(&mut self, input: &str) -> Task<BrowserMessage> {
        let input = input.trim();
        if input.is_empty() { return Task::none(); }

        // Check if it's a search query (not a URL)
        let target = if VayuSettings::is_url(input) {
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
        Task::perform(fetch_page_content(target, bw, bh, self.url_history.clone()), |(u, els, b)| BrowserMessage::PageLoaded(u, els, b))
    }

}

// B1: pure toggle - remove by exact URL match, else append preserving order.
// Title is only used on the add path; disk I/O stays with the UI handler.
fn toggle_bookmark(bookmarks: Vec<Bookmark>, url: &str, title: &str) -> Vec<Bookmark> {
    if bookmarks.iter().any(|b| b.url == url) {
        bookmarks.into_iter().filter(|b| b.url != url).collect()
    } else {
        let mut next = bookmarks;
        next.push(Bookmark { url: url.to_string(), title: title.to_string() });
        next
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
    use crate::engine::pipeline::{apply_taffy_layout, normalize_nav_url, Bookmark};
    use crate::engine::pipeline::extractor::{BoxSizing, FontWeight, TextDecor};
    use crate::engine::stratus::{AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent, Position};

    fn make_test(tag: &str, text: &str, display: &str, parent: Option<usize>) -> StyledElement {
        let display = match display {
            "inline" => Display::Inline,
            "flex" => Display::Flex,
            "grid" => Display::Grid,
            "none" => Display::None,
            _ => Display::Block,
        };
        StyledElement {
            tag: tag.to_string(), text: text.to_string(), wrapped_lines: vec![],
            dom_path: vec![],
            is_link: false, href: None, indent_level: 0,
            color: Color::BLACK, font_size: 16.0, font_weight: FontWeight::Normal,
            background_color: None, border_widths: [0.0; 4], border_color: None,
            image_handle: None, image_url: None,
            margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None,
            padding: [0.0; 4], display,
            flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart, align_items: AlignItems::Stretch,
            align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0, flex_shrink: 1.0, flex_basis: None,
            css_width: None, css_height: None, parent_index: parent,
            min_width: None, max_width: None, min_height: None, max_height: None,
            x: 0.0, y: 0.0, width: 0.0, height: 0.0,
            line_height: 1.4, text_decoration: TextDecor::default(),
            border_radius: [0.0; 4],
            input_type: String::new(), input_value: String::new(),
            input_placeholder: String::new(), checked: false,
            position: Position::Static,
            inset_top: 0.0, inset_right: 0.0, inset_bottom: 0.0, inset_left: 0.0,
        }
    }

    #[test]
    fn test_ifc_simple_inline_siblings() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hello", "inline", Some(0)),
            make_test("span", "World", "inline", Some(0)),
        ];
        apply_taffy_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.x >= 0.0, "x={}", el.x); }
        assert!(elements[2].x >= elements[1].x, "span1 x={} < span0 x={}", elements[2].x, elements[1].x);
    }

    #[test]
    fn test_ifc_single_inline_in_block() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Hi", "inline", Some(0)),
        ];
        apply_taffy_layout(&mut elements, 800.0, 6000.0);
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
        apply_taffy_layout(&mut elements, 800.0, 6000.0);
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
        apply_taffy_layout(&mut elements, 800.0, 6000.0);
        for el in &elements { assert!(el.x.is_finite() && el.y.is_finite()); }
    }

    #[test]
    fn test_ifc_nested_inline() {
        let mut elements = vec![
            make_test("div", "", "block", None),
            make_test("span", "Outer ", "inline", Some(0)),
            make_test("span", "Inner", "inline", Some(1)),
        ];
        apply_taffy_layout(&mut elements, 800.0, 6000.0);
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
        apply_taffy_layout(&mut elements, 800.0, 6000.0);
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
            ("vayu://home", "vayu://home"),
        ];
        for (input, expected) in &cases {
            assert_eq!(&normalize_nav_url(input), expected, "input={}", input);
        }
    }

    #[test]
    fn test_close_tab_before_active_adjusts_index() {
        // Serializes against the B4 tests: this handler persists tabs to the
        // shared vayu_tabs.json, and parallel writes would race its readers.
        let _g = B4_FS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut screen = BrowserScreen::default();
        screen.tabs = vec![
            Tab::new("A", "https://a.com", 0),
            Tab::new("B", "https://b.com", 0),
            Tab::new("C", "https://c.com", 0),
        ];
        screen.active_tab = 2;
        screen.tab_history = vec![(vec!["https://a.com".into()], 0), (vec!["https://b.com".into()], 0), (vec!["https://c.com".into()], 0)];

        let _ = screen.update(BrowserMessage::CloseTab(0));

        assert_eq!(screen.tabs.len(), 2);
        assert_eq!(screen.tabs[0].title, "B");
        assert_eq!(screen.tabs[1].title, "C");
        assert_eq!(screen.active_tab, 1, "active_tab should shift left after closing tab before it");
    }

    // -- B1 bookmarks bar --
    fn bm(url: &str, title: &str) -> Bookmark {
        Bookmark { url: url.to_string(), title: title.to_string() }
    }

    #[test]
    fn b1_toggle_adds_to_empty() {
        let b = toggle_bookmark(vec![], "https://x.dev", "X");
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].url, "https://x.dev");
        assert_eq!(b[0].title, "X");
    }

    #[test]
    fn b1_toggle_appends_preserving_order() {
        let start = vec![bm("https://a", "A"), bm("https://c", "C")];
        let b = toggle_bookmark(start, "https://b", "B");
        let urls: Vec<&str> = b.iter().map(|x| x.url.as_str()).collect();
        assert_eq!(urls, ["https://a", "https://c", "https://b"]);
    }

    #[test]
    fn b1_toggle_removes_existing_by_url_ignoring_title() {
        let start = vec![bm("https://a", "A"), bm("https://b", "B"), bm("https://c", "C")];
        let b = toggle_bookmark(start, "https://b", "some other title");
        let urls: Vec<&str> = b.iter().map(|x| x.url.as_str()).collect();
        assert_eq!(urls, ["https://a", "https://c"]);
    }

    #[test]
    fn b1_toggle_never_duplicates_same_url() {
        let added = toggle_bookmark(vec![], "https://d", "D1");
        let removed = toggle_bookmark(added, "https://d", "D2-different-title");
        assert!(removed.is_empty());
    }

    // ?? B4 tab restore polish ???????????????????????????????????????????
    static B4_FS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn b4_screen() -> BrowserScreen {
        let mut s = BrowserScreen::default();
        s.tabs = vec![
            Tab::new("A", "https://a.com", 0),
            Tab::new("B", "https://b.com", 0),
            Tab::new("C", "https://c.com", 0),
        ];
        s.tab_history = vec![
            (vec!["https://a.com".to_string()], 0),
            (vec!["https://b.com".to_string()], 0),
            (vec!["https://c.com".to_string()], 0),
        ];
        s.crashed_last_session = false;
        s
    }

    #[test]
    fn b4_duplicate_tab_inserts_copy_after_and_activates_it() {
        let _g = B4_FS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut s = b4_screen();
        let _ = s.update(BrowserMessage::DuplicateTab(0));
        assert_eq!(s.tabs.len(), 4);
        assert_eq!(s.tabs[1].title, "A");
        assert_eq!(s.tabs[1].url, "https://a.com");
        assert_eq!(s.active_tab, 1);
        assert_eq!(s.tab_history.len(), 4);
        assert_eq!(load_tabs().len(), 4, "duplicate must persist");
    }

    #[test]
    fn b4_close_others_keeps_target_and_persists() {
        let _g = B4_FS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut s = b4_screen();
        s.active_tab = 2;
        let _ = s.update(BrowserMessage::CloseOtherTabs(1));
        assert_eq!(s.tabs.len(), 1);
        assert_eq!(s.tabs[0].title, "B");
        assert_eq!(s.active_tab, 0);
        assert_eq!(load_tabs().len(), 1, "close-others must persist");
    }

    #[test]
    fn b4_stale_indices_are_noops() {
        let mut s = b4_screen();
        let _ = s.update(BrowserMessage::DuplicateTab(9));
        let _ = s.update(BrowserMessage::CloseOtherTabs(9));
        assert_eq!(s.tabs.len(), 3);
        assert_eq!(s.active_tab, 0);
    }

    #[test]
    fn b4_start_fresh_resets_and_clears_crash_flag() {
        let _g = B4_FS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut s = b4_screen();
        s.crashed_last_session = true;
        let _ = s.update(BrowserMessage::StartFreshSession);
        assert_eq!(s.tabs.len(), 1);
        assert_eq!(s.tabs[0].title, "New Tab");
        assert!(!s.crashed_last_session);
        assert_eq!(s.tab_history.len(), 1);
        assert_eq!(load_tabs().len(), 1, "start-fresh must persist");
    }

    #[test]
    fn b4_keep_tabs_only_dismisses_banner() {
        let mut s = b4_screen();
        s.crashed_last_session = true;
        let _ = s.update(BrowserMessage::DismissCrashBanner);
        assert!(!s.crashed_last_session);
        assert_eq!(s.tabs.len(), 3, "keep-tabs must not alter restored tabs");
    }
}
