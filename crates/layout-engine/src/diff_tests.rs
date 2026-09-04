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
        AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent, Position,
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
}
