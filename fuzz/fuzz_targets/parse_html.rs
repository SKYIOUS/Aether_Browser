//! Fuzz the real HTML pipeline: parse_document -> DomSink (TreeSink incl. the
//! A5 map operations) -> materialized aether-dom tree.
//!
//! Production input is always a decoded `String` (reqwest charset handling),
//! so invalid UTF-8 is not a parser concern: non-UTF-8 inputs are skipped.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else { return };
    let _ = vayu_browser::engine::parser::parse_html(input);
});
