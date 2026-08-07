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

/// Measure the visual width of a text string at a given font size.
///
/// This uses actual glyph shaping via `cosmic-text` instead of the old
/// `unicode_width::UnicodeWidthStr::width() * font_size * 0.58` heuristic.
/// It correctly handles proportional fonts, ligatures, kerning, CJK, and
/// complex scripts.
///
/// # Performance Note
///
/// A new `Buffer::new()` allocation + glyph shaping + `FontSystem` mutex lock
/// occurs on every call. This is the known hot-path cost when invoked from
/// `wrap_text`'s per-word loop or inline positioning. The mutex on
/// `FONT_SYSTEM` is a contention point under concurrent layout passes.
/// A `ThreadLocal<Buffer>` cache is not feasible because `Buffer` borrows
/// `FontSystem` at creation time; caching requires a per-`FontSystem` buffer
/// pool. Tracked as a follow-up performance ticket.
pub fn measure_text_width(text: &str, font_size: f32) -> f32 {
    if text.is_empty() {
        return 0.0;
    }

    let fs = if font_size.is_finite() { font_size.clamp(6.0, 200.0) } else { 16.0 };
    let metrics = Metrics::new(fs, fs * 1.2);
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
        fs * 0.5 * text.len() as f32
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
