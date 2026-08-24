# AGENTS.md - Vayu Browser

## Quick Commands
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test (all):** `cargo test`
- **Single test:** `cargo test <test_name>`
- **Status:** Compiles clean, all tests pass (~444 across workspace + integration).

## Architecture
- **Language:** Rust (edition 2021)
- **UI:** Iced 0.13 (`canvas`, `image`, `tiny-skia`, `wgpu`, `tokio`), theme forced Light
- **Entry point:** `src/main.rs` → `src/lib.rs` → `src/ui/mod.rs` (`VayuApp`)
- **Workspace members:** `korlang`, `crates/aether-dom`, `crates/aether-css`
- **HTML parsing:** `html5ever` 0.39 via custom `TreeSink` in `src/engine/parser.rs` → produces `aether_dom::Node`
- **CSS parsing:** `cssparser` 0.37 in `src/engine/stratus.rs`; selector matching via `selectors` crate in `src/engine/js/selector_engine.rs`
- **Layout:** `taffy` 0.12 — `apply_taffy_layout()` in `src/engine/pipeline/layout.rs`
- **Text measurement:** `cosmic-text` (fontdb + rustybuzz) in `src/engine/text.rs`, LRU-cached; no more char-width heuristic
- **JS engine:** `rquickjs` 0.12 — `src/engine/js/` (`JsBridge` flat DOM behind `Arc<Mutex<>>`)
- **Korlang:** embedded scripting VM (`korlang/`) driving sidebar + status bar UI

## Module map (src/engine/)
- `parser.rs` — html5ever → aether_dom
- `stratus.rs` — CSS tokenizer/parser/resolver (cssparser)
- `text.rs` — glyph-accurate text measurement (cosmic-text)
- `style.rs` — thin wrapper: Node → ElementData → stratus resolve
- `net/` — blocking reqwest client, cookies, cache, CSP checks, URL utils
- `js/` — JsBridge, timers, events, fetch/XHR, storage, selectors (selectors crate)
- `pipeline/` — orchestrates one page load:
  - `fetcher.rs` — fetch HTML (max 1MB), collect `<style>` + external CSS (max 50 each), parse CSS
  - `extractor.rs` — walk DOM → `Vec<StyledElement>` with computed styles (max depth 50, max 2000 elements)
  - `layout.rs` — `apply_taffy_layout()` computes x/y/w/h incl. inline formatting context
  - `navigator.rs` — Tab model, URL normalization, tab/bookmark persistence

## UI structure (src/ui/)
- `mod.rs` — VayuApp: screen routing (Browser / Settings / Palette)
- `screens/browser/`
  - `mod.rs` — BrowserScreen state machine: update(), view(), navigation, history, tabs, JS dispatch
  - `canvas.rs` — PageCanvas (iced canvas Program): draws elements, handles click hit-testing, focus ring
  - `tab_bar.rs` — tab strip rendering
  - `workspaces.rs` — sidebar (workspaces/collections via korlang VM)
  - `devtools.rs` — console/elements/network overlay
- `screens/settings.rs` — settings screen + VayuSettings persistence
- `screens/palette.rs` — command palette
- `style.rs` — shared style helpers/colors

## Rendering Pipeline
```
fetch_page_content() (fetcher.rs, async on Iced thread)
  1. fetch HTML (reqwest blocking, max 1MB)
  2. parse_html() → html5ever → aether_dom::Node tree
  3. extract <style> blocks and <link rel=stylesheet> URLs (≤50 each, ≤500KB per source)
  4. parse CSS through Stratus (cssparser-based, no input-length truncation)
  5. extract scripts; JsBridge executes them against flat DOM; bridge.to_dom()
  6. extract_elements() → Vec<StyledElement> (computed styles, images decoded to Handle::from_rgba)
  7. apply_taffy_layout() → positions/sizes per element (block + inline contexts)
  8. PageCanvas draws elements at computed positions; click hit-testing maps back to messages
```

## Key Conventions
- `resolve_url()`/`normalize_url()` in `net/mod.rs`; `normalize_nav_url()` in `navigator.rs`
- Runtime data files (gitignored): `vayu_settings.json`, `vayu_tabs.json`, `vayu_local_storage.json`, `vayu_cookies.json`
- Global state: `OnceLock<Mutex<...>>` statics for HTTP client, caches, cookie jar, localStorage
- `.cargo/config.toml` uses `rust-lld` as linker (ships with every toolchain; do not switch to lld-link unless installed)

## Important Gotchas
- **Iced 0.13 API:** `Task::perform(future, mapper)` — there is no `iced::Command`
- **Element caps** still exist: 2000 elements, depth 50, text 5000 chars, 1MB HTML — large pages get truncated
- **Blocking reqwest** everywhere via `run_blocking()` thread spawn
- **Child modules can access parent private fields** — browser submodules read BrowserScreen fields directly
- **Tab::new(title, url, workspace_id)** takes 3 args
- **CSP support:** `net::csp_blocks_scripts()` / `net::csp_blocks_styles()` checked before processing
- **wgpu 0.19** (via iced) has future-incompat warnings — harmless today, revisit when iced upgrades wgpu

## Testing
- ~444 tests total: integration tests in `tests/`, unit tests inline under `#[cfg(test)]`
- Layout tests use `apply_taffy_layout(&mut elements, width, height)` directly — no network needed
- No mock network layer; `BrowserScreen::navigate()` does real HTTP

## Removed (do not reintroduce)
- `crates/aether-html` (replaced by html5ever), `crates/aether-caelum` (replaced by taffy)
- `build.rs` + `css-caelum-bridge.json` codegen bridge
- `MAX_INPUT_LENGTH`/`MAX_ITERATIONS` CSS truncation, `CHAR_W_SCALE` heuristic
