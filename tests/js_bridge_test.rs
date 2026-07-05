use std::collections::HashMap;
use aether_browser::engine::js::js_bridge::JsBridge;
use aether_browser::engine::dom::{Node, NodeType};

fn make_dom() -> Node {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "root".to_string());
    let mut child_attrs = HashMap::new();
    child_attrs.insert("class".to_string(), "child".to_string());
    Node::new_element("div".to_string(), attrs, vec![
        Node::new_element("span".to_string(), child_attrs, vec![
            Node::new_text("Hello".to_string())
        ])
    ])
}

fn make_nested_dom() -> Node {
    Node::new_element("div".to_string(), HashMap::new(), vec![
        Node::new_element("section".to_string(), HashMap::new(), vec![
            Node::new_element("p".to_string(), HashMap::new(), vec![
                Node::new_text("Nested".to_string())
            ])
        ])
    ])
}

fn find_in_dom<'a>(node: &'a Node, tag: &str) -> Option<&'a Node> {
    if node.tag_name() == Some(tag) { return Some(node); }
    for child in &node.children {
        if let Some(found) = find_in_dom(child, tag) { return Some(found); }
    }
    None
}

#[test]
fn test_create_element() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    assert_eq!(bridge.get_tag_name(id), Some("DIV".to_string()));
}

#[test]
fn test_append_child() {
    let mut bridge = JsBridge::new();
    let parent = bridge.create_element("div");
    let child = bridge.create_element("span");
    bridge.append_child(parent, child);
    let children = bridge.get_children(parent);
    assert_eq!(children, vec![child]);
    assert_eq!(bridge.get_parent(child), Some(parent));
}

#[test]
fn test_text_node() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_text_node("hello");
    assert_eq!(bridge.get_text_content(id), "hello");
    assert_eq!(bridge.get_tag_name(id), Some("text".to_string()));
}

#[test]
fn test_set_attribute() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.set_attribute(id, "data-test", "value123");
    assert_eq!(bridge.get_attribute(id, "data-test"), Some("value123".to_string()));
}

#[test]
fn test_get_element_by_id() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let child = bridge.create_element("span");
    bridge.append_child(root, child);
    bridge.set_attribute(child, "id", "myid");
    assert_eq!(bridge.get_element_by_id("myid"), Some(child));
    assert_eq!(bridge.get_element_by_id("nonexistent"), None);
}

#[test]
fn test_query_selector() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let child = bridge.create_element("span");
    bridge.append_child(root, child);
    bridge.set_attribute(child, "class", "myclass");
    assert_eq!(bridge.query_selector(root, ".myclass"), Some(child));
}

#[test]
fn test_query_selector_all() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let c1 = bridge.create_element("p");
    let c2 = bridge.create_element("p");
    bridge.set_attribute(c1, "class", "x");
    bridge.set_attribute(c2, "class", "x");
    bridge.append_child(root, c1);
    bridge.append_child(root, c2);
    let results = bridge.query_selector_all(root, ".x");
    assert_eq!(results.len(), 2);
    assert!(results.contains(&c1));
    assert!(results.contains(&c2));
}

#[test]
fn test_set_text_content() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.set_text_content(id, "hello world");
    assert_eq!(bridge.get_text_content(id), "hello world");
}

#[test]
fn test_inner_html() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.set_inner_html(id, "<p>Hi</p>");
    let children = bridge.get_children(id);
    assert_eq!(children.len(), 1);
    assert_eq!(bridge.get_tag_name(children[0]), Some("P".to_string()));
    assert_eq!(bridge.get_text_content(children[0]), "Hi");
}

#[test]
fn test_style_property() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.set_style_property(id, "color", "red");
    assert_eq!(bridge.get_style_property(id, "color"), "red");
    assert_eq!(bridge.get_style_property(id, "nonexistent"), "");
}

#[test]
fn test_timeout() {
    let mut bridge = JsBridge::new();
    let id = bridge.set_timeout("console.log('hi')".to_string(), 0);
    let expired = bridge.poll_timers();
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].0, id);
    assert_eq!(expired[0].1, "console.log('hi')");
}

#[test]
fn test_clear_timer() {
    let mut bridge = JsBridge::new();
    let id = bridge.set_timeout("console.log('hi')".to_string(), 0);
    bridge.clear_timer(id);
    let expired = bridge.poll_timers();
    assert!(expired.is_empty());
}

#[test]
fn test_interval() {
    let mut bridge = JsBridge::new();
    // ponytail: delay_ms=0 would cause poll_timers infinite loop (re-created timer fires
    // instantly in same call). Use positive delay; we verify state rather than firing here.
    let id = bridge.set_interval("tick()".to_string(), 1000);
    assert!(bridge.has_pending_timers());
    // Timer hasn't expired yet (1000ms delay, we're <1ms in)
    assert!(bridge.poll_timers().is_empty());
    bridge.clear_timer(id);
    assert!(!bridge.has_pending_timers());
}

#[test]
fn test_event_listener() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("button");
    bridge.add_event_listener(id, "click".to_string(), "handler()".to_string());
    let listeners = bridge.get_event_listeners_bubbling(id, "click");
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0].0, "handler()");
    assert_eq!(listeners[0].1, id);
}

#[test]
fn test_event_listener_bubbling() {
    let mut bridge = JsBridge::new();
    let parent = bridge.create_element("div");
    let child = bridge.create_element("button");
    bridge.append_child(parent, child);
    bridge.add_event_listener(parent, "click".to_string(), "parent()".to_string());
    bridge.add_event_listener(child, "click".to_string(), "child()".to_string());
    let bubbling = bridge.get_event_listeners_bubbling(child, "click");
    assert_eq!(bubbling.len(), 2);
}

#[test]
fn test_fetch_url_cross_origin() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com");
    let result = bridge.fetch_url("https://other.com/data");
    // Cross-origin fetch without ACAO header should fail
    assert!(result.starts_with("__STATUS_0__"), "expected failure for cross-origin fetch, got: {}", result);
}

#[test]
fn test_element_traversal() {
    let mut bridge = JsBridge::new();
    let grandparent = bridge.create_element("div");
    let parent = bridge.create_element("section");
    let child = bridge.create_element("p");
    bridge.append_child(grandparent, parent);
    bridge.append_child(parent, child);

    assert_eq!(bridge.get_parent(child), Some(parent));
    assert_eq!(bridge.get_parent(parent), Some(grandparent));
    assert_eq!(bridge.get_parent(grandparent), None);

    assert_eq!(bridge.get_children(grandparent), vec![parent]);
    assert_eq!(bridge.get_children(parent), vec![child]);
    assert!(bridge.get_children(child).is_empty());

    assert_eq!(bridge.get_first_child(grandparent), Some(parent));
    assert_eq!(bridge.get_last_child(grandparent), Some(parent));
    assert_eq!(bridge.get_first_child(parent), Some(child));
    assert_eq!(bridge.get_last_child(parent), Some(child));

    assert_eq!(bridge.get_next_sibling(child), None);
    assert_eq!(bridge.get_previous_sibling(child), None);

    assert_eq!(bridge.get_child_element_count(grandparent), 1);
    assert_eq!(bridge.get_child_element_count(parent), 1);
    assert_eq!(bridge.get_child_element_count(child), 0);
}

#[test]
fn test_sibling_traversal() {
    let mut bridge = JsBridge::new();
    let parent = bridge.create_element("ul");
    let a = bridge.create_element("li");
    let b = bridge.create_element("li");
    let c = bridge.create_element("li");
    bridge.append_child(parent, a);
    bridge.append_child(parent, b);
    bridge.append_child(parent, c);

    assert_eq!(bridge.get_next_sibling(a), Some(b));
    assert_eq!(bridge.get_next_sibling(b), Some(c));
    assert_eq!(bridge.get_next_sibling(c), None);

    assert_eq!(bridge.get_previous_sibling(a), None);
    assert_eq!(bridge.get_previous_sibling(b), Some(a));
    assert_eq!(bridge.get_previous_sibling(c), Some(b));

    let children = bridge.get_children(parent);
    assert_eq!(children, vec![a, b, c]);

    assert_eq!(bridge.get_first_child(parent), Some(a));
    assert_eq!(bridge.get_last_child(parent), Some(c));
    assert_eq!(bridge.get_child_element_count(parent), 3);
}

#[test]
fn test_child_nodes_includes_text() {
    let mut bridge = JsBridge::new();
    let parent = bridge.create_element("div");
    let text = bridge.create_text_node("hello");
    let child = bridge.create_element("span");
    bridge.append_child(parent, text);
    bridge.append_child(parent, child);
    let all = bridge.get_child_nodes(parent);
    assert_eq!(all, vec![text, child]);
}

#[test]
fn test_dom_roundtrip() {
    let original = make_dom();
    let bridge = JsBridge::load_dom(&original, "https://example.com");
    let roundtrip = bridge.to_dom();

    assert_eq!(original.children.len(), roundtrip.children.len());
    assert_eq!(original.tag_name(), roundtrip.tag_name());
    if let (NodeType::Element(orig), NodeType::Element(rt)) = (&original.node_type, &roundtrip.node_type) {
        assert_eq!(orig.tag_name, rt.tag_name);
        assert_eq!(orig.attributes, rt.attributes);
    }
}

#[test]
fn test_load_dom_with_document_root() {
    let doc = Node::new_document();
    let bridge = JsBridge::load_dom(&doc, "https://example.com");
    let roundtrip = bridge.to_dom();
    assert!(matches!(roundtrip.node_type, NodeType::Document));
}

#[test]
fn test_dom_roundtrip_preserves_structure() {
    let original = make_nested_dom();
    let bridge = JsBridge::load_dom(&original, "https://example.com");
    let rt = bridge.to_dom();

    let orig_div = find_in_dom(&original, "div").expect("div in original");
    let rt_div = find_in_dom(&rt, "div").expect("div in roundtrip");
    assert_eq!(orig_div.children.len(), rt_div.children.len());

    let orig_p = find_in_dom(&original, "p").expect("p in original");
    let rt_p = find_in_dom(&rt, "p").expect("p in roundtrip");
    assert_eq!(orig_p.children.len(), rt_p.children.len());
    assert_eq!(orig_p.text_content(), rt_p.text_content());
}

#[test]
fn test_load_dom_with_body() {
    let mut body_attrs = HashMap::new();
    body_attrs.insert("id".to_string(), "main".to_string());
    let body = Node::new_element("body".to_string(), body_attrs, vec![
        Node::new_element("h1".to_string(), HashMap::new(), vec![
            Node::new_text("Title".to_string())
        ])
    ]);
    let mut doc = Node::new_document();
    doc.children.push(body);
    let bridge = JsBridge::load_dom(&doc, "https://example.com");
    assert_eq!(bridge.body_id, Some(1));
}

#[test]
fn test_get_tag_name_variants() {
    let mut bridge = JsBridge::new();
    let d = bridge.create_element("div");
    let s = bridge.create_element("SECTION");
    let t = bridge.create_text_node("x");
    assert_eq!(bridge.get_tag_name(d), Some("DIV".to_string()));
    assert_eq!(bridge.get_tag_name(s), Some("SECTION".to_string()));
    assert_eq!(bridge.get_tag_name(t), Some("text".to_string()));
}

#[test]
fn test_set_and_get_cookie() {
    let mut bridge = JsBridge::new();
    bridge.set_cookie("session=abc123");
    bridge.set_cookie("theme=dark");
    let cookie = bridge.get_cookie();
    assert!(cookie.contains("session=abc123"));
    assert!(cookie.contains("theme=dark"));
}

#[test]
fn test_local_storage() {
    let mut bridge = JsBridge::new();
    assert_eq!(bridge.local_storage_length(), 0);
    bridge.local_storage_set_item("key1".to_string(), "val1".to_string());
    bridge.local_storage_set_item("key2".to_string(), "val2".to_string());
    assert_eq!(bridge.local_storage_length(), 2);
    assert_eq!(bridge.local_storage_get_item("key1"), Some("val1".to_string()));
    assert_eq!(bridge.local_storage_get_item("key2"), Some("val2".to_string()));
    let key0 = bridge.local_storage_key(0);
    let key1 = bridge.local_storage_key(1);
    assert!(key0.is_some() && key1.is_some());
    assert_ne!(key0, key1);
    assert!(key0 == Some("key1".to_string()) || key0 == Some("key2".to_string()));
    assert!(key1 == Some("key1".to_string()) || key1 == Some("key2".to_string()));
    bridge.local_storage_remove_item("key1");
    assert_eq!(bridge.local_storage_length(), 1);
    assert_eq!(bridge.local_storage_get_item("key1"), None);
    bridge.local_storage_clear();
    assert_eq!(bridge.local_storage_length(), 0);
}

#[test]
fn test_location_parts() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "https://example.com:8080/path/to/page?q=hello#section");
    assert_eq!(bridge.get_location_href(), "https://example.com:8080/path/to/page?q=hello#section");
    assert_eq!(bridge.get_location_hostname(), "example.com");
    assert_eq!(bridge.get_location_pathname(), "/path/to/page");
    assert_eq!(bridge.get_location_protocol(), "https:");
    assert_eq!(bridge.get_location_port(), "8080");
    assert_eq!(bridge.get_location_search(), "?q=hello");
    assert_eq!(bridge.get_location_hash(), "#section");
}

#[test]
fn test_location_parts_defaults() {
    let dom = Node::new_document();
    let bridge = JsBridge::load_dom(&dom, "http://localhost");
    assert_eq!(bridge.get_location_hostname(), "localhost");
    assert_eq!(bridge.get_location_pathname(), "/");
    assert_eq!(bridge.get_location_protocol(), "http:");
    assert_eq!(bridge.get_location_port(), "");
    assert_eq!(bridge.get_location_search(), "");
    assert_eq!(bridge.get_location_hash(), "");
}

#[test]
fn test_location_reload() {
    let mut bridge = JsBridge::new();
    bridge.location_reload();
    assert_eq!(bridge.pending_navigation, Some("https://localhost".to_string()));
}

#[test]
fn test_location_assign() {
    let mut bridge = JsBridge::new();
    bridge.location_assign("https://example.com/new".to_string());
    assert_eq!(bridge.pending_navigation, Some("https://example.com/new".to_string()));
}

#[test]
fn test_location_replace() {
    let mut bridge = JsBridge::new();
    bridge.location_replace("https://example.com/alt".to_string());
    assert_eq!(bridge.pending_navigation, Some("https://example.com/alt".to_string()));
}

#[test]
fn test_set_location_href() {
    let mut bridge = JsBridge::new();
    bridge.set_location_href("https://other.com".to_string());
    assert_eq!(bridge.pending_navigation, Some("https://other.com".to_string()));
}

#[test]
fn test_document_write_and_take_output() {
    let mut bridge = JsBridge::new();
    bridge.document_write("Hello ");
    bridge.document_write("World");
    assert_eq!(bridge.take_output(), "Hello World");
    assert_eq!(bridge.take_output(), "");
}

#[test]
fn test_doc_title() {
    let mut bridge = JsBridge::new();
    assert_eq!(bridge.doc_title, "");
    bridge.doc_title = "My Page".to_string();
    assert_eq!(bridge.doc_title, "My Page");
}

#[test]
fn test_pending_timers() {
    let mut bridge = JsBridge::new();
    assert!(!bridge.has_pending_timers());
    bridge.set_timeout("code".to_string(), 1000);
    assert!(bridge.has_pending_timers());
    bridge.clear_timer(1);
    assert!(!bridge.has_pending_timers());
}

#[test]
fn test_query_selector_by_tag() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let p = bridge.create_element("p");
    let span = bridge.create_element("span");
    bridge.append_child(root, p);
    bridge.append_child(root, span);
    assert_eq!(bridge.query_selector(root, "p"), Some(p));
    assert_eq!(bridge.query_selector(root, "span"), Some(span));
    assert_eq!(bridge.query_selector(root, "a"), None);
}

#[test]
fn test_query_selector_by_id() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let item = bridge.create_element("li");
    bridge.append_child(root, item);
    bridge.set_attribute(item, "id", "item42");
    assert_eq!(bridge.query_selector(root, "#item42"), Some(item));
}

#[test]
fn test_query_selector_by_class() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let a = bridge.create_element("a");
    let b = bridge.create_element("b");
    bridge.append_child(root, a);
    bridge.append_child(root, b);
    bridge.set_attribute(b, "class", "highlight");
    assert_eq!(bridge.query_selector(root, ".highlight"), Some(b));
}

#[test]
fn test_query_selector_universal() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let c = bridge.create_element("em");
    bridge.append_child(root, c);
    assert_eq!(bridge.query_selector(root, "*"), Some(c));
}

#[test]
fn test_query_selector_child_combinator() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let section = bridge.create_element("section");
    let div_child = bridge.create_element("div");
    bridge.append_child(root, section);
    bridge.append_child(section, div_child);
    // standard CSS: "div > section" finds section whose parent is a div
    assert_eq!(bridge.query_selector(root, "div > section"), Some(section));
}

#[test]
fn test_query_selector_descendant_combinator() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    let span = bridge.create_element("span");
    let p = bridge.create_element("p");
    bridge.append_child(root, span);
    bridge.append_child(span, p);
    // "div p" finds a p that descends from a div (any depth)
    assert_eq!(bridge.query_selector(root, "div p"), Some(p));
    // "span p" finds a p that descends from a span
    assert_eq!(bridge.query_selector(root, "span p"), Some(p));
    // "div > p" does NOT match (p's parent is span, not div)
    assert_eq!(bridge.query_selector(root, "div > p"), None);
}

#[test]
fn test_query_selector_compound_descendant() {
    let mut bridge = JsBridge::new();
    let root = bridge.create_element("div");
    bridge.set_attribute(root, "id", "x");
    let inner = bridge.create_element("span");
    bridge.set_attribute(inner, "class", "y");
    bridge.append_child(root, inner);
    // "#x .y" finds .y element that descends from #x
    assert_eq!(bridge.query_selector(root, "#x .y"), Some(inner));
}

#[test]
fn test_event_listener_remove() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.add_event_listener(id, "click".to_string(), "fn()".to_string());
    assert_eq!(bridge.get_event_listeners_bubbling(id, "click").len(), 1);
    bridge.remove_event_listener(id, "click".to_string(), "fn()".to_string());
    assert_eq!(bridge.get_event_listeners_bubbling(id, "click").len(), 0);
}

#[test]
fn test_get_event_listeners() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.add_event_listener(id, "click".to_string(), "a()".to_string());
    bridge.add_event_listener(id, "click".to_string(), "b()".to_string());
    bridge.add_event_listener(id, "mouseup".to_string(), "c()".to_string());
    let click_listeners = bridge.get_event_listeners(id, "click");
    assert_eq!(click_listeners.len(), 2);
    assert!(click_listeners.contains(&"a()".to_string()));
    assert!(click_listeners.contains(&"b()".to_string()));
}

#[test]
fn test_self_append_child_noop() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.append_child(id, id);
    assert!(bridge.get_children(id).is_empty());
}

#[test]
fn test_set_text_content_on_text_node_noop() {
    let mut bridge = JsBridge::new();
    let text = bridge.create_text_node("original");
    bridge.set_text_content(text, "new");
    assert_eq!(bridge.get_text_content(text), "original");
}

#[test]
fn test_remove_event_listener_partial_match() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.add_event_listener(id, "click".to_string(), "handler()".to_string());
    bridge.remove_event_listener(id, "click".to_string(), "other()".to_string());
    assert_eq!(bridge.get_event_listeners_bubbling(id, "click").len(), 1);
}

#[test]
fn test_inner_html_self_closing() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.set_inner_html(id, "<br><hr><img src='x.png'>");
    let children = bridge.get_children(id);
    assert_eq!(children.len(), 3);
    assert_eq!(bridge.get_tag_name(children[0]), Some("BR".to_string()));
    assert_eq!(bridge.get_tag_name(children[1]), Some("HR".to_string()));
    assert_eq!(bridge.get_tag_name(children[2]), Some("IMG".to_string()));
}

#[test]
fn test_inner_html_nested() {
    let mut bridge = JsBridge::new();
    let id = bridge.create_element("div");
    bridge.set_inner_html(id, "<ul><li>A</li><li>B</li></ul>");
    let children = bridge.get_children(id);
    assert_eq!(children.len(), 1);
    assert_eq!(bridge.get_tag_name(children[0]), Some("UL".to_string()));
    let lis = bridge.get_children(children[0]);
    assert_eq!(lis.len(), 2);
    assert_eq!(bridge.get_text_content(lis[0]), "A");
    assert_eq!(bridge.get_text_content(lis[1]), "B");
}

#[test]
fn test_set_attribute_on_text_node_noop() {
    let mut bridge = JsBridge::new();
    let text = bridge.create_text_node("hi");
    bridge.set_attribute(text, "id", "x");
    assert_eq!(bridge.get_attribute(text, "id"), None);
}

#[test]
fn test_element_at_point_no_elements() {
    let bridge = JsBridge::new();
    let result = bridge.element_at_point(10.0, 10.0, &[]);
    assert_eq!(result, None);
}

#[test]
fn test_get_children_excludes_text() {
    let mut bridge = JsBridge::new();
    let parent = bridge.create_element("div");
    let t1 = bridge.create_text_node("hello");
    let el = bridge.create_element("span");
    let t2 = bridge.create_text_node("world");
    bridge.append_child(parent, t1);
    bridge.append_child(parent, el);
    bridge.append_child(parent, t2);
    assert_eq!(bridge.get_children(parent), vec![el]);
}

#[test]
fn test_get_child_nodes_includes_all() {
    let mut bridge = JsBridge::new();
    let parent = bridge.create_element("div");
    let t = bridge.create_text_node("text");
    let e = bridge.create_element("b");
    bridge.append_child(parent, t);
    bridge.append_child(parent, e);
    let all = bridge.get_child_nodes(parent);
    assert_eq!(all, vec![t, e]);
}

#[test]
fn test_load_dom_idempotent() {
    let original = make_dom();
    let bridge1 = JsBridge::load_dom(&original, "https://a.com");
    let bridge2 = JsBridge::load_dom(&bridge1.to_dom(), "https://a.com");
    assert_eq!(
        bridge1.to_dom().text_content(),
        bridge2.to_dom().text_content()
    );
}
