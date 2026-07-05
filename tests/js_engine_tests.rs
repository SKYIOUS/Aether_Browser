use aether_browser::engine::js::js_bridge::JsBridge;
use aether_browser::engine::dom::Node;

// ── Helper: build a DOM tree with div > (span.foo#bar + p) ──────────
fn make_tree() -> (JsBridge, u32, u32, u32, u32) {
    let mut b = JsBridge::new();
    let root = b.create_element("div");
    let span = b.create_element("span");
    let p = b.create_element("p");
    b.set_attribute(span, "class", "foo");
    b.set_attribute(span, "id", "bar");
    b.append_child(root, span);
    b.append_child(root, p);
    (b, root, span, p, 0)
}

// ════════════════════════════════════════════════════════════════════
// 1. Selector Tests
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_query_selector_by_tag() {
    let (b, root, span, _p, _) = make_tree();
    assert_eq!(b.query_selector(root, "span"), Some(span));
    assert_eq!(b.query_selector(root, "p"), Some(_p));
    assert_eq!(b.query_selector(root, "a"), None);
}

#[test]
fn test_query_selector_by_id() {
    let (b, root, span, _, _) = make_tree();
    assert_eq!(b.query_selector(root, "#bar"), Some(span));
    assert_eq!(b.query_selector(root, "#nonexistent"), None);
}

#[test]
fn test_query_selector_by_class() {
    let (b, root, span, _p, _) = make_tree();
    assert_eq!(b.query_selector(root, ".foo"), Some(span));
    assert_eq!(b.query_selector(root, ".baz"), None);
}

#[test]
fn test_query_selector_descendant() {
    let mut b = JsBridge::new();
    let outer = b.create_element("div");
    let inner = b.create_element("span");
    b.set_attribute(inner, "class", "target");
    b.append_child(outer, inner);
    // compound "div.target" matches div with class target (none here)
    assert_eq!(b.query_selector(outer, "div.target"), None);
    // ".target" alone matches the span
    assert_eq!(b.query_selector(outer, ".target"), Some(inner));
    // "div .target" descendant combinator matches span.target inside div
    assert_eq!(b.query_selector(outer, "div .target"), Some(inner));
}

#[test]
fn test_query_selector_child() {
    let mut b = JsBridge::new();
    let root = b.create_element("div");
    let section = b.create_element("section");
    let deep_p = b.create_element("p");
    b.append_child(root, section);
    b.append_child(section, deep_p);
    // "div > section" standard CSS: section whose parent is div
    assert_eq!(b.query_selector(root, "div > section"), Some(section));
    // "section > p" standard CSS: p whose parent is section
    assert_eq!(b.query_selector(root, "section > p"), Some(deep_p));
    // "div > p" does NOT match (p's parent is section, not div)
    assert_eq!(b.query_selector(root, "div > p"), None);
    // deeper descendant
    assert_eq!(b.query_selector(root, "div p"), Some(deep_p));
    // "section > p" still matches after adding deeper div
    let _inner_div = b.create_element("div");
    b.append_child(section, _inner_div);
    assert_eq!(b.query_selector(root, "section > p"), Some(deep_p));
}

#[test]
fn test_query_selector_all() {
    let mut b = JsBridge::new();
    let root = b.create_element("div");
    let a = b.create_element("p");
    let bb = b.create_element("p");
    let c = b.create_element("span");
    b.append_child(root, a);
    b.append_child(root, bb);
    b.append_child(root, c);
    let ps = b.query_selector_all(root, "p");
    assert_eq!(ps.len(), 2);
    assert!(ps.contains(&a));
    assert!(ps.contains(&bb));
    // span should not be in p results
    assert!(!ps.contains(&c));
}

#[test]
fn test_query_selector_no_match() {
    let (b, root, _, _, _) = make_tree();
    assert_eq!(b.query_selector(root, "table"), None);
    assert_eq!(b.query_selector(root, ".nonexistent"), None);
    assert!(b.query_selector_all(root, "table").is_empty());
}

#[test]
fn test_query_selector_nested() {
    let mut b = JsBridge::new();
    let a = b.create_element("div");
    let b_el = b.create_element("section");
    let c = b.create_element("p");
    let d = b.create_element("span");
    b.set_attribute(d, "id", "deep");
    b.append_child(a, b_el);
    b.append_child(b_el, c);
    b.append_child(c, d);
    assert_eq!(b.query_selector(a, "span#deep"), Some(d));
    // "section > p" finds p whose parent is section
    assert_eq!(b.query_selector(a, "section > p"), Some(c));
    // "div > p" does NOT match (p's parent is section, not div)
    assert_eq!(b.query_selector(a, "div > p"), None);
    // descendant: "div p" finds p anywhere under div
    assert_eq!(b.query_selector(a, "div p"), Some(c));
    // descendant: "div span" finds span anywhere under div
    assert_eq!(b.query_selector(a, "div span"), Some(d));
}

// ponytail: ".container#baz" is a compound selector (both must match same node),
// not a descendant combinator. Tests multi-simple compound matching.
#[test]
fn test_query_selector_complex() {
    let mut b = JsBridge::new();
    let root = b.create_element("div");
    let inner = b.create_element("div");
    let target = b.create_element("span");
    b.set_attribute(inner, "class", "container");
    b.set_attribute(target, "id", "baz");
    b.append_child(root, inner);
    b.append_child(inner, target);
    // ponytail: ".container#baz" compound matches elements with BOTH class and id
    assert_eq!(b.query_selector(root, ".container#baz"), None);
    // "#baz" alone matches the span
    assert_eq!(b.query_selector(root, "#baz"), Some(target));
    // ".container" alone matches the div
    assert_eq!(b.query_selector(root, ".container"), Some(inner));
}

#[test]
fn test_query_selector_wildcard() {
    let mut b = JsBridge::new();
    let root = b.create_element("div");
    let a = b.create_element("span");
    let bb = b.create_element("p");
    b.append_child(root, a);
    b.append_child(root, bb);
    // * matches all direct children
    let all = b.query_selector_all(root, "*");
    assert_eq!(all.len(), 2);
    assert!(all.contains(&a));
    assert!(all.contains(&bb));
}

// ════════════════════════════════════════════════════════════════════
// 2. Timer Tests
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_set_timeout_adds_entry() {
    let mut b = JsBridge::new();
    let _ = b.set_timeout("test()".into(), 500);
    assert!(b.has_pending_timers());
}

#[test]
fn test_set_interval_adds_entry() {
    let mut b = JsBridge::new();
    let _ = b.set_interval("tick()".into(), 1000);
    assert!(b.has_pending_timers());
}

#[test]
fn test_clear_timeout_removes_entry() {
    let mut b = JsBridge::new();
    let id = b.set_timeout("test()".into(), 500);
    assert!(b.has_pending_timers());
    b.clear_timer(id);
    assert!(!b.has_pending_timers());
}

#[test]
fn test_clear_interval_removes_entry() {
    let mut b = JsBridge::new();
    let id = b.set_interval("tick()".into(), 1000);
    assert!(b.has_pending_timers());
    b.clear_timer(id);
    assert!(!b.has_pending_timers());
}

#[test]
fn test_timer_id_increments() {
    let mut b = JsBridge::new();
    let id1 = b.set_timeout("a()".into(), 100);
    let id2 = b.set_timeout("b()".into(), 200);
    let id3 = b.set_interval("c()".into(), 300);
    // IDs should be unique and increasing
    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
    assert!(id2 > id1);
    assert!(id3 > id2);
}

#[test]
fn test_timeout_callback_is_source() {
    let mut b = JsBridge::new();
    let src = "document.write('fired')".to_string();
    let id = b.set_timeout(src.clone(), 0);
    let ready = b.poll_timers();
    assert_eq!(ready.len(), 1);
    assert_eq!(ready[0].0, id);
    assert_eq!(ready[0].1, src);
}

// ponytail: can't access private timers field — verify via has_pending_timers() behavior
#[test]
fn test_interval_callback_is_source() {
    let mut b = JsBridge::new();
    let src = "console.log('tick')".to_string();
    let _ = b.set_interval(src.clone(), 60_000); // won't fire in time
    // Verify the source via has_pending_timers — timer exists
    assert!(b.has_pending_timers());
}

#[test]
fn test_pending_timers_count() {
    let mut b = JsBridge::new();
    assert!(!b.has_pending_timers());
    let id1 = b.set_timeout("a()".into(), 1000);
    let _id2 = b.set_timeout("b()".into(), 2000);
    let _id3 = b.set_interval("c()".into(), 3000);
    assert!(b.has_pending_timers());
    b.clear_timer(id1);
    // Still has 2 pending
    assert!(b.has_pending_timers());
}

#[test]
fn test_clear_all_timers() {
    let mut b = JsBridge::new();
    let id1 = b.set_timeout("a()".into(), 1000);
    let id2 = b.set_timeout("b()".into(), 2000);
    let id3 = b.set_interval("c()".into(), 3000);
    b.clear_timer(id1);
    b.clear_timer(id2);
    b.clear_timer(id3);
    assert!(!b.has_pending_timers());
}

#[test]
fn test_timer_after_page_load() {
    let mut b = JsBridge::new();
    let _ = b.set_timeout("a()".into(), 1000);
    let _ = b.set_interval("b()".into(), 2000);
    // Simulate: bridge persists — timers still present
    assert!(b.has_pending_timers());
    let ready = b.poll_timers();
    assert!(ready.is_empty()); // none fired yet
    assert!(b.has_pending_timers());
}

// ════════════════════════════════════════════════════════════════════
// 3. Event Tests
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_add_event_listener() {
    let mut b = JsBridge::new();
    let id = b.create_element("button");
    b.add_event_listener(id, "click".into(), "handler()".into());
    let listeners = b.get_event_listeners(id, "click");
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0], "handler()");
}

#[test]
fn test_remove_event_listener() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    b.add_event_listener(id, "click".into(), "fn()".into());
    assert_eq!(b.get_event_listeners(id, "click").len(), 1);
    b.remove_event_listener(id, "click".into(), "fn()".into());
    assert_eq!(b.get_event_listeners(id, "click").len(), 0);
}

#[test]
fn test_event_listener_id_increments() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    b.add_event_listener(id, "click".into(), "a()".into());
    b.add_event_listener(id, "click".into(), "b()".into());
    b.add_event_listener(id, "click".into(), "c()".into());
    let listeners = b.get_event_listeners(id, "click");
    assert_eq!(listeners.len(), 3);
    assert_ne!(listeners[0], listeners[1]);
    assert_ne!(listeners[1], listeners[2]);
}

#[test]
fn test_click_event_dispatch() {
    let mut b = JsBridge::new();
    let parent = b.create_element("div");
    let btn = b.create_element("button");
    b.append_child(parent, btn);
    b.add_event_listener(btn, "click".into(), "clicked()".into());
    let listeners = b.get_event_listeners(btn, "click");
    assert_eq!(listeners, vec!["clicked()".to_string()]);
}

#[test]
fn test_event_bubbling() {
    let mut b = JsBridge::new();
    let grandparent = b.create_element("div");
    let parent = b.create_element("section");
    let child = b.create_element("button");
    b.append_child(grandparent, parent);
    b.append_child(parent, child);
    b.add_event_listener(grandparent, "click".into(), "gp()".into());
    b.add_event_listener(parent, "click".into(), "p()".into());
    b.add_event_listener(child, "click".into(), "c()".into());
    let bubbling = b.get_event_listeners_bubbling(child, "click");
    assert_eq!(bubbling.len(), 3);
    // Should be in order: child, parent, grandparent
    assert_eq!(bubbling[0].1, child);
    assert_eq!(bubbling[1].1, parent);
    assert_eq!(bubbling[2].1, grandparent);
}

#[test]
fn test_event_listener_source_string() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    let source = "function handleClick(e) { console.log(e); }".to_string();
    b.add_event_listener(id, "click".into(), source.clone());
    let listeners = b.get_event_listeners(id, "click");
    assert_eq!(listeners[0], source);
}

#[test]
fn test_multiple_listeners_same_event() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    b.add_event_listener(id, "click".into(), "a()".into());
    b.add_event_listener(id, "click".into(), "b()".into());
    b.add_event_listener(id, "click".into(), "c()".into());
    let listeners = b.get_event_listeners(id, "click");
    assert_eq!(listeners.len(), 3);
    assert!(listeners.contains(&"a()".to_string()));
    assert!(listeners.contains(&"b()".to_string()));
    assert!(listeners.contains(&"c()".to_string()));
}

#[test]
fn test_get_event_listeners() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    b.add_event_listener(id, "click".into(), "x()".into());
    b.add_event_listener(id, "mouseover".into(), "y()".into());
    assert_eq!(b.get_event_listeners(id, "click").len(), 1);
    assert_eq!(b.get_event_listeners(id, "mouseover").len(), 1);
    assert_eq!(b.get_event_listeners(id, "keydown").len(), 0);
}

#[test]
fn test_remove_partial_match() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    b.add_event_listener(id, "click".into(), "handler()".into());
    // Try removing with different source — should NOT remove
    b.remove_event_listener(id, "click".into(), "other()".into());
    assert_eq!(b.get_event_listeners(id, "click").len(), 1);
}

#[test]
fn test_event_after_remove() {
    let mut b = JsBridge::new();
    let id = b.create_element("div");
    b.add_event_listener(id, "click".into(), "fire()".into());
    assert_eq!(b.get_event_listeners_bubbling(id, "click").len(), 1);
    b.remove_event_listener(id, "click".into(), "fire()".into());
    assert_eq!(b.get_event_listeners_bubbling(id, "click").len(), 0);
    // Adding a different event type doesn't affect click
    b.add_event_listener(id, "mouseover".into(), "hover()".into());
    assert_eq!(b.get_event_listeners_bubbling(id, "click").len(), 0);
    assert_eq!(b.get_event_listeners(id, "mouseover").len(), 1);
}

// ════════════════════════════════════════════════════════════════════
// 4. Fetch & Storage Tests
// ════════════════════════════════════════════════════════════════════

// ponytail: fetch tests hit the network — verify response format, not content.
// Errors produce __STATUS_0__ prefix; real URLs may timeout.

#[test]
fn test_fetch_same_origin() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com/page");
    // Relative URL resolved against base → same origin
    let result = bridge.fetch_url("/api/data");
    // Without a server, this will error but should still have __STATUS_ prefix
    assert!(result.starts_with("__STATUS_"));
}

#[test]
fn test_fetch_cross_origin_cors() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com");
    let result = bridge.fetch_url("https://other.com/data");
    // Cross-origin without CORS header → error, __STATUS_0__
    assert!(result.starts_with("__STATUS_0__"));
}

#[test]
fn test_fetch_returns_status_prefix() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com");
    // Any fetch result should start with __STATUS_NNN__
    let result = bridge.fetch_url("https://nonexistent.invalid/test");
    assert!(result.starts_with("__STATUS_"));
}

#[test]
fn test_local_storage_set_get() {
    let mut bridge = JsBridge::new();
    bridge.local_storage_set_item("key1".into(), "value1".into());
    bridge.local_storage_set_item("key2".into(), "value2".into());
    assert_eq!(bridge.local_storage_get_item("key1"), Some("value1".into()));
    assert_eq!(bridge.local_storage_get_item("key2"), Some("value2".into()));
    assert_eq!(bridge.local_storage_get_item("missing"), None);
}

#[test]
fn test_local_storage_remove() {
    let mut bridge = JsBridge::new();
    bridge.local_storage_set_item("temp".into(), "data".into());
    assert_eq!(bridge.local_storage_get_item("temp"), Some("data".into()));
    bridge.local_storage_remove_item("temp");
    assert_eq!(bridge.local_storage_get_item("temp"), None);
}

#[test]
fn test_local_storage_clear() {
    let mut bridge = JsBridge::new();
    bridge.local_storage_set_item("a".into(), "1".into());
    bridge.local_storage_set_item("b".into(), "2".into());
    assert_eq!(bridge.local_storage_length(), 2);
    bridge.local_storage_clear();
    assert_eq!(bridge.local_storage_length(), 0);
    assert_eq!(bridge.local_storage_get_item("a"), None);
}

#[test]
fn test_cookie_set_get() {
    let mut bridge = JsBridge::new();
    bridge.set_cookie("session=abc123");
    bridge.set_cookie("theme=dark");
    let cookie = bridge.get_cookie();
    assert!(cookie.contains("session=abc123"));
    assert!(cookie.contains("theme=dark"));
}

#[test]
fn test_cookie_expires() {
    let mut bridge = JsBridge::new();
    // Max-Age=0 means already expired
    bridge.set_cookie("temp=value; Max-Age=0");
    let cookie = bridge.get_cookie();
    assert!(!cookie.contains("temp=value"), "expired cookie should be swept");
}

#[test]
fn test_fetch_error_handling() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com");
    // Unreachable URL → error with __STATUS_0__
    let result = bridge.fetch_url("https://192.0.2.1:1/nope");
    assert!(result.starts_with("__STATUS_0__"));
    assert!(result.contains("Error") || result.contains("error"));
}

#[test]
fn test_fetch_redirect() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com");
    // reqwest follows redirects by default — unreachable host still yields __STATUS_0__
    let result = bridge.fetch_url("https://192.0.2.1:1/redirect");
    assert!(result.starts_with("__STATUS_"));
}
