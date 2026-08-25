//! Fuzz the A1 budget helpers through the same public surface the pipeline
//! uses (exposed via pipeline::fuzz_surface).

#![no_main]

use libfuzzer_sys::fuzz_target;
use vayu_browser::engine::pipeline::fuzz_surface::{
    apply_html_budget, css_sources_within_total_budget, trim_to_budget,
};

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else { return };

    // HTML budget: never longer than the cap, always char-boundary safe.
    let capped = apply_html_budget(input.to_string(), 4096);
    assert!(capped.len() <= 4096);
    assert!(capped.is_char_boundary(capped.len()));

    let trimmed = trim_to_budget(input, 1024);
    assert!(trimmed.len() <= 1024);
    assert!(input.starts_with(trimmed), "trim must be a prefix of its input");

    let sources: Vec<&str> = input.split('\u{0}').collect();
    let (_, total) = css_sources_within_total_budget(&sources, 0, 2048);
    assert!(total <= 2048);

    // Cumulative accounting must stay monotonic across calls.
    let (_, total2) = css_sources_within_total_budget(&sources, total, 2048);
    assert!(total2 >= total);
});
