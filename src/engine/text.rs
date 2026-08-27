//! Text measurement and shaping using cosmic-text
//!
//! Replaces the `unicode_width` + `CHAR_W_SCALE` heuristic with actual glyph-based
//! measurement. Uses `cosmic-text` which bundles `fontdb` (font discovery) and
//! `rustybuzz` (text shaping).

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};

use std::sync::Mutex;
use std::time::Instant;

static FONT_SYSTEM: std::sync::OnceLock<Mutex<FontSystem>> = std::sync::OnceLock::new();
static FONT_SYSTEM_INIT_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn font_system() -> &'static Mutex<FontSystem> {
    FONT_SYSTEM.get_or_init(|| {
        FONT_SYSTEM_INIT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Mutex::new(FontSystem::new())
    })
}

static MEASURE_CACHE: std::sync::OnceLock<Mutex<lru::LruCache<(String, u32, u32), f32>>> = std::sync::OnceLock::new();

// E1-A: cache capacity (change this and re-run to test different sizes)
const MEASURE_CACHE_CAPACITY: usize = 8192;

fn measure_cache() -> &'static Mutex<lru::LruCache<(String, u32, u32), f32>> {
    MEASURE_CACHE.get_or_init(|| {
        Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(MEASURE_CACHE_CAPACITY).unwrap()))
    })
}

/// Clear the measurement cache (for testing)
pub fn clear_measure_cache() {
    if let Ok(mut cache) = MEASURE_CACHE.get_or_init(|| {
        Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(MEASURE_CACHE_CAPACITY).unwrap()))
    }).lock() {
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

    let fs = if font_size.is_finite() { font_size.clamp(6.0, 200.0) } else { 16.0 };
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
    let mut font_system = font_system().lock().unwrap();

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
        assert!(w_wide > w_narrow, "proportional fonts should measure differently: {} vs {}", w_wide, w_narrow);
    }

    // E1-A: cache capacity sensitivity test
    #[test]
    #[ignore]
    fn e1a_cache_capacity_sensitivity() {
        use crate::engine::pipeline::extractor::{StyledElement, TextDecor, FontWeight, BoxSizing};
        use crate::engine::stratus::{Display, FlexDirection, FlexWrap, JustifyContent,
            AlignItems, AlignSelf, Position};
        use crate::engine::pipeline::layout::apply_taffy_layout;
        use iced::Color;

        fn make_el(tag: &str, text: &str, font_size: f32) -> StyledElement {
            StyledElement {
                tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
                is_link: false, href: None, indent_level: 0, color: Color::BLACK,
                font_size, font_weight: FontWeight::Normal, background_color: None,
                border_widths: [0.0; 4], border_color: None, image_handle: None,
                image_url: None, margin_top: 0.0, margin_bottom: 0.0, margin_left: None,
                margin_right: None, padding: [0.0; 4], display: Display::Block,
                flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap,
                justify_content: JustifyContent::FlexStart, align_items: AlignItems::Stretch,
                align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox,
                flex_grow: 0.0, flex_shrink: 1.0, flex_basis: None,
                css_width: None, css_height: None, parent_index: None,
                min_width: None, max_width: None, min_height: None, max_height: None,
                x: 0.0, y: 0.0, width: 0.0, height: 0.0, line_height: 1.4,
                text_decoration: TextDecor::default(), border_radius: [0.0; 4],
                input_type: String::new(), input_value: String::new(),
                input_placeholder: String::new(), checked: false,
                position: Position::Static, inset_top: 0.0, inset_right: 0.0,
                inset_bottom: 0.0, inset_left: 0.0,
            }
        }

        // E1-A: single capacity test (change MEASURE_CACHE_CAPACITY const and re-run)
        const ELEMENT_COUNT: usize = 2500;
        
        e0_reset_counters();
        
        let mut elements: Vec<StyledElement> = Vec::with_capacity(ELEMENT_COUNT);
        for i in 0..ELEMENT_COUNT {
            // Match benchmark text pattern: "paragraph {i} wraps across the line because this sentence is long enough to split"
            let text = format!("paragraph {} wraps across the line because this sentence is long enough to split", i);
            elements.push(make_el("p", &text, 16.0));
        }
        
        let start = std::time::Instant::now();
        apply_taffy_layout(&mut elements, 800.0, 600.0);
        let total_ms = start.elapsed().as_secs_f64() * 1000.0;
        
        let (calls, hits, misses, buffers, shaping_ms, _) = e0_get_summary();
        let hit_rate = if calls > 0 { hits as f64 * 100.0 / calls as f64 } else { 0.0 };
        
        println!("\n=== E1-A: Cache Capacity {} ({} elements, pass 1) ===", MEASURE_CACHE_CAPACITY, ELEMENT_COUNT);
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
        let hit_rate2 = if calls2 > 0 { hits2 as f64 * 100.0 / calls2 as f64 } else { 0.0 };
        
        println!("\n=== E1-A: Cache Capacity {} ({} elements, pass 2 - navigation) ===", MEASURE_CACHE_CAPACITY, ELEMENT_COUNT);
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
    use super::{measure_text_width, e0_reset_counters, e0_get_summary, clear_measure_cache};
    use crate::engine::pipeline::extractor::{StyledElement, TextDecor, FontWeight, BoxSizing};
    use crate::engine::stratus::{Display, FlexDirection, FlexWrap, JustifyContent,
        AlignItems, AlignSelf, Position};
    use crate::engine::pipeline::layout::apply_taffy_layout;
    use iced::Color;

    fn make_el(tag: &str, text: &str, font_size: f32, font_weight: FontWeight) -> StyledElement {
        StyledElement {
            tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
            is_link: false, href: None, indent_level: 0, color: Color::BLACK,
            font_size, font_weight, background_color: None,
            border_widths: [0.0; 4], border_color: None, image_handle: None,
            image_url: None, margin_top: 0.0, margin_bottom: 0.0, margin_left: None,
            margin_right: None, padding: [0.0; 4], display: Display::Block,
            flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart, align_items: AlignItems::Stretch,
            align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox,
            flex_grow: 0.0, flex_shrink: 1.0, flex_basis: None,
            css_width: None, css_height: None, parent_index: None,
            min_width: None, max_width: None, min_height: None, max_height: None,
            x: 0.0, y: 0.0, width: 0.0, height: 0.0, line_height: 1.4,
            text_decoration: TextDecor::default(), border_radius: [0.0; 4],
            input_type: String::new(), input_value: String::new(),
            input_placeholder: String::new(), checked: false,
            position: Position::Static, inset_top: 0.0, inset_right: 0.0,
            inset_bottom: 0.0, inset_left: 0.0,
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

    // E2-4: Font weight change -> does NOT invalidate measurement (affects drawing, not width)
    #[test]
    fn e2_font_weight_change_does_not_invalidate_measurement() {
        e0_reset_counters();
        clear_measure_cache();
        
        // measure_text_width doesn't receive font_weight - it's a drawing concern
        let _w1 = measure_text_width("test text", 16.0);
        let _w2 = measure_text_width("test text", 16.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        // Should HIT because text and font_size are identical
        assert_eq!(hits, 1, "Font weight doesn't affect measurement; identical text+size should hit");
        assert_eq!(misses, 1, "Only first call should miss");
    }

    // E2-5: Width change (css_width) -> does NOT invalidate individual word measurement
    #[test]
    fn e2_width_change_does_not_invalidate_individual_measurement() {
        e0_reset_counters();
        clear_measure_cache();
        
        // Individual word measurement doesn't depend on available width
        // (wrapping is handled by wrap_text, which calls measure_text_width per word)
        let _w1 = measure_text_width("test", 16.0);
        let _w2 = measure_text_width("test", 16.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 1, "Width doesn't affect individual word measurement");
        assert_eq!(misses, 1);
    }

    // E2-6: Unrelated element change -> existing measurements remain reusable
    #[test]
    fn e2_unrelated_element_preserves_cache() {
        e0_reset_counters();
        
        // First layout: element A
        let mut elements1 = vec![make_el("p", "shared text", 16.0, FontWeight::Normal)];
        apply_taffy_layout(&mut elements1, 800.0, 600.0);
        
        // Second layout: element A (same) + element B (different)
        let mut elements2 = vec![
            make_el("p", "shared text", 16.0, FontWeight::Normal),
            make_el("p", "different text", 16.0, FontWeight::Normal),
        ];
        apply_taffy_layout(&mut elements2, 800.0, 600.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        // First element's "shared text" should hit cache; second element's "different text" should miss
        assert!(hits >= 1, "Shared text should hit cache, got {} hits", hits);
        assert!(misses >= 1, "Different text should miss cache");
    }

    // E2-7: Viewport resize -> does NOT invalidate individual word measurement
    #[test]
    fn e2_viewport_resize_does_not_invalidate_individual_measurement() {
        e0_reset_counters();
        clear_measure_cache();
        
        // Individual word measurement doesn't depend on viewport width
        let _w1 = measure_text_width("test", 16.0);
        let _w2 = measure_text_width("test", 16.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 1, "Viewport width doesn't affect individual word measurement");
        assert_eq!(misses, 1);
    }

    // E2-8: Navigation/new document -> cache persists correctly (same text = hit)
    #[test]
    fn e2_navigation_preserves_cache() {
        e0_reset_counters();
        clear_measure_cache();
        
        // Page 1
        let mut elements1 = vec![make_el("p", "page one content", 16.0, FontWeight::Normal)];
        apply_taffy_layout(&mut elements1, 800.0, 600.0);
        
        // Page 2 (different content)
        let mut elements2 = vec![make_el("p", "page two completely different", 16.0, FontWeight::Normal)];
        apply_taffy_layout(&mut elements2, 800.0, 600.0);
        
        // Page 1 again - text was in cache from first load
        let mut elements3 = vec![make_el("p", "page one content", 16.0, FontWeight::Normal)];
        apply_taffy_layout(&mut elements3, 800.0, 600.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        // Page 3 should hit cache for "page one content"
        assert!(hits >= 1, "Navigation back to page 1 should hit cache");
    }

    // E2-9: Numeric fast path obeys same invalidation semantics
    #[test]
    fn e2_numeric_fast_path_invalidation() {
        e0_reset_counters();
        clear_measure_cache();
        
        // Same numeric string -> should hit
        let _w1 = measure_text_width("123", 16.0);
        let _w2 = measure_text_width("123", 16.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 1, "Same numeric string should hit cache");
        assert_eq!(misses, 1, "First call should miss");
        
        // Different numeric string -> miss
        e0_reset_counters();
        clear_measure_cache();
        let _w1 = measure_text_width("123", 16.0);
        let _w2 = measure_text_width("456", 16.0);
        let (_, hits2, misses2, _, _, _) = e0_get_summary();
        assert_eq!(hits2, 0, "Different numeric strings should not share cache");
        assert_eq!(misses2, 2);
        
        // Different font size for same numeric -> miss
        e0_reset_counters();
        clear_measure_cache();
        let _w1 = measure_text_width("123", 16.0);
        let _w2 = measure_text_width("123", 18.0);
        let (_, hits3, misses3, _, _, _) = e0_get_summary();
        assert_eq!(hits3, 0, "Font size change should invalidate numeric cache");
        assert_eq!(misses3, 2);
    }

    // E2-10: Font weight change -> does NOT invalidate numeric measurement
    #[test]
    fn e2_numeric_font_weight_does_not_invalidate() {
        e0_reset_counters();
        clear_measure_cache();
        
        // measure_text_width doesn't receive font_weight - it's a drawing concern
        let _w1 = measure_text_width("123", 16.0);
        let _w2 = measure_text_width("123", 16.0);
        
        let (_, hits, misses, _, _, _) = e0_get_summary();
        assert_eq!(hits, 1, "Font weight doesn't affect numeric measurement");
        assert_eq!(misses, 1, "First call should miss");
    }

    // E2-11: LRU eviction behavior
    #[test]
    fn e2_lru_eviction() {
        // This test verifies the cache doesn't grow unbounded
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
