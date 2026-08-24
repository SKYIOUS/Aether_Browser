# Vayu Browser

An experimental web browser engine written in Rust. No Blink/Gecko/WebKit code;
external components sit behind seams we own (see `CONTEXT.md`).

## What Works
- HTML parsing via html5ever (WHATWG tokenizer/tree builder) into `aether-dom`
- CSS: cssparser-based tokenizer (Stratus) + Servo `selectors` matching, cascade in `aether-css`
- Layout: taffy 0.12 (block/inline/flex/grid) with an inline formatting context
- Text: cosmic-text glyph measurement/shaping with fallback fonts, LRU-cached (CJK/non-Latin works)
- JavaScript: QuickJS (rquickjs 0.12) behind the JsBridge seam; native replacement (`aether-js`) planned — see `docs/adr/0003`
- SVG rendering via resvg/usvg; raster images via `image`
- Tabs with per-tab history, workspaces sidebar (Korlang VM), bookmarks storage
- Cookies with HttpOnly/Secure/SameSite attributes; CSP directive checks for scripts/styles/images/connects
- Settings and command palette screens; Iced 0.13 canvas painting

## Architecture
- `src/engine/` — pipeline stages: fetcher → parser → stratus/selectors → js bridge → layout → paint support
- `src/ui/` — Iced shell: browser screen + canvas, tab bar, devtools overlay, settings, palette
- `crates/aether-dom`, `crates/aether-css` — workspace crates (DOM tree, CSS values/cascade)
- `korlang/` — embedded UI DSL compiler + stack VM driving sidebar/chrome
- Docs: `PLAN.md` (what's next) · `CONTEXT.md` (vocabulary/doctrine) · `docs/architecture/issues.md` (debt ledger) · `docs/adr/` (decisions)

## Build & Run
```
cargo build
cargo run
cargo test
```

## Limitations
Prototype — not a production browser.
- No process isolation or sandbox; single-process with blocking network I/O
- Page caps: 2000 elements, depth 50, 5000 chars/text node, 1MB HTML, ≤50 style blocks
- No CSS animations/transitions, custom properties, calc(), float/table layout
- No audio/video; no @font-face yet; Intl absent
- Event system is minimal (no bubbling/capture); JS callbacks re-parsed per tick
- Live debt list: `docs/architecture/issues.md`
