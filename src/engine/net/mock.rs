use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static MOCK: OnceLock<Mutex<Option<MockHttpResponder>>> = OnceLock::new();

fn mock_lock() -> &'static Mutex<Option<MockHttpResponder>> {
    MOCK.get_or_init(|| Mutex::new(None))
}

#[derive(Default)]
pub struct MockHttpResponder {
    pub html_responses: HashMap<String, String>,
    pub css_responses: HashMap<String, String>,
    pub binary_responses: HashMap<String, Vec<u8>>,
}

impl MockHttpResponder {
    pub fn new() -> Self { Self::default() }
    pub fn html(mut self, url: &str, body: &str) -> Self {
        self.html_responses.insert(url.to_string(), body.to_string()); self
    }
    pub fn css(mut self, url: &str, body: &str) -> Self {
        self.css_responses.insert(url.to_string(), body.to_string()); self
    }
    pub fn binary(mut self, url: &str, body: Vec<u8>) -> Self {
        self.binary_responses.insert(url.to_string(), body); self
    }
}

pub fn set_mock(m: MockHttpResponder) {
    if let Ok(mut guard) = mock_lock().lock() { *guard = Some(m); }
}

pub fn clear_mock() {
    if let Ok(mut guard) = mock_lock().lock() { *guard = None; }
}

pub fn resolve_html(url: &str) -> Option<String> {
    if let Ok(guard) = mock_lock().lock() {
        if let Some(ref m) = *guard { return m.html_responses.get(url).cloned(); }
    }
    None
}

pub fn resolve_css(url: &str) -> Option<String> {
    if let Ok(guard) = mock_lock().lock() {
        if let Some(ref m) = *guard { return m.css_responses.get(url).cloned(); }
    }
    None
}

pub fn resolve_binary(url: &str) -> Option<Vec<u8>> {
    if let Ok(guard) = mock_lock().lock() {
        if let Some(ref m) = *guard { return m.binary_responses.get(url).cloned(); }
    }
    None
}
