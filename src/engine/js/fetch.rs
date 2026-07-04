use super::js_bridge::JsBridge;
use crate::plog;

impl JsBridge {
    // ponytail: returns body prefixed with __STATUS_NNN__ for JS parsing
    fn fetch_url_inner(&self, url: &str, _use_xhr: bool) -> String {
        let resolved = crate::engine::net::resolve_url(url, &self.current_url);
        let origin = &self.current_url;
        plog!("net", "Fetching: {} (origin: {})", resolved, origin);
        if crate::engine::net::is_same_origin(&resolved, origin) {
            match crate::engine::net::fetch(&resolved) {
                Ok((body, status)) => format!("__STATUS_{}__{}", status, body),
                Err(e) => format!("__STATUS_0__Error: {}", e),
            }
        } else {
            match crate::engine::net::fetch_with_cors(&resolved, origin) {
                Ok((body, status)) => format!("__STATUS_{}__{}", status, body),
                Err(e) => format!("__STATUS_0__Error: {}", e),
            }
        }
    }

    pub fn fetch_url(&self, url: &str) -> String {
        self.fetch_url_inner(url, false)
    }

    // ponytail: sync XHR — blocks JS thread, keep timeout short (3s)
    pub fn fetch_url_xhr(&self, url: &str) -> String {
        self.fetch_url_inner(url, true)
    }
}