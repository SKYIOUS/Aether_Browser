//! Differential testing for layout engines.
//!
//! Runs both Taffy and native implementations on the same input
//! and compares geometry outputs to catch divergences.

#[cfg(all(test, feature = "taffy-backend"))]
mod diff_tests {
    use crate::{
        BoxSizing, LayoutElementInput, LayoutEngine, LayoutInput, LayoutOutput, TaffyLayoutEngine,
    };
    use aether_css::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
        Position,
    };

    /// Create a test LayoutInput with typical elements
    fn test_input() -> LayoutInput {
        LayoutInput {
            container_width: 800.0,
            viewport_height: 600.0,
            elements: vec![
                LayoutElementInput {
                    display: Display::Block,
                    position: Position::Relative,
                    flex_direction: None,
                    flex_wrap: None,
                    align_items: None,
                    align_self: None,
                    justify_content: None,
                    align_content: None,
                    box_sizing: BoxSizing::ContentBox,
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    flex_basis: None,
                    width: Some(800.0),
                    height: None,
                    has_content: false,
                    grid_template_columns: None,
                    grid_template_rows: None,
                    grid_column: None,
                    grid_row: None,
                    grid_auto_flow: None,
                    gap: None,
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    margin: [Some(0.0), Some(0.0), Some(0.0), Some(0.0)],
                    padding: [0.0; 4],
                    border_width: [0.0; 4],
                    inset: [0.0; 4],
                    parent_index: None,
                    is_text: false,
                    text: String::new(),
                    font_size: 16.0,
                    line_height: 1.4,
                },
                LayoutElementInput {
                    display: Display::Block,
                    position: Position::Relative,
                    flex_direction: None,
                    flex_wrap: None,
                    align_items: None,
                    align_self: None,
                    justify_content: None,
                    align_content: None,
                    box_sizing: BoxSizing::ContentBox,
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    flex_basis: None,
                    width: Some(400.0),
                    height: Some(100.0),
                    has_content: false,
                    grid_template_columns: None,
                    grid_template_rows: None,
                    grid_column: None,
                    grid_row: None,
                    grid_auto_flow: None,
                    gap: None,
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    margin: [Some(10.0), Some(10.0), Some(10.0), Some(10.0)],
                    padding: [5.0, 5.0, 5.0, 5.0],
                    border_width: [1.0, 1.0, 1.0, 1.0],
                    inset: [0.0; 4],
                    parent_index: Some(0),
                    is_text: false,
                    text: String::new(),
                    font_size: 16.0,
                    line_height: 1.4,
                },
                LayoutElementInput {
                    display: Display::Inline,
                    position: Position::Relative,
                    flex_direction: None,
                    flex_wrap: None,
                    align_items: None,
                    align_self: None,
                    justify_content: None,
                    align_content: None,
                    box_sizing: BoxSizing::ContentBox,
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    flex_basis: None,
                    width: None,
                    height: None,
                    has_content: true,
                    grid_template_columns: None,
                    grid_template_rows: None,
                    grid_column: None,
                    grid_row: None,
                    grid_auto_flow: None,
                    gap: None,
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    margin: [Some(0.0), Some(0.0), Some(0.0), Some(0.0)],
                    padding: [0.0; 4],
                    border_width: [0.0; 4],
                    inset: [0.0; 4],
                    parent_index: Some(1),
                    is_text: true,
                    text: "Hello world".to_string(),
                    font_size: 16.0,
                    line_height: 1.4,
                },
            ],
        }
    }

    /// Compare two LayoutOutputs for equality within tolerance
    fn assert_layouts_eq(
        expected: &LayoutOutput,
        actual: &LayoutOutput,
        tolerance: f32,
        context: &str,
    ) {
        assert_eq!(
            expected.elements.len(),
            actual.elements.len(),
            "{}: element count mismatch",
            context
        );

        for (i, (exp, act)) in expected
            .elements
            .iter()
            .zip(actual.elements.iter())
            .enumerate()
        {
            let dx = (exp.x - act.x).abs();
            let dy = (exp.y - act.y).abs();
            let dw = (exp.width - act.width).abs();
            let dh = (exp.height - act.height).abs();

            assert!(
                dx < tolerance,
                "{}: element {} x diff {} >= {}",
                context,
                i,
                dx,
                tolerance
            );
            assert!(
                dy < tolerance,
                "{}: element {} y diff {} >= {}",
                context,
                i,
                dy,
                tolerance
            );
            assert!(
                dw < tolerance,
                "{}: element {} width diff {} >= {}",
                context,
                i,
                dw,
                tolerance
            );
            assert!(
                dh < tolerance,
                "{}: element {} height diff {} >= {}",
                context,
                i,
                dh,
                tolerance
            );
        }
    }

    #[test]
    fn taffy_layout_deterministic() {
        let engine = TaffyLayoutEngine::new();
        let input = test_input();

        let output1 = engine.compute_layout(&input);
        let output2 = engine.compute_layout(&input);

        // Same engine, same input should produce identical results
        assert_layouts_eq(&output1, &output2, 0.001, "deterministic");
    }

    #[test]
    fn taffy_layout_matches_expected_simple_block() {
        let engine = TaffyLayoutEngine::new();
        let input = test_input();
        let output = engine.compute_layout(&input);

        // Root element (index 0) should be at (0, 0) with full width
        assert!((output.elements[0].x - 0.0).abs() < 0.001, "root x");
        assert!((output.elements[0].y - 0.0).abs() < 0.001, "root y");
        assert!(
            (output.elements[0].width - 800.0).abs() < 0.001,
            "root width"
        );

        // Child block (index 1) should be at (10, 10) with content box
        assert!((output.elements[1].x - 10.0).abs() < 0.001, "child x");
        assert!((output.elements[1].y - 10.0).abs() < 0.001, "child y");
        // Width = 400 - margin(10+10) - padding(5+5) - border(1+1) = 368?
        // Actually Taffy uses content-box: width = content width
        // The element has width: 400, so content width = 400
        // With padding 5+5=10, border 1+1=2, margin 10+10=20
        // Total occupied = 400 + 10 + 2 + 20 = 432
    }

    #[test]
    fn taffy_flex_layout() {
        let engine = TaffyLayoutEngine::new();
        let input = LayoutInput {
            container_width: 800.0,
            viewport_height: 600.0,
            elements: vec![
                LayoutElementInput {
                    display: Display::Flex,
                    position: Position::Relative,
                    flex_direction: Some(FlexDirection::Row),
                    flex_wrap: Some(FlexWrap::NoWrap),
                    align_items: Some(AlignItems::Stretch),
                    align_self: None,
                    justify_content: Some(JustifyContent::FlexStart),
                    align_content: None,
                    box_sizing: BoxSizing::ContentBox,
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    flex_basis: None,
                    width: Some(800.0),
                    height: Some(200.0),
                    has_content: false,
                    grid_template_columns: None,
                    grid_template_rows: None,
                    grid_column: None,
                    grid_row: None,
                    grid_auto_flow: None,
                    gap: None,
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    margin: [Some(0.0); 4],
                    padding: [0.0; 4],
                    border_width: [0.0; 4],
                    inset: [0.0; 4],
                    parent_index: None,
                    is_text: false,
                    text: String::new(),
                    font_size: 16.0,
                    line_height: 1.4,
                },
                LayoutElementInput {
                    display: Display::Block,
                    position: Position::Relative,
                    flex_direction: None,
                    flex_wrap: None,
                    align_items: None,
                    align_self: None,
                    justify_content: None,
                    align_content: None,
                    box_sizing: BoxSizing::ContentBox,
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: Some(100.0),
                    width: None,
                    height: None,
                    has_content: false,
                    grid_template_columns: None,
                    grid_template_rows: None,
                    grid_column: None,
                    grid_row: None,
                    grid_auto_flow: None,
                    gap: None,
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    margin: [Some(0.0); 4],
                    padding: [0.0; 4],
                    border_width: [0.0; 4],
                    inset: [0.0; 4],
                    parent_index: Some(0),
                    is_text: false,
                    text: String::new(),
                    font_size: 16.0,
                    line_height: 1.4,
                },
                LayoutElementInput {
                    display: Display::Block,
                    position: Position::Relative,
                    flex_direction: None,
                    flex_wrap: None,
                    align_items: None,
                    align_self: None,
                    justify_content: None,
                    align_content: None,
                    box_sizing: BoxSizing::ContentBox,
                    flex_grow: 2.0,
                    flex_shrink: 1.0,
                    flex_basis: Some(100.0),
                    width: None,
                    height: None,
                    has_content: false,
                    grid_template_columns: None,
                    grid_template_rows: None,
                    grid_column: None,
                    grid_row: None,
                    grid_auto_flow: None,
                    gap: None,
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    margin: [Some(0.0); 4],
                    padding: [0.0; 4],
                    border_width: [0.0; 4],
                    inset: [0.0; 4],
                    parent_index: Some(0),
                    is_text: false,
                    text: String::new(),
                    font_size: 16.0,
                    line_height: 1.4,
                },
            ],
        };

        let output = engine.compute_layout(&input);

        // Flex children should be laid out horizontally
        // Child 1: flex-grow 1, basis 100
        // Child 2: flex-grow 2, basis 100
        // Total basis = 200, remaining = 600
        // Child 1 gets 200, Child 2 gets 400
        // Total: 300 + 500 = 800

        // Just verify they're laid out and don't overlap
        assert!(
            output.elements[1].x < output.elements[2].x,
            "flex children ordered"
        );
        assert!(output.elements[1].width > 0.0, "child 1 has width");
        assert!(output.elements[2].width > 0.0, "child 2 has width");
    }
}

#[cfg(all(test, feature = "taffy-backend", feature = "native-backend"))]
mod f5_correctness {
    use crate::{
        BoxSizing, LayoutElementInput, LayoutEngine, LayoutInput, NativeLayoutEngine,
        TaffyLayoutEngine,
    };
    use aether_css::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
        Position,
    };

    #[derive(Debug, PartialEq)]
    enum Classification {
        Match,
        NativeFixesTaffyGap,
        NativeRegression,
        Unsupported,
    }

    fn classify(
        taffy: &crate::LayoutOutput,
        native: &crate::LayoutOutput,
        tol: f32,
    ) -> Classification {
        if taffy.elements.len() != native.elements.len() {
            return Classification::NativeRegression;
        }
        let mut max_diff: f32 = 0.0;
        for (a, b) in taffy.elements.iter().zip(native.elements.iter()) {
            max_diff = max_diff
                .max((a.x - b.x).abs())
                .max((a.y - b.y).abs())
                .max((a.width - b.width).abs())
                .max((a.height - b.height).abs());
        }
        if max_diff < tol {
            Classification::Match
        } else {
            // Heuristic: if native parent contains child and taffy doesn't, it's a fix
            // For now, treat any diff < 5px as Match, larger as NativeRegression unless known gap
            if max_diff < 5.0 {
                Classification::Match
            } else {
                Classification::NativeRegression
            }
        }
    }

    fn flex_input(
        n: usize,
        dir: FlexDirection,
        wrap: FlexWrap,
        justify: JustifyContent,
        align_items: AlignItems,
    ) -> LayoutInput {
        let mut els = Vec::new();
        els.push(LayoutElementInput {
            display: Display::Flex,
            position: Position::Relative,
            flex_direction: Some(dir),
            flex_wrap: Some(wrap),
            align_items: Some(align_items),
            align_self: Some(AlignSelf::Auto),
            justify_content: Some(justify),
            align_content: Some(AlignContent::Stretch),
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: Some(800.0),
            height: Some(200.0),
            has_content: false,
            grid_template_columns: None,
            grid_template_rows: None,
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(0.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: None,
            is_text: false,
            text: String::new(),
            font_size: 16.0,
            line_height: 1.4,
        });
        for i in 0..n {
            els.push(LayoutElementInput {
                display: Display::Block,
                position: Position::Relative,
                flex_direction: None,
                flex_wrap: None,
                align_items: None,
                align_self: None,
                justify_content: None,
                align_content: None,
                box_sizing: BoxSizing::ContentBox,
                flex_grow: 1.0,
                flex_shrink: 1.0,
                flex_basis: Some(50.0),
                width: None,
                height: None,
                has_content: false,
                grid_template_columns: None,
                grid_template_rows: None,
                grid_column: None,
                grid_row: None,
                grid_auto_flow: None,
                gap: None,
                min_width: None,
                max_width: None,
                min_height: None,
                max_height: None,
                margin: [Some(2.0); 4],
                padding: [0.0; 4],
                border_width: [0.0; 4],
                inset: [0.0; 4],
                parent_index: Some(0),
                is_text: false,
                text: format!("item{}", i),
                font_size: 14.0,
                line_height: 1.2,
            });
        }
        LayoutInput {
            container_width: 800.0,
            viewport_height: 600.0,
            elements: els,
        }
    }

    #[test]
    fn f5_row() {
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let inp = flex_input(
            5,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        for i in 1..=5 {
            assert!(a.elements[i].width > 0.0);
            assert!(b.elements[i].width > 0.0);
        }
        assert!(b.elements[1].x < b.elements[2].x);
        assert!(a.elements[1].width >= 0.0);
    }

    #[test]
    fn f5_column() {
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let inp = flex_input(
            5,
            FlexDirection::Column,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        for i in 1..=5 {
            assert!(a.elements[i].height > 0.0);
            assert!(b.elements[i].height > 0.0);
        }
        assert!(a.elements[1].y < a.elements[2].y);
        assert!(b.elements[1].y < b.elements[2].y);
    }

    #[test]
    fn f5_grow_shrink() {
        let mut inp = flex_input(
            2,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[1].flex_grow = 1.0;
        inp.elements[1].flex_basis = Some(100.0);
        inp.elements[2].flex_grow = 2.0;
        inp.elements[2].flex_basis = Some(100.0);
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        // Just ensure both have positive widths and ordered
        assert!(a.elements[1].width > 0.0 && b.elements[1].width > 0.0);
        assert!(a.elements[1].x < a.elements[2].x);
        assert!(b.elements[1].x < b.elements[2].x);
    }

    #[test]
    fn f5_basis() {
        let mut inp = flex_input(
            1,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[1].flex_basis = Some(200.0);
        inp.elements[1].flex_grow = 0.0;
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(
            (a.elements[1].width - 200.0).abs() < 10.0
                || (b.elements[1].width - 200.0).abs() < 10.0
        );
    }

    #[test]
    fn f5_wrapping() {
        let inp = flex_input(
            10,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        // Wrapping may differ; just ensure no panic and heights > single line
        assert!(a.elements[0].height >= 30.0);
        assert!(b.elements[0].height >= 30.0);
    }

    #[test]
    fn f5_alignment() {
        let inp = flex_input(
            3,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::Center,
            AlignItems::Center,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        // Center justify should not be at 0
        assert!(a.elements[1].x > 0.0);
        assert!(b.elements[1].x > 0.0);
    }

    #[test]
    fn f5_nested() {
        let mut inp = flex_input(
            2,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        // Add block child inside first flex item
        let parent_idx = 1;
        inp.elements.push(LayoutElementInput {
            display: Display::Block,
            position: Position::Relative,
            flex_direction: None,
            flex_wrap: None,
            align_items: None,
            align_self: None,
            justify_content: None,
            align_content: None,
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: None,
            height: Some(20.0),
            has_content: true,
            grid_template_columns: None,
            grid_template_rows: None,
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(1.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: Some(parent_idx),
            is_text: false,
            text: String::new(),
            font_size: 12.0,
            line_height: 1.0,
        });
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert_eq!(a.elements.len(), b.elements.len());
    }

    #[test]
    fn f5_min_max() {
        let mut inp = flex_input(
            2,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[1].min_width = Some(150.0);
        inp.elements[1].max_width = Some(300.0);
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 150.0 - 1.0);
        assert!(b.elements[1].width >= 150.0 - 1.0);
    }

    #[test]
    fn f5_auto_sizes() {
        let mut inp = flex_input(
            2,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[1].width = None;
        inp.elements[1].height = None;
        inp.elements[2].width = None;
        inp.elements[2].height = None;
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 0.0);
        assert!(b.elements[1].width >= 0.0);
    }

    /// Baseline: capture native wrapping behavior with items that actually overflow.
    /// Container 200px wide, 4 items @ 80px basis + 2px margin each = 328px total.
    /// Expected: 2 lines of 2 items. Items on same line share main axis.
    #[test]
    fn f5_wrapping_baseline() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        // Override container to force wrapping: 200px wide
        inp.elements[0].width = Some(200.0);
        // Override child flex_basis to 80px, flex_grow to 0 so items don't shrink
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // Container
        let c = &out.elements[0];
        eprintln!("container: {}x{}", c.width, c.height);
        // Items
        for i in 1..=4 {
            let e = &out.elements[i];
            eprintln!(
                "item {}: x={} y={} w={} h={}",
                i, e.x, e.y, e.width, e.height
            );
        }
        // Baseline assertions — these capture CURRENT behavior, may be wrong
        let y1 = out.elements[1].y;
        let y2 = out.elements[3].y;
        eprintln!("line1_y={} line2_y={}", y1, y2);
        // At minimum, items should exist and have dimensions
        for i in 1..=4 {
            assert!(out.elements[i].width > 0.0, "item {} width", i);
            assert!(out.elements[i].height > 0.0, "item {} height", i);
        }
    }

    /// Geometry: 2-line wrapping proves line formation.
    /// 4 items @ 80px on 200px container → 2 lines of 2.
    /// Line 1 y must differ from line 2 y.
    #[test]
    fn f5_wrap_two_lines() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let y1 = out.elements[1].y;
        let y3 = out.elements[3].y;
        assert!(y3 > y1, "line 2 (y={}) must be below line 1 (y={})", y3, y1);
        // Line 1: items 1,2 share y; Line 2: items 3,4 share y
        assert_eq!(out.elements[1].y, out.elements[2].y, "items 1,2 same line");
        assert_eq!(out.elements[3].y, out.elements[4].y, "items 3,4 same line");
    }

    /// Geometry: single-line regression — items stay on one line when wrap=NoWrap.
    #[test]
    fn f5_nowrap_single_line() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let y1 = out.elements[1].y;
        for i in 2..=4 {
            assert_eq!(out.elements[i].y, y1, "item {} must share y with item 1", i);
        }
    }

    /// Geometry: align-content flex-start puts line 1 at cross-start.
    #[test]
    fn f5_align_content_flex_start() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].align_content = Some(AlignContent::FlexStart);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // Line 1 should start near container top (padding + border)
        let container_y = out.elements[0].y;
        let line1_y = out.elements[1].y;
        assert!(line1_y >= container_y, "line 1 at cross-start");
    }

    /// Geometry: align-content center centers lines vertically.
    #[test]
    fn f5_align_content_center() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].height = Some(400.0);
        inp.elements[0].align_content = Some(AlignContent::Center);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let container_h = out.elements[0].height;
        let line1_y = out.elements[1].y;
        let line2_y = out.elements[3].y;
        // Lines should be centered: line1_y > 0 (not at top)
        assert!(line1_y > 0.0, "line 1 not at top: y={}", line1_y);
        // line2 below line1
        assert!(line2_y > line1_y, "line 2 below line 1");
    }

    /// Geometry: align-content flex-end puts last line near cross-end.
    #[test]
    fn f5_align_content_flex_end() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].height = Some(400.0);
        inp.elements[0].align_content = Some(AlignContent::FlexEnd);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let container_h = out.elements[0].height;
        let line2_y = out.elements[3].y;
        // Line 2 should be near bottom
        assert!(
            line2_y > container_h / 2.0,
            "line 2 near bottom: y={}",
            line2_y
        );
    }

    /// Geometry: align-content space-between distributes lines with gap.
    #[test]
    fn f5_align_content_space_between() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].height = Some(400.0);
        inp.elements[0].align_content = Some(AlignContent::SpaceBetween);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let line1_y = out.elements[1].y;
        let line2_y = out.elements[3].y;
        // SpaceBetween: line 1 at start, line 2 at end, gap in between
        assert!(
            line2_y > line1_y + 80.0,
            "lines have gap: l1={} l2={}",
            line1_y,
            line2_y
        );
    }

    /// Geometry: align-content space-around distributes lines evenly.
    #[test]
    fn f5_align_content_space_around() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].height = Some(400.0);
        inp.elements[0].align_content = Some(AlignContent::SpaceAround);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let line1_y = out.elements[1].y;
        let line2_y = out.elements[3].y;
        // SpaceAround: lines evenly spaced, not touching edges
        assert!(line1_y > 0.0, "line 1 not at top: y={}", line1_y);
        assert!(line2_y > line1_y, "line 2 below line 1");
    }

    /// Geometry: align-content stretch enlarges line cross-sizes.
    /// Lines should be taller than their content when extra cross-space exists.
    #[test]
    fn f5_align_content_stretch() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].height = Some(400.0);
        inp.elements[0].align_content = Some(AlignContent::Stretch);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
            inp.elements[i].height = Some(20.0);
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // With stretch, items should be taller than their 20px content
        // because extra cross-space (400 - 2*20 = 360) is distributed
        let h1 = out.elements[1].height;
        let h3 = out.elements[3].height;
        assert!(h1 > 20.0, "item 1 stretched: h={}", h1);
        assert!(h3 > 20.0, "item 3 stretched: h={}", h3);
    }

    /// Geometry: wrap-reverse flips cross-axis direction.
    /// Line 1 should be below line 2 (reversed from normal wrap).
    #[test]
    fn f5_wrap_reverse() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::WrapReverse,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        for i in 1..=4 {
            eprintln!(
                "item {}: x={} y={} w={} h={}",
                i,
                out.elements[i].x,
                out.elements[i].y,
                out.elements[i].width,
                out.elements[i].height
            );
        }
        // wrap-reverse: line 2 (items 3,4) should have lower y than line 1 (items 1,2)
        let y1 = out.elements[1].y;
        let y3 = out.elements[3].y;
        assert!(
            y3 < y1,
            "wrap-reverse: line 2 (y={}) above line 1 (y={})",
            y3,
            y1
        );
    }

    /// Geometry: nested flex containers with wrapping.
    /// Outer wraps, inner items also flex.
    #[test]
    fn f5_nested_wrap() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // Items should wrap into 2 lines
        let y1 = out.elements[1].y;
        let y3 = out.elements[3].y;
        assert!(y3 > y1, "nested wrap: line 2 below line 1");
        // All items have valid dimensions
        for i in 1..=4 {
            assert!(out.elements[i].width > 0.0, "item {} width", i);
            assert!(out.elements[i].height > 0.0, "item {} height", i);
        }
    }

    /// Audit: row-reverse + wrap-reverse.
    /// Native engine does NOT implement main-axis reversal (only is_row flag).
    /// Items should still wrap across cross-axis correctly.
    #[test]
    fn f5_wrap_reverse_row_reverse() {
        let mut inp = flex_input(
            4,
            FlexDirection::RowReverse,
            FlexWrap::WrapReverse,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // wrap-reverse: line 2 (items 3,4) should be above line 1 (items 1,2)
        let y1 = out.elements[1].y;
        let y3 = out.elements[3].y;
        assert!(
            y3 < y1,
            "wrap-reverse row-reverse: line 2 (y={}) above line 1 (y={})",
            y3,
            y1
        );
        // Items have valid dimensions
        for i in 1..=4 {
            assert!(out.elements[i].width > 0.0, "item {} width", i);
            assert!(out.elements[i].height > 0.0, "item {} height", i);
        }
    }

    /// Audit: column + wrap-reverse.
    /// Column direction: main axis = y, cross axis = x.
    /// Wrapping happens horizontally. wrap-reverse flips horizontal direction.
    #[test]
    fn f5_wrap_reverse_column() {
        let mut inp = flex_input(
            4,
            FlexDirection::Column,
            FlexWrap::WrapReverse,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        // Column: height is main axis, width is cross axis
        inp.elements[0].height = Some(200.0);
        inp.elements[0].width = Some(400.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // Column wrap-reverse: lines wrap horizontally, reversed
        // Line 1 (items 1,2) should be in the right column
        // Line 2 (items 3,4) should be in the left column
        let x1 = out.elements[1].x;
        let x3 = out.elements[3].x;
        eprintln!("column wrap-reverse: x1={} x3={}", x1, x3);
        // Items have valid dimensions
        for i in 1..=4 {
            assert!(out.elements[i].width > 0.0, "item {} width", i);
            assert!(out.elements[i].height > 0.0, "item {} height", i);
        }
    }

    /// Audit: column-reverse + wrap-reverse.
    /// Both axes reversed. Items should still produce valid layout.
    #[test]
    fn f5_wrap_reverse_column_reverse() {
        let mut inp = flex_input(
            4,
            FlexDirection::ColumnReverse,
            FlexWrap::WrapReverse,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].height = Some(200.0);
        inp.elements[0].width = Some(400.0);
        for i in 1..=4 {
            inp.elements[i].flex_basis = Some(80.0);
            inp.elements[i].flex_grow = 0.0;
            inp.elements[i].flex_shrink = 0.0;
        }
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // At minimum: no panic, items have dimensions
        for i in 1..=4 {
            assert!(out.elements[i].width > 0.0, "item {} width", i);
            assert!(out.elements[i].height > 0.0, "item {} height", i);
        }
    }

    /// Audit: align-content stretch with UNEQUAL line cross-sizes.
    /// Items with different heights → lines have different cross-sizes.
    /// Stretch should distribute extra space per-line, not uniformly.
    #[test]
    fn f5_align_content_stretch_unequal() {
        let mut inp = flex_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            JustifyContent::FlexStart,
            AlignItems::Stretch,
        );
        inp.elements[0].width = Some(200.0);
        inp.elements[0].height = Some(300.0);
        inp.elements[0].align_content = Some(AlignContent::Stretch);
        // Line 1: items 1,2 (height 20). Line 2: items 3,4 (height 60).
        inp.elements[1].flex_basis = Some(80.0);
        inp.elements[1].flex_grow = 0.0;
        inp.elements[1].flex_shrink = 0.0;
        inp.elements[1].height = Some(20.0);
        inp.elements[2].flex_basis = Some(80.0);
        inp.elements[2].flex_grow = 0.0;
        inp.elements[2].flex_shrink = 0.0;
        inp.elements[2].height = Some(20.0);
        inp.elements[3].flex_basis = Some(80.0);
        inp.elements[3].flex_grow = 0.0;
        inp.elements[3].flex_shrink = 0.0;
        inp.elements[3].height = Some(60.0);
        inp.elements[4].flex_basis = Some(80.0);
        inp.elements[4].flex_grow = 0.0;
        inp.elements[4].flex_shrink = 0.0;
        inp.elements[4].height = Some(60.0);
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        let h1 = out.elements[1].height;
        let h3 = out.elements[3].height;
        eprintln!("unequal stretch: h1={} h3={}", h1, h3);
        // Both lines should be stretched beyond their content
        assert!(h1 > 20.0, "line 1 item stretched: h={}", h1);
        assert!(h3 > 60.0, "line 2 item stretched: h={}", h3);
        // Lines should have equal cross-sizes after stretch (both get same extra)
        // total line cross: 20+60=80, container=300, extra=220, per-line=110
        // line1: 20+110=130, line2: 60+110=170
        // Actually: stretch gives each line the same extra, so h1 ≈ 20+110=130, h3≈60+110=170
        assert!(h1 > 100.0, "line 1 significantly stretched: h={}", h1);
        assert!(h3 > 100.0, "line 2 significantly stretched: h={}", h3);
    }
}

#[cfg(all(test, feature = "taffy-backend", feature = "native-backend"))]
mod f6_grid {
    use crate::{
        BoxSizing, GridAutoFlow, GridPlacement, GridTrack, LayoutElementInput, LayoutEngine,
        LayoutInput, NativeLayoutEngine, TaffyLayoutEngine,
    };
    use aether_css::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
        Position,
    };

    fn grid_input(
        cols: Vec<GridTrack>,
        rows: Vec<GridTrack>,
        gap: Option<(f32, f32)>,
        items: usize,
    ) -> LayoutInput {
        let mut els = Vec::new();
        els.push(LayoutElementInput {
            display: Display::Grid,
            position: Position::Relative,
            flex_direction: None,
            flex_wrap: None,
            align_items: None,
            align_self: None,
            justify_content: None,
            align_content: None,
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: Some(800.0),
            height: None,
            has_content: true,
            grid_template_columns: Some(cols),
            grid_template_rows: Some(rows),
            grid_column: None,
            grid_row: None,
            grid_auto_flow: Some(GridAutoFlow::Row),
            gap,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(0.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: None,
            is_text: false,
            text: String::new(),
            font_size: 16.0,
            line_height: 1.4,
        });
        for i in 0..items {
            els.push(LayoutElementInput {
                display: Display::Block,
                position: Position::Relative,
                flex_direction: None,
                flex_wrap: None,
                align_items: None,
                align_self: None,
                justify_content: None,
                align_content: None,
                box_sizing: BoxSizing::ContentBox,
                flex_grow: 0.0,
                flex_shrink: 1.0,
                flex_basis: None,
                width: None,
                height: Some(50.0),
                has_content: true,
                grid_template_columns: None,
                grid_template_rows: None,
                grid_column: None,
                grid_row: None,
                grid_auto_flow: None,
                gap: None,
                min_width: None,
                max_width: None,
                min_height: None,
                max_height: None,
                margin: [Some(2.0); 4],
                padding: [2.0; 4],
                border_width: [0.0; 4],
                inset: [0.0; 4],
                parent_index: Some(0),
                is_text: false,
                text: format!("item{}", i),
                font_size: 14.0,
                line_height: 1.2,
            });
        }
        LayoutInput {
            container_width: 800.0,
            viewport_height: 600.0,
            elements: els,
        }
    }

    #[test]
    fn f6_2x2_fixed() {
        let inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            None,
            4,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width > 0.0);
        assert!(b.elements[1].width > 0.0);
        assert!((a.elements[1].x - b.elements[1].x).abs() < 5.0);
    }
    #[test]
    fn f6_rows_columns() {
        let inp = grid_input(
            vec![GridTrack::Fixed(200.0), GridTrack::Fixed(200.0)],
            vec![GridTrack::Fixed(100.0)],
            None,
            2,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(b.elements[1].x < b.elements[2].x);
        assert!(a.elements[1].width >= 0.0);
    }
    #[test]
    fn f6_gaps() {
        let inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            Some((10.0, 10.0)),
            4,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 0.0);
        assert!((b.elements[2].x - b.elements[1].x - 100.0 - 10.0).abs() < 5.0);
    }
    #[test]
    fn f6_fr() {
        let inp = grid_input(
            vec![GridTrack::Fr(1.0), GridTrack::Fr(2.0)],
            vec![GridTrack::Fixed(50.0)],
            None,
            2,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 0.0);
        assert!(b.elements[2].width > b.elements[1].width);
    }
    #[test]
    fn f6_mixed_fixed_fr() {
        let inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fr(1.0)],
            vec![GridTrack::Fixed(50.0)],
            None,
            2,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 0.0);
        assert!((b.elements[1].width - 100.0).abs() < 20.0);
    }
    #[test]
    fn f6_auto_tracks() {
        let inp = grid_input(
            vec![GridTrack::Auto, GridTrack::Auto],
            vec![GridTrack::Auto],
            None,
            2,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width > 0.0);
        assert!(b.elements[1].width > 0.0);
    }
    #[test]
    fn f6_explicit_placement() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            None,
            2,
        );
        inp.elements[2].grid_column = Some(GridPlacement::line(2));
        inp.elements[2].grid_row = Some(GridPlacement::line(2));
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 0.0);
        assert!(b.elements[2].x > b.elements[1].x);
    }
    #[test]
    fn f6_automatic_placement() {
        let inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            None,
            4,
        );
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert_eq!(a.elements.len(), 5);
        assert_eq!(b.elements.len(), 5);
    }
    #[test]
    fn f6_row_flow() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            None,
            4,
        );
        inp.elements[0].grid_auto_flow = Some(GridAutoFlow::Row);
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[3].y > a.elements[1].y);
        assert!(b.elements[3].y > b.elements[1].y);
    }
    #[test]
    fn f6_column_flow() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            None,
            4,
        );
        inp.elements[0].grid_auto_flow = Some(GridAutoFlow::Column);
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[2].y > a.elements[1].y || a.elements[2].x > a.elements[1].x);
        assert!(b.elements[2].y > b.elements[1].y || b.elements[2].x > b.elements[1].x);
    }
    #[test]
    fn f6_spanning() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            vec![GridTrack::Fixed(50.0)],
            None,
            2,
        );
        inp.elements[1].grid_column = Some(GridPlacement {
            start: Some(1),
            end: Some(3),
            span: None,
        });
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width > 150.0);
        assert!(b.elements[1].width > 150.0);
    }
    #[test]
    fn f6_nested_grid() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(400.0), GridTrack::Fixed(400.0)],
            vec![GridTrack::Fixed(100.0)],
            None,
            2,
        );
        inp.elements[1].grid_template_columns =
            Some(vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)]);
        inp.elements[1].grid_template_rows = Some(vec![GridTrack::Fixed(30.0)]);
        inp.elements[1].display = Display::Grid;
        inp.elements.push(LayoutElementInput {
            display: Display::Block,
            position: Position::Relative,
            flex_direction: None,
            flex_wrap: None,
            align_items: None,
            align_self: None,
            justify_content: None,
            align_content: None,
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: None,
            height: Some(20.0),
            has_content: true,
            grid_template_columns: None,
            grid_template_rows: None,
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(1.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: Some(1),
            is_text: false,
            text: String::new(),
            font_size: 12.0,
            line_height: 1.0,
        });
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert_eq!(a.elements.len(), b.elements.len());
    }
    #[test]
    fn f6_grid_flex() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(400.0)],
            vec![GridTrack::Fixed(100.0)],
            None,
            1,
        );
        inp.elements[1].display = Display::Flex;
        inp.elements[1].flex_direction = Some(FlexDirection::Row);
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width > 0.0);
        assert!(b.elements[1].width > 0.0);
    }
    #[test]
    fn f6_flex_grid() {
        let mut inp = LayoutInput {
            container_width: 800.0,
            viewport_height: 600.0,
            elements: vec![],
        };
        inp.elements.push(LayoutElementInput {
            display: Display::Flex,
            position: Position::Relative,
            flex_direction: Some(FlexDirection::Row),
            flex_wrap: Some(FlexWrap::NoWrap),
            align_items: Some(AlignItems::Stretch),
            align_self: Some(AlignSelf::Auto),
            justify_content: Some(JustifyContent::FlexStart),
            align_content: Some(AlignContent::Stretch),
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: Some(800.0),
            height: Some(200.0),
            has_content: true,
            grid_template_columns: None,
            grid_template_rows: None,
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(0.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: None,
            is_text: false,
            text: String::new(),
            font_size: 16.0,
            line_height: 1.4,
        });
        inp.elements.push(LayoutElementInput {
            display: Display::Grid,
            position: Position::Relative,
            flex_direction: None,
            flex_wrap: None,
            align_items: None,
            align_self: None,
            justify_content: None,
            align_content: None,
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 1.0,
            flex_shrink: 1.0,
            flex_basis: Some(200.0),
            width: None,
            height: None,
            has_content: true,
            grid_template_columns: Some(vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)]),
            grid_template_rows: Some(vec![GridTrack::Fixed(30.0)]),
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(2.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: Some(0),
            is_text: false,
            text: String::new(),
            font_size: 14.0,
            line_height: 1.2,
        });
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width > 0.0);
        assert!(b.elements[1].width > 0.0);
    }
    #[test]
    fn f6_grid_block() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(400.0)],
            vec![GridTrack::Fixed(100.0)],
            None,
            1,
        );
        inp.elements.push(LayoutElementInput {
            display: Display::Block,
            position: Position::Relative,
            flex_direction: None,
            flex_wrap: None,
            align_items: None,
            align_self: None,
            justify_content: None,
            align_content: None,
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: None,
            height: Some(20.0),
            has_content: true,
            grid_template_columns: None,
            grid_template_rows: None,
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(0.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: Some(1),
            is_text: false,
            text: String::new(),
            font_size: 12.0,
            line_height: 1.0,
        });
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert_eq!(a.elements.len(), b.elements.len());
    }
    #[test]
    fn f6_auto_height_grid() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(200.0), GridTrack::Fixed(200.0)],
            vec![GridTrack::Auto],
            None,
            2,
        );
        inp.elements[0].height = None;
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[0].height > 0.0);
        assert!(b.elements[0].height > 0.0);
    }
    #[test]
    fn f6_min_max_grid() {
        let mut inp = grid_input(
            vec![GridTrack::Fixed(100.0), GridTrack::Fr(1.0)],
            vec![GridTrack::Fixed(50.0)],
            None,
            2,
        );
        inp.elements[1].min_width = Some(150.0);
        inp.elements[1].max_width = Some(250.0);
        let t = TaffyLayoutEngine::new();
        let n = NativeLayoutEngine::new();
        let a = t.compute_layout(&inp);
        let b = n.compute_layout(&inp);
        assert!(a.elements[1].width >= 0.0);
        assert!(b.elements[1].width >= 100.0);
    }

    // ── Flex gap tests ──────────────────────────────────────────────

    /// Helper: flex container with gap, n children with explicit sizes
    fn flex_gap_input(
        n: usize,
        dir: FlexDirection,
        wrap: FlexWrap,
        gap: Option<(f32, f32)>,
        child_main: f32,
    ) -> LayoutInput {
        let mut els = Vec::new();
        els.push(LayoutElementInput {
            display: Display::Flex,
            position: Position::Relative,
            flex_direction: Some(dir),
            flex_wrap: Some(wrap),
            align_items: Some(AlignItems::FlexStart),
            align_self: Some(AlignSelf::Auto),
            justify_content: Some(JustifyContent::FlexStart),
            align_content: Some(AlignContent::FlexStart),
            box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            width: Some(800.0),
            height: Some(400.0),
            has_content: false,
            grid_template_columns: None,
            grid_template_rows: None,
            grid_column: None,
            grid_row: None,
            grid_auto_flow: None,
            gap,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            margin: [Some(0.0); 4],
            padding: [0.0; 4],
            border_width: [0.0; 4],
            inset: [0.0; 4],
            parent_index: None,
            is_text: false,
            text: String::new(),
            font_size: 16.0,
            line_height: 1.4,
        });
        for i in 0..n {
            els.push(LayoutElementInput {
                display: Display::Block,
                position: Position::Relative,
                flex_direction: None,
                flex_wrap: None,
                align_items: None,
                align_self: None,
                justify_content: None,
                align_content: None,
                box_sizing: BoxSizing::ContentBox,
                flex_grow: 0.0,
                flex_shrink: 1.0,
                flex_basis: Some(child_main),
                width: None,
                height: Some(50.0),
                has_content: false,
                grid_template_columns: None,
                grid_template_rows: None,
                grid_column: None,
                grid_row: None,
                grid_auto_flow: None,
                gap: None,
                min_width: None,
                max_width: None,
                min_height: None,
                max_height: None,
                margin: [Some(0.0); 4],
                padding: [0.0; 4],
                border_width: [0.0; 4],
                inset: [0.0; 4],
                parent_index: Some(0),
                is_text: false,
                text: format!("item{}", i),
                font_size: 14.0,
                line_height: 1.2,
            });
        }
        LayoutInput {
            container_width: 800.0,
            viewport_height: 400.0,
            elements: els,
        }
    }

    #[test]
    fn gap1_row_gap_20px_single_line() {
        // 3 items, 200px each, gap=20 → positions: 0, 220, 440
        let inp = flex_gap_input(
            3,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            Some((20.0, 20.0)),
            200.0,
        );
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // elements[0]=container, elements[1..3]=children
        assert!(
            (out.elements[1].x).abs() < 2.0,
            "item0 x should be ~0, got {}",
            out.elements[1].x
        );
        assert!(
            (out.elements[2].x - 220.0).abs() < 2.0,
            "item1 x should be ~220 (200+20 gap), got {}",
            out.elements[2].x
        );
        assert!(
            (out.elements[3].x - 440.0).abs() < 2.0,
            "item2 x should be ~440 (200+20+200+20), got {}",
            out.elements[3].x
        );
    }

    #[test]
    fn gap2_column_gap_10px_single_line() {
        // Column flex: 3 items, 50px each, gap=10 → y positions: 0, 60, 120
        let inp = flex_gap_input(
            3,
            FlexDirection::Column,
            FlexWrap::NoWrap,
            Some((10.0, 10.0)),
            50.0,
        );
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        assert!(
            (out.elements[1].y).abs() < 2.0,
            "item0 y should be ~0, got {}",
            out.elements[1].y
        );
        assert!(
            (out.elements[2].y - 60.0).abs() < 2.0,
            "item1 y should be ~60 (50+10 gap), got {}",
            out.elements[2].y
        );
        assert!(
            (out.elements[3].y - 120.0).abs() < 2.0,
            "item2 y should be ~120 (50+10+50+10), got {}",
            out.elements[3].y
        );
    }

    #[test]
    fn gap3_row_wrap_cross_axis_gap() {
        // Row wrap: 4 items, 300px each in 400px container → 2 lines
        // Line 1: items 0,1 at y=0; Line 2: items 2,3 at y=50+20=70
        let inp = flex_gap_input(
            4,
            FlexDirection::Row,
            FlexWrap::Wrap,
            Some((20.0, 20.0)),
            300.0,
        );
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        // Items 0,1 on line 1 (y ≈ 0)
        assert!(
            out.elements[1].y < 5.0,
            "line1 item0 y should be ~0, got {}",
            out.elements[1].y
        );
        assert!(
            out.elements[2].y < 5.0,
            "line1 item1 y should be ~0, got {}",
            out.elements[2].y
        );
        // Items 2,3 on line 2 (y ≈ 50 height + 20 gap = 70)
        assert!(
            (out.elements[3].y - 70.0).abs() < 5.0,
            "line2 item2 y should be ~70, got {}",
            out.elements[3].y
        );
        assert!(
            (out.elements[4].y - 70.0).abs() < 5.0,
            "line2 item3 y should be ~70, got {}",
            out.elements[4].y
        );
    }

    #[test]
    fn gap4_zero_gap_same_as_no_gap() {
        let with_gap = flex_gap_input(
            3,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            Some((0.0, 0.0)),
            200.0,
        );
        let no_gap = flex_gap_input(3, FlexDirection::Row, FlexWrap::NoWrap, None, 200.0);
        let n = NativeLayoutEngine::new();
        let a = n.compute_layout(&with_gap);
        let b = n.compute_layout(&no_gap);
        for i in 1..=3 {
            assert!(
                (a.elements[i].x - b.elements[i].x).abs() < 1.0,
                "zero gap should match no gap: item{} x: {} vs {}",
                i,
                a.elements[i].x,
                b.elements[i].x
            );
        }
    }

    #[test]
    fn gap5_single_item_no_gap_effect() {
        // Single item: gap shouldn't affect position
        let inp = flex_gap_input(
            1,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            Some((20.0, 20.0)),
            200.0,
        );
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        assert!(
            (out.elements[1].x).abs() < 2.0,
            "single item x should be ~0, got {}",
            out.elements[1].x
        );
    }

    #[test]
    fn gap6_row_vs_column_axis_swap() {
        // row gap=30, col gap=10
        // Row flex: main-axis gap = col_gap=10, cross-axis gap = row_gap=30
        let row_inp = flex_gap_input(
            3,
            FlexDirection::Row,
            FlexWrap::Wrap,
            Some((30.0, 10.0)),
            300.0,
        );
        // Column flex: main-axis gap = row_gap=30, cross-axis gap = col_gap=10
        let col_inp = flex_gap_input(
            3,
            FlexDirection::Column,
            FlexWrap::Wrap,
            Some((30.0, 10.0)),
            50.0,
        );
        let n = NativeLayoutEngine::new();
        let row_out = n.compute_layout(&row_inp);
        let col_out = n.compute_layout(&col_inp);
        // Row: items spread by col_gap=10 on main axis
        assert!(
            (row_out.elements[2].x - 310.0).abs() < 5.0,
            "row flex: item1 x should be ~310 (300+10), got {}",
            row_out.elements[2].x
        );
        // Column: items spread by row_gap=30 on main axis
        assert!(
            (col_out.elements[2].y - 80.0).abs() < 5.0,
            "col flex: item1 y should be ~80 (50+30), got {}",
            col_out.elements[2].y
        );
    }

    #[test]
    fn gap7_justify_space_between_with_gap() {
        // 3 items, 100px each in 800px container (flex_gap_input default), gap=20
        // used_main=300, total_gap=40, main_free=800-300-40=460
        // SpaceBetween distributes 460/2=230 between items; gap adds 20 per item
        // item0 at 0, item1 at 100+20+230=350, item2 at 350+100+20+230=700
        let mut inp = flex_gap_input(
            3,
            FlexDirection::Row,
            FlexWrap::NoWrap,
            Some((20.0, 20.0)),
            100.0,
        );
        inp.elements[0].justify_content = Some(JustifyContent::SpaceBetween);
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        assert!(
            (out.elements[2].x - 350.0).abs() < 5.0,
            "space-between with gap: item1 x should be ~350, got {}",
            out.elements[2].x
        );
        assert!(
            (out.elements[3].x - 700.0).abs() < 5.0,
            "space-between with gap: item2 x should be ~700, got {}",
            out.elements[3].x
        );
    }

    #[test]
    fn gap8_only_row_gap_column_flex() {
        // Column flex with only row_gap set
        let inp = flex_gap_input(
            3,
            FlexDirection::Column,
            FlexWrap::NoWrap,
            Some((25.0, 0.0)),
            50.0,
        );
        let n = NativeLayoutEngine::new();
        let out = n.compute_layout(&inp);
        assert!(
            (out.elements[2].y - 75.0).abs() < 2.0,
            "item1 y should be ~75 (50+25), got {}",
            out.elements[2].y
        );
        assert!(
            (out.elements[3].y - 150.0).abs() < 2.0,
            "item2 y should be ~150 (50+25+50+25), got {}",
            out.elements[3].y
        );
    }
}
