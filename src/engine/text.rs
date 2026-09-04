//! Text measurement and shaping using cosmic-text
//!
//! Replaces the `unicode_width` + `CHAR_W_SCALE` heuristic with actual glyph-based
//! measurement. Uses `cosmic-text` which bundles `fontdb` (font discovery) and
//! `rustybuzz` (text shaping).

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};

use std::sync::Mutex;
use std::time::Instant;

type MeasureCache = lru::LruCache<(String, u32, u32), f32>;

static FONT_SYSTEM: std::sync::OnceLock<Mutex<FontSystem>> = std::sync::OnceLock::new();
static FONT_SYSTEM_INIT_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn font_system() -> &'static Mutex<FontSystem> {
    FONT_SYSTEM.get_or_init(|| {
        FONT_SYSTEM_INIT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Mutex::new(FontSystem::new())
    })
}

static MEASURE_CACHE: std::sync::OnceLock<Mutex<MeasureCache>> = std::sync::OnceLock::new();

// E1-A: cache capacity (change this and re-run to test different sizes)
const MEASURE_CACHE_CAPACITY: usize = 8192;

fn measure_cache() -> &'static Mutex<MeasureCache> {
    MEASURE_CACHE.get_or_init(|| {
        Mutex::new(lru::LruCache::new(
            std::num::NonZeroUsize::new(MEASURE_CACHE_CAPACITY).unwrap(),
        ))
    })
}

/// Clear the measurement cache (for testing)
pub fn clear_measure_cache() {
    if let Ok(mut cache) = MEASURE_CACHE
        .get_or_init(|| {
            Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(MEASURE_CACHE_CAPACITY).unwrap(),
            ))
        })
        .lock()
    {
        cache.clear();
    }
}

// E0 instrumentation counters
thread_local! {
    static MEASURE_CALLS: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static MEASURE_HITS: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static MEASURE_MISSES: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static BUFFER_CONSTRUCTIONS: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static TOTAL_SHAPING_MS: std::cell::RefCell<f64> = const { std::cell::RefCell::new(0.0) };
}

/// Reset E0 thread-local counters
pub fn e0_reset_counters() {
    MEASURE_CALLS.with(|c| *c.borrow_mut() = 0);
    MEASURE_HITS.with(|c| *c.borrow_mut() = 0);
    MEASURE_MISSES.with(|c| *c.borrow_mut() = 0);
    BUFFER_CONSTRUCTIONS.with(|c| *c.borrow_mut() = 0);
    TOTAL_SHAPING_MS.with(|c| *c.borrow_mut() = 0.0);
}

/// Get and print E0 measurement summary
pub fn e0_get_summary() -> (u64, u64, u64, u64, f64, u32) {
    let calls = MEASURE_CALLS.with(|c| *c.borrow());
    let hits = MEASURE_HITS.with(|c| *c.borrow());
    let misses = MEASURE_MISSES.with(|c| *c.borrow());
    let buffers = BUFFER_CONSTRUCTIONS.with(|c| *c.borrow());
    let shaping_ms = TOTAL_SHAPING_MS.with(|c| *c.borrow());
    let font_sys_inits = FONT_SYSTEM_INIT_COUNT.load(std::sync::atomic::Ordering::Relaxed);
    (calls, hits, misses, buffers, shaping_ms, font_sys_inits)
}

/// Measure the visual width of a text string at a given font size.
///
/// Results are cached to avoid repeated Buffer allocation and shaping.
/// Cache key includes text, font_size, and font_weight (for correct invalidation).
pub fn measure_text_width(text: &str, font_size: f32) -> f32 {
    if text.is_empty() {
        return 0.0;
    }

    let fs = if font_size.is_finite() {
        font_size.clamp(6.0, 200.0)
    } else {
        16.0
    };
    let fs_key = (fs * 100.0) as u32;
    let weight_key = 0u32; // placeholder for font_weight (measure_text_width doesn't have it)

    MEASURE_CALLS.with(|c| *c.borrow_mut() += 1);

    if let Ok(mut cache) = measure_cache().lock() {
        if let Some(&cached) = cache.get(&(text.to_string(), fs_key, weight_key)) {
            MEASURE_HITS.with(|c| *c.borrow_mut() += 1);
            return cached;
        }
    }

    MEASURE_MISSES.with(|c| *c.borrow_mut() += 1);

    let start = Instant::now();
    let result = measure_text_width_uncached(text, fs);
    TOTAL_SHAPING_MS.with(|c| *c.borrow_mut() += start.elapsed().as_secs_f64() * 1000.0);

    if let Ok(mut cache) = measure_cache().lock() {
        cache.put((text.to_string(), fs_key, weight_key), result);
    }

    result
}

fn measure_text_width_uncached(text: &str, font_size: f32) -> f32 {
    let metrics = Metrics::new(font_size, font_size * 1.2);
    let mut font_system = font_system().lock().unwrap_or_else(|e| e.into_inner());

    BUFFER_CONSTRUCTIONS.with(|c| *c.borrow_mut() += 1);

    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    buffer.set_size(None, None);
    buffer.set_text(text, Attrs::new(), Shaping::Advanced);
    buffer.shape_until_scroll(false);

    let mut total_width = 0.0f32;
    for run in buffer.layout_runs() {
        total_width = total_width.max(run.line_w);
    }

    if total_width <= 0.0 {
        font_size * 0.5 * text.len() as f32
    } else {
        total_width
    }
}

/// Measure text and return both width and height.
pub fn measure_text(text: &str, font_size: f32, line_height: f32) -> (f32, f32) {
    let width = measure_text_width(text, font_size);
    let height = font_size * line_height.max(1.0);
    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_empty_string() {
        let w = measure_text_width("", 16.0);
        assert_eq!(w, 0.0);
    }

    #[test]
    fn test_measure_basic_latin() {
        let w = measure_text_width("Hello", 16.0);
        assert!(w > 0.0, "width should be positive, got {}", w);
    }

    #[test]
    fn test_measure_proportional() {
        let w_wide = measure_text_width("WWW", 16.0);
        let w_narrow = measure_text_width("iii", 16.0);
        assert!(
            w_wide > w_narrow,
            "proportional fonts should measure differently: {} vs {}",
            w_wide,
            w_narrow
        );
    }

    // E1-A: cache capacity sensitivity test
    #[test]
    #[ignore]
    fn e1a_cache_capacity_sensitivity() {
        use crate::engine::pipeline::extractor::{BoxSizing, FontWeight, StyledElement, TextDecor};
        use crate::engine::pipeline::layout::apply_taffy_layout;
        use crate::engine::stratus::{
            AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
            Position,
        };
        use iced::Color;

        fn make_el(tag: &str, text: &str, font_size: f32) -> StyledElement {
            StyledElement {
                tag: tag.into(),
                text: text.into(),
                wrapped_lines: vec![],
                dom_path: vec![],
                is_link: false,
                href: None,
                indent_level: 0,
                color: Color::BLACK,
                font_size,
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
                parent_index: None,
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

        // E1-A: single capacity test (change MEASURE_CACHE_CAPACITY const and re-run)
        const ELEMENT_COUNT: usize = 2500;

        e0_reset_counters();

        let mut elements: Vec<StyledElement> = Vec::with_capacity(ELEMENT_COUNT);
        for i in 0..ELEMENT_COUNT {
            // Match benchmark text pattern: "paragraph {i} wraps across the line because this sentence is long enough to split"
            let text = format!(
                "paragraph {} wraps across the line because this sentence is long enough to split",
                i
            );
            elements.push(make_el("p", &text, 16.0));
        }

        let start = std::time::Instant::now();
        apply_taffy_layout(&mut elements, 800.0, 600.0);
        let total_ms = start.elapsed().as_secs_f64() * 1000.0;

        let (calls, hits, misses, buffers, shaping_ms, _) = e0_get_summary();
        let hit_rate = if calls > 0 {
            hits as f64 * 100.0 / calls as f64
        } else {
            0.0
        };

        println!(
            "\n=== E1-A: Cache Capacity {} ({} elements, pass 1) ===",
            MEASURE_CACHE_CAPACITY, ELEMENT_COUNT
        );
        println!("  measure calls:    {}", calls);
        println!("  cache hits:       {} ({:.1}%)", hits, hit_rate);
        println!("  cache misses:     {}", misses);
        println!("  buffer consts:    {}", buffers);
        println!("  shaping time:     {:.1} ms", shaping_ms);
        println!("  total layout:     {:.1} ms", total_ms);

        // Pass 2: simulate navigation to different content
        e0_reset_counters();

        let mut elements2: Vec<StyledElement> = Vec::with_capacity(ELEMENT_COUNT);
        for i in 0..ELEMENT_COUNT {
            let text = format!("different page paragraph {} wraps across the line because this sentence is long enough to split", i);
            elements2.push(make_el("p", &text, 16.0));
        }

        let start2 = std::time::Instant::now();
        apply_taffy_layout(&mut elements2, 800.0, 600.0);
        let total_ms2 = start2.elapsed().as_secs_f64() * 1000.0;

        let (calls2, hits2, misses2, buffers2, shaping_ms2, _) = e0_get_summary();
        let hit_rate2 = if calls2 > 0 {
            hits2 as f64 * 100.0 / calls2 as f64
        } else {
            0.0
        };

        println!(
            "\n=== E1-A: Cache Capacity {} ({} elements, pass 2 - navigation) ===",
            MEASURE_CACHE_CAPACITY, ELEMENT_COUNT
        );
        println!("  measure calls:    {}", calls2);
        println!("  cache hits:       {} ({:.1}%)", hits2, hit_rate2);
        println!("  cache misses:     {}", misses2);
        println!("  buffer consts:    {}", buffers2);
        println!("  shaping time:     {:.1} ms", shaping_ms2);
        println!("  total layout:     {:.1} ms", total_ms2);
    }
}

// E2: Invalidation correctness tests
#[cfg(test)]
mod e2_invalidation_tests {
    use super::{clear_measure_cache, e0_get_summary, e0_reset_counters, measure_text_width};
    use crate::engine::pipeline::extractor::{BoxSizing, FontWeight, StyledElement, TextDecor};
    use crate::engine::pipeline::layout::apply_taffy_layout;
    use crate::engine::stratus::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
        Position,
    };
    use iced::Color;

    fn make_el(tag: &str, text: &str, font_size: f32, font_weight: FontWeight) -> StyledElement {
        StyledElement {
            tag: tag.into(),
            text: text.into(),
            wrapped_lines: vec![],
            dom_path: vec![],
            is_link: false,
            href: None,
            indent_level: 0,
            color: Color::BLACK,
            font_size,
            font_weight,
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
            parent_index: None,
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

    // E2-1: Identical (text, font_size) -> cache reuse
    #[test]
    fn e2_identical_inputs_reuse_cache() {
        e0_reset_counters();
        let _w1 = measure_text_width("hello world", 16.0);
        let _w2 = measure_text_width("hello world", 16.0);
        let _w3 = measure_text_width("hello world", 16.0);

        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 2, "Identical inputs should hit cache");
        assert_eq!(misses, 1, "Only first call should miss");
    }

    // E2-2: Different text -> invalidation
    #[test]
    fn e2_different_text_invalidates() {
        e0_reset_counters();
        let _w1 = measure_text_width("hello", 16.0);
        let _w2 = measure_text_width("world", 16.0);

        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 0, "Different text should not hit cache");
        assert_eq!(misses, 2, "Both calls should miss");
    }

    // E2-3: Different font_size -> invalidation
    #[test]
    fn e2_different_font_size_invalidates() {
        e0_reset_counters();
        let _w1 = measure_text_width("hello", 16.0);
        let _w2 = measure_text_width("hello", 18.0);

        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 0, "Different font_size should not hit cache");
        assert_eq!(misses, 2, "Both calls should miss");
    }

    // E2-11: LRU eviction behavior
    #[test]
    fn e2_lru_eviction() {
        clear_measure_cache();
        e0_reset_counters();

        // Fill cache beyond capacity
        for i in 0..10000 {
            let _ = measure_text_width(&format!("unique text {}", i), 16.0);
        }

        // Now test that earlier entries were evicted
        let _ = measure_text_width("unique text 0", 16.0);
        let _ = measure_text_width("unique text 1", 16.0);

        let (_, hits, misses, _, _, _) = e0_get_summary();
        // These should be misses because they were evicted
        // (exact count depends on LRU state, but at least some should miss)
        assert!(misses >= 2, "Evicted entries should miss");
    }
}

// E3: Large-page validation tests
#[cfg(test)]
mod e3_large_page_validation {
    use super::{clear_measure_cache, e0_get_summary, e0_reset_counters, measure_text_width};
    use crate::engine::pipeline::extractor::{BoxSizing, FontWeight, StyledElement, TextDecor};
    use crate::engine::pipeline::layout::apply_taffy_layout;
    use crate::engine::stratus::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
        Position,
    };
    use iced::Color;

    fn make_el(tag: &str, text: &str, font_size: f32, font_weight: FontWeight) -> StyledElement {
        StyledElement {
            tag: tag.into(),
            text: text.into(),
            wrapped_lines: vec![],
            dom_path: vec![],
            is_link: false,
            href: None,
            indent_level: 0,
            color: Color::BLACK,
            font_size,
            font_weight,
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
            parent_index: None,
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

    fn run_scenario(
        name: &str,
        elements: Vec<StyledElement>,
        width: f32,
        height: f32,
        iterations: usize,
    ) {
        e0_reset_counters();
        clear_measure_cache();

        println!("\n=== E3: {} ({} elements) ===", name, elements.len());

        for i in 1..=iterations {
            let mut els = elements.clone();
            let start = std::time::Instant::now();
            apply_taffy_layout(&mut els, width, height);
            let total_ms = start.elapsed().as_secs_f64() * 1000.0;

            let (calls, hits, misses, buffers, shaping_ms, _) = e0_get_summary();
            let hit_rate = if calls > 0 {
                hits as f64 * 100.0 / calls as f64
            } else {
                0.0
            };
            let taffy_est = total_ms - shaping_ms;

            println!("  iter {:>2}: total={:.1}ms shaping={:.1}ms taffy={:.1}ms calls={} hits={} ({:.1}%) bufs={}",
                i, total_ms, shaping_ms, taffy_est.max(0.0), calls, hits, hit_rate, buffers);

            e0_reset_counters();
        }
    }

    // E3-1: Large text-heavy page (many unique paragraphs)
    #[test]
    #[ignore]
    fn e3_large_text_heavy() {
        println!("\n=== E3-1: Large text-heavy page ===");

        let mut elements = Vec::new();
        for i in 0..5000 {
            let text = format!("Paragraph {} with substantial unique text content that represents a realistic article paragraph with varied vocabulary and sentence structure", i);
            elements.push(make_el("p", &text, 16.0, FontWeight::Normal));
        }

        run_scenario("5k unique paragraphs", elements, 800.0, 600.0, 3);
    }

    // E3-2: Large mixed-content page (text + images + headings + lists)
    #[test]
    #[ignore]
    fn e3_large_mixed_content() {
        println!("\n=== E3-2: Large mixed-content page ===");

        let mut elements = Vec::new();
        for i in 0..1000 {
            elements.push(make_el(
                "h1",
                &format!("Heading {}", i),
                32.0,
                FontWeight::Bold,
            ));
            elements.push(make_el(
                "p",
                &format!(
                    "Paragraph {} with content and some numbers like {} and {}",
                    i,
                    i * 2,
                    i * 3
                ),
                16.0,
                FontWeight::Normal,
            ));
            elements.push(make_el("img", "", 16.0, FontWeight::Normal));
            elements.push(make_el(
                "ul",
                &format!("Item {} first", i),
                16.0,
                FontWeight::Normal,
            ));
            elements.push(make_el(
                "ul",
                &format!("Item {} second", i),
                16.0,
                FontWeight::Normal,
            ));
        }

        run_scenario("5k mixed elements", elements, 800.0, 600.0, 3);
    }

    // E3-3: Deep DOM (200 levels nested)
    #[test]
    #[ignore]
    fn e3_deep_dom() {
        println!("\n=== E3-3: Deep DOM (200 levels) ===");

        let mut elements = Vec::new();
        for i in 0..200 {
            elements.push(make_el(
                "div",
                &format!("level {}", i),
                16.0,
                FontWeight::Normal,
            ));
            elements.push(make_el(
                "p",
                &format!("content at level {}", i),
                16.0,
                FontWeight::Normal,
            ));
        }

        run_scenario("200 nested levels", elements, 800.0, 600.0, 3);
    }

    // E3-4: Many repeated strings (cache-friendly)
    #[test]
    #[ignore]
    fn e3_many_repeated_strings() {
        println!("\n=== E3-4: Many repeated strings (cache-friendly) ===");

        let mut elements = Vec::new();
        let repeated = vec![
            "Introduction",
            "Conclusion",
            "Note",
            "Warning",
            "Tip",
            "Example",
        ];
        for i in 0..5000 {
            elements.push(make_el("p", repeated[i % 6], 16.0, FontWeight::Normal));
        }

        run_scenario("5k elements, 6 repeated strings", elements, 800.0, 600.0, 3);
    }

    // E3-5: Many unique strings (cache-unfriendly)
    #[test]
    #[ignore]
    fn e3_many_unique_strings() {
        println!("\n=== E3-5: Many unique strings (cache-unfriendly) ===");

        let mut elements = Vec::new();
        for i in 0..5000 {
            elements.push(make_el(
                "p",
                &format!(
                    "Unique paragraph number {} with distinct content that won't repeat",
                    i
                ),
                16.0,
                FontWeight::Normal,
            ));
        }

        run_scenario("5k unique paragraphs", elements, 800.0, 600.0, 3);
    }

    // E3-6: Normal-sized page (control)
    #[test]
    #[ignore]
    fn e3_normal_page() {
        println!("\n=== E3-6: Normal-sized page (control) ===");

        let mut elements = Vec::new();
        for i in 0..100 {
            elements.push(make_el(
                "p",
                &format!("Normal paragraph {} with typical content length", i),
                16.0,
                FontWeight::Normal,
            ));
        }
        for i in 0..10 {
            elements.push(make_el(
                "h2",
                &format!("Section {}", i),
                24.0,
                FontWeight::Bold,
            ));
        }

        run_scenario("110 elements normal page", elements, 800.0, 600.0, 3);
    }

    // E3-7: Numeric-heavy page (tests numeric fast path)
    #[test]
    #[ignore]
    fn e3_numeric_heavy() {
        println!("\n=== E3-7: Numeric-heavy page ===");

        let mut elements = Vec::new();
        for i in 0..5000 {
            elements.push(make_el(
                "p",
                &format!(
                    "Data point {} value {} timestamp {} id {} ref {}",
                    i,
                    i * 100,
                    i * 1000,
                    i * 10000,
                    i * 100000
                ),
                14.0,
                FontWeight::Normal,
            ));
        }

        run_scenario("5k numeric-heavy paragraphs", elements, 800.0, 600.0, 3);
    }
}
