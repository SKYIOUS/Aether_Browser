//! Layout engine adapter — bridges pipeline's StyledElement to layout-engine crate.

use super::extractor::{BoxSizing as ElBoxSizing, StyledElement};
use crate::engine::stratus as css;
use layout_engine::{
    BoxSizing as LayoutBoxSizing, LayoutElementInput, LayoutElementOutput, LayoutEngine,
    LayoutInput,
};

#[cfg(feature = "layout-native")]
use layout_engine::NativeLayoutEngine;
#[cfg(feature = "layout-taffy")]
use layout_engine::TaffyLayoutEngine;

/// Convert StyledElement to LayoutElementInput
fn styled_to_layout_input(el: &StyledElement, _container_width: f32) -> LayoutElementInput {
    let fs = if el.font_size.is_finite() {
        el.font_size.clamp(6.0, 200.0)
    } else {
        16.0
    };

    LayoutElementInput {
        display: el.display,
        position: el.position,
        flex_direction: Some(el.flex_direction),
        flex_wrap: Some(el.flex_wrap),
        align_items: Some(el.align_items),
        align_self: Some(el.align_self),
        justify_content: Some(el.justify_content),
        align_content: Some(el.align_content),
        box_sizing: match el.box_sizing {
            ElBoxSizing::BorderBox => LayoutBoxSizing::BorderBox,
            ElBoxSizing::ContentBox => LayoutBoxSizing::ContentBox,
        },
        flex_grow: el.flex_grow,
        flex_shrink: el.flex_shrink,
        flex_basis: el.flex_basis,
        width: el.css_width,
        height: el.css_height,
        min_width: el.min_width,
        max_width: el.max_width,
        min_height: el.min_height,
        max_height: el.max_height,
        margin: [
            Some(el.margin_top),
            el.margin_right.or(Some(0.0)),
            Some(el.margin_bottom),
            el.margin_left.or(Some(0.0)),
        ],
        padding: el.padding,
        border_width: el.border_widths,
        inset: [el.inset_top, el.inset_right, el.inset_bottom, el.inset_left],
        parent_index: el.parent_index,
        is_text: el.text.is_empty() && el.wrapped_lines.is_empty() && el.image_handle.is_none(),
        text: el.text.clone(),
        font_size: fs,
        line_height: el.line_height.max(1.0),
        has_content: !el.wrapped_lines.is_empty()
            || el.image_handle.is_some()
            || el.css_width.is_some()
            || el.css_height.is_some(),
        grid_template_columns: None,
        grid_template_rows: None,
        grid_column: None,
        grid_row: None,
        grid_auto_flow: None,
        gap: None,
    }
}

/// Convert LayoutElementOutput back to StyledElement fields
fn apply_layout_output(elements: &mut [StyledElement], outputs: &[LayoutElementOutput]) {
    for (el, out) in elements.iter_mut().zip(outputs.iter()) {
        el.x = out.x;
        el.y = out.y;
        el.width = out.width;
        el.height = out.height;
    }
}

/// Get the layout engine instance
#[cfg(all(feature = "layout-taffy", not(feature = "layout-native")))]
fn get_layout_engine() -> &'static dyn LayoutEngine {
    use std::sync::OnceLock;
    static ENGINE: OnceLock<TaffyLayoutEngine> = OnceLock::new();
    ENGINE.get_or_init(|| TaffyLayoutEngine::new())
}

#[cfg(feature = "layout-native")]
fn get_layout_engine() -> &'static dyn LayoutEngine {
    use std::sync::OnceLock;
    static ENGINE: OnceLock<NativeLayoutEngine> = OnceLock::new();
    ENGINE.get_or_init(|| NativeLayoutEngine::new())
}

#[cfg(not(any(feature = "layout-taffy", feature = "layout-native")))]
fn get_layout_engine() -> &'static dyn LayoutEngine {
    panic!("No layout engine available. Enable 'layout-taffy' or 'layout-native' feature.")
}

/// Main entry point — replaces apply_taffy_layout
pub fn apply_layout(elements: &mut [StyledElement], container_width: f32, viewport_h: f32) {
    if elements.is_empty() {
        return;
    }

    let valid_count = elements
        .iter()
        .filter(|el| el.display != css::Display::None)
        .count();
    if valid_count == 0 {
        return;
    }

    // Text wrapping for all elements (including empty text to match original behavior)
    for el in elements.iter_mut() {
        if el.display == css::Display::None {
            continue;
        }
        let fs = if el.font_size.is_finite() {
            el.font_size.clamp(6.0, 200.0)
        } else {
            16.0
        };
        let pb = el.padding[1] + el.padding[3] + el.border_widths[1] + el.border_widths[3];
        let available_width = el.css_width.unwrap_or(container_width) - pb;
        let available = if available_width.is_finite() && available_width > 0.0 {
            available_width
        } else {
            container_width
        };
        let lines = crate::engine::pipeline::layout::wrap_text(&el.text, available, fs);
        if el.css_height.is_none() {
            el.css_height = Some(fs * el.line_height.max(1.0) * lines.len() as f32);
        }
        el.wrapped_lines = lines;
        if el.display == css::Display::Inline && el.min_width.is_none() && el.css_width.is_none() {
            let text_w = crate::engine::text::measure_text_width(&el.text, fs);
            el.min_width = Some(text_w + pb);
        }
    }

    // Build layout input
    let layout_input = LayoutInput {
        container_width,
        viewport_height: viewport_h,
        elements: elements
            .iter()
            .map(|el| styled_to_layout_input(el, container_width))
            .collect(),
    };

    // Compute layout via abstraction
    let engine = get_layout_engine();
    let output = engine.compute_layout(&layout_input);

    // Apply results
    apply_layout_output(elements, &output.elements);

    // Post-process: inline children positioning
    post_process_inline_children(elements, container_width);
}

/// Post-process inline children (same logic as before)
fn post_process_inline_children(elements: &mut [StyledElement], _container_width: f32) {
    let n = elements.len();
    if n == 0 {
        return;
    }

    // Build inline children map - O(n)
    let mut inline_children: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        if elements[i].display == css::Display::Inline {
            if let Some(pidx) = elements[i].parent_index {
                if pidx < n {
                    inline_children[pidx].push(i);
                }
            }
        }
    }

    // Process inline children
    for i in 0..n {
        let children = &inline_children[i];
        if children.is_empty() {
            continue;
        }
        if elements[i].display == css::Display::None || elements[i].display == css::Display::Inline
        {
            continue;
        }
        let fs = if elements[i].font_size.is_finite() {
            elements[i].font_size.clamp(6.0, 200.0)
        } else {
            16.0
        };
        let mut cumulative = crate::engine::text::measure_text_width(&elements[i].text, fs);
        for &child_idx in children {
            elements[child_idx].x = elements[i].x + cumulative;
            let child_w = if elements[child_idx].css_width.is_some() {
                elements[child_idx].width
            } else {
                let child_text_w =
                    crate::engine::text::measure_text_width(&elements[child_idx].text, fs);
                elements[child_idx].width = child_text_w.max(fs);
                elements[child_idx].width
            };
            if elements[child_idx].css_height.is_none() {
                elements[child_idx].height = fs * elements[i].line_height.max(1.0);
            }
            cumulative += child_w;
        }
    }
}

// Re-export for backward compatibility
pub use crate::engine::pipeline::layout::{
    apply_taffy_layout as apply_taffy_layout_legacy, get_layout_pass_count,
    reset_layout_pass_count, wrap_text,
};

/// Backward-compatible wrapper — now uses the layout-engine abstraction
pub fn apply_taffy_layout(elements: &mut [StyledElement], container_width: f32, viewport_h: f32) {
    apply_layout(elements, container_width, viewport_h);
}
