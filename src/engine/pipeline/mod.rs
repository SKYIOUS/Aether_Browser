pub mod extractor;
pub mod fetcher;
pub mod layout;
pub mod navigator;

pub use extractor::StyledElement;
pub use fetcher::fetch_page_content;
pub use layout::apply_taffy_layout;
pub use navigator::{normalize_nav_url, save_tabs, load_tabs, Tab, load_bookmarks, save_bookmarks, Bookmark, session_was_unclean, mark_session_started, mark_session_clean_exit};

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
