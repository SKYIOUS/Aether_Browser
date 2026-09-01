pub mod extractor;
pub mod fetcher;
pub mod layout;
pub mod layout_adapter;
pub mod navigator;

pub use extractor::StyledElement;
pub use fetcher::fetch_page_content;
pub use layout_adapter::apply_taffy_layout;
pub use navigator::{
    load_bookmarks, load_tabs, mark_session_clean_exit, mark_session_started, normalize_nav_url,
    save_bookmarks, save_tabs, session_was_unclean, Bookmark, Tab,
};

/// Fuzz-target surface (PLAN C4): real budget helpers + the element ceiling,
/// hidden from docs.
#[doc(hidden)]
pub mod fuzz_surface {
    pub use super::extractor::MAX_ELEMENTS;
    pub use super::fetcher::{apply_html_budget, css_sources_within_total_budget, trim_to_budget};
}

use std::sync::atomic::{AtomicBool, Ordering};

static JS_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_js_enabled(enabled: bool) {
    JS_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn is_js_enabled() -> bool {
    JS_ENABLED.load(Ordering::Relaxed)
}
