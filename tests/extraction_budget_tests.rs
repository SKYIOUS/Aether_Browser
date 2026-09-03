use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::pipeline::extractor::extract_elements;
use vayu_browser::engine::stratus::{CustomPropertyMap, Stylesheet};

fn extract(html: &str) -> Vec<vayu_browser::engine::pipeline::extractor::StyledElement> {
    let dom = parse_html(html);
    let sheet = Stylesheet { rules: vec![] };
    let mut elements = Vec::new();
    extract_elements(
        &dom,
        &mut elements,
        0,
        &sheet,
        None,
        None,
        vec![],
        800.0,
        600.0,
        &CustomPropertyMap::new(),
        None,
    );
    elements
}

// Regression guard for PLAN A1: the old 2000-element cap silently dropped
// content on long pages. The budget is a safety ceiling far above real
// page sizes, not a fidelity target � a normal long article must extract whole.
#[test]
fn extraction_passes_old_element_cap() {
    let mut html = String::from("<html><body>");
    for i in 0..2500 {
        html.push_str(&format!("<p>paragraph {} with some body text</p>", i));
    }
    html.push_str("</body></html>");

    let elements = extract(&html);
    assert!(
        elements.len() > 2000,
        "expected more than the old 2000-element cap, got {}",
        elements.len()
    );
    let last = elements.last().expect("elements extracted");
    assert_eq!(last.text, "paragraph 2499 with some body text");
}

// Old depth cap (50) cut off deeply nested DOM trees. Budget raises the
// ceiling; content nested past the old limit must survive.
#[test]
fn extraction_passes_old_depth_cap() {
    const DEPTH: usize = 60;
    let mut html = String::from("<html><body>");
    for _ in 0..DEPTH {
        html.push_str("<div><section>");
    }
    html.push_str("<p>deepest marker</p>");
    for _ in 0..DEPTH {
        html.push_str("</section></div>");
    }
    html.push_str("</body></html>");

    let elements = extract(&html);
    assert!(
        elements.iter().any(|e| e.text == "deepest marker"),
        "text at nesting depth {} was dropped",
        DEPTH
    );
}

// Text nodes over the old 5000-char limit were dropped entirely. Uses a <div>
// because some tags (<p>, headings, links) consume their text in the tag
// handler and never hit the loose-Text-node guard.
#[test]
fn text_node_over_old_limit_is_kept() {
    let big = "x".repeat(6000);
    let html = format!("<html><body><div>{}</div></body></html>", big);

    let elements = extract(&html);
    assert!(
        elements.iter().any(|e| e.text.len() >= 5000),
        "6000-char text node should be kept in full"
    );
}

// A document whose interesting content sits past the old 1MB HTML boundary
// must be reachable end-to-end through parse + extract. The filler pushes both
// byte size and element count over the old caps so this fails on pre-A1 code.
#[test]
fn content_beyond_old_1mb_boundary_is_extracted() {
    let filler_item =
        "<p>lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod</p>";
    let mut html = String::from("<html><body><p class=\"early\">early-marker</p>");
    // ~78 bytes x 14000 � 1.1MB total; element count also far past the old 2000 cap
    for _ in 0..14000 {
        html.push_str(filler_item);
    }
    html.push_str("<p class=\"late\">late-marker-past-1mb</p></body></html>");
    assert!(html.len() > 1_000_000, "doc must exceed the old 1MB cap");

    let elements = extract(&html);
    assert!(elements.iter().any(|e| e.text == "early-marker"));
    assert!(
        elements.iter().any(|e| e.text == "late-marker-past-1mb"),
        "content past the old 1MB / 2000-element boundary was dropped"
    );
}
