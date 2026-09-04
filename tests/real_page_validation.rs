use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::pipeline::apply_taffy_layout;
use vayu_browser::engine::pipeline::extractor::extract_elements;
use vayu_browser::engine::stratus;
use vayu_browser::engine::stratus::{CustomPropertyMap, Display, FlexWrap};

fn pipeline(
    html: &str,
    css: &str,
) -> Vec<vayu_browser::engine::pipeline::extractor::StyledElement> {
    let dom = parse_html(html);
    let stylesheet = stratus::parse(css);
    let mut elements = Vec::new();
    extract_elements(
        &dom,
        &mut elements,
        0,
        &stylesheet,
        None,
        None,
        vec![],
        800.0,
        600.0,
        &CustomPropertyMap::new(),
        None,
    );
    let count = elements.len().min(2000);
    apply_taffy_layout(&mut elements[..count], 800.0, 600.0);
    elements
}

fn flex_containers(
    elements: &[vayu_browser::engine::pipeline::extractor::StyledElement],
) -> Vec<usize> {
    elements
        .iter()
        .enumerate()
        .filter(|(_, e)| e.display == Display::Flex)
        .map(|(i, _)| i)
        .collect()
}

// ---------------------------------------------------------------------------
// 1. Percentage-width nested containers
// ---------------------------------------------------------------------------

#[test]
fn pct_nested_containers_width() {
    let html = r#"
        <div class="outer">
            <div class="inner">
                <p>content</p>
            </div>
        </div>
    "#;
    let css = r#"
        .outer { width: 600px; }
        .inner { width: 50%; }
    "#;
    let els = pipeline(html, css);
    // Find the outer div (direct child of body with explicit width)
    let outer_idx = els
        .iter()
        .position(|e| e.tag == "div" && e.css_width == Some(600.0));
    assert!(outer_idx.is_some(), "should find outer div");
    // Find the inner div (child of outer)
    let inner_idx = els
        .iter()
        .position(|e| e.tag == "div" && e.parent_index == Some(outer_idx.unwrap()));
    assert!(inner_idx.is_some(), "should find inner div");
    let inner = &els[inner_idx.unwrap()];
    let tolerance = 30.0;
    assert!(
        (inner.width - 300.0).abs() < tolerance,
        "inner div width should be ~300px (50% of 600), got {}",
        inner.width
    );
}

#[test]
fn pct_nested_containers_three_levels() {
    let html = r#"
        <div class="l1">
            <div class="l2">
                <div class="l3">
                    <p>deep</p>
                </div>
            </div>
        </div>
    "#;
    let css = r#"
        .l1 { width: 800px; }
        .l2 { width: 75%; }
        .l3 { width: 50%; }
    "#;
    let els = pipeline(html, css);
    // l1 = explicit 800px, l2 = 75% = 600px, l3 = 50% of l2 = 300px
    // Find l1 (800px), then l2 (child of l1), then l3 (child of l2)
    let l1_idx = els
        .iter()
        .position(|e| e.tag == "div" && e.css_width == Some(800.0));
    assert!(l1_idx.is_some(), "should find l1 div");
    let l2_idx = els
        .iter()
        .position(|e| e.tag == "div" && e.parent_index == Some(l1_idx.unwrap()));
    assert!(l2_idx.is_some(), "should find l2 div");
    let l3_idx = els
        .iter()
        .position(|e| e.tag == "div" && e.parent_index == Some(l2_idx.unwrap()));
    assert!(l3_idx.is_some(), "should find l3 div");
    let l3 = &els[l3_idx.unwrap()];
    // l2 = 75% of 800 = 600. l3 = 50% of 600 = 300.
    let tolerance = 40.0;
    assert!(
        (l3.width - 300.0).abs() < tolerance,
        "l3 width should be ~300px (50% of 75% of 800), got {}",
        l3.width
    );
}

// ---------------------------------------------------------------------------
// 2. Flex rows with gap
// ---------------------------------------------------------------------------

#[test]
fn flex_row_gap_basic() {
    let html = r#"
        <div class="container">
            <div class="a">A</div>
            <div class="b">B</div>
            <div class="c">C</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; gap: 20px; width: 800px; }
        .a, .b, .c { width: 200px; height: 50px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    // Find child divs of the flex container
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(
        children.len(),
        3,
        "should have 3 flex children. got {}: {:?}",
        children.len(),
        els.iter()
            .map(|e| (&e.tag, e.parent_index, e.display))
            .collect::<Vec<_>>()
    );
    let tolerance = 15.0;
    let (_, a) = &children[0];
    let (_, b) = &children[1];
    assert!(
        (b.x - a.x - a.width - 20.0).abs() < tolerance,
        "gap between A and B should be ~20px. A.x={}, A.w={}, B.x={}, gap={}",
        a.x,
        a.width,
        b.x,
        b.x - a.x - a.width
    );
}

// ---------------------------------------------------------------------------
// 3. justify-content + gap
// ---------------------------------------------------------------------------

#[test]
fn flex_justify_center_gap() {
    let html = r#"
        <div class="container">
            <div class="a">A</div>
            <div class="b">B</div>
            <div class="c">C</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; justify-content: center; gap: 10px; width: 800px; }
        .a, .b, .c { width: 100px; height: 50px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 3, "should have 3 flex children");
    let _tolerance = 30.0;
    let (_, a) = &children[0];
    // Total content: 300 + 20 gap = 320. Center in 800: offset = 240
    assert!(
        a.x > 100.0,
        "items should not be flush-left with center justify. A.x={}",
        a.x
    );
}

#[test]
fn flex_justify_space_between_gap() {
    let html = r#"
        <div class="container">
            <div class="a">A</div>
            <div class="b">B</div>
            <div class="c">C</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; justify-content: space-between; gap: 10px; width: 800px; }
        .a, .b, .c { width: 100px; height: 50px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 3, "should have 3 flex children");
    let tolerance = 30.0;
    let (_, a) = &children[0];
    let (_, c) = &children[2];
    assert!(
        a.x < tolerance,
        "item A should be near left edge with space-between. A.x={}",
        a.x
    );
    assert!(
        c.x + c.width > 700.0,
        "item C should be near right edge. C.x={}, C.w={}",
        c.x,
        c.width
    );
}

// ---------------------------------------------------------------------------
// 4. flex-wrap + gap
// ---------------------------------------------------------------------------

#[test]
fn flex_wrap_with_gap() {
    let html = r#"
        <div class="container">
            <div class="a">A</div>
            <div class="b">B</div>
            <div class="c">C</div>
            <div class="d">D</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; flex-wrap: wrap; gap: 15px; width: 500px; }
        .a, .b, .c, .d { width: 200px; height: 50px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 4, "should have 4 flex children");
    let _tolerance = 15.0;
    let (_, a) = &children[0];
    let (_, c) = &children[2];
    // A, B on line 1 (y~0), C, D on line 2 (y>50)
    assert!(
        c.y > a.y + 40.0,
        "item C should be on line 2 below A. A.y={}, C.y={}",
        a.y,
        c.y
    );
}

// ---------------------------------------------------------------------------
// 5. align-content + gap (multi-line flex)
// ---------------------------------------------------------------------------

#[test]
fn flex_align_content_center_wrap_gap() {
    let html = r#"
        <div class="container">
            <div class="a">A</div>
            <div class="b">B</div>
            <div class="c">C</div>
            <div class="d">D</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; flex-wrap: wrap; align-content: center; gap: 20px; width: 500px; height: 400px; }
        .a, .b, .c, .d { width: 200px; height: 50px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 4, "should have 4 flex children");
    let (_, a) = &children[0];
    // 2 lines, each 50px, 20px gap = 120px total. Centered in 400px: offset ~140
    assert!(
        a.y > 80.0,
        "items should be vertically centered. A.y={}",
        a.y
    );
}

// ---------------------------------------------------------------------------
// 6. Column flex + gap
// ---------------------------------------------------------------------------

#[test]
fn flex_column_gap() {
    let html = r#"
        <div class="container">
            <div class="a">A</div>
            <div class="b">B</div>
            <div class="c">C</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; flex-direction: column; gap: 25px; width: 300px; }
        .a, .b, .c { height: 40px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 3, "should have 3 flex children");
    let tolerance = 15.0;
    let (_, a) = &children[0];
    let (_, b) = &children[1];
    let (_, c) = &children[2];
    let gap_ab = b.y - a.y - a.height;
    let gap_bc = c.y - b.y - b.height;
    assert!(
        (gap_ab - 25.0).abs() < tolerance,
        "gap A->B should be ~25px. got {}",
        gap_ab
    );
    assert!(
        (gap_bc - 25.0).abs() < tolerance,
        "gap B->C should be ~25px. got {}",
        gap_bc
    );
}

// ---------------------------------------------------------------------------
// 7. Nested percentage + flex combinations
// ---------------------------------------------------------------------------

#[test]
fn pct_parent_flex_children_gap() {
    let html = r#"
        <div class="outer">
            <div class="inner">
                <div class="a">A</div>
                <div class="b">B</div>
            </div>
        </div>
    "#;
    let css = r#"
        .outer { width: 800px; }
        .inner { width: 50%; display: flex; gap: 10px; }
        .a, .b { flex-grow: 1; height: 30px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex inner div");
    let inner = &els[flex_els[0]];
    let tolerance = 30.0;
    assert!(
        (inner.width - 400.0).abs() < tolerance,
        "inner div should be 50% of 800 = 400px. got {}",
        inner.width
    );
}

#[test]
fn flex_inside_pct_width_sidebar() {
    let html = r#"
        <div class="layout">
            <div class="sidebar">S</div>
            <div class="main">
                <div class="card">C1</div>
                <div class="card">C2</div>
                <div class="card">C3</div>
            </div>
        </div>
    "#;
    let css = r#"
        .layout { display: flex; width: 100%; }
        .sidebar { width: 25%; height: 600px; }
        .main { width: 75%; display: flex; flex-wrap: wrap; gap: 10px; }
        .card { width: 200px; height: 100px; }
    "#;
    let els = pipeline(html, css);
    // Find the .main flex container (has flex-wrap and is child of the layout flex)
    let main_idx = els.iter().position(|e| {
        e.tag == "div" && e.display == Display::Flex && e.flex_wrap == FlexWrap::Wrap
    });
    assert!(main_idx.is_some(), "should find .main flex container");
    let main = &els[main_idx.unwrap()];
    let cards: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            e.tag == "div" && e.parent_index == Some(main_idx.unwrap()) && e.height > 50.0
        })
        .collect();
    assert_eq!(cards.len(), 3, "should have 3 cards");
    // Cards should be 200px wide
    let tolerance = 30.0;
    assert!(
        (cards[0].1.width - 200.0).abs() < tolerance,
        "card width should be ~200px. got {}",
        cards[0].1.width
    );
    // Main container should be smaller than viewport (75% of parent)
    assert!(
        main.width < 750.0,
        "main should be narrower than viewport. got {}",
        main.width
    );
}

// ---------------------------------------------------------------------------
// 8. Common Tailwind/Bootstrap-style patterns
// ---------------------------------------------------------------------------

#[test]
fn tailwind_card_grid() {
    let html = r#"
        <div class="grid">
            <div class="card">C1</div>
            <div class="card">C2</div>
            <div class="card">C3</div>
        </div>
    "#;
    let css = r#"
        .grid { display: flex; flex-wrap: wrap; gap: 16px; width: 1200px; }
        .card { width: 380px; height: 200px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex grid");
    let grid_idx = flex_els[0];
    let cards: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.tag == "div" && e.parent_index == Some(grid_idx) && e.height > 100.0)
        .collect();
    assert_eq!(cards.len(), 3, "should have 3 cards");
    // All on one row: same y
    let tolerance = 20.0;
    assert!(
        (cards[0].1.y - cards[1].1.y).abs() < tolerance,
        "cards should be on same row. C1.y={}, C2.y={}",
        cards[0].1.y,
        cards[1].1.y
    );
    assert!(
        cards[0].1.width > 300.0,
        "card width should be substantial. got {}",
        cards[0].1.width
    );
}

#[test]
fn bootstrap_button_group() {
    let html = r#"
        <div class="btn-group">
            <div class="btn">Btn1</div>
            <div class="btn">Btn2</div>
            <div class="btn">Btn3</div>
        </div>
    "#;
    let css = r#"
        .btn-group { display: flex; gap: 0; }
        .btn { padding: 10px 20px; height: 40px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex btn-group");
    let bg_idx = flex_els[0];
    let buttons: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.tag == "div" && e.parent_index == Some(bg_idx) && e.text.is_empty())
        .collect();
    assert_eq!(buttons.len(), 3, "should have 3 buttons");
    // Zero gap: buttons should be adjacent
    let tolerance = 5.0;
    let (_, b1) = &buttons[0];
    let (_, b2) = &buttons[1];
    let (_, b3) = &buttons[2];
    let gap_12 = b2.x - (b1.x + b1.width);
    let gap_23 = b3.x - (b2.x + b2.width);
    assert!(
        gap_12.abs() < tolerance,
        "zero gap: buttons should be adjacent. gap={}",
        gap_12
    );
    assert!(
        gap_23.abs() < tolerance,
        "zero gap: buttons should be adjacent. gap={}",
        gap_23
    );
}

#[test]
fn navbar_flex_space_between() {
    let html = r#"
        <nav class="navbar">
            <div class="logo">Logo</div>
            <div class="links">
                <div>Home</div>
                <div>About</div>
                <div>Contact</div>
            </div>
        </nav>
    "#;
    let css = r#"
        .navbar { display: flex; justify-content: space-between; align-items: center; width: 1200px; height: 60px; }
        .logo { width: 150px; height: 40px; }
        .links { display: flex; gap: 20px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(flex_els.len() >= 1, "should have flex containers");
    // The navbar is the outermost flex container
    let nav_idx = flex_els
        .iter()
        .find(|&&i| {
            els[i].parent_index.is_some() && {
                let pi = els[i].parent_index.unwrap();
                pi < els.len() && (els[pi].tag == "body" || els[pi].tag == "html")
            }
        })
        .or(flex_els.first())
        .unwrap();
    let _nav = &els[*nav_idx];
    // Find logo and links as children of nav
    let logo = els
        .iter()
        .find(|e| e.parent_index == Some(*nav_idx) && e.width > 100.0 && e.width < 200.0);
    let links = els
        .iter()
        .find(|e| e.parent_index == Some(*nav_idx) && e.display == Display::Flex);
    assert!(logo.is_some(), "should find logo");
    assert!(links.is_some(), "should find links container");
    let logo = logo.unwrap();
    let links = links.unwrap();
    assert!(
        logo.x < links.x,
        "logo should be left of links. logo.x={}, links.x={}",
        logo.x,
        links.x
    );
}

#[test]
fn hero_section_percentage_and_flex() {
    let html = r#"
        <section class="hero">
            <div class="text">
                <h1>Welcome</h1>
                <p>Subtitle text</p>
            </div>
            <div class="image">IMG</div>
        </section>
    "#;
    let css = r#"
        .hero { display: flex; width: 100%; height: 500px; }
        .text { width: 50%; }
        .image { width: 50%; height: 100%; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex hero section");
    let hero_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(hero_idx) && e.tag == "div")
        .collect();
    assert!(children.len() >= 2, "hero should have at least 2 children");
    let tolerance = 30.0;
    assert!(
        (children[0].1.width - 400.0).abs() < tolerance,
        "text div should be 50% of 800 = 400px. got {}",
        children[0].1.width
    );
}

// ---------------------------------------------------------------------------
// 9. Regression: elements should not overlap at origin
// ---------------------------------------------------------------------------

#[test]
fn flex_items_not_all_at_origin() {
    let html = r#"
        <div class="container">
            <div class="a">1</div>
            <div class="b">2</div>
            <div class="c">3</div>
        </div>
    "#;
    let css = r#"
        .container { display: flex; gap: 10px; width: 600px; }
        .a, .b, .c { width: 100px; height: 50px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex container");
    let fc_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(fc_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 3, "should have 3 flex children");
    assert!(
        children[1].1.x > children[0].1.x + 50.0,
        "item 2 should be right of item 1. item1.x={}, item2.x={}",
        children[0].1.x,
        children[1].1.x
    );
    assert!(
        children[2].1.x > children[1].1.x + 50.0,
        "item 3 should be right of item 2. item2.x={}, item3.x={}",
        children[1].1.x,
        children[2].1.x
    );
}

#[test]
fn block_stack_not_all_at_origin() {
    let html = r#"
        <div class="container">
            <p class="p1">First</p>
            <p class="p2">Second</p>
            <p class="p3">Third</p>
        </div>
    "#;
    let css = r#"
        .container { width: 400px; }
        .p1, .p2, .p3 { margin: 10px 0; height: 30px; }
    "#;
    let els = pipeline(html, css);
    let ps: Vec<_> = els.iter().filter(|e| e.tag == "p").collect();
    assert_eq!(ps.len(), 3, "should have 3 paragraphs");
    assert!(
        ps[1].y > ps[0].y + 10.0,
        "second p should be below first. p1.y={}, p2.y={}",
        ps[0].y,
        ps[1].y
    );
    assert!(
        ps[2].y > ps[1].y + 10.0,
        "third p should be below second. p2.y={}, p3.y={}",
        ps[1].y,
        ps[2].y
    );
}

// ---------------------------------------------------------------------------
// 10. Percentage width inside flex parent
// ---------------------------------------------------------------------------

#[test]
fn flex_child_with_pct_width_in_flex_parent() {
    let html = r#"
        <div class="outer">
            <div class="a">A</div>
            <div class="b">B</div>
        </div>
    "#;
    let css = r#"
        .outer { display: flex; width: 800px; }
        .a { width: 25%; height: 100px; }
        .b { width: 75%; height: 100px; }
    "#;
    let els = pipeline(html, css);
    let flex_els = flex_containers(&els);
    assert!(!flex_els.is_empty(), "should have flex outer");
    let outer_idx = flex_els[0];
    let children: Vec<_> = els
        .iter()
        .enumerate()
        .filter(|(_, e)| e.parent_index == Some(outer_idx) && e.tag == "div")
        .collect();
    assert_eq!(children.len(), 2, "should have 2 items");
    let tolerance = 30.0;
    let (_, a) = &children[0];
    let (_, b) = &children[1];
    assert!(
        (a.width - 200.0).abs() < tolerance,
        "A should be 25% of 800 = 200px. got {}",
        a.width
    );
    assert!(
        (b.width - 600.0).abs() < tolerance,
        "B should be 75% of 800 = 600px. got {}",
        b.width
    );
    assert!(
        b.x > a.x + a.width - tolerance,
        "B should be right of A. A.x={}, A.w={}, B.x={}",
        a.x,
        a.width,
        b.x
    );
}
