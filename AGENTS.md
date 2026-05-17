# AGENTS.md - Aether Browser

## Quick Commands
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test (all):** `cargo test`
- **Single test:** `cargo test <test_name>`
- **Note:** 4 pre-existing Caelum doc-test failures are unrelated (unresolved `Caelum` import)

## Architecture
- **Language:** Rust (edition 2021)
- **UI:** Iced 0.13 (features: `tiny-skia`, `wgpu`) with `#[theme(iced::Theme::Light)]`
- **Networking:** ureq 3.3.0 with `native-tls`
- **Entry point:** `src/main.rs` → `src/ui/` (AetherApp)
- **CSS engine:** `src/engine/stratus/` (**Stratus**) — the primary CSS parser/resolver. `src/engine/css/mod.rs` is a thin re-export shim for backward compat.
- **Layout engine:** `src/engine/caelum/` — embedded Taffy. ⚠️ **NOT used for page rendering** — pages render as a flat list of `StyledElement` via `extract_elements()`.
- **JS engine:** `src/engine/js/` — rquickjs wrapper. ⚠️ **NOT wired into browser page loading** — `<script>` tags are skipped.
- **Engine modules compiled:** `src/engine/mod.rs` exports: `events, css, dom, image, js, layout, net, parser, renderer, style, stratus, caelum`. (`standards/` and `process/` exist on disk but are **not** compiled.)
- **Default page:** `aether://design/spatial-minimalism` (static welcome text)

## Rendering Pipeline
```
fetch_page_content() — async, runs on Iced background thread via Task::perform
  1. fetch HTML (ureq, max 500KB)
  2. parse DOM (src/engine/parser/)
  3. extract <style> blocks and <link rel="stylesheet"> URLs
  4. fetch external CSS (resolve URL against page base, max 50KB per source)
  5. parse all CSS through Stratus (MAX_INPUT_LENGTH=30000, MAX_ITERATIONS=50000)
  6. extract_elements() — walks DOM, applies Stratus computed styles, produces Vec<StyledElement>
     Limits: depth 30, max 300 elements, text max 1000 chars
  7. truncate to 300 elements
  8. Elements rendered in view() as a scrollable column with per-tag top margins
```

## Key Conventions
- Engine modules live in `src/engine/` with docs in `docs/`
- All CSS parsing goes through Stratus, not the legacy parser
- `resolve_url()` in `net/mod.rs` handles relative URL resolution against base URL
- `should_skip_tag()` and `should_skip_content()` filter non-content HTML elements (script, style, meta, link, head, svg, form, iframe, etc.)
- Page rendering uses computed CSS styles (color, font-size, font-weight) from Stratus per element
- Text nodes inherit parent element's computed style via `parent_style` parameter
- `<p>` handler: collects only direct text children (not child links), recurses into element children separately
- `<a>` tags inside `<p>` are extracted as separate clickable link elements
- Per-tag top margins: h1=24px, h2=20px, h3-h6=16px, p=12px, li=8px, a=4px, _=2px
- ureq HTTP timeout: 15s global (`timeout_global(Some(Duration::from_secs(15)))`)

## Important Gotchas
- **Stratus silently truncates CSS** at 30,000 chars (MAX_INPUT_LENGTH) and 50,000 iterations (MAX_ITERATIONS) — check parser logs
- **External CSS is capped at 50KB** in browser.rs before being sent to Stratus
- **Only 1 `<style>` block and 1 external CSS** are processed (`style_limit = 1`, `link_limit = 1`)
- **`normalize_url()`** prepends `https://` if no scheme present
- **`fetch_bytes()`** for images uses blocking ureq (no timeout_global set)
- **Iced 0.13 API:** `Task::perform(future, mapper)` — no `iced::Command` in this version

## Testing
- ~119+118 unit tests across `src/lib.rs` and `src/main.rs` (80%+ pass rate)
- Tests in `tests/compliance/html5.rs` is a **placeholder** (single `assert!(true)`)
- No integration test framework — tests are inline `#[test]` in modules
- No mock network layer — `BrowserScreen::navigate()` does real HTTP calls
- `src/ui/screens/browser_test.rs` has a basic state-change test (no mocking)

## Known Stale Documentation
- **ROADMAP.md** claims JS is "integrated into browser — scripts extracted and executed" — this is **not implemented**. `<script>` tags are skipped in `should_skip_tag()`.
- **ROADMAP.md** claims Caelum is used for page layout — it is **not**. Pages use flat `StyledElement` extraction.
