# AGENTS.md - Aether Browser

## Quick Commands
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test (all):** `cargo test`
- **Single test:** `cargo test <test_name>`
- **Note:** 4 pre-existing Caelum doc-test failures are unrelated (unresolved `Caelum` import)

## Architecture
- **Language:** Rust (edition 2021)
- **UI:** Iced 0.13 (features: `canvas`, `tiny-skia`, `wgpu`, `tokio`) with `#[theme(iced::Theme::Light)]`
- **Networking:** ureq 3.3.0 with `native-tls`
- **Entry point:** `src/main.rs` → `src/ui/` (AetherApp)
- **CSS engine:** `src/engine/stratus/` (**Stratus**) — the primary CSS parser/resolver. `src/engine/css/mod.rs` is a thin re-export shim for backward compat.
- **Layout engine:** `src/engine/caelum/` — embedded Taffy, integrated via `apply_caelum_layout()` for block-layout position/size computation
- **JS engine:** `src/engine/js/` — rquickjs 0.11 wrapper. Wired into page loading via `JsBridge` (flat DOM tree behind `Arc<Mutex<>>`). `<script>` tags are extracted, executed before `extract_elements()`. Bridge persists in `BrowserScreen` for post-load timer/event dispatch.
- **Engine modules compiled:** `src/engine/mod.rs` exports: `events, css, dom, image, js, layout, net, parser, renderer, style, stratus, caelum`
- **Default page:** `aether://design/spatial-minimalism` (static welcome text)

## Rendering Pipeline
```
fetch_page_content() — async, runs on Iced background thread via Task::perform
  1. fetch HTML (ureq, max 500KB)
  2. parse DOM (src/engine/parser/)
  3. extract <style> blocks and <link rel="stylesheet"> URLs
  4. fetch external CSS (resolve URL against page base, max 50KB per source)
  5. parse all CSS through Stratus (MAX_INPUT_LENGTH=30000, MAX_ITERATIONS=50000)
  6. extract_scripts() — finds <script> tags (inline + external)
  7. Load DOM into JsBridge (flat Vec<FlatNode>), execute scripts via execute_with_bridge()
  8. bridge.to_dom() replaces DOM, inject_js_output() adds document.write() text
  9. extract_elements() — walks modified DOM, applies Stratus computed styles, produces Vec<StyledElement>
     Limits: depth 30, max 300 elements, text max 1000 chars
  10. truncate to 300 elements
  11. fetch image bytes for <img> elements via fetch_bytes()
  12. apply_caelum_layout() — builds Caelum block tree, computes x/y/width/height per element
  13. Elements rendered via PageCanvas (iced::widget::canvas::Program) at Caelum-computed positions
  14. Bridge persists in BrowserScreen for post-load timer tick / event dispatch
```

## JS APIs Available (Phase 4)
- **DOM:** `document.createElement()`, `document.createTextNode()`, `document.getElementById()`, `element.appendChild()`, `element.setAttribute()`, `element.getAttribute()`, `element.textContent` (get/set), `element.innerHTML` (get/set), `element.id`, `element.className`
- **Query selectors:** `document.querySelector()`, `document.querySelectorAll()`, `document.getElementsByTagName()`, `element.querySelector()`, `element.querySelectorAll()` — supports tag, `.class`, `#id`, `*`, descendant (` `), child (`>`) combinators
- **DOM traversal:** `element.parentNode`, `element.children`, `element.childNodes`, `element.firstChild`, `element.lastChild`, `element.nextSibling`, `element.previousSibling`, `element.childElementCount`
- **Timer:** `window.setTimeout(fn, ms)`, `window.setInterval(fn, ms)`, `window.clearTimeout(id)`, `window.clearInterval(id)` — callbacks stored as source strings, executed in fresh context each tick
- **Location:** `window.location.href` (get/set), `.hostname`, `.pathname`, `.protocol`, `.port`, `.search`, `.hash`, `.reload()`, `.assign(url)`, `.replace(url)` — setting href triggers browser navigation
- **Events:** `element.addEventListener(type, handler)`, `element.removeEventListener(type, handler)` — click events dispatched on canvas click, bubbling up through ancestors
- **Style:** `element.style.color`, `.backgroundColor`, `.fontSize`, `.fontWeight`, `.marginTop`, `.marginBottom`, `.padding`, `.border`, `.width`, `.height`, `.display`, `.textAlign` (get/set via inline styles)
- **HTTP:** `window.fetch(url)` returns `{ok, status, statusText, text(), json()}`, `window.XMLHttpRequest` (basic sync implementation)
- **Console:** `console.log()`, `.warn()`, `.error()`, `.assert()`, `.count()`, `.time()`/`.timeEnd()`, `.group()`/`.groupEnd()`, `.table()`
- **Navigator:** `window.navigator.userAgent`, `.platform`, `.language`
- **Limitation:** Closures don't serialize across timer/event boundaries — use source-code-safe patterns

## PageCanvas Drawing Order (per element)
1. Background fill rectangle at (x, y, width, height)
2. Border rectangles (top, right, bottom, left edges)
3. Text on top (offset by border widths)
4. Images rendered as placeholder rectangles (actual pixel rendering not yet available in Iced 0.13 Canvas Frame API)

## Key Conventions
- Engine modules live in `src/engine/` with docs in `docs/`
- All CSS parsing goes through Stratus, not the legacy parser
- `resolve_url()` in `net/mod.rs` handles relative URL resolution against base URL; strips extra `/` from `//`-prefixed URLs
- `should_skip_tag()` and `should_skip_content()` filter non-content HTML elements (script, style, meta, link, head, svg, form, iframe, etc.) — **img is NOT skipped**
- Page rendering uses computed CSS styles (color, font-size, font-weight, background-color, border-widths, border-color) from Stratus per element
- Text nodes inherit parent element's computed style via `parent_style` parameter
- `<p>` handler: collects only direct text children (not child links), recurses into element children separately
- `<a>` tags inside `<p>` are extracted as separate clickable link elements
- `<img>` tags extracted with src/alt, image bytes fetched in fetch_page_content(), rendered as placeholder boxes
- CSS margins extracted per-element via `margins_for_node()`, used in Caelum tree as `margin_top`/`margin_bottom`
- CSS borders extracted per-element via `border_for_node()`, stored as `[top, right, bottom, left]` widths + `border_color`
- Per-tag UA default margins: h1=24px, h2=20px, h3-h6=16px, p=12px, li=8px, a=4px, img=4px, _=0px
- ureq HTTP timeout: 15s global (`timeout_global(Some(Duration::from_secs(15)))`)

## Important Gotchas
- **Stratus silently truncates CSS** at 30,000 chars (MAX_INPUT_LENGTH) and 50,000 iterations (MAX_ITERATIONS) — check parser logs
- **External CSS is capped at 50KB** in browser.rs before being sent to Stratus
- **Only 1 `<style>` block and 1 external CSS** are processed (`style_limit = 1`, `link_limit = 1`)
- **`normalize_url()`** prepends `https://` if no scheme present
- **`fetch_bytes()`** for images uses blocking ureq (no timeout_global set)
- **Iced 0.13 API:** `Task::perform(future, mapper)` — no `iced::Command` in this version
- **No `draw_image()`** on `Frame` in Iced 0.13 — images rendered as placeholder rectangles
- **`Frame::fill_rectangle`** draws background behind each element at its Caelum-computed position/size; borders drawn as thin rectangles on each edge

## Key Fields on StyledElement
- `color`, `font_size`, `font_weight` — text styling
- `background_color: Option<Color>` — CSS background-color
- `margin_top`, `margin_bottom` — CSS margins (used by Caelum layout)
- `border_widths: [f32; 4]` — [top, right, bottom, left] border widths
- `border_color: Option<Color>` — border color from CSS
- `x`, `y`, `width`, `height` — Caelum-computed layout position/size
- `image_bytes: Option<Vec<u8>>`, `image_url: Option<String>` — image support
- `is_link`, `href` — link detection for click handling

## Testing
- ~119+118 unit tests across `src/lib.rs` and `src/main.rs` (all pass)
- Tests in `tests/compliance/html5.rs` is a **placeholder** (single `assert!(true)`)
- No integration test framework — tests are inline `#[test]` in modules
- No mock network layer — `BrowserScreen::navigate()` does real HTTP calls
- `src/ui/screens/browser_test.rs` has a basic state-change test (no mocking)

## Known Stale Documentation
- **ROADMAP.md** claims Caelum is used for page layout — it **IS now used** via `apply_caelum_layout()`, but as a flat block list, not a tree walk.
