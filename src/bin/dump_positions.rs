use layout_engine::{LayoutEngine, LayoutInput, NativeLayoutEngine};
use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::pipeline::apply_taffy_layout;
use vayu_browser::engine::pipeline::extractor::extract_elements;
use vayu_browser::engine::stratus;
use vayu_browser::engine::stratus::CustomPropertyMap;

fn styled_to_input(
    el: &vayu_browser::engine::pipeline::StyledElement,
) -> layout_engine::LayoutElementInput {
    use layout_engine::BoxSizing as EngineBoxSizing;
    layout_engine::LayoutElementInput {
        display: el.display,
        position: el.position,
        flex_direction: Some(el.flex_direction),
        flex_wrap: Some(el.flex_wrap),
        align_items: Some(el.align_items),
        align_self: Some(el.align_self),
        justify_content: Some(el.justify_content),
        align_content: Some(el.align_content),
        box_sizing: match el.box_sizing {
            vayu_browser::engine::pipeline::extractor::BoxSizing::ContentBox => {
                EngineBoxSizing::ContentBox
            }
            vayu_browser::engine::pipeline::extractor::BoxSizing::BorderBox => {
                EngineBoxSizing::BorderBox
            }
        },
        flex_grow: el.flex_grow,
        flex_shrink: el.flex_shrink,
        flex_basis: el.flex_basis,
        grid_template_columns: None,
        grid_template_rows: None,
        grid_column: None,
        grid_row: None,
        grid_auto_flow: Some(layout_engine::GridAutoFlow::Row),
        gap: None,
        width: el.css_width,
        height: el.css_height,
        min_width: el.min_width,
        max_width: el.max_width,
        min_height: el.min_height,
        max_height: el.max_height,
        margin: [
            Some(el.margin_top),
            el.margin_right,
            Some(el.margin_bottom),
            el.margin_left,
        ],
        padding: el.padding,
        border_width: el.border_widths,
        inset: [el.inset_top, el.inset_right, el.inset_bottom, el.inset_left],
        parent_index: el.parent_index,
        is_text: el.tag == "text",
        text: el.text.clone(),
        font_size: el.font_size,
        line_height: el.line_height,
        has_content: !el.text.is_empty() || el.css_width.is_some() || el.css_height.is_some(),
    }
}

fn dump(name: &str, html: &str, css: &str) {
    let dom = parse_html(html);
    let sheet = if css.is_empty() {
        stratus::parse("")
    } else {
        stratus::parse(css)
    };
    let mut els = Vec::new();
    extract_elements(
        &dom,
        &mut els,
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
    println!("=== {} ===", name);
    println!("extracted {} elements:", els.len());
    for (i, e) in els.iter().enumerate() {
        println!("  els[{}] tag={} text={:?} parent={:?} display={:?} position={:?} css_w={:?} css_h={:?} margin={}/{:?}/{:?}/{:?} flex_grow={} y_before_layout={} height_before={}", i, e.tag, e.text, e.parent_index, e.display, e.position, e.css_width, e.css_height, e.margin_top, e.margin_right, e.margin_bottom, e.margin_left, e.flex_grow, e.y, e.height);
    }
    let inputs: Vec<_> = els.iter().map(styled_to_input).collect();
    let input = LayoutInput {
        container_width: 800.0,
        viewport_height: 600.0,
        elements: inputs,
    };
    let native = NativeLayoutEngine::new().compute_layout(&input);
    let mut taffy_els = els.clone();
    apply_taffy_layout(&mut taffy_els, 800.0, 600.0);
    println!("Native output:");
    for (i, o) in native.elements.iter().enumerate() {
        println!(
            "  out[{}] x={:.1} y={:.1} w={:.1} h={:.1}",
            i, o.x, o.y, o.width, o.height
        );
    }
    println!("Taffy output (via StyledElement):");
    for (i, e) in taffy_els.iter().enumerate() {
        println!(
            "  taffy[{}] x={:.1} y={:.1} w={:.1} h={:.1}",
            i, e.x, e.y, e.width, e.height
        );
    }
    // semantic expected checks
    if name == "simple_div_paragraph" {
        // div should contain 2 p's stacked; y monotonic
        if native.elements.len() >= 3 {
            let div_h = native.elements[0].height;
            let p1_h = native.elements[1].height;
            let p2_h = native.elements[2].height;
            let p1_y = native.elements[1].y;
            let p2_y = native.elements[2].y;
            println!("semantic: div h={:.1} p1 y={:.1} h={:.1} p2 y={:.1} h={:.1} -> p2 should be >= p1.h + margin (8) ", div_h, p1_y, p1_h, p2_y, p2_h);
            println!(
                "  expected: p2.y ≈ p1.y + p1.h + 8 (collapsed) ; div.h ≈ p2.y + p2.h - div.y"
            );
        }
    }
    if name == "inline_siblings" && native.elements.len() >= 4 {
        for (i, (ne, te)) in native.elements[1..4]
            .iter()
            .zip(taffy_els[1..4].iter())
            .enumerate()
        {
            println!(
                "  inline {} x={:.1} y={:.1} w={:.1} h={:.1} (taffy x={:.1} y={:.1})",
                i + 1,
                ne.x,
                ne.y,
                ne.width,
                ne.height,
                te.x,
                te.y
            );
        }
        println!("semantic: inline spans should share same y (baseline) and x monotonic");
    }
    if name == "absolute_positioning" && native.elements.len() >= 2 {
        let parent = &native.elements[0];
        let child = &native.elements[1];
        let exp_x = parent.x + 20.0;
        let exp_y = parent.y + 10.0;
        println!(
            "semantic: abs child expected x={:.1} y={:.1} got x={:.1} y={:.1} (parent {:?})",
            exp_x, exp_y, child.x, child.y, parent
        );
        println!(
            "  taffy child x={:.1} y={:.1}",
            taffy_els[1].x, taffy_els[1].y
        );
    }
    if name == "flex_row" && native.elements.len() >= 4 {
        for (i, (ne, te)) in native.elements[1..4]
            .iter()
            .zip(taffy_els[1..4].iter())
            .enumerate()
        {
            println!(
                "  flex child {} native x={:.1} w={:.1} y={:.1} vs taffy x={:.1} w={:.1} y={:.1}",
                i + 1,
                ne.x,
                ne.width,
                ne.y,
                te.x,
                te.width,
                te.y
            );
        }
        println!("semantic: flex children expected w≈266.7 each, x 0,266.7,533.3 y 0");
    }
    println!();
}

fn main() {
    dump("simple_div_paragraph", "<div class=\"container\"><p id=\"first\">Hello</p><p class=\"highlight\">World</p></div>", ".container { display: block; width: 800px; background-color: #fff; }\np { display: block; color: #333; font-size: 16px; margin-top: 8px; margin-bottom: 8px; }\n.highlight { color: red; font-weight: bold; }");
    dump(
        "inline_siblings",
        "<div><span>item1</span><span>item2</span><span>item3</span></div>",
        "div { width: 800px; }\nspan { display: inline; margin: 4px; }",
    );
    dump("absolute_positioning", "<div class=\"rel\"><div class=\"abs\">abs</div></div>", ".rel { position: relative; width: 800px; height: 600px; }\n.abs { position: absolute; top: 10px; left: 20px; width: 100px; height: 50px; }");
    dump("flex_row", "<div class=\"fc\"><div class=\"item\">1</div><div class=\"item\">2</div><div class=\"item\">3</div></div>", ".fc { display: flex; width: 800px; }\n.item { flex-grow: 1; flex-shrink: 1; flex-basis: 0px; height: 50px; }");
    dump(
        "pe_tall",
        "<div class=\"pe\"><div class=\"tall\">child tall</div></div>",
        ".pe { width: 800px; }\n.tall { height: 200px; }",
    );
    dump(
        "wide_container",
        "<div class=\"wide\"><div class=\"a\">a</div><div class=\"a\">b</div></div>",
        ".wide { width: 2000px; }\n.a { width: 400px; height: 20px; }",
    );
    dump(
        "flex_column",
        "<div class=\"fcc\"><div class=\"col\">a</div><div class=\"col\">b</div></div>",
        ".fcc { display: flex; flex-direction: column; width: 400px; }\n.col { height: 50px; }",
    );
    dump("nested_flex", "<div class=\"nf\"><div class=\"flex1\"><div class=\"inner2\">inner</div></div><div class=\"sib\">sibling</div></div>", ".nf { display: flex; width: 800px; }\n.flex1 { flex: 1; display: flex; }\n.inner2 { flex: 1; height: 30px; }\n.sib { flex: 1; height: 30px; }");
    dump("all_display_types", "<div class=\"container\"><div>block</div><div class=\"inline\">inline</div><div class=\"ib\">inline-block</div><div class=\"flex\">flex</div></div>", ".container { width: 800px; margin: 4px; }\n.inline { display: inline; }\n.ib { display: inline-block; width: 100px; height: 50px; }\n.flex { display: flex; }");
    // also dump the direct StyledElement path for layout_stress parent_expands (bypassing extraction)
    {
        use aether_css::AlignContent;
        use vayu_browser::engine::pipeline::extractor::{BoxSizing, FontWeight, TextDecor};
        use vayu_browser::engine::pipeline::StyledElement;
        use vayu_browser::engine::stratus::{
            AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent, Position,
        };
        fn make_el(tag: &str, parent: Option<usize>) -> StyledElement {
            StyledElement {
                tag: tag.to_string(),
                text: String::new(),
                wrapped_lines: vec![],
                dom_path: vec![],
                is_link: false,
                href: None,
                indent_level: 0,
                color: iced::Color::BLACK,
                font_size: 16.0,
                font_weight: FontWeight::Normal,
                font_family: None,
                text_align: None,
                visibility: None,
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
                display: Display::Block,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::NoWrap,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                align_self: AlignSelf::Auto,
                align_content: AlignContent::Stretch,
                box_sizing: BoxSizing::ContentBox,
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
                text_decoration: TextDecor::default(),
                border_radius: [0.0; 4],
                input_type: String::new(),
                input_value: String::new(),
                input_placeholder: String::new(),
                checked: false,
                position: Position::Static,
                inset_top: 0.0,
                inset_right: 0.0,
                inset_bottom: 0.0,
                inset_left: 0.0,
                row_gap: 0.0,
                column_gap: 0.0,
            }
        }
        let els = vec![
            make_el("parent", None),
            StyledElement {
                css_height: Some(200.0),
                ..make_el("child", Some(0))
            },
        ];
        let mut taffy_els = els.clone();
        vayu_browser::engine::pipeline::apply_taffy_layout(&mut taffy_els, 800.0, 600.0);
        println!("=== layout_stress parent_expands (Taffy direct) ===");
        for (i, e) in taffy_els.iter().enumerate() {
            println!(
                "  taffy[{}] {} x{:.1} y{:.1} w{:.1} h{:.1}",
                i, e.tag, e.x, e.y, e.width, e.height
            );
        }
        let inputs: Vec<_> = els.iter().map(styled_to_input).collect();
        let out =
            layout_engine::NativeLayoutEngine::new().compute_layout(&layout_engine::LayoutInput {
                container_width: 800.0,
                viewport_height: 600.0,
                elements: inputs,
            });
        println!("=== layout_stress parent_expands (Native direct) ===");
        for (i, o) in out.elements.iter().enumerate() {
            println!(
                "  native[{}] x{:.1} y{:.1} w{:.1} h{:.1}",
                i, o.x, o.y, o.width, o.height
            );
        }
        println!(
            "semantic: parent auto-height should be >= child bottom (200) per CSS block→block"
        );
    }
}
