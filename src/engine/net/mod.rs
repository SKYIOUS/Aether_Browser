//! Networking module for fetching web resources.
pub mod mock;

use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;
use std::fmt;

use crate::plog;

// ── Error type ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum FetchError {
    Http(u16, String),
    Network(String),
    Timeout,
    EmptyBody,
    CrossOrigin { target: String, origin: String },
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(code, msg) => write!(f, "HTTP {}: {}", code, msg),
            Self::Network(msg) => write!(f, "Network: {}", msg),
            Self::Timeout => write!(f, "Request timed out"),
            Self::EmptyBody => write!(f, "Empty response body"),
            Self::CrossOrigin { target, origin } => {
                write!(f, "Cross-origin fetch blocked: '{}' ≠ origin '{}'", target, origin)
            }
        }
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            FetchError::Timeout
        } else if let Some(status) = e.status() {
            FetchError::Http(status.as_u16(), e.to_string())
        } else {
            FetchError::Network(e.to_string())
        }
    }
}

static CLIENT: OnceLock<Result<reqwest::blocking::Client, String>> = OnceLock::new();
fn client() -> Option<&'static reqwest::blocking::Client> {
    let entry = CLIENT.get_or_init(|| {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|e| format!("reqwest client: {}", e))
    });
    match entry {
        Ok(c) => Some(c),
        Err(e) => {
            plog!("net", "Client init failed: {}", e);
            None
        }
    }
}

// ── HTTP cache ────────────────────────────────────────────────────────

type Cache = HashMap<String, (String, Instant)>;

fn cache() -> &'static RwLock<Cache> {
    static CACHE: OnceLock<RwLock<Cache>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

const CACHE_TTL: Duration = Duration::from_secs(60);

fn cache_get(url: &str) -> Option<String> {
    let map = cache().read().ok()?;
    if let Some((body, time)) = map.get(url) {
        if time.elapsed() < CACHE_TTL {
            return Some(body.clone());
        }
    }
    None
}

fn cache_set(url: &str, body: &str) {
    if let Ok(mut map) = cache().write() {
        map.insert(url.to_string(), (body.to_string(), Instant::now()));
    }
}

// ── CSP ───────────────────────────────────────────────────────────────

fn header_value<'a>(headers: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    headers.iter().find_map(|(k, v)| k.eq_ignore_ascii_case(key).then_some(v.as_str()))
}

fn csp_directive_blocks(csp: &str, directive: &str) -> bool {
    csp.split(';').any(|part| {
        let trimmed = part.trim();
        trimmed.starts_with(directive) && trimmed.contains("'none'")
    })
}

/// Checks Content-Security-Policy headers and applies a conservative block for
/// pages that explicitly opt out of all content or scripts.
pub fn check_csp(url: &str, headers: &HashMap<String, String>) -> bool {
    if let Some(csp) = header_value(headers, "content-security-policy") {
        plog!("csp", "Content-Security-Policy for {}: {}", url, csp);
        if csp_directive_blocks(csp, "default-src") || csp_directive_blocks(csp, "script-src") {
            plog!("csp", "Blocking {} due to restrictive CSP", url);
            return false;
        }
    }
    true
}

pub fn csp_blocks_scripts(headers: &HashMap<String, String>) -> bool {
    header_value(headers, "content-security-policy")
        .is_some_and(|csp| csp_directive_blocks(csp, "default-src") || csp_directive_blocks(csp, "script-src"))
}

pub fn csp_blocks_styles(headers: &HashMap<String, String>) -> bool {
    header_value(headers, "content-security-policy")
        .is_some_and(|csp| csp_directive_blocks(csp, "default-src") || csp_directive_blocks(csp, "style-src"))
}

pub struct Response {
    pub body: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub final_url: String,
}

impl Response {
    pub fn is_redirect(&self) -> bool {
        self.status >= 300 && self.status < 400
    }
}

/// Fetches the HTML content from the given URL.
pub fn fetch(url: &str) -> Result<String, FetchError> {
    if let Some(cached) = cache_get(url) {
        plog!("cache", "HIT: {}", url);
        return Ok(cached);
    }
    if let Some(body) = mock::resolve_html(url) {
        plog!("mock", "Serving HTML for {}", url);
        return Ok(body);
    }
    if let Some(body) = mock::resolve_css(url) {
        plog!("mock", "Serving CSS for {}", url);
        return Ok(body);
    }
    match fetch_with_redirects(url, 5) {
        Ok(resp) => {
            if resp.body.is_empty() {
                return Err(FetchError::EmptyBody);
            }
            cache_set(url, &resp.body);
            Ok(resp.body)
        }
        Err(e) => Err(e),
    }
}

/// Fetches content with automatic redirect handling.
pub fn fetch_with_redirects(url: &str, max_redirects: usize) -> Result<Response, FetchError> {
    fetch_inner(url, max_redirects)
}

fn fetch_inner(url: &str, max_redirects: usize) -> Result<Response, FetchError> {
    let final_url = normalize_url(url);
    plog!("net", "Fetching: {}", final_url);

    let _start = std::time::Instant::now();

    let cl = client().ok_or_else(|| FetchError::Network("HTTP client not available".to_string()))?;
    let resp = match cl.get(&final_url).send() {
        Ok(r) => r,
        Err(e) => return Err(FetchError::from(e)),
    };
    let status: u16 = resp.status().as_u16();
    plog!("net", "Got response, status: {}", status);

    if status >= 400 {
        return Err(FetchError::Http(status, format!("HTTP error {}", status)));
    }

    let mut headers = HashMap::new();
    for name in resp.headers().keys() {
        if let Some(value) = resp.headers().get(name) {
            if let Ok(v) = value.to_str() {
                headers.insert(name.to_string(), v.to_string());
            }
        }
    }

    if !check_csp(&final_url, &headers) {
        return Err(FetchError::Network("Blocked by Content-Security-Policy".to_string()));
    }

    if resp.status().is_redirection() && max_redirects > 0 {
        if let Some(location) = headers.get("location") {
            plog!("net", "Redirect to: {}", location);
            let next = resolve_url(location, &final_url);
            return fetch_inner(&next, max_redirects - 1);
        }
    }

    let body = resp.text().map_err(|e| FetchError::Network(format!("Failed to read body: {}", e)))?;
    plog!("net", "Body length: {}", body.len());

    Ok(Response { body, status, headers, final_url })
}

/// Fetches binary content (images, etc.) from the given URL.
pub fn fetch_bytes(url: &str) -> Result<Vec<u8>, FetchError> {
    if let Some(bytes) = mock::resolve_binary(url) {
        plog!("mock", "Serving binary for {}", url);
        return Ok(bytes);
    }
    let mut current_url = normalize_url(url);
    plog!("net", "Fetching binary: {}", current_url);

    for _ in 0..5 {
        let cl = client().ok_or_else(|| FetchError::Network("HTTP client not available".to_string()))?;
        let resp = match cl.get(&current_url).send() {
            Ok(r) => r,
            Err(e) => return Err(FetchError::from(e)),
        };
        let status = resp.status().as_u16();
        plog!("net", "Got binary response, status: {}", status);
        if status >= 400 {
            return Err(FetchError::Http(status, format!("HTTP error {}", status)));
        }
        if resp.status().is_redirection() {
            let headers = resp.headers().clone();
            if let Some(location) = headers.get("location").and_then(|v| v.to_str().ok()) {
                current_url = resolve_url(location, &current_url);
                continue;
            }
        }
        let bytes = resp.bytes().map_err(|e| FetchError::Network(format!("Failed to read bytes: {}", e)))?.to_vec();
        plog!("net", "Fetched {} bytes", bytes.len());
        return Ok(bytes);
    }
    Err(FetchError::Network("Too many redirects".to_string()))
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

/// Resolve a potentially relative URL against a base URL.
pub fn resolve_url(url: &str, base_url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }

    if url.starts_with("//") {
        let stripped = url.trim_start_matches('/');
        return format!("https://{}", stripped);
    }

    let base = normalize_url(base_url);

    let scheme_end = base.find("://").map(|i| i + 3).unwrap_or(0);
    if scheme_end == 0 { return base; }
    let host_end = base[scheme_end..]
        .find('/')
        .map(|i| scheme_end + i)
        .unwrap_or(base.len());
    let origin = &base[..host_end];

    if url.starts_with('/') {
        return format!("{}{}", origin, url);
    }

    let base_path = &base[host_end..];
    let mut segments: Vec<&str> = base_path.split('/').filter(|s| !s.is_empty()).collect();
    if !base_path.ends_with('/') {
        segments.pop();
    }

    for segment in url.split('/') {
        match segment {
            "." | "" => {}
            ".." => {
                segments.pop();
            }
            s => segments.push(s),
        }
    }

    if segments.is_empty() {
        origin.to_string()
    } else {
        format!("{}/{}", origin, segments.join("/"))
    }
}
