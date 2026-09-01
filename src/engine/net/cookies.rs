// Cookie lifecycle + security policy (PLAN C2).
//
// Pure policy helpers live here so rules stay unit-testable; the jar, its
// persistence, and the send-collector are the only stateful pieces.
//
// Known limitation (documented in PLAN C2): registrable-domain calculation is
// a naive last-two-labels approximation - no public-suffix list. IP hosts and
// single-label hosts (localhost) are treated as whole registrable domains.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::time::Instant;

use crate::plog;

const MAX_COOKIE_LINE_BYTES: usize = 4096;
const COOKIE_FILE: &str = "vayu_cookies.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SameSite {
    Strict,
    #[default]
    Lax,
    /// Serialized as "none" so attribute-name round-trips stay readable.
    #[serde(rename = "none")]
    NoneSamesite,
}

impl SameSite {
    fn from_attr(value: Option<&str>) -> Self {
        match value.map(|v| v.to_ascii_lowercase()) {
            Some(v) if v == "strict" => SameSite::Strict,
            Some(v) if v == "none" => SameSite::NoneSamesite,
            // Unspecified AND unknown values both land on Lax (modern default).
            _ => SameSite::Lax,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieRecord {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub host_only: bool,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    /// Instant is not serde-stable across machines; persistence is best-effort.
    #[serde(skip)]
    pub expires: Option<Instant>,
}

impl CookieRecord {
    fn expired_at(&self, now: Instant) -> bool {
        self.expires.map(|e| e <= now).unwrap_or(false)
    }
}

/// Naive registrable domain: last two labels. IPs and single-label hosts
/// return unchanged.
pub fn registrable_site(host: &str) -> String {
    let h = host.trim().to_ascii_lowercase();
    if h.parse::<std::net::IpAddr>().is_ok() {
        return h;
    }
    let labels: Vec<&str> = h.split('.').filter(|l| !l.is_empty()).collect();
    match labels.len() {
        n if n <= 2 => h,
        n => {
            let keep_from = labels[..n - 2].iter().map(|l| l.len() + 1).sum::<usize>();
            h[keep_from..].to_string()
        }
    }
}

/// Schemeful site: scheme + naive registrable domain (ports ignored).
pub fn schemeful_site(url: &str) -> String {
    let (scheme, rest) = match url.split_once("://") {
        Some((s, r)) => (s.to_ascii_lowercase(), r),
        None => ("https".to_string(), url),
    };
    let authority = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    let host = authority.rsplit_once(':').map_or(authority, |(h, _)| h);
    format!("{scheme}://{}", registrable_site(host))
}

/// RFC 6265 §5.1.4 default-path: strip the last segment of the request path.
pub fn default_path(request_path: &str) -> String {
    if request_path.is_empty() || !request_path.starts_with('/') {
        return "/".to_string();
    }
    match request_path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(idx) => request_path[..idx].to_string(),
        None => "/".to_string(),
    }
}

/// RFC 6265 §5.1.4 path-match with the directory-boundary rule:
/// `/foo` matches `/foo/bar` but never `/foobar`.
pub fn path_match(request_path: &str, cookie_path: &str) -> bool {
    if request_path == cookie_path {
        return true;
    }
    if !request_path.starts_with(cookie_path) {
        return false;
    }
    request_path[cookie_path.len()..].starts_with('/') || cookie_path.ends_with('/')
}

/// RFC 6265 §5.1.3 domain-match restricted by host-only flag.
pub fn domain_match(request_host: &str, cookie_domain: &str, host_only: bool) -> bool {
    let req = request_host.to_ascii_lowercase();
    let dom = cookie_domain.to_ascii_lowercase();
    if host_only {
        return req == dom;
    }
    req == dom || (req.ends_with(&format!(".{dom}")) && req.parse::<std::net::IpAddr>().is_err())
}

/// Parse a raw Set-Cookie line against the request context.
/// Returns None when policy rejects the cookie outright:
/// oversized lines, SameSite=None without Secure, or a Domain attribute that
/// does not domain-match the origin host (including bare-TLD attempts).
pub fn parse_set_cookie(
    request_host: &str,
    request_path: &str,
    header: &str,
) -> Option<CookieRecord> {
    if header.len() > MAX_COOKIE_LINE_BYTES {
        plog!(
            "COOKIE",
            "Rejected Set-Cookie over {} bytes",
            MAX_COOKIE_LINE_BYTES
        );
        return None;
    }
    let trimmed = header.trim();
    let (name_value, attr_str) = match trimmed.find(';') {
        Some(pos) => (&trimmed[..pos], &trimmed[pos + 1..]),
        None => (trimmed, ""),
    };
    let (name, value) = name_value.split_once('=')?;
    let name = name.trim();
    let value = value.trim();
    if name.is_empty() || value.is_empty() {
        return None;
    }

    let host = request_host.to_ascii_lowercase();

    // Attribute pass: match names case-insensitively, keep original slices
    // for values that stay case-sensitive (Path).
    let mut secure = false;
    let mut http_only = false;
    let mut ss_attr: Option<String> = None;
    let mut domain_attr: Option<String> = None;
    let mut path_attr: Option<String> = None;
    let mut max_age_attr: Option<i64> = None;
    let mut expires_attr: Option<String> = None;
    for part in attr_str.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let lower = part.to_ascii_lowercase();
        if lower == "secure" {
            secure = true;
        } else if lower == "httponly" {
            http_only = true;
        } else if let Some(v) = lower.strip_prefix("samesite=") {
            ss_attr = Some(v.to_string());
        } else if lower.strip_prefix("domain=").is_some() {
            domain_attr = Some(part[7..].trim().to_string());
        } else if lower.strip_prefix("path=").is_some() {
            path_attr = Some(part[5..].trim().to_string());
        } else if let Some(v) = lower.strip_prefix("max-age=") {
            max_age_attr = v.trim().parse::<i64>().ok();
        } else if lower.strip_prefix("expires=").is_some() {
            expires_attr = Some(part[8..].trim().to_string());
        }
    }

    let same_site = SameSite::from_attr(ss_attr.as_deref());

    if same_site == SameSite::NoneSamesite && !secure {
        plog!("COOKIE", "Rejected SameSite=None without Secure");
        return None;
    }

    let mut domain = host.clone();
    let mut host_only = true;
    if let Some(d) = domain_attr {
        let d = d.trim_start_matches('.').to_ascii_lowercase();
        if d.is_empty() {
            return None;
        }
        // Naive public-suffix guard: single-label Domain only valid when the
        // host itself is that single label (localhost-style).
        if !d.contains('.') && host.contains('.') {
            plog!("COOKIE", "Rejected bare-TLD Domain={}", d);
            return None;
        }
        let matches = host == d || host.ends_with(&format!(".{d}"));
        if !matches {
            plog!("COOKIE", "Rejected Domain={} for host {}", d, host);
            return None;
        }
        domain = d;
        host_only = false;
    }

    let path = match path_attr {
        Some(p) if p.starts_with('/') => p,
        _ => default_path(request_path),
    };

    let expires = max_age_attr
        .map(|secs| {
            if secs <= 0 {
                Instant::now() - std::time::Duration::from_secs(1)
            } else {
                Instant::now() + std::time::Duration::from_secs(secs as u64)
            }
        })
        .or_else(|| expires_attr.as_deref().and_then(parse_rfc1123_date_local));

    Some(CookieRecord {
        name: name.to_string(),
        value: value.to_string(),
        domain,
        host_only,
        path,
        secure,
        http_only,
        same_site,
        expires,
    })
}

/// SameSite send rule. `initiator_site` is None only for top-level document
/// navigations where no initiator site applies; Strict still requires the
/// navigation itself to be same-site.
pub fn same_site_allowed(
    same_site: SameSite,
    initiator_site: Option<&str>,
    target_url: &str,
    top_level_navigation: bool,
) -> bool {
    let same_site_of = |initiator: &str| schemeful_site(initiator) == schemeful_site(target_url);
    match same_site {
        SameSite::NoneSamesite => true,
        // No initiator means no cross-site relationship is knowable - the
        // filter stays out of the way rather than guessing.
        SameSite::Lax | SameSite::Strict if initiator_site.is_none() => true,
        SameSite::Strict => initiator_site.map(same_site_of).unwrap_or(false),
        SameSite::Lax => initiator_site.map(same_site_of).unwrap_or(false) || top_level_navigation,
    }
}

// ── Jar ─────────────────────────────────────────────────────────────────────

#[derive(Default)]
struct Jar(Vec<CookieRecord>);

fn jar() -> &'static RwLock<Jar> {
    static JAR: OnceLock<RwLock<Jar>> = OnceLock::new();
    JAR.get_or_init(|| RwLock::new(load_jar()))
}

fn load_jar() -> Jar {
    let data = match std::fs::read_to_string(COOKIE_FILE) {
        Ok(d) => d,
        Err(_) => return Jar(vec![]),
    };
    if let Ok(records) = serde_json::from_str::<Vec<CookieRecord>>(&data) {
        return Jar(records);
    }
    // Legacy format (pre-C2): origin-keyed nested map without domain/path.
    if let Ok(legacy) = serde_json::from_str::<HashMap<String, HashMap<String, LegacyAttrs>>>(&data)
    {
        let records = legacy
            .into_iter()
            .flat_map(|(origin_key, cookies)| {
                let host = legacy_origin_host(&origin_key);
                cookies.into_iter().map(move |(name, attrs)| CookieRecord {
                    name,
                    value: attrs.value,
                    domain: host.clone(),
                    host_only: true,
                    path: "/".to_string(),
                    secure: attrs.secure,
                    http_only: attrs.http_only,
                    same_site: SameSite::from_attr(Some(&attrs.same_site)),
                    expires: attrs.expires,
                })
            })
            .collect();
        return Jar(records);
    }
    Jar(vec![])
}

fn legacy_origin_host(origin_key: &str) -> String {
    origin_key
        .split_once("://")
        .map(|(_, rest)| rest.to_string())
        .unwrap_or_else(|| origin_key.to_string())
}

#[derive(Deserialize)]
struct LegacyAttrs {
    value: String,
    #[serde(default)]
    http_only: bool,
    #[serde(default)]
    secure: bool,
    #[serde(default)]
    same_site: String,
    #[serde(default, skip)]
    expires: Option<Instant>,
}

fn save_jar(jar: &Jar) {
    match serde_json::to_string(&jar.0) {
        Ok(json) => {
            if let Err(e) = std::fs::write(COOKIE_FILE, json) {
                plog!("COOKIE", "save failed: {}", e);
            }
        }
        Err(e) => {
            // Braces required: plog! expands to multiple statements.
            plog!("COOKIE", "serialize failed: {}", e);
        }
    }
}

/// Insert/replace keyed on (name, domain, host_only, path).
fn store_record(record: CookieRecord) {
    if let Ok(mut jar) = jar().write() {
        jar.0.retain(|c| {
            !(c.name == record.name
                && c.domain == record.domain
                && c.host_only == record.host_only
                && c.path == record.path)
        });
        jar.0.push(record);
    }
}

/// Called from fetch paths when a response carries Set-Cookie.
pub fn set_cookie_from_response(request_url: &str, header: &str) {
    let (request_host, request_path) = split_url_parts(request_url);
    let request_host = request_host
        .rsplit_once(':')
        .map_or(&request_host[..], |(h, _)| h);

    let record = match parse_set_cookie(request_host, &request_path, header) {
        Some(r) => r,
        None => return,
    };
    store_record(record);
    save_current_jar();
}

fn save_current_jar() {
    if let Ok(jar) = jar().read() {
        save_jar(&jar);
    }
}

/// Collect the Cookie header value for an outgoing request.
///
/// * `initiator_url` - referring page URL; None means a top-level document
///   navigation (SameSite=Lax may cross sites, Strict may not).
/// * Expired records are dropped opportunistically under the write lock.
pub fn get_cookies_for_request(
    request_url: &str,
    initiator_url: Option<&str>,
    top_level_navigation: bool,
) -> String {
    let now = Instant::now();
    let (host_port, request_path) = split_url_parts(request_url);
    let host = host_port
        .rsplit_once(':')
        .map_or(host_port.as_str(), |(h, _)| h);

    let mut pairs: Vec<(String, String)> = Vec::new();
    let expired_any;
    if let Ok(mut jar) = jar().write() {
        let before = jar.0.len();
        jar.0.retain(|c| !c.expired_at(now));
        expired_any = jar.0.len() != before;
        for c in &jar.0 {
            if c.expired_at(now) {
                continue;
            }
            if c.secure && !request_url.starts_with("https://") {
                continue;
            }
            if !domain_match(host, &c.domain, c.host_only) {
                continue;
            }
            if !path_match(&request_path, &c.path) {
                continue;
            }
            if !same_site_allowed(
                c.same_site,
                initiator_url,
                request_url,
                top_level_navigation,
            ) {
                continue;
            }
            pairs.push((c.name.clone(), c.value.clone()));
        }
        if expired_any {
            drop(jar);
            save_current_jar();
        }
    }
    pairs
        .into_iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// RFC 1123 `Expires=` dates, interpreted as local time (pre-C2 behavior kept).
fn parse_rfc1123_date_local(s: &str) -> Option<Instant> {
    let s = s.strip_suffix(" GMT")?;
    let (_wkday, rest) = s.split_once(", ")?;
    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let day = parts[0].parse::<u32>().ok()? as u64;
    let month = match parts[1] {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    } as u64;
    let year = parts[2].parse::<i64>().ok()?;
    if !(1970..=3000).contains(&year) {
        return None;
    }
    fn is_leap(y: i64) -> bool {
        y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
    }
    let year_diff = year - 1970;
    let leap_years_before = year_diff / 4 - year_diff / 100 + year_diff / 400;
    let month_days: [i64; 12] = [
        31,
        if is_leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if !(1..=12).contains(&month) {
        return None;
    }
    let days_since_epoch =
        year_diff * 365 + leap_years_before + month_days[(month - 1) as usize] + (day as i64) - 1;
    let secs = (days_since_epoch.max(0) as u64) * 86400;
    Some(Instant::now() + std::time::Duration::from_secs(secs))
}

fn split_url_parts(url: &str) -> (String, String) {
    let rest = match url.split_once("://") {
        Some((_, r)) => r,
        None => url,
    };
    let slash = rest.find('/').unwrap_or(rest.len());
    let host_port = rest[..slash].to_string();
    let path = if slash < rest.len() {
        rest[slash..].to_string()
    } else {
        "/".to_string()
    };
    (host_port, path)
}
