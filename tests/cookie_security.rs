// C2: cookie security policy - pure-function matrix.
//
// Every rule here is deterministic: parse/match/send decisions with no I/O.
// Loopback integration lives in net_security.rs.

use vayu_browser::engine::net::cookies::{
    default_path, domain_match, path_match, parse_set_cookie, registrable_site, schemeful_site,
    same_site_allowed, SameSite,
};

fn set(host: &str, path: &str, header: &str) -> Option<vayu_browser::engine::net::cookies::CookieRecord> {
    parse_set_cookie(host, path, header)
}

// ── Set-time: Domain ────────────────────────────────────────────────────────

#[test]
fn c2_domain_attribute_enables_parent_domain_sharing() {
    let c = set("app.example.com", "/", "sid=1; Domain=example.com").expect("valid parent domain");
    assert_eq!(c.domain, "example.com");
    assert!(!c.host_only);
}

#[test]
fn c2_domain_mismatch_is_rejected_entirely() {
    assert!(set("127.0.0.1", "/", "sid=1; Domain=localhost").is_none());
    assert!(set("app.example.com", "/", "sid=1; Domain=other.com").is_none());
}

#[test]
fn c2_tld_only_domain_rejected() {
    // Naive public-suffix guard: a single-label Domain is never acceptable
    // unless the host itself is that single label (localhost-style).
    assert!(set("app.example.com", "/", "sid=1; Domain=com").is_none());
    assert!(set("localhost", "/", "sid=1; Domain=localhost").is_some());
}

#[test]
fn c2_no_domain_attr_means_host_only() {
    let c = set("example.com", "/", "sid=1").expect("host-only cookie");
    assert!(c.host_only);
    assert_eq!(c.domain, "example.com");
    assert!(
        !domain_match("sub.example.com", &c.domain, c.host_only),
        "host-only cookies must not leak to subdomains"
    );
}

// ── Set-time: SameSite / Secure / size ──────────────────────────────────────

#[test]
fn c2_samesite_none_requires_secure() {
    assert!(set("example.com", "/", "sid=1; SameSite=None").is_none(), "None without Secure must drop");
    assert!(set("example.com", "/", "sid=1; SameSite=None; Secure").is_some());
}

#[test]
fn c2_unspecified_samesite_defaults_to_lax_not_permissive() {
    let c = set("example.com", "/", "sid=1").expect("parse");
    assert_eq!(c.same_site, SameSite::Lax);
    let unknown = set("example.com", "/", "sid=1; SameSite=Bogus").expect("unknown falls back");
    assert_eq!(unknown.same_site, SameSite::Lax);
}

#[test]
fn c2_cookie_line_size_boundary_4096_exact() {
    // name= + value + attrs serialized as the exact Set-Cookie line.
    let value_4090 = "v".repeat(4090);
    let ok = format!("sid={value_4090}"); // len = 4 + 4090 = 4094... adjust below
    let _ = ok;
    let make = |value_len: usize| {
        let v = "v".repeat(value_len.saturating_sub(4));
        format!("sid={v}")
    };
    let at_limit = make(4096);
    assert_eq!(at_limit.len(), 4096, "fixture must sit exactly on the boundary");
    assert!(set("example.com", "/", &at_limit).is_some(), "4096 bytes accepted");
    let over = make(4097);
    assert_eq!(over.len(), 4097);
    assert!(set("example.com", "/", &over).is_none(), "4097 bytes rejected");
}

// ── Path handling ───────────────────────────────────────────────────────────

#[test]
fn c2_default_path_strips_last_segment() {
    assert_eq!(default_path("/a/b/c.html"), "/a/b");
    assert_eq!(default_path("/a/"), "/a");
    assert_eq!(default_path("/root"), "/");
    assert_eq!(default_path(""), "/");
}

#[test]
fn c2_path_match_honors_directory_boundaries() {
    assert!(path_match("/foo/bar", "/foo"), "/foo matches /foo/bar");
    assert!(!path_match("/foobar", "/foo"), "/foo must NOT match /foobar");
    assert!(path_match("/foo", "/foo"));
    assert!(path_match("/foo/", "/foo"), "trailing-slash request matches its own dir");
    assert!(path_match("/", "/"));
}

// ── Send matrix (SameSite x initiator x top-level) ─────────────────────────

#[test]
fn c2_send_matrix_full_table() {
    use SameSite::{Lax, Strict};
    let ss_none = SameSite::NoneSamesite;
    let cases: &[(SameSite, Option<&str>, bool, bool, &str)] = &[
        // (cookie, initiator_site, top_level_nav, expected_sent, why)
        (Lax, Some("https://site.com"), false, true, "same-site subresource"),
        (Lax, Some("https://evil.com"), false, false, "cross-site subresource"),
        (Lax, Some("https://evil.com"), true, true, "cross-site TOP-LEVEL nav"),
        (Strict, Some("https://site.com"), false, true, "same-site strict"),
        (Strict, Some("https://evil.com"), true, false, "strict blocked even on cross-site nav"),
        (ss_none, Some("https://evil.com"), false, true, "None travels anywhere (set-time Secure guard applies)"),
        (Lax, None, true, true, "document navigation without initiator"),
    ];
    for &(ss, initiator, top, expected, why) in cases {
        assert_eq!(
            same_site_allowed(ss, initiator.map(str::to_string).as_deref(), "https://site.com", top),
            expected,
            "{why}"
        );
    }
}

// ── Site calculation (naive registrable domain - documented limitation) ────

#[test]
fn c2_schemeful_site_uses_registrable_domain_and_scheme() {
    assert_eq!(schemeful_site("https://a.app.example.com:8443/x"), "https://example.com");
    assert_eq!(schemeful_site("http://EXAMPLE.com/"), "http://example.com");
    assert_eq!(schemeful_site("https://localhost:3000"), "https://localhost");
    assert_ne!(
        schemeful_site("https://example.com"),
        schemeful_site("http://example.com"),
        "cross-scheme sites differ"
    );
}
