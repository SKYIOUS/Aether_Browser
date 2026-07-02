//! Networking module for fetching web resources.
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;
use ureq;

// ── HTTP cache ────────────────────────────────────────────────────────

type Cache = HashMap<String, (String, Instant)>;

fn cache() -> &'static Mutex<Cache> {
    static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

const CACHE_TTL: Duration = Duration::from_secs(60);

fn cache_get(url: &str) -> Option<String> {
    let map = cache().lock().ok()?;
    if let Some((body, time)) = map.get(url) {
        if time.elapsed() < CACHE_TTL {
            return Some(body.clone());
        }
    }
    None
}

fn cache_set(url: &str, body: &str) {
    if let Ok(mut map) = cache().lock() {
        map.insert(url.to_string(), (body.to_string(), Instant::now()));
    }
}

// ── CSP ───────────────────────────────────────────────────────────────

/// Checks Content-Security-Policy headers.
/// Currently warn-only: returns true (permissive) but logs violations.
pub fn check_csp(url: &str, headers: &HashMap<String, String>) -> bool {
    if let Some(csp) = headers.get("content-security-policy") {
        eprintln!("[CSP] Content-Security-Policy for {}: {}", url, csp);
    }
    true
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
pub fn fetch(url: &str) -> String {
    if let Some(cached) = cache_get(url) {
        println!("[CACHE] HIT: {}", url);
        return cached;
    }
    let result = match fetch_with_redirects(url, 5) {
        Ok(resp) => resp.body,
        Err(e) => format!("Error: {}", e),
    };
    if !result.starts_with("Error") {
        cache_set(url, &result);
    }
    result
}

/// Fetches content with automatic redirect handling.
pub fn fetch_with_redirects(url: &str, max_redirects: usize) -> Result<Response, String> {
    fetch_inner(url, max_redirects)
}

fn fetch_inner(url: &str, max_redirects: usize) -> Result<Response, String> {
    let final_url = normalize_url(url);
    println!("Fetching: {}", final_url);

    let _start = std::time::Instant::now();

    match ureq::get(&final_url)
        .config()
        .timeout_global(Some(std::time::Duration::from_secs(15)))
        .build()
        .call()
    {
        Ok(mut response) => {
            let status: u16 = response.status().as_u16();
            println!("Got response, status: {}", status);
            
            let mut headers = HashMap::new();
            for name in response.headers().keys() {
                if let Some(value) = response.headers().get(name) {
                    if let Ok(v) = value.to_str() {
                        headers.insert(name.to_string(), v.to_string());
                    }
                }
            }

            check_csp(&final_url, &headers);

            let body = match response.body_mut().read_to_string() {
                Ok(b) => {
                    println!("Body length: {}", b.len());
                    b
                }
                Err(e) => format!("Failed to read body: {}", e),
            };
            
            let status_code = response.status();
            if status_code.is_redirection() && max_redirects > 0 {
                if let Some(location) = headers.get("location") {
                    println!("Redirect to: {}", location);
                    return fetch_inner(location, max_redirects - 1);
                }
            }
            
            Ok(Response {
                body,
                status,
                headers,
                final_url,
            })
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(format!("Error fetching URL: {}", e))
        }
    }
}

/// Fetches binary content (images, etc.) from the given URL.
pub fn fetch_bytes(url: &str) -> Vec<u8> {
    let final_url = normalize_url(url);

    println!("Fetching binary: {}", final_url);
    
    match ureq::get(&final_url).call() {
        Ok(mut response) => {
            println!("Got response, status: {}", response.status().as_u16());
            match response.body_mut().read_to_vec() {
                Ok(bytes) => {
                    let len = bytes.len();
                    println!("Fetched {} bytes", len);
                    bytes
                }
                Err(e) => {
                    println!("Failed to read bytes: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            println!("Error fetching binary: {}", e);
            Vec::new()
        }
    }
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