//! Fuzz URL normalization/resolution/redirect decisions exactly as the
//! network layer uses them (C1's redirect gate included).

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else { return };
    let normalized = vayu_browser::engine::net::normalize_url(input);
    let resolved = vayu_browser::engine::net::resolve_url("/path", &normalized);
    let _ = vayu_browser::engine::net::redirect_target(&normalized, input);

    // Invariant: resolution never fabricates a scheme-less authority.
    if resolved.contains("://") {
        let after_scheme = resolved.split("://").nth(1).unwrap_or("");
        assert!(!after_scheme.starts_with('/'), "resolved URL lost its host: {resolved}");
    }
});
