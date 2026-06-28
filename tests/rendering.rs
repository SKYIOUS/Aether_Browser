use aether_browser::engine::parser::Parser;
use aether_browser::engine::css;
use aether_browser::engine::dom::NodeType;
use aether_browser::ui::screens::browser::{BrowserScreen, BrowserMessage};

#[test]
fn test_basic_rendering_pipeline() {
    let html = r#"
        <div style="color: red; width: 100px; height: 50px;">
            <p>Hello World</p>
        </div>
    "#.to_string();

    let mut parser = Parser::new(html);
    let dom = parser.parse_node();

    if let NodeType::Element(ref data) = dom.node_type {
        assert_eq!(data.tag_name, "div");
    } else {
        panic!("Root should be an element");
    }
    assert_eq!(dom.children.len(), 1);

    let css_text = "div { display: block; } p { color: blue; }".to_string();
    let mut css_parser = css::Parser::new(css_text);
    let rules = css_parser.parse_rules();
    assert_eq!(rules.len(), 2);
}

#[test]
fn test_browser_screen_state() {
    let mut screen = BrowserScreen::new();
    assert_eq!(screen.private_mode, false);
    assert_eq!(screen.devtools_open, false);

    let _ = screen.update(BrowserMessage::TogglePrivate);
    assert_eq!(screen.private_mode, true);

    let _ = screen.update(BrowserMessage::ToggleDevTools);
    assert_eq!(screen.devtools_open, true);
}

#[test]
fn test_caelum_spatial_init() {
    use aether_browser::engine::caelum::style::Style;
    let style: Style<String> = Style::DEFAULT;
    assert_eq!(style.opacity, 1.0);
    assert_eq!(style.z_index, 0);
}
