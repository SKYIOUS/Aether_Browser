use crate::plog;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
pub struct Tab {
    pub title: String,
    pub url: String,
    #[serde(skip)]
    pub created_at: Instant,
    #[serde(skip)]
    pub last_accessed: Instant,
    #[serde(skip)]
    pub is_hovered: bool,
    #[serde(skip)]
    pub hover_started: Option<Instant>,
}

// Custom deserialization to handle skipped Instant fields
impl<'de> Deserialize<'de> for Tab {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            title: String,
            url: String,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        let now = Instant::now();
        
        Ok(Tab {
            title: helper.title,
            url: helper.url,
            created_at: now,
            last_accessed: now,
            is_hovered: false,
            hover_started: None,
        })
    }
}

impl Default for Tab {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            title: "New Tab".to_string(),
            url: "about:blank".to_string(),
            created_at: now,
            last_accessed: now,
            is_hovered: false,
            hover_started: None,
        }
    }
}

impl Tab {
    pub fn new(title: &str, url: &str) -> Self {
        let now = Instant::now();
        Self {
            title: title.to_string(),
            url: url.to_string(),
            created_at: now,
            last_accessed: now,
            is_hovered: false,
            hover_started: None,
        }
    }
    
    pub fn update_accessed(&mut self) {
        self.last_accessed = Instant::now();
    }
    
    pub fn set_hover(&mut self, hovered: bool) {
        if hovered && !self.is_hovered {
            self.hover_started = Some(Instant::now());
        } else if !hovered {
            self.hover_started = None;
        }
        self.is_hovered = hovered;
    }
    
    pub fn should_switch_on_hover(&self) -> bool {
        self.is_hovered && self.hover_started.map_or(false, |start| start.elapsed().as_millis() >= 300)
    }
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
        .into_iter()
        .map(|mut tab| {
            let now = Instant::now();
            tab.created_at = now;
            tab.last_accessed = now;
            tab.is_hovered = false;
            tab.hover_started = None;
            tab
        })
        .collect()
}
