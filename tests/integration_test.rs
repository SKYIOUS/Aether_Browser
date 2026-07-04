use aether_browser::engine::parser::Parser;
use aether_browser::engine::dom::NodeType;

#[test]
fn test_parsing_div_with_paragraph() {
    let html = String::from("<div><p>Hello, world!</p></div>");
    let mut parser = Parser::new(html);
    let doc = parser.parse_node();

    assert!(matches!(doc.node_type, NodeType::Element(_)));
    assert_eq!(doc.tag_name(), Some("div"));
    assert_eq!(doc.children.len(), 1);

    let p = &doc.children[0];
    assert!(matches!(p.node_type, NodeType::Element(_)));
    assert_eq!(p.tag_name(), Some("p"));
    assert_eq!(p.children.len(), 1);
    assert!(matches!(p.children[0].node_type, NodeType::Text(_)));

    let text = doc.text_content();
    assert_eq!(text, "Hello, world!");
}

#[test]
fn test_parsing_multiple_elements() {
    let html = String::from("<div><h1>Title</h1><p>Content here.</p></div>");
    let mut parser = Parser::new(html);
    let doc = parser.parse_node();

    assert_eq!(doc.tag_name(), Some("div"));
    assert_eq!(doc.children.len(), 2);
    assert_eq!(doc.children[0].tag_name(), Some("h1"));
    assert_eq!(doc.children[1].tag_name(), Some("p"));

    let text = doc.text_content();
    assert!(text.contains("Title"));
    assert!(text.contains("Content here."));
}

#[test]
fn test_should_skip_tag_filters() {
    use aether_browser::engine::pipeline::extractor::should_skip_tag;
    assert!(should_skip_tag("script"));
    assert!(should_skip_tag("style"));
    assert!(!should_skip_tag("div"));
    assert!(!should_skip_tag("p"));
    assert!(!should_skip_tag("img"));
}

#[test]
fn test_extract_and_layout_pipeline() {
    use aether_browser::engine::parser::Parser;
    use aether_browser::engine::stratus;
    use aether_browser::engine::pipeline::extractor::extract_elements;
    use aether_browser::engine::pipeline::layout::apply_caelum_layout;

    let html = r#"
        <div class="container">
            <p id="first">Hello</p>
            <p class="highlight">World</p>
        </div>
    "#.to_string();

    let css = r#"
        .container { display: block; width: 800px; background-color: #fff; }
        p { display: block; color: #333; font-size: 16px; margin-top: 8px; margin-bottom: 8px; }
        .highlight { color: red; font-weight: bold; }
    "#.to_string();

    let mut parser = Parser::new(html);
    let dom = parser.parse_node();
    let stylesheet = stratus::parse(&css);

    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &stylesheet, None, None, vec![], 800.0, 600.0);

    assert!(!elements.is_empty(), "should extract elements");

    let container = elements.iter().find(|e| e.tag == "div");
    assert!(container.is_some(), "should find div");

    let first_p = elements.iter().find(|e| e.tag == "p" && e.text.contains("Hello"));
    assert!(first_p.is_some(), "should find first <p>");
    let first_p = first_p.unwrap();
    assert_eq!(first_p.color, iced::Color::from_rgb(0x33 as f32 / 255.0, 0x33 as f32 / 255.0, 0x33 as f32 / 255.0));

    let highlight = elements.iter().find(|e| e.tag == "p" && e.text.contains("World"));
    assert!(highlight.is_some(), "should find highlighted <p>");
    let highlight = highlight.unwrap();
    let red = iced::Color::from_rgb(1.0, 0.0, 0.0);
    assert!((highlight.color.r - red.r).abs() < 0.01, "highlight color should be red");
    assert_eq!(highlight.font_weight, "bold");

    let max_el = elements.len().min(2000);
    apply_caelum_layout(&mut elements[..max_el], 800.0, 600.0);

    let container = elements.iter().find(|e| e.tag == "div").unwrap();
    assert!(container.width > 0.0, "container should have width");
    assert!(container.height > 0.0, "container should have height");
    let first_p = elements.iter().find(|e| e.tag == "p" && e.text.contains("Hello")).unwrap();
    assert!(first_p.y > container.y || (first_p.y - container.y).abs() < 0.001, "p.y should be >= container.y");
}

#[test]
fn test_inner_html_strips_script_tags() {
    use aether_browser::engine::js::js_bridge::JsBridge;
    use aether_browser::engine::dom::Node;

    let dom = Node::new_element("div".to_string(), [("id", "x")].into_iter().map(|(k,v)|(k.to_string(),v.to_string())).collect(), vec![]);
    let mut bridge = JsBridge::load_dom(&dom, "about:blank");
    let root = bridge.get_element_by_id("x").unwrap();

    bridge.set_inner_html(root, "<script>alert('xss')</script><p>safe</p>");

    let children = bridge.get_child_nodes(root);
    // script elements are fully stripped
    let script = children.iter().find(|&&id| {
        bridge.get_tag_name(id).map(|t| t == "SCRIPT").unwrap_or(false)
    });
    assert!(script.is_none(), "script tag should be stripped from innerHTML");
    // <p> should be present
    assert!(children.iter().any(|&id| {
        bridge.get_tag_name(id).map(|t| t == "P").unwrap_or(false)
    }), "<p> should remain after set_inner_html");
    let p_id = children.iter().find(|&&id| {
        bridge.get_tag_name(id).map(|t| t == "P").unwrap_or(false)
    }).unwrap();
    let p_text = bridge.get_text_content(*p_id);
    assert_eq!(p_text.trim(), "safe");
}

#[test]
fn test_set_attribute_rejects_event_handlers() {
    use aether_browser::engine::js::js_bridge::JsBridge;
    use aether_browser::engine::dom::Node;

    let dom = Node::new_element("div".to_string(), [("id", "x")].into_iter().map(|(k,v)|(k.to_string(),v.to_string())).collect(), vec![]);
    let mut bridge = JsBridge::load_dom(&dom, "about:blank");
    let root = bridge.get_element_by_id("x").unwrap();

    bridge.set_attribute(root, "onclick", "alert(1)");
    assert_eq!(bridge.get_attribute(root, "onclick"), None, "onclick should be rejected");

    bridge.set_attribute(root, "ONLOAD", "evil()");
    assert_eq!(bridge.get_attribute(root, "onload"), None, "case-variant onload should be rejected");
    assert_eq!(bridge.get_attribute(root, "ONLOAD"), None, "case-variant onload should be rejected by key");

    bridge.set_attribute(root, "href", "javascript:alert(1)");
    assert_eq!(bridge.get_attribute(root, "href"), None, "javascript: href should be rejected");

    bridge.set_attribute(root, "class", "safe");
    assert_eq!(bridge.get_attribute(root, "class"), Some("safe".into()), "class should be allowed");
}

#[test]
fn test_set_attribute_rejects_srcdoc() {
    use aether_browser::engine::js::js_bridge::JsBridge;
    use aether_browser::engine::dom::Node;

    let dom = Node::new_element("div".to_string(), [("id", "x")].into_iter().map(|(k,v)|(k.to_string(),v.to_string())).collect(), vec![]);
    let mut bridge = JsBridge::load_dom(&dom, "about:blank");
    let root = bridge.get_element_by_id("x").unwrap();

    bridge.set_attribute(root, "srcdoc", "<script>alert(1)</script>");
    assert_eq!(bridge.get_attribute(root, "srcdoc"), None, "srcdoc should be rejected");
}

