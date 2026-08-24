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

type Route = (&'static str, u16, Vec<(&'static str, &'static str)>, &'static str);

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
        ("/start", 302, vec![("Location", "/mid")], ""),
        ("/mid", 302, vec![("Location", "/end")], ""),
        ("/end", 200, vec![], "done"),
    ]);
    let cl = net::build_http_client().expect("client");
    let resp = net::fetch_redirects_with_client(&cl, &server.url("/start"), 1, None)
        .expect("capped chain must terminate with the in-flight 3xx");
    assert_eq!(resp.status, 302);
    assert!(resp.final_url.ends_with("/mid"), "terminal url should be /mid, got {}", resp.final_url);
    assert_eq!(server.count(), 2, "exactly start+mid requested; /end must never be fetched");
}

#[test]
fn c1_full_chain_follows_to_completion() {
    let server = spawn_server(vec![
        ("/start", 302, vec![("Location", "/mid")], ""),
        ("/mid", 302, vec![("Location", "/end")], ""),
        ("/end", 200, vec![], "done"),
    ]);
    let cl = net::build_http_client().expect("client");
    let resp = net::fetch_redirects_with_client(&cl, &server.url("/start"), 5, None)
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
        ("/a", 302, vec![("Location", "/b"), ("Set-Cookie", "sid=1")], ""),
        ("/b", 200, vec![], "ok"),
    ]);
    let resp = net::fetch_with_redirects(&server.url("/a"), 5, None).expect("hop chain succeeds");
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
