use aether_browser::engine::pipeline::{apply_caelum_layout, StyledElement};
use iced::Color;

fn make_el(tag: &str, parent: Option<usize>) -> StyledElement {
    StyledElement {
        tag: tag.to_string(),
        text: String::new(),
        wrapped_lines: vec![],
        is_link: false,
        href: None,
        indent_level: 0,
        color: Color::BLACK,
        font_size: 16.0,
        font_weight: "normal".to_string(),
        background_color: None,
        border_widths: [0.0; 4],
        border_color: None,
        image_handle: None,
        image_url: None,
        margin_top: 0.0,
        margin_bottom: 0.0,
        margin_left: None,
        margin_right: None,
        padding: [0.0; 4],
        display: "block".to_string(),
        flex_direction: "row".to_string(),
        flex_wrap: "nowrap".to_string(),
        justify_content: "flex-start".to_string(),
        align_items: "stretch".to_string(),
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: None,
        css_width: None,
        css_height: None,
        parent_index: parent,
        min_width: None,
        max_width: None,
        min_height: None,
        max_height: None,
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
        line_height: 1.4,
        text_decoration: String::new(),
        text_transform: String::new(),
        border_radius: [0.0; 4],
    }
}

fn check_positions(elements: &[StyledElement]) {
    for (i, el) in elements.iter().enumerate() {
        assert!(el.x.is_finite(), "el[{}] x not finite: {}", i, el.x);
        assert!(el.y.is_finite(), "el[{}] y not finite: {}", i, el.y);
        assert!(el.width.is_finite() && el.width >= 0.0, "el[{}] bad width: {}", i, el.width);
        assert!(el.height.is_finite() && el.height >= 0.0, "el[{}] bad height: {}", i, el.height);
    }
}

#[test]
fn single_block_element() {
    let mut elements = vec![make_el("div", None)];
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
    assert_eq!(elements[0].x, 0.0);
    assert_eq!(elements[0].y, 0.0);
}

#[test]
fn two_blocks_parent_child() {
    let mut elements = vec![
        make_el("div", None),
        make_el("p", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
    assert!(elements[1].x >= 0.0);
    assert!(elements[1].y >= 0.0);
}

#[test]
fn thousand_flat_elements() {
    let mut elements = vec![make_el("root", None)];
    for _ in 0..999 {
        elements.push(make_el("div", Some(0)));
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn thousand_inline_siblings() {
    let mut elements = vec![make_el("root", None)];
    for i in 0..1000 {
        let mut el = make_el("span", Some(0));
        el.display = "inline".to_string();
        el.text = format!("item{}", i);
        elements.push(el);
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn deep_nesting_50() {
    let mut elements = vec![make_el("root", None)];
    for i in 0..50 {
        elements.push(make_el("div", Some(i)));
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn deep_nesting_100() {
    let mut elements = vec![make_el("root", None)];
    for i in 0..100 {
        elements.push(make_el("div", Some(i)));
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn mixed_inline_block() {
    let mut elements = vec![make_el("root", None)];
    for i in 0..20 {
        let display = if i % 2 == 0 { "inline" } else { "block" };
        let mut el = make_el("child", Some(0));
        el.display = display.to_string();
        if display == "inline" {
            el.text = format!("span{}", i);
        }
        elements.push(el);
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn large_text_elements() {
    let long_text = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ".repeat(10);
    assert!(long_text.len() > 500);
    let mut elements = vec![make_el("root", None)];
    for _ in 0..10 {
        let mut el = make_el("p", Some(0));
        el.text = long_text.clone();
        elements.push(el);
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn all_display_types() {
    let mut elements = vec![make_el("root", None)];
    for (i, disp) in ["block", "inline", "inline-block", "flex", "none"].iter().enumerate() {
        let mut el = make_el("child", Some(0));
        el.display = disp.to_string();
        if *disp == "inline" || *disp == "inline-block" {
            el.text = format!("child{}", i);
        }
        if *disp == "inline-block" {
            el.css_width = Some(100.0);
            el.css_height = Some(50.0);
        }
        elements.push(el);
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn margins_affect_layout() {
    let mut elements = vec![make_el("root", None)];
    for i in 0..5 {
        let mut el = make_el("div", Some(0));
        el.margin_top = 20.0;
        el.margin_bottom = 20.0;
        el.css_height = Some(30.0);
        elements.push(el);
    }
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
    for i in 2..elements.len() {
        assert!(
            elements[i].y > elements[i - 1].y,
            "el[{}] y={} not > el[{}] y={}",
            i, elements[i].y, i - 1, elements[i - 1].y
        );
    }
}

#[test]
fn padding_contains_children() {
    let mut elements = vec![
        StyledElement {
            padding: [10.0; 4],
            css_width: Some(400.0),
            css_height: Some(300.0),
            ..make_el("div", None)
        },
        StyledElement {
            css_width: Some(100.0),
            css_height: Some(50.0),
            ..make_el("child", Some(0))
        },
    ];
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
    let child = &elements[1];
    let parent = &elements[0];
    assert!(child.x >= parent.x, "child.x {} < parent.x {}", child.x, parent.x);
    assert!(child.y >= parent.y, "child.y {} < parent.y {}", child.y, parent.y);
}

#[test]
fn borders_no_crash() {
    let mut elements = vec![
        StyledElement {
            border_widths: [2.0, 2.0, 2.0, 2.0],
            border_color: Some(Color::BLACK),
            css_width: Some(200.0),
            css_height: Some(100.0),
            ..make_el("div", None)
        },
        StyledElement {
            border_widths: [1.0, 1.0, 1.0, 1.0],
            border_color: Some(Color::from_rgb(1.0, 0.0, 0.0)),
            css_width: Some(100.0),
            css_height: Some(50.0),
            ..make_el("child", Some(0))
        },
    ];
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
    check_positions(&elements);
}

#[test]
fn empty_slice_no_panic() {
    let mut elements: Vec<StyledElement> = vec![];
    apply_caelum_layout(&mut elements, 800.0, 6000.0);
}

#[test]
fn wide_container() {
    let mut elements = vec![make_el("root", None)];
    for _ in 0..50 {
        let mut el = make_el("div", Some(0));
        el.css_width = Some(400.0);
        el.css_height = Some(20.0);
        elements.push(el);
    }
    apply_caelum_layout(&mut elements, 2000.0, 6000.0);
    check_positions(&elements);
    for el in &elements {
        assert!(el.width <= 2000.0, "width {} exceeds container {}", el.width, 2000.0);
    }
}
