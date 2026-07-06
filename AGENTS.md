# AGENTS.md - Aether Browser

## Quick Commands
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test (all):** `cargo test`
- **Single test:** `cargo test <test_name>`
- **Current status:** Code compiles and passes basic rendering and logic tests.

## Architecture
- **Language:** Rust (edition 2021)
- **UI:** Iced 0.13 (features: `canvas`, `image`, `tiny-skia`, `wgpu`, `tokio`) with `theme(|_| iced::Theme::Light)`
- **Networking:** reqwest 0.12 with `blocking`, `http2`, `native-tls`
- **Entry point:** `src/main.rs` → `src/ui/` (AetherApp)
- **Workspace:** members = `korlang`, `crates/aether-dom`, `crates/aether-html`, `crates/aether-css`, `crates/aether-caelum`
- **CSS engine:** `aether-css` crate (re-exported from `src/engine/stratus.rs`)
- **Layout engine:** `aether-caelum` crate (re-exported from `src/engine/caelum.rs`), integrated via `apply_caelum_layout()` in `pipeline/layout.rs`
- **JS engine:** `src/engine/js/` — rquickjs 0.11 wrapper. Wired into page loading via `JsBridge` (flat DOM tree behind `Arc<Mutex<>>`). `<script>` tags extracted, executed before `extract_elements()`. Bridge persists in `BrowserScreen` for post-load timer/event dispatch.
- **Code generation:** `build.rs` reads `css-caelum-bridge.json`, generates `bridge_gen.rs` mapping stratus ↔ caelum enums
- **Engine modules compiled:** `src/engine/mod.rs` exports: `dom, js, net, parser, style, stratus, caelum, pipeline`
- **Default page:** `aether://design/spatial-minimalism` (static welcome text)

## Rendering Pipeline
```
fetch_page_content() — async, runs on Iced background thread via Task::perform (src/engine/pipeline/fetcher.rs)
  1. fetch HTML (reqwest, max 1MB)
  2. parse DOM (src/engine/parser/)
  3. extract <style> blocks and <link rel="stylesheet"> URLs
  4. fetch external CSS (resolve URL against page base, max 500KB per source)
  5. parse all CSS through Stratus (aether-css crate, MAX_INPUT_LENGTH=30000, MAX_ITERATIONS=50000)
  6. extract_scripts() — finds <script> tags (inline + external)
  7. Load DOM into JsBridge (flat Vec<FlatNode>), execute scripts via JSEngine::execute_with_bridge()
  8. bridge.to_dom() replaces DOM, inject_js_output() adds document.write() text
  9. extract_elements() — walks modified DOM, applies Stratus computed styles, produces Vec<StyledElement>
     Limits: depth 50, max 2000 elements, text max 5000 chars
  10. truncate to 2000 elements
  11. fetch image bytes for <img> elements via fetch_bytes(), decode via image crate to Handle::from_rgba
  12. apply_caelum_layout() — builds Caelum tree (aether-caelum crate), computes x/y/width/height per element
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
4. Images rendered via `iced::widget::image::Handle` (decoded via `image` crate, loaded as RGBA)

## Key Conventions
- Engine modules live in `src/engine/` with docs in `docs/`; external crates in `crates/`
- All CSS parsing goes through `aether-css` (the Stratus crate), re-exported from `src/engine/stratus.rs`
- `resolve_url()` in `net/mod.rs` handles relative URL resolution against base URL; strips extra `/` from `//`-prefixed URLs
- `should_skip_tag()` and `should_skip_content()` in `pipeline/extractor.rs` filter non-content HTML elements (script, style, meta, link, head, svg, path, br, hr, template, etc.) — **img is NOT skipped**
- Page rendering uses computed CSS styles (color, font-size, font-weight, background-color, border-widths, border-color) from Stratus per element
- Text nodes inherit parent element's computed style via `parent_style` parameter
- `<p>` handler: collects only direct text children (not child links), recurses into element children separately
- `<a>` tags inside `<p>` are extracted as separate clickable link elements
- `<img>` tags extracted with src/alt, image bytes fetched in fetch_page_content(), decoded via `image` crate to `Handle::from_rgba`
- CSS margins extracted per-element via `compute_full_style()`, used in Caelum tree as `margin_top`/`margin_bottom`
- CSS borders extracted per-element via `compute_full_style()`, stored as `[top, right, bottom, left]` widths + `border_color`
- Per-tag UA default margins: h1=24px, h2=20px, h3-h6=16px, p=12px, li=8px, a=4px, img=4px, _=0px
- reqwest blocking HTTP client with 15s timeout (`timeout(Duration::from_secs(15))`)

## Important Gotchas
- **Stratus silently truncates CSS** at 30,000 chars (MAX_INPUT_LENGTH) and 50,000 iterations (MAX_ITERATIONS) — check parser logs
- **External CSS is capped at 500KB** in fetcher.rs before being sent to Stratus
- **Up to 50 `<style>` blocks and 50 external CSS** are processed (`style_limit = 50`, `link_limit = 50`)
- **`normalize_url()`** prepends `https://` if no scheme present
- **`fetch_bytes()`** for images uses blocking reqwest via `run_blocking()` (spawns a thread)
- **Iced 0.13 API:** `Task::perform(future, mapper)` — no `iced::Command` in this version
- **Images decoded via `image` crate** → `iced::widget::image::Handle::from_rgba` (not placeholder rectangles)
- **`Frame::fill_rectangle`** draws background behind each element at its Caelum-computed position/size; borders drawn as thin rectangles on each edge
- **CSP support:** `net::csp_blocks_scripts()` / `net::csp_blocks_styles()` checked before processing
- **CSS cache:** LRU-style cache of 100 entries, evicted when full
- **Code compiles and is functional.**

## Key Fields on StyledElement
- `color`, `font_size`, `font_weight` — text styling
- `background_color: Option<Color>` — CSS background-color
- `margin_top`, `margin_bottom` — CSS margins (used by Caelum layout)
- `border_widths: [f32; 4]` — [top, right, bottom, left] border widths
- `border_color: Option<Color>` — border color from CSS
- `image_handle: Option<Handle>`, `image_url: Option<String>` — image support
- `is_link`, `href` — link detection for click handling
- `is_hidden: bool` — set when display=none
- `parent_index: Option<usize>` — index into element vec for parent (not flat DOM)
- `x`, `y`, `width`, `height` — Caelum-computed layout position/size

## Testing
- ~101 integration tests in `tests/` + 9 unit tests in `browser.rs` + ~115 crate tests across `aether-css`, `aether-caelum`, `aether-dom`, `aether-html` (~225 total)
- `tests/compliance/html5.rs` is NOT a placeholder — it validates `should_skip_tag()` against tag lists
- No mock network layer — `BrowserScreen::navigate()` does real HTTP calls
- No `browser_test.rs` — browser tests live inline in `src/ui/screens/browser.rs` under `#[cfg(test)]`

## Known Stale Documentation
- **ROADMAP.md** claims Caelum is used for page layout — it **IS now used** via `apply_caelum_layout()`, but as a flat block list, not a tree walk.
- **ROADMAP.md** lists `src/engine/caelum/` internal module structure — Caelum is now `crates/aether-caelum` and re-exported from `src/engine/caelum.rs`
- **ROADMAP.md** still references old `src/lib.rs` → `main.rs` test splitting from before workspace refactor
