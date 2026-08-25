// C1: redirect authority + TLS backend tests.
//
// Loopback-only: a scripted TcpListener serves deterministic hops so redirect
// semantics are tested offline. Server-side request/header captures prove WHO
// owns following - before Policy::none(), reqwest auto-followed internally
// and our manual loop (downgrade guard, per-hop cookies/CSP/CORS) never ran.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use vayu_browser::engine::net;

type Route = (&'static str, u16, Vec<(&'static str, String)>, &'static str);

struct Server {
    base: String,
    seen: Arc<AtomicUsize>,
    /// Full request header block for each connection, in arrival order.
    requests: Arc<Mutex<Vec<String>>>,
}

/// Serves each route exactly once on an ephemeral port. One thread per conn.
fn spawn_server(routes: Vec<Route>) -> Server {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("local addr").port();
    let seen = Arc::new(AtomicUsize::new(0));
    let requests: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let (t_seen, t_requests) = (Arc::clone(&seen), Arc::clone(&requests));
    std::thread::spawn(move || {
        for route in routes {
            let (conn, _) = match listener.accept() {
                Ok(pair) => pair,
                Err(_) => break,
            };
            t_seen.fetch_add(1, Ordering::SeqCst);
            handle_conn(conn, &route, &t_requests);
        }
    });
    Server { base: format!("http://127.0.0.1:{port}"), seen, requests }
}

impl Server {
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base, path)
    }
    fn count(&self) -> usize {
        self.seen.load(Ordering::SeqCst)
    }
    /// Headers of the Nth request (0-based), or empty string if absent.
    fn request_headers(&self, n: usize) -> String {
        self.requests
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(n)
            .cloned()
            .unwrap_or_default()
    }
}

fn handle_conn(mut conn: TcpStream, route: &Route, log: &Mutex<Vec<String>>) {
    let mut reader = BufReader::new(&conn);
    let mut block = String::new();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                if line == "\r\n" || line == "\n" {
                    break;
                }
                block.push_str(&line);
            }
        }
    }
    log.lock().unwrap_or_else(|e| e.into_inner()).push(block);

    let (path, status, headers, body) = route;
    // Route sanity: the connection we're answering must be for our path.
    let served_for = log.lock().unwrap_or_else(|e| e.into_inner()).last().cloned().unwrap_or_default();
    debug_assert!(served_for.starts_with(&format!("GET {path} ")), "route order mismatch: {served_for}");

    let reason = match status {
        200 => "OK",
        301 => "Moved Permanently",
        302 => "Found",
        _ => "OK",
    };
    let mut resp =
        format!("HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nConnection: close\r\n", body.len());
    for (k, v) in headers {
        resp.push_str(&format!("{k}: {v}\r\n"));
    }
    resp.push_str("\r\n");
    write_response(&mut conn, &resp, body);
}

fn write_response(conn: &mut TcpStream, head: &str, body: &str) {
    let _ = conn.write_all(head.as_bytes());
    let _ = conn.write_all(body.as_bytes());
    let _ = conn.flush();
}

#[test]
fn c1_redirect_cap_returns_terminal_302_and_stops_the_chain() {
    let server = spawn_server(vec![
        ("/start", 302, vec![("Location", String::from("/mid"))], ""),
        ("/mid", 302, vec![("Location", String::from("/end"))], ""),
        ("/end", 200, vec![], "done"),
    ]);
    let cl = net::build_http_client().expect("client");
    let resp = net::fetch_redirects_with_client(&cl, &server.url("/start"), 1, None, None, false)
        .expect("capped chain must terminate with the in-flight 3xx");
    assert_eq!(resp.status, 302);
    assert!(resp.final_url.ends_with("/mid"), "terminal url should be /mid, got {}", resp.final_url);
    assert_eq!(server.count(), 2, "exactly start+mid requested; /end must never be fetched");
}

#[test]
fn c1_full_chain_follows_to_completion() {
    let server = spawn_server(vec![
        ("/start", 302, vec![("Location", String::from("/mid"))], ""),
        ("/mid", 302, vec![("Location", String::from("/end"))], ""),
        ("/end", 200, vec![], "done"),
    ]);
    let cl = net::build_http_client().expect("client");
    let resp = net::fetch_redirects_with_client(&cl, &server.url("/start"), 5, None, None, true)
        .expect("chain within cap must succeed");
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, "done");
    assert!(resp.final_url.ends_with("/end"));
    assert_eq!(server.count(), 3);
}

// Before redirect authority moved into fetch_inner, reqwest auto-followed
// hops itself and our hand-rolled jar was never consulted between them -
// cookies set on hop A silently vanished for hop B.
#[test]
fn c1_cookies_are_reattached_on_every_redirect_hop() {
    let server = spawn_server(vec![
        ("/a", 302, vec![("Location", "/b".to_string()), ("Set-Cookie", String::from("sid=1"))], ""),
        ("/b", 200, vec![], "ok"),
    ]);
    let resp = net::fetch_with_redirects(&server.url("/a"), 5, None, None, false)
        .expect("hop chain succeeds");
    assert_eq!(resp.status, 200);
    assert_eq!(server.count(), 2);
    let hop_b = server.request_headers(1);
    assert!(
        hop_b.to_lowercase().contains("cookie: sid=1"),
        "hop B must receive the cookie set on hop A; got headers:\n{hop_b}"
    );
}

// The scheme-downgrade rejection must happen at the redirect DECISION point,
// before any second request could be issued. redirect_target is that gate.
#[test]
fn c1_https_to_http_downgrade_rejected_at_decision_point() {
    assert_eq!(
        net::redirect_target("https://example.com/a", "http://example.com/b"),
        None,
        "https->http must be refused outright"
    );
    assert_eq!(
        net::redirect_target("https://example.com/a", "/b"),
        Some("https://example.com/b".to_string()),
        "same-scheme relative redirects resolve normally"
    );
    assert_eq!(
        net::redirect_target("http://example.com/a", "https://example.com/b"),
        Some("https://example.com/b".to_string()),
        "upgrades are allowed"
    );
}

// ?? C2 cookie security (loopback) ??????????????????????????????????????????

fn header_of(server: &Server, n: usize) -> String {
    server.request_headers(n)
}

#[test]
fn c2_domain_mismatch_cookie_is_never_sent() {
    let cl = net::build_http_client().expect("client");
    let server = spawn_server(vec![
        ("/set", 200, vec![("Set-Cookie", "sid=9; Domain=localhost".to_string())], ""),
        ("/check", 200, vec![], "ok"),
    ]);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/set"), 0, None, None, false);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/check"), 0, None, None, false);
    let sent = header_of(&server, 1);
    assert!(
        !sent.to_lowercase().contains("sid=9"),
        "cookie with mismatched Domain must not be stored or sent; got:\n{sent}"
    );
}

#[test]
fn c2_secure_cookie_never_sent_over_http() {
    let cl = net::build_http_client().expect("client");
    let server = spawn_server(vec![
        ("/set", 200, vec![("Set-Cookie", String::from("sec=1; Secure"))], ""),
        ("/check", 200, vec![], "ok"),
    ]);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/set"), 0, None, None, false);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/check"), 0, None, None, false);
    let sent = header_of(&server, 1);
    assert!(
        !sent.to_lowercase().contains("sec=1"),
        "Secure cookie must not travel over http; got:\n{sent}"
    );
}

#[test]
fn c2_oversized_set_cookie_is_dropped() {
    let cl = net::build_http_client().expect("client");
    let big: &'static str = Box::leak(format!("big={}; Path=/", "x".repeat(4200)).into_boxed_str());
    let server = spawn_server(vec![
        ("/set", 200, vec![("Set-Cookie", big.to_string())], ""),
        ("/check", 200, vec![], "ok"),
    ]);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/set"), 0, None, None, false);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/check"), 0, None, None, false);
    let sent = header_of(&server, 1);
    assert!(!sent.contains("big="), "oversized cookie must be dropped");
}

#[test]
fn c2_path_boundary_foo_matches_bar_not_foobar() {
    let cl = net::build_http_client().expect("client");
    let server = spawn_server(vec![
        ("/foo/set", 200, vec![("Set-Cookie", String::from("p=1; Path=/foo"))], ""),
        ("/foo/bar", 200, vec![], "in-path"),
        ("/foobar", 200, vec![], "outside"),
    ]);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/foo/set"), 0, None, None, false);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/foo/bar"), 0, None, None, false);
    let _ = net::fetch_redirects_with_client(&cl, &server.url("/foobar"), 0, None, None, false);
    let in_scope = header_of(&server, 1);
    let out_scope = header_of(&server, 2);
    assert!(in_scope.contains("p=1"), "Path=/foo must cover /foo/bar");
    assert!(!out_scope.contains("p=1"), "Path=/foo must not cover /foobar");
}

// ?? C3 CSP resource authority ??????????????????????????????????????????????

use vayu_browser::engine::net::{parse_csp, ResourceKind};

fn style_policy_self(page: &str) -> () {
    // store_csp persists per-origin; helper keeps test intent readable.
    let mut headers = std::collections::HashMap::new();
    headers.insert("content-security-policy".to_string(), "style-src 'self'".to_string());
    net::store_csp(page, &headers);
}

// Characterization: freeze existing csp_allows_* semantics before authority
// moves behind fetch_resource. These pass against current code by design.
#[test]
fn c3_characterization_style_src_self_semantics() {
    let policy = parse_csp("style-src 'self'");
    assert!(net::csp_allows_style_url("https://site.com/a.css", "https://site.com", &policy));
    assert!(!net::csp_allows_style_url("https://evil.com/a.css", "https://site.com", &policy));
    assert!(
        net::csp_allows_style_url("https://any.com/a.css", "https://site.com", &parse_csp("")),
        "empty policy allows everything"
    );
    assert!(
        !net::csp_allows_style_url("https://any.com/a.css", "https://site.com", &parse_csp("style-src 'none'")),
    );
}

#[test]
fn c3_characterization_directive_fallback_to_default_src() {
    // effective_sources_for falls back to default-src when style-src absent.
    let policy = parse_csp("default-src 'self'");
    assert!(!net::csp_allows_style_url("https://evil.com/a.css", "https://site.com", &policy));
    assert!(net::csp_allows_style_url("https://site.com/a.css", "https://site.com", &policy));
}

#[test]
fn c3_direct_disallowed_host_blocked_before_connect() {
    style_policy_self("http://127.0.0.1:1");
    let cl = net::build_http_client().expect("client");
    let server = spawn_server(vec![("/evil.css", 200, vec![], "evil{}")]);
    let err = match net::fetch_resource_with_client(&cl, &server.url("/evil.css"), "http://127.0.0.1:1", ResourceKind::Style, None, false) {
        Err(e) => e,
        Ok(r) => panic!("disallowed style host must be blocked, got status {}", r.status),
    };
    let _ = err.to_string(); // Display exercised
    assert_eq!(server.count(), 0, "blocked resource must never be fetched");
}

#[test]
fn c3_redirect_hop_into_disallowed_host_is_blocked_and_evil_body_never_consumed() {
    // evil.example/style.css lives on a second listener (different port =
    // different origin), so 'self' on the page origin cannot cover it.
    let evil = spawn_server(vec![("/evil.css", 200, vec![], "evil{}")]);
    let allowed = spawn_server(vec![(
        "/style.css",
        302,
        vec![("Location", format!("{}{}", evil.base, "/evil.css"))],
        "",
    )]);

    let page = &allowed.base;
    style_policy_self(page);

    let cl = net::build_http_client().expect("client");
    let err = match net::fetch_resource_with_client(&cl, &allowed.url("/style.css"), page, ResourceKind::Style, None, false) {
        Err(e) => e,
        Ok(r) => panic!("redirect into disallowed host must be blocked, got status {} from {}", r.status, r.final_url),
    };

    assert!(err.to_string().to_lowercase().contains("csp") || err.to_string().to_lowercase().contains("policy"),
        "error should identify the CSP block, got: {err}");
    assert_eq!(evil.count(), 0, "no request may reach the disallowed host");
    assert_eq!(allowed.count(), 1, "chain stops after the violating hop");
}

// Inverse case: an allowed -> allowed redirect keeps loading.
#[test]
fn c3_allowed_to_allowed_redirect_still_loads() {
    let server = spawn_server(vec![
        ("/a.css", 302, vec![("Location", "/b.css".to_string())], ""),
        ("/b.css", 200, vec![], ".b{}"),
    ]);
    let page = &server.base;
    style_policy_self(page);

    let cl = net::build_http_client().expect("client");
    let resp = net::fetch_resource_with_client(&cl, &server.url("/a.css"), page, ResourceKind::Style, None, false)
        .expect("same-site redirect chain must load");
    assert_eq!(resp.body, ".b{}");
    assert_eq!(server.count(), 2);
}

// ?? D0 mock latency knob ???????????????????????????????????????????????????

#[test]
fn d0_mock_delay_applies_per_resolved_request() {
    use std::time::Instant;
    vayu_browser::engine::net::mock::set_mock(
        vayu_browser::engine::net::mock::MockHttpResponder::new()
            .delay_ms(120)
            .html("mock://doc", "<p>x</p>"),
    );
    let start = Instant::now();
    let first = vayu_browser::engine::net::fetch("mock://doc", None);
    let first_elapsed = start.elapsed();

    // Second fetch hits the HTTP cache above the mock: no second delay.
    let second = vayu_browser::engine::net::fetch("mock://doc", None);

    vayu_browser::engine::net::mock::clear_mock();
    assert!(first.is_ok() && second.is_ok());
    assert!(first_elapsed.as_millis() >= 120, "first fetch must pay the mock delay");
}
