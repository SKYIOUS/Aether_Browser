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

/// Returns true if the CSP directive explicitly uses `'none'`.
fn csp_directive_is_none(csp: &str, directive: &str) -> bool {
    csp.split(';').any(|part| {
        let trimmed = part.trim();
        trimmed.starts_with(directive) && trimmed.contains("'none'")
    })
}

/// Returns true if the CSP directive includes `'self'` (allows same-origin).
// ponytail: no support for specific origin allowlists or port restrictions
fn csp_directive_allows_self(csp: &str, directive: &str) -> bool {
    csp.split(';').any(|part| {
        let trimmed = part.trim();
        if trimmed.starts_with(directive) {
            let rest = &trimmed[directive.len()..].trim();
            rest.contains("'self'") || (!rest.contains("'none'") && !rest.is_empty())
        } else {
            false
        }
    })
}

fn csp_url_matches_origin(url: &str, origin_url: &str) -> bool {
    let parts = parse_simple_url(url);
    let origin_parts = parse_simple_url(origin_url);
    parts.protocol == origin_parts.protocol && parts.hostname == origin_parts.hostname && parts.port == origin_parts.port
}

/// Checks Content-Security-Policy headers. Returns false only when the policy
/// explicitly blocks the resource (e.g. `'none'` or a non-matching allowlist
/// without `'self'` for same-origin resources).
// ponytail: only handles 'none', 'self', and basic same-origin matching
pub fn check_csp(url: &str, headers: &HashMap<String, String>) -> bool {
    if let Some(csp) = header_value(headers, "content-security-policy") {
        plog!("csp", "Content-Security-Policy for {}: {}", url, csp);
        if csp_directive_is_none(csp, "default-src") || csp_directive_is_none(csp, "script-src") {
            plog!("csp", "Blocking {} due to CSP 'none'", url);
            return false;
        }
        // ponytail: check default-src or script-src for the resource URL
        if csp_directive_allows_self(csp, "default-src") || csp_directive_allows_self(csp, "script-src") {
            let origin = if let Some(first_url) = headers.get("final_url").or_else(|| headers.get("url")) {
                first_url
            } else {
                return true; // no origin to compare against, allow
            };
            if !csp_url_matches_origin(url, origin) {
                plog!("csp", "Blocking {}: CSP allows 'self' only", url);
                return false;
            }
        }
    }
    true
}

pub fn csp_blocks_scripts(headers: &HashMap<String, String>) -> bool {
    header_value(headers, "content-security-policy")
        .is_some_and(|csp| csp_directive_is_none(csp, "default-src") || csp_directive_is_none(csp, "script-src"))
}

pub fn csp_blocks_styles(headers: &HashMap<String, String>) -> bool {
    header_value(headers, "content-security-policy")
        .is_some_and(|csp| csp_directive_is_none(csp, "default-src") || csp_directive_is_none(csp, "style-src"))
}

// ponytail: simple URL parser for CSP origin matching
fn parse_simple_url<'a>(url: &'a str) -> UrlParts<'a> {
    let mut protocol = "https:";
    let rest = if let Some(pos) = url.find("://") {
        protocol = &url[..pos + 1];
        &url[pos + 3..]
    } else { url };
    let hostname = if let Some(pos) = rest.find('/') { &rest[..pos] } else { rest };
    let mut port = "";
    let hostname = if hostname.starts_with('[') {
        if let Some(br) = hostname.find(']') {
            if br + 1 < hostname.len() && hostname.as_bytes()[br + 1] == b':' {
                port = &hostname[br + 2..];
            }
            hostname
        } else { hostname }
    } else if let Some(pos) = hostname.find(':') {
        port = &hostname[pos + 1..];
        &hostname[..pos]
    } else { hostname };
    UrlParts { protocol, hostname, port }
}

struct UrlParts<'a> { protocol: &'a str, hostname: &'a str, port: &'a str }

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

/// Fetches content from the given URL, returning body + HTTP status code.
// ponytail: cache uses raw URL, not normalized — may cause miss/hit mismatch
pub fn fetch(url: &str) -> Result<(String, u16), FetchError> {
    if let Some(cached) = cache_get(url) {
        plog!("cache", "HIT: {}", url);
        return Ok((cached, 200));
    }
    if let Some(body) = mock::resolve_html(url) {
        plog!("mock", "Serving HTML for {}", url);
        return Ok((body, 200));
    }
    if let Some(body) = mock::resolve_css(url) {
        plog!("mock", "Serving CSS for {}", url);
        return Ok((body, 200));
    }
    match fetch_with_redirects(url, 5) {
        Ok(resp) => {
            if resp.body.is_empty() {
                return Err(FetchError::EmptyBody);
            }
            cache_set(url, &resp.body);
            Ok((resp.body, resp.status))
        }
        Err(e) => Err(e),
    }
}

// ponytail: sync XHR — blocks JS thread, keep timeout short
pub fn fetch_xhr(url: &str) -> String {
    if let Some(body) = mock::resolve_html(url) {
        return body;
    }
    if let Some(body) = mock::resolve_css(url) {
        return body;
    }
    let cl = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
    {
        Ok(c) => c,
        Err(e) => return format!("Error: {}", e),
    };
    let final_url = normalize_url(url);
    match cl.get(&final_url).send() {
        Ok(r) => r.text().unwrap_or_default(),
        Err(e) => format!("Error: {}", e),
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

type ImageCache = HashMap<String, (Vec<u8>, Instant)>;

fn image_cache() -> &'static RwLock<ImageCache> {
    static IMG_CACHE: OnceLock<RwLock<ImageCache>> = OnceLock::new();
    IMG_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Fetches binary content (images, etc.) from the given URL.
// ponytail: simple image cache with 60s TTL, capped at 100 entries
pub fn fetch_bytes(url: &str) -> Result<Vec<u8>, FetchError> {
    if let Some(bytes) = mock::resolve_binary(url) {
        plog!("mock", "Serving binary for {}", url);
        return Ok(bytes);
    }
    if let Ok(cache) = image_cache().read() {
        if let Some((bytes, time)) = cache.get(url) {
            if time.elapsed() < Duration::from_secs(60) {
                plog!("cache", "Image HIT: {}", url);
                return Ok(bytes.clone());
            }
        }
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
        if let Ok(mut cache) = image_cache().write() {
            if cache.len() > 100 {
                cache.clear();
                plog!("cache", "Image cache evicted (size > 100)");
            }
            cache.insert(url.to_string(), (bytes.clone(), Instant::now()));
        }
        return Ok(bytes);
    }
    Err(FetchError::Network("Too many redirects".to_string()))
}

pub fn normalize_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        let cleaned = url.trim_start_matches('/');
        format!("https://{}", cleaned)
    }
}

/// Resolve a potentially relative URL against a base URL.
pub fn resolve_url(url: &str, base_url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }

    if url.starts_with("//") {
        return format!("https:{}", url);
    }

    let base = normalize_url(base_url);

    let scheme_end = base.find("://").map(|i| i + 3).unwrap_or(0);
    if scheme_end == 0 { return base; }
    let after_scheme = &base[scheme_end..];
    let host_end = if after_scheme.starts_with('[') {
        // ponytail: assumes well-formed IPv6 like [::1], no nested brackets
        after_scheme.find("]").map(|i| scheme_end + i + 1).unwrap_or(base.len())
    } else {
        after_scheme.find('/').map(|i| scheme_end + i).unwrap_or(base.len())
    };
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
