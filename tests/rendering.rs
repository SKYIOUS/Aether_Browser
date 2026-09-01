use vayu_browser::engine::dom::NodeType;
use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::stratus;

#[test]
fn test_basic_rendering_pipeline() {
    let html = r#"
        <div style="color: red; width: 100px; height: 50px;">
            <p>Hello World</p>
        </div>
    "#
    .to_string();

    let dom = parse_html(&html);

    let html_elem = dom
        .children
        .iter()
        .find(|c| c.tag_name() == Some("html"))
        .expect("should find html");
    let body = html_elem
        .children
        .iter()
        .find(|c| c.tag_name() == Some("body"))
        .expect("should find body");
    let div = body
        .children
        .iter()
        .find(|c| c.tag_name() == Some("div"))
        .expect("should find div");

    if let NodeType::Element(ref data) = div.node_type {
        assert_eq!(data.tag_name, "div");
    } else {
        panic!("Root should be an element");
    }
    assert_eq!(body.children.len(), 3);
    let _div = body
        .children
        .iter()
        .find(|c| c.tag_name() == Some("div"))
        .expect("should find div");

    let css_text = "div { display: block; } p { color: blue; }".to_string();
    let stylesheet = stratus::parse(&css_text);
    assert_eq!(stylesheet.rules.len(), 2);
}

#[test]
fn test_taffy_spatial_init() {
    use taffy::Style;
    let style: Style = Style::DEFAULT;
    assert_eq!(style.display, taffy::Display::Flex);
    assert_eq!(style.position, taffy::Position::Relative);
}
