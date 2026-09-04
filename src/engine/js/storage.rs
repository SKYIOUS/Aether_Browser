use super::js_bridge::{CookieEntry, JsBridge};
use std::time::Instant;

impl JsBridge {
    // ── Cookie methods ─────────────────────────────────────────────

    pub fn get_cookie(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let origin = self.current_origin();
        let is_https = origin.starts_with("https://");
        if let Ok(mut guard) = super::js_bridge::cookie_store().write() {
            super::js_bridge::sweep_expired_cookies(&mut guard);
            if let Some(cookies) = guard.get(&origin) {
                for (key, entry) in cookies.iter() {
                    if entry.http_only {
                        continue;
                    }
                    if entry.secure && !is_https {
                        continue;
                    }
                    parts.push(format!("{}={}", key, entry.value));
                }
            }
        }
        parts.join("; ")
    }

    pub fn set_cookie(&mut self, cookie_str: &str) {
        if let Some(eq_pos) = cookie_str.find('=') {
            let key = cookie_str[..eq_pos].trim().to_string();
            let value_end = cookie_str[eq_pos + 1..]
                .find(';')
                .map(|p| eq_pos + 1 + p)
                .unwrap_or(cookie_str.len());
            let value = cookie_str[eq_pos + 1..value_end].trim().to_string();
            if !key.is_empty() {
                let expires = super::js_bridge::parse_cookie_expiry(cookie_str);
                if expires.as_ref().is_some_and(|e| *e <= Instant::now()) {
                    if let Ok(mut guard) = super::js_bridge::cookie_store().write() {
                        guard.entry(self.current_origin()).or_default().remove(&key);
                    }
                    return;
                }
                let mut http_only = false;
                let mut secure = false;
                let mut same_site = String::new();
                for part in cookie_str[value_end..].split(';') {
                    let part = part.trim().to_lowercase();
                    if part == "httponly" {
                        http_only = true;
                    } else if part == "secure" {
                        secure = true;
                    } else if let Some(ss) = part.strip_prefix("samesite=") {
                        same_site = ss.to_string();
                    }
                }
                if let Ok(mut guard) = super::js_bridge::cookie_store().write() {
                    super::js_bridge::sweep_expired_cookies(&mut guard);
                    let origin = self.current_origin();
                    let total: usize = guard.values().map(|m| m.len()).sum();
                    let origin_has_room = guard.get(&origin).map(|m| m.len() < 50).unwrap_or(true);
                    if !origin_has_room || total >= 500 {
                        return;
                    }
                    guard.entry(origin).or_default().insert(
                        key,
                        CookieEntry {
                            value,
                            expires,
                            http_only,
                            secure,
                            same_site,
                        },
                    );
                }
            }
        }
    }

    // ── LocalStorage methods ────────────────────────────────────────

    pub fn local_storage_get_item(&self, key: &str) -> Option<String> {
        let origin = self.current_origin();
        super::js_bridge::local_storage_store()
            .read()
            .ok()
            .and_then(|guard| guard.get(&origin).and_then(|od| od.data.get(key).cloned()))
    }

    pub fn local_storage_set_item(&mut self, key: String, value: String) {
        let new_entry_size = key.len() + value.len();
        if let Ok(mut guard) = super::js_bridge::local_storage_store().write() {
            let origin_data = guard.entry(self.current_origin()).or_default();
            let old_size = origin_data
                .data
                .get(&key)
                .map_or(0, |v| key.len() + v.len());
            let new_used = origin_data.used_bytes - old_size + new_entry_size;
            if new_used > super::js_bridge::LOCAL_STORAGE_QUOTA {
                return;
            }
            origin_data.data.insert(key, value);
            origin_data.used_bytes = new_used;
        }
        super::js_bridge::save_local_storage();
    }

    pub fn local_storage_remove_item(&mut self, key: &str) {
        if let Ok(mut guard) = super::js_bridge::local_storage_store().write() {
            if let Some(origin_data) = guard.get_mut(&self.current_origin()) {
                if let Some(removed) = origin_data.data.remove(key) {
                    origin_data.used_bytes -= key.len() + removed.len();
                }
            }
        }
        super::js_bridge::save_local_storage();
    }

    pub fn local_storage_clear(&mut self) {
        if let Ok(mut guard) = super::js_bridge::local_storage_store().write() {
            guard.remove(&self.current_origin());
        }
        super::js_bridge::save_local_storage();
    }

    pub fn local_storage_key(&self, index: i32) -> Option<String> {
        if index < 0 {
            return None;
        }
        super::js_bridge::local_storage_store()
            .read()
            .ok()
            .and_then(|guard| {
                guard
                    .get(&self.current_origin())
                    .and_then(|od| od.data.keys().nth(index as usize).cloned())
            })
    }

    pub fn local_storage_length(&self) -> i32 {
        super::js_bridge::local_storage_store()
            .read()
            .ok()
            .and_then(|guard| {
                guard
                    .get(&self.current_origin())
                    .map(|od| od.data.len() as i32)
            })
            .unwrap_or(0)
    }

    pub fn local_storage_used_bytes(&self) -> usize {
        super::js_bridge::local_storage_store()
            .read()
            .ok()
            .and_then(|guard| guard.get(&self.current_origin()).map(|od| od.used_bytes))
            .unwrap_or(0)
    }
}
