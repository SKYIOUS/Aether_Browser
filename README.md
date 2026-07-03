# Aether Browser

An experimental/prototype web browser engine written in Rust.

## What Works
- HTML parsing with basic DOM tree construction
- CSS parsing and style resolution (via Stratus/aether-css)
- Layout via Caelum (embedded Taffy-derived engine for flexbox, grid, block)
- JavaScript runtime via QuickJS (rquickjs) with basic DOM API shim
- Page rendering via Iced 0.13 canvas (text + colored rectangles)
- Tab management (multiple tabs with per-tab history)
- Korlang UI scripting language (experimental, used for status bar and sidebar)
- Settings and command palette screens

## Architecture
- src/engine/ — Core engine modules (DOM, CSS parser, JS bridge, networking)
- src/ui/ — Iced-based shell (browser screen, settings, palette)
- crates/ — Workspace crates (aether-dom, aether-html, aether-css, aether-caelum)
- korlang/ — Korlang language compiler and VM

## Build & Run
cargo build
cargo run
cargo test

## Limitations
This is a prototype, not a production browser.
- No security sandbox, no CSP enforcement, no process isolation
- Images rendered as placeholder rectangles (Iced 0.13 canvas limitation)
- Single-process, blocking I/O for network requests
- Maximum 2000 DOM elements per page, 50 style blocks, 500KB CSS limit
- Audio/video playback not implemented
- HTML5/CSS3 compliance is partial at best
