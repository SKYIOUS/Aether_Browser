//! Networking module for fetching web resources.
pub mod mock;

use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;
use std::fmt;
use std::path::PathBuf;

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
            .user_agent("Aether/0.2.0 (Rust; +https://aether-browser.dev)")
            .danger_accept_invalid_certs(false)
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

// ── Cookie jar ────────────────────────────────────────────────────────

type CookieJar = HashMap<String, HashMap<String, String>>;

fn cookie_jar() -> &'static RwLock<CookieJar> {
    static JAR: OnceLock<RwLock<CookieJar>> = OnceLock::new();
    JAR.get_or_init(|| RwLock::new(load_cookies().unwrap_or_default()))
}

fn cookie_file() -> Option<PathBuf> {
    std::env::current_dir().ok().map(|p| p.join("aether_cookies.json"))
}

fn load_cookies() -> Option<CookieJar> {
    let path = cookie_file()?;
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

fn save_cookies() {
    if let Ok(jar) = cookie_jar().read() {
        if let Some(path) = cookie_file() {
            if let Ok(json) = serde_json::to_string(&*jar) {
                if let Err(e) = std::fs::write(&path, json) {
                    plog!("COOKIE", "save_cookies write failed: {}", e);
                }
            }
        }
    }
}

fn maybe_save_cookies() {
    static LAST_SAVE: OnceLock<Mutex<Instant>> = OnceLock::new();
    let last = LAST_SAVE.get_or_init(|| Mutex::new(Instant::now()));
    let mut last = last.lock().unwrap();
    if last.elapsed() > Duration::from_secs(30) {
        save_cookies();
        *last = Instant::now();
    }
}

fn cookie_origin_key(url: &str) -> String {
    let s = url.trim();
    let (protocol, rest) = if let Some(pos) = s.find("://") {
        (s[..pos + 1].to_string(), &s[pos + 3..])
    } else {
        ("https:".to_string(), s)
    };
    let host_and_port = rest.split('/').next().unwrap_or(rest);
    if host_and_port.is_empty() {
        return "null".to_string();
    }
    let (hostname, port) = if host_and_port.starts_with('[') {
        if let Some(br) = host_and_port.find(']') {
            let h = &host_and_port[..=br];
            if br + 1 < host_and_port.len() && host_and_port.as_bytes()[br + 1] == b':' {
                (h.to_string(), host_and_port[br + 2..].to_string())
            } else {
                (h.to_string(), String::new())
            }
        } else {
            (host_and_port.to_string(), String::new())
        }
    } else if let Some(pos) = host_and_port.find(':') {
        (host_and_port[..pos].to_string(), host_and_port[pos + 1..].to_string())
    } else {
        (host_and_port.to_string(), String::new())
    };
    if port.is_empty() {
        format!("{}//{}", protocol, hostname)
    } else {
        format!("{}//{}:{}", protocol, hostname, port)
    }
}

pub fn set_cookie_from_response(url: &str, set_cookie_header: &str) {
    let trimmed = set_cookie_header.trim();
    if let Some(eq_pos) = trimmed.find('=') {
        let key = trimmed[..eq_pos].trim().to_string();
        let value = trimmed[eq_pos + 1..].split(';').next().unwrap_or("").trim().to_string();
        if !key.is_empty() && !value.is_empty() {
            if let Ok(mut jar) = cookie_jar().write() {
                let origin = cookie_origin_key(url);
                let total: usize = jar.values().map(|m| m.len()).sum();
                let origin_has_room = jar.get(&origin).map(|m| m.len() < 50).unwrap_or(true);
                if !origin_has_room || total >= 500 { return; }
                jar.entry(origin).or_default().insert(key, value);
                save_cookies();
            }
        }
    }
}

pub fn get_cookies_for_url(url: &str) -> String {
    let mut parts = Vec::new();
    if let Ok(jar) = cookie_jar().read() {
        let origin = cookie_origin_key(url);
        if let Some(cookies) = jar.get(&origin) {
            for (key, value) in cookies {
                parts.push(format!("{}={}", key, value));
            }
        }
    }
    maybe_save_cookies();
    parts.join("; ")
}

// ── CSP Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CspDirective {
    DefaultSrc,
    ScriptSrc,
    StyleSrc,
    ImgSrc,
    ConnectSrc,
    FrameSrc,
    ReportUri,
    ReportTo,
    UpgradeInsecureRequests,
    BlockAllMixedContent,
    Other(String),
}

impl CspDirective {
    fn from_str(s: &str) -> Self {
        match s {
            "default-src" => CspDirective::DefaultSrc,
            "script-src" => CspDirective::ScriptSrc,
            "style-src" => CspDirective::StyleSrc,
            "img-src" => CspDirective::ImgSrc,
            "connect-src" => CspDirective::ConnectSrc,
            "frame-src" => CspDirective::FrameSrc,
            "report-uri" => CspDirective::ReportUri,
            "report-to" => CspDirective::ReportTo,
            "upgrade-insecure-requests" => CspDirective::UpgradeInsecureRequests,
            "block-all-mixed-content" => CspDirective::BlockAllMixedContent,
            _ => CspDirective::Other(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CspSource {
    Self_,
    None,
    UnsafeInline,
    UnsafeEval,
    Scheme(String),
    Host { host: String, port: Option<u16>, scheme: Option<String> },
    Wildcard,
    StrictDynamic,
}

#[derive(Debug, Clone)]
pub struct CspDirectiveEntry {
    pub directive: CspDirective,
    pub sources: Vec<CspSource>,
}

#[derive(Debug, Clone, Default)]
pub struct CspPolicy {
    pub directives: Vec<CspDirectiveEntry>,
}

impl CspPolicy {
    pub fn is_empty(&self) -> bool { self.directives.is_empty() }

    fn sources_for(&self, directive: &CspDirective) -> Option<&[CspSource]> {
        self.directives.iter().find(|e| e.directive == *directive).map(|e| e.sources.as_slice())
    }

    fn effective_sources_for(&self, directive: &CspDirective) -> &[CspSource] {
        self.sources_for(directive)
            .or_else(|| if *directive != CspDirective::DefaultSrc { self.sources_for(&CspDirective::DefaultSrc) } else { None })
            .unwrap_or(&[])
    }

    /// Check if a specific URL is allowed by a directive.
    pub fn allows_url(&self, directive: &CspDirective, url: &str, origin: &str) -> bool {
        let sources = self.effective_sources_for(directive);
        if sources.is_empty() { return true; }
        if sources.contains(&CspSource::None) { return false; }
        if sources.contains(&CspSource::Wildcard) { return true; }

        let parts = parse_simple_url(url);

        for s in sources {
            match s {
                CspSource::Self_ => {
                    if csp_url_matches_origin(&parts, origin) { return true; }
                }
                CspSource::Scheme(scheme) => {
                    if parts.protocol == scheme.as_str() { return true; }
                }
                CspSource::Host { host, port, scheme } => {
                    if let Some(ref s) = scheme {
                        if parts.protocol.trim_end_matches(':') != s.as_str() { continue; }
                    }
                    if let Some(p) = port {
                        let url_port = parts.port.parse::<u16>().unwrap_or(if parts.protocol == "http:" { 80 } else { 443 });
                        if url_port != *p { continue; }
                    }
                    if host.starts_with("*.") {
                        let suffix = &host[1..];
                        if parts.hostname.ends_with(suffix) || parts.hostname == &host[2..] { return true; }
                    } else if parts.hostname == host.as_str() {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Check if 'unsafe-inline' is in the directive (allows inline code).
    pub fn allows_inline(&self, directive: &CspDirective) -> bool {
        self.effective_sources_for(directive).contains(&CspSource::UnsafeInline)
    }

    /// Check if 'unsafe-eval' is allowed for script-src.
    pub fn allows_eval(&self) -> bool {
        self.effective_sources_for(&CspDirective::ScriptSrc).contains(&CspSource::UnsafeEval)
    }

    /// Returns true if all scripts should be blocked (only 'none' or empty).
    pub fn blocks_all_scripts(&self) -> bool {
        let srcs = self.effective_sources_for(&CspDirective::ScriptSrc);
        srcs.is_empty() || (srcs.len() == 1 && srcs[0] == CspSource::None)
    }

    /// Returns true if all styles should be blocked (only 'none' or empty).
    pub fn blocks_all_styles(&self) -> bool {
        let srcs = self.effective_sources_for(&CspDirective::StyleSrc);
        srcs.is_empty() || (srcs.len() == 1 && srcs[0] == CspSource::None)
    }
}

// ── CSP Parser ────────────────────────────────────────────────────────

fn parse_csp_source(token: &str) -> Option<CspSource> {
    match token {
        "'self'" | "self" => Some(CspSource::Self_),
        "'none'" | "none" => Some(CspSource::None),
        "'unsafe-inline'" | "unsafe-inline" => Some(CspSource::UnsafeInline),
        "'unsafe-eval'" | "unsafe-eval" => Some(CspSource::UnsafeEval),
        "'strict-dynamic'" | "strict-dynamic" => Some(CspSource::StrictDynamic),
        "*" => Some(CspSource::Wildcard),
        t if t.starts_with("'nonce-") && t.ends_with('\'') => Some(CspSource::Self_),
        t if t.ends_with(':') => Some(CspSource::Scheme(t.to_string())),
        t => {
            let (scheme, rest) = if let Some(pos) = t.find("://") {
                (Some(t[..pos].to_string()), &t[pos + 3..])
            } else { (None, t) };

            if rest.is_empty() || rest == "*" { return None; }

            let (host_str, port) = if rest.starts_with('[') {
                if let Some(br) = rest.find(']') {
                    let ip = rest[..=br].to_string();
                    if br + 1 < rest.len() && rest.as_bytes()[br + 1] == b':' {
                        let p = rest[br + 2..].parse::<u16>().ok();
                        (ip, p)
                    } else { (ip, None) }
                } else { (rest.to_string(), None) }
            } else if let Some(pos) = rest.rfind(':') {
                if let Ok(p) = rest[pos + 1..].parse::<u16>() {
                    (rest[..pos].to_string(), Some(p))
                } else { (rest.to_string(), None) }
            } else { (rest.to_string(), None) };

            if host_str.is_empty() { None }
            else { Some(CspSource::Host { host: host_str, port, scheme }) }
        }
    }
}

pub fn parse_csp(header: &str) -> CspPolicy {
    let mut policy = CspPolicy::default();
    for part in header.split(';') {
        let trimmed = part.trim();
        if trimmed.is_empty() { continue; }
        let mut tokens = trimmed.split_ascii_whitespace();
        let name = match tokens.next() { Some(n) => n, None => continue };
        let directive = CspDirective::from_str(name);
        let sources: Vec<CspSource> = tokens.filter_map(parse_csp_source).collect();
        policy.directives.push(CspDirectiveEntry { directive, sources });
    }
    policy
}

pub fn parse_csp_from_headers(headers: &HashMap<String, String>) -> CspPolicy {
    header_value(headers, "content-security-policy")
        .map(parse_csp)
        .unwrap_or_default()
}

// ── CSP Violation Logging ─────────────────────────────────────────────

fn log_violation(directive: &str, resource: &str, origin: &str) {
    eprintln!("[CSP] Blocked by {}: {} (origin: {})", directive, resource, origin);
    plog!("csp-violation", "Blocked {} by {} (origin: {})", resource, directive, origin);
}

// ── CSP Store (per-origin) ────────────────────────────────────────────

type CspStore = HashMap<String, CspPolicy>;

fn csp_store() -> &'static RwLock<CspStore> {
    static STORE: OnceLock<RwLock<CspStore>> = OnceLock::new();
    STORE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn origin_from_url_inner(url: &str) -> String {
    let p = parse_simple_url(url);
    if p.port.is_empty() { format!("{}//{}", p.protocol, p.hostname) }
    else { format!("{}//{}:{}", p.protocol, p.hostname, p.port) }
}

pub fn store_csp(url: &str, headers: &HashMap<String, String>) {
    if let Some(csp_str) = header_value(headers, "content-security-policy") {
        let origin = origin_from_url_inner(url);
        let policy = parse_csp(csp_str);
        if let Ok(mut store) = csp_store().write() {
            store.insert(origin, policy);
        }
    }
}

pub fn get_csp_for(url: &str) -> CspPolicy {
    let origin = origin_from_url_inner(url);
    csp_store().read().ok().and_then(|s| s.get(&origin).cloned()).unwrap_or_default()
}

// ── Public CSP Check Functions ────────────────────────────────────────

/// Check if a resource is allowed by the response's own CSP header.
// ponytail: used for initial page-load fetch; also stores CSP for the origin
pub fn check_csp(url: &str, headers: &HashMap<String, String>) -> bool {
    let policy = parse_csp_from_headers(headers);
    if policy.is_empty() { return true; }
    let origin = origin_from_url_inner(url);
    let allowed = policy.allows_url(&CspDirective::DefaultSrc, url, &origin);
    if !allowed {
        log_violation("default-src", url, &origin);
    }
    store_csp(url, headers);
    allowed
}

/// Returns true if the page's CSP blocks ALL external & inline scripts.
pub fn csp_blocks_scripts(headers: &HashMap<String, String>) -> bool {
    let policy = parse_csp_from_headers(headers);
    if policy.is_empty() { return false; }
    policy.blocks_all_scripts() && !policy.allows_inline(&CspDirective::ScriptSrc)
}

/// Returns true if the page's CSP blocks ALL external & inline styles.
pub fn csp_blocks_styles(headers: &HashMap<String, String>) -> bool {
    let policy = parse_csp_from_headers(headers);
    if policy.is_empty() { return false; }
    policy.blocks_all_styles() && !policy.allows_inline(&CspDirective::StyleSrc)
}

/// Check if a specific script URL is allowed by the page's CSP policy.
pub fn csp_allows_script_url(script_url: &str, page_url: &str, policy: &CspPolicy) -> bool {
    if policy.is_empty() { return true; }
    let origin = origin_from_url_inner(page_url);
    let allowed = policy.allows_url(&CspDirective::ScriptSrc, script_url, &origin);
    if !allowed { log_violation("script-src", script_url, &origin); }
    allowed
}

/// Check if a specific style URL is allowed by the page's CSP policy.
pub fn csp_allows_style_url(style_url: &str, page_url: &str, policy: &CspPolicy) -> bool {
    if policy.is_empty() { return true; }
    let origin = origin_from_url_inner(page_url);
    let allowed = policy.allows_url(&CspDirective::StyleSrc, style_url, &origin);
    if !allowed { log_violation("style-src", style_url, &origin); }
    allowed
}

/// Check if an image URL is allowed by the page's CSP policy.
pub fn csp_allows_image_url(img_url: &str, page_url: &str, policy: &CspPolicy) -> bool {
    if policy.is_empty() { return true; }
    let origin = origin_from_url_inner(page_url);
    let allowed = policy.allows_url(&CspDirective::ImgSrc, img_url, &origin);
    if !allowed { log_violation("img-src", img_url, &origin); }
    allowed
}

/// Check if a connect/fetch/XHR URL is allowed by the page's CSP policy.
pub fn csp_allows_connect_url(connect_url: &str, page_url: &str, policy: &CspPolicy) -> bool {
    if policy.is_empty() { return true; }
    let origin = origin_from_url_inner(page_url);
    let allowed = policy.allows_url(&CspDirective::ConnectSrc, connect_url, &origin);
    if !allowed { log_violation("connect-src", connect_url, &origin); }
    allowed
}

/// Check if inline scripts are allowed by the page's CSP policy.
pub fn csp_allows_inline_script(policy: &CspPolicy) -> bool {
    policy.is_empty() || policy.allows_inline(&CspDirective::ScriptSrc)
}

/// Check if inline styles are allowed by the page's CSP policy.
pub fn csp_allows_inline_style(policy: &CspPolicy) -> bool {
    policy.is_empty() || policy.allows_inline(&CspDirective::StyleSrc)
}

// ── URL Helpers ───────────────────────────────────────────────────────

fn header_value<'a>(headers: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    headers.iter().find_map(|(k, v)| k.eq_ignore_ascii_case(key).then_some(v.as_str()))
}

fn csp_url_matches_origin(parts: &UrlParts<'_>, origin: &str) -> bool {
    let o = parse_simple_url(origin);
    parts.protocol == o.protocol && parts.hostname == o.hostname && parts.port == o.port
}

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
    match fetch_with_redirects(url, 5, None) {
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

/// Fetches with CORS origin header + ACAO response checking.
pub fn fetch_with_cors(url: &str, origin: &str) -> Result<(String, u16), FetchError> {
    match fetch_with_redirects(url, 5, Some(origin)) {
        Ok(resp) => {
            if resp.body.is_empty() {
                return Err(FetchError::EmptyBody);
            }
            Ok((resp.body, resp.status))
        }
        Err(e) => Err(e),
    }
}

/// Fetches content with automatic redirect handling.
pub fn fetch_with_redirects(url: &str, max_redirects: usize, origin: Option<&str>) -> Result<Response, FetchError> {
    fetch_inner(url, max_redirects, origin)
}

fn is_scheme_downgrade(original_url: &str, redirect_url: &str) -> bool {
    let orig_is_https = original_url.starts_with("https://");
    let redir_is_http = redirect_url.starts_with("http://");
    if orig_is_https && redir_is_http {
        plog!("net", "Blocked HTTPS→HTTP downgrade redirect: {} → {}", original_url, redirect_url);
        return true;
    }
    false
}

fn fetch_inner(url: &str, max_redirects: usize, origin: Option<&str>) -> Result<Response, FetchError> {
    let final_url = normalize_url(url);
    plog!("net", "Fetching: {}", final_url);

    let _start = std::time::Instant::now();

    let cl = client().ok_or_else(|| FetchError::Network("HTTP client not available".to_string()))?;
    let cookies = get_cookies_for_url(&final_url);
    let mut req = cl.get(&final_url);
    if !cookies.is_empty() {
        req = req.header("Cookie", &cookies);
    }
    if let Some(origin) = origin {
        let origin_url = normalize_url(origin);
        req = req.header("Origin", &origin_url);
    }
    let resp = match req.send() {
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
                if name.as_str().eq_ignore_ascii_case("set-cookie") {
                    set_cookie_from_response(&final_url, v);
                }
            }
        }
    }

    // CORS check: if origin is provided and request is cross-origin, require ACAO
    if let Some(origin) = origin {
        let normalized_origin = normalize_url(origin);
        if !is_same_origin(&final_url, &normalized_origin) {
            let acao = headers.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case("access-control-allow-origin"))
                .map(|(_, v)| v.as_str());
            let allowed = match acao {
                Some("*") => true,
                Some(header_origin) => is_same_origin(header_origin, &normalized_origin),
                None => false,
            };
            if !allowed {
                return Err(FetchError::CrossOrigin {
                    target: final_url.clone(),
                    origin: normalized_origin,
                });
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
            if is_scheme_downgrade(&final_url, &next) {
                plog!("net", "HTTPS→HTTP downgrade blocked, returning current response");
            } else {
                return fetch_inner(&next, max_redirects - 1, origin);
            }
        }
    }

    let body = resp.text().map_err(|e| FetchError::Network(format!("Failed to read body: {}", e)))?;
    plog!("net", "Body length: {}", body.len());

    Ok(Response { body, status, headers, final_url })
}

pub fn is_same_origin(a: &str, b: &str) -> bool {
    let pa = parse_simple_url(a);
    let pb = parse_simple_url(b);
    pa.protocol == pb.protocol
        && pa.hostname.to_lowercase() == pb.hostname.to_lowercase()
        && pa.port == pb.port
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
