use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub title: String,
    pub url: String,
}

pub fn normalize_nav_url(url: &str) -> String {
    let s = url.trim();
    if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("aether://") || s.starts_with("about:") {
        s.to_string()
    } else if s.starts_with("//") {
        let stripped = s.trim_start_matches('/');
        format!("https://{}", stripped)
    } else {
        format!("https://{}", s)
    }
}

pub fn save_tabs(tabs: &[Tab]) {
    if let Ok(json) = serde_json::to_string(tabs) {
        let _ = std::fs::write("aether_tabs.json", json);
    }
}

pub fn load_tabs() -> Vec<Tab> {
    std::fs::read_to_string("aether_tabs.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}
