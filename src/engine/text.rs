//! Text measurement and shaping using cosmic-text
//!
//! Replaces the `unicode_width` + `CHAR_W_SCALE` heuristic with actual glyph-based
//! measurement. Uses `cosmic-text` which bundles `fontdb` (font discovery) and
//! `rustybuzz` (text shaping).

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};

use std::sync::Mutex;

static FONT_SYSTEM: std::sync::OnceLock<Mutex<FontSystem>> = std::sync::OnceLock::new();

fn font_system() -> &'static Mutex<FontSystem> {
    FONT_SYSTEM.get_or_init(|| Mutex::new(FontSystem::new()))
}

static MEASURE_CACHE: std::sync::OnceLock<Mutex<lru::LruCache<(String, u32), f32>>> = std::sync::OnceLock::new();

fn measure_cache() -> &'static Mutex<lru::LruCache<(String, u32), f32>> {
    MEASURE_CACHE.get_or_init(|| {
        Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(512).unwrap()))
    })
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

    if let Ok(mut cache) = measure_cache().lock() {
        if let Some(&cached) = cache.get(&(text.to_string(), fs_key)) {
            return cached;
        }
    }

    let result = measure_text_width_uncached(text, fs);

    if let Ok(mut cache) = measure_cache().lock() {
        cache.put((text.to_string(), fs_key), result);
    }

    result
}

fn measure_text_width_uncached(text: &str, font_size: f32) -> f32 {
    let metrics = Metrics::new(font_size, font_size * 1.2);
    let mut font_system = font_system().lock().unwrap();

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
}
