use crate::plog;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub title: String,
    pub url: String,
}

pub fn normalize_nav_url(url: &str) -> String {
    let s = url.trim();
    if s.is_empty() { return "about:blank".to_string(); }
    if s.starts_with("aether://") || s.starts_with("about:") {
        return s.to_string();
    }
    crate::engine::net::normalize_url(s)
}

pub fn save_tabs(tabs: &[Tab]) {
    match serde_json::to_string(tabs) {
        Ok(json) => {
            if let Err(e) = std::fs::write("aether_tabs.json", json) {
                plog!("tabs", "Failed to save tabs: {}", e);
            }
        }
        Err(e) => {
            plog!("tabs", "Failed to serialize tabs: {}", e);
        }
    }
}

pub fn load_tabs() -> Vec<Tab> {
    std::fs::read_to_string("aether_tabs.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}
