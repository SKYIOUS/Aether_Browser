pub mod extractor;
pub mod fetcher;
pub mod layout;
pub mod navigator;

pub use extractor::StyledElement;
pub use fetcher::fetch_page_content;
pub use layout::apply_caelum_layout;
pub use navigator::{normalize_nav_url, save_tabs, load_tabs, Tab};

use std::sync::atomic::{AtomicBool, Ordering};

static JS_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_js_enabled(enabled: bool) {
    JS_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn is_js_enabled() -> bool {
    JS_ENABLED.load(Ordering::Relaxed)
}
