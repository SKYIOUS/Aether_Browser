//! Fuzz the real CSS front end (stratus::parse -> cssparser tokenizer ->
//! rule building). No input truncation exists since the C-clamp removal, so
//! arbitrary lengths/sequences reach the parser verbatim.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else { return };
    // Rules are retained only to force full materialization.
    let sheet = vayu_browser::engine::stratus::parse(input);
    let _ = sheet.rules.len();
});
