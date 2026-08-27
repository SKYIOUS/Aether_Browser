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

static MEASURE_CACHE: std::sync::OnceLock<Mutex<lru::LruCache<(String, u32), f32>>> = std::sync::OnceLock::new();

fn measure_cache() -> &'static Mutex<lru::LruCache<(String, u32), f32>> {
    MEASURE_CACHE.get_or_init(|| {
        Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(512).unwrap()))
    })
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
pub fn measure_text_width(text: &str, font_size: f32) -> f32 {
    if text.is_empty() {
        return 0.0;
    }

    let fs = if font_size.is_finite() { font_size.clamp(6.0, 200.0) } else { 16.0 };
    let fs_key = (fs * 100.0) as u32;

    MEASURE_CALLS.with(|c| *c.borrow_mut() += 1);

    if let Ok(mut cache) = measure_cache().lock() {
        if let Some(&cached) = cache.get(&(text.to_string(), fs_key)) {
            MEASURE_HITS.with(|c| *c.borrow_mut() += 1);
            return cached;
        }
    }

    MEASURE_MISSES.with(|c| *c.borrow_mut() += 1);

    let start = Instant::now();
    let result = measure_text_width_uncached(text, fs);
    TOTAL_SHAPING_MS.with(|c| *c.borrow_mut() += start.elapsed().as_secs_f64() * 1000.0);

    if let Ok(mut cache) = measure_cache().lock() {
        cache.put((text.to_string(), fs_key), result);
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

    // E0 instrumentation test
    #[test]
    #[ignore]
    fn e0_measurement_volume() {
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

        fn run_case(name: &str, element_count: usize, varied_text: bool) {
            e0_reset_counters();
            
            let mut elements: Vec<StyledElement> = Vec::with_capacity(element_count);
            for i in 0..element_count {
                let text = if varied_text {
                    format!("Paragraph {} with unique text content for measurement purposes", i)
                } else {
                    "Shared text content".to_string()
                };
                elements.push(make_el("p", &text, 16.0));
            }
            
            let text_elements = elements.iter().filter(|e| !e.text.is_empty()).count();
            
            let start = std::time::Instant::now();
            apply_taffy_layout(&mut elements, 800.0, 600.0);
            let total_ms = start.elapsed().as_secs_f64() * 1000.0;
            
            let (calls, hits, misses, buffers, shaping_ms, font_inits) = e0_get_summary();
            
            println!("\n=== E0: {} ({} elements, {} text) ===", name, element_count, text_elements);
            println!("  total layout time:     {:.2} ms", total_ms);
            println!("  text elements:         {}", text_elements);
            println!("  measure calls:         {}", calls);
            println!("  cache hits:            {} ({:.1}%)", hits, if calls > 0 { hits as f64 * 100.0 / calls as f64 } else { 0.0 });
            println!("  cache misses:          {}", misses);
            println!("  buffer constructions:  {}", buffers);
            println!("  shaping time:          {:.2} ms", shaping_ms);
            println!("  FontSystem inits:      {}", font_inits);
            
            // Taffy time ≈ total - shaping - overhead
            let taffy_est = total_ms - shaping_ms;
            println!("  Taffy+application est: {:.2} ms", taffy_est.max(0.0));
        }

        println!("\n=== E0 Measurement Volume Attribution ===");
        
        // Cold runs
        run_case("200 varied", 200, true);
        run_case("2000 varied", 2000, true);
        run_case("5000 varied", 5000, true);
        
        // Warm runs (re-use cached measurements)
        run_case("200 varied WARM", 200, true);
        run_case("2000 varied WARM", 2000, true);
        run_case("5000 varied WARM", 5000, true);
        
        // Shared text runs (cache-friendly)
        run_case("5000 shared", 5000, false);
        run_case("5000 shared WARM", 5000, false);
    }
}
