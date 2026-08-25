use crate::plog;
use serde::{Serialize, Deserialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct Tab {
    pub title: String,
    pub url: String,
    pub workspace_id: usize,
    #[serde(skip)]
    pub created_at: Instant,
    #[serde(skip)]
    pub last_accessed: Instant,
    #[serde(skip)]
    pub is_hovered: bool,
    #[serde(skip)]
    pub hover_started: Option<Instant>,
}

// Custom deserialization to handle skipped Instant fields and new workspace_id
impl<'de> Deserialize<'de> for Tab {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            title: String,
            url: String,
            #[serde(default)]
            workspace_id: usize,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        let now = Instant::now();
        
        Ok(Tab {
            title: helper.title,
            url: helper.url,
            workspace_id: helper.workspace_id,
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
            workspace_id: 0,
            created_at: now,
            last_accessed: now,
            is_hovered: false,
            hover_started: None,
        }
    }
}

impl Tab {
    pub fn new(title: &str, url: &str, workspace_id: usize) -> Self {
        let now = Instant::now();
        Self {
            title: title.to_string(),
            url: url.to_string(),
            workspace_id,
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
        self.is_hovered && self.hover_started.is_some_and(|start| start.elapsed().as_millis() >= 300)
    }
}

pub fn normalize_nav_url(url: &str) -> String {
    let s = url.trim();
    if s.is_empty() { return "about:blank".to_string(); }
    if s.starts_with("vayu://") || s.starts_with("about:") {
        return s.to_string();
    }
    crate::engine::net::normalize_url(s)
}

pub fn save_tabs(tabs: &[Tab]) {
    match serde_json::to_string(tabs) {
        Ok(json) => {
            if let Err(e) = std::fs::write("vayu_tabs.json", json) {
                plog!("tabs", "Failed to save tabs: {}", e);
            }
        }
        Err(e) => {
            plog!("tabs", "Failed to serialize tabs: {}", e);
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

pub fn save_bookmarks(bookmarks: &[Bookmark]) {
    match serde_json::to_string_pretty(bookmarks) {
        Ok(json) => {
            if let Err(e) = std::fs::write("vayu_bookmarks.json", json) {
                plog!("bookmarks", "Failed to save: {}", e);
            }
        }
        Err(e) => {
            plog!("bookmarks", "Failed to serialize: {}", e);
        }
    }
}

pub fn load_bookmarks() -> Vec<Bookmark> {
    std::fs::read_to_string("vayu_bookmarks.json")
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<Bookmark>>(&s).ok())
        .unwrap_or_default()
}

// Crash-recovery sentinel: present while a session runs; a startup that finds
// it means the previous run never reached clean-exit. Deliberately tiny -
// fixed path, best-effort I/O, cleanup failure must not block shutdown.
const SESSION_LOCK_FILE: &str = "vayu_session.lock";

pub fn session_was_unclean() -> bool {
    std::path::Path::new(SESSION_LOCK_FILE).exists()
}

pub fn mark_session_started() {
    if let Err(e) = std::fs::write(SESSION_LOCK_FILE, "alive") {
        plog!("session", "Failed to write lock: {}", e);
    }
}

pub fn mark_session_clean_exit() {
    // Best effort: a leftover sentinel only causes one spurious banner.
    let _ = std::fs::remove_file(SESSION_LOCK_FILE);
}

pub fn load_tabs() -> Vec<Tab> {
    std::fs::read_to_string("vayu_tabs.json")
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<Tab>>(&s).ok())
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
