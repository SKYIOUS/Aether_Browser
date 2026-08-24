# Vayu Browser — Consolidated Plan

> Single source of truth for what's next. Supersedes drafts in `.kilo/plans/`
> (kept local-only). Domain vocabulary lives in `CONTEXT.md`.

## Where we are (Aug 2026)

The crate-adoption roadmap from `.kilo/plans/vayu-future-roadmap-plan.md` is
**done and committed** (`0bd1626`, `d1d238d`):

| Component | Old | Now |
|---|---|---|
| HTML parse | custom `aether-html` | html5ever 0.39 + custom TreeSink |
| CSS parse | `aether-css` w/ 30K truncation | cssparser 0.37 in Stratus, no truncation |
| Selectors | hand-rolled in js_bridge | Servo `selectors` |
| Layout | flat block list (`aether-caelum`) | taffy 0.12 tree layout + inline context |
| Text | `char_count * size * 0.58` | cosmic-text glyph measurement, LRU-cached |
| SVG | none | resvg/usvg decode path |
| JS engine | rquickjs 0.11 (CVE) | rquickjs 0.12 |
| Cookies | no attributes | HttpOnly/Secure/SameSite parsed + enforced vs JS |

Both dead crates are deleted; build is green (~444 tests); linker is `rust-lld`.

## Goal

**Daily-usable browser.** Chosen direction: page fidelity first — real pages
should render completely and scroll smoothly before adding features.

## Track 2 — Native JS engine (`aether-js`, parallel)

Decision: [ADR-0003](docs/adr/0003-native-javascript-engine.md) — build our own
ECMAScript engine in Rust; detailed plan:
[docs/architecture/js-engine-plan.md](docs/architecture/js-engine-plan.md).

- Runs **parallel** to Phases A–D; it gates nothing in them.
- QuickJS stays the live runtime until the swap gate (R4/R5 pass rates +
  real-page smoke suite). JsBridge is the only swap point — capability modules
  survive the swap.
- Debt items #1 and #3 in [issues.md](docs/architecture/issues.md) are design
  inputs to the host layer, not interim work.

---

## Phase A — Page Fidelity (NEXT)

Modern pages silently lose content today. Four caps do it, and painting is O(all
elements):

1. **Cap removal with a memory budget** (`src/engine/pipeline/extractor.rs`,
   `fetcher.rs`)
   - Element cap 2000 → budget-based (e.g. keep extracting; stop at ~100k
     elements or RAM guard). Depth 50 → ~200. Text node 5000 chars → raise to
     64k. HTML 1MB → 5MB. CSS sources 50 → raise with total-bytes cap.
   - Acceptance: a full Wikipedia article and a Bootstrap dashboard extract
     without silent drops; test asserts element counts above old caps.

2. **Viewport culling in paint + hit-test**
   (`src/ui/screens/browser/canvas.rs`)
   - Elements are y-sortable after layout: binary-search the visible range,
     draw only those; hit-test iterates the same slice. Scrollbar math uses
     max(y+h) which we already compute.
   - Acceptance: 50k-element page paints in <16ms/frame; click targets stay
     exact (existing hit tests).

3. **StyledElement slimming** (`extractor.rs`) — prerequisite for 2x element
   counts staying cheap
   - Move repeated `String` fields to enums/compact types (`display`,
     `position`, `flex_direction`, `font_weight`), drop duplicated raw-CSS
     strings once mapped. Do NOT attempt the full ComputedStyle/LayoutInput/
     LayoutOutput three-way split yet — slim first, split only if profiling
     demands it.

4. **Wrap text once** (`src/engine/pipeline/layout.rs`)
   - `wrap_text` runs during height estimation AND again post-layout. Make
     wrapping happen once, feed wrapped line count into taffy heights.

5. **Finish the TreeSink stubs** (`src/engine/parser.rs`)
   - `reparent_children`, `append_before_sibling`, `remove_from_parent` are
     no-ops → foster-parenting and misnested-tag recovery silently produce
     wrong trees. Implement them against the children-map (the info is already
     tracked); add table/misnested-tag regression tests.

**Out of scope for A:** web fonts (`@font-face`), animations, async network.

## Phase B — UX Completion

Wire what's half-built instead of adding new surface:
- Bookmarks bar (`show_bookmarks_bar` setting exists; `load_bookmarks`/
  `save_bookmarks` exist in navigator.rs; nothing renders them).
- Dark mode + accent color: settings UI picks an index but never persists to
  `VayuSettings` nor calls theme/style layer.
- History UI (back/forward works; no history list view).
- Tab restore polish: crash recovery banner, duplicate-tab, close-others.
- Multi-window correctness check (global statics make this fragile — see debt).

## Phase C — Hardening

- TLS: switch reqwest `native-tls` → `rustls-tls`.
- Cookie jar: SameSite enforcement on cross-site sends (parsing exists,
  enforcement doesn't); per-cookie size limit.
- CSP: extend checks to stylesheets loaded via `<link>` (currently checked for inline/scripts only).
- HTML parser fuzzing (cargo-fuzz over `parse_html`).
- Sandboxing decision ADR: QuickJS capability policy choke point is documented
  in CONTEXT.md; decide process-isolation later, don't build now.

## Phase D — Performance & Advanced (post-fidelity)

- Async reqwest (tokio) for parallel subresource fetches.
- `@font-face` + fontdb loading pipeline (cosmic-text already bundles fontdb).
- CSS transitions/animations engine (rquickjs timers exist; needs interpolation).
- Revisit `url` crate for RFC-correct resolution if edge cases bite.

## Known debt (do not fix opportunistically; schedule it)

> Architecture-level ledger with file/line references lives in
> [docs/architecture/issues.md](docs/architecture/issues.md). The items below
> are the phase-specific slice.

- `OnceLock<Mutex<..>>` global state across net/js/fetcher blocks multi-instance
  and cross-test isolation (ISSUE 15 in `.kilo/plans`). Fix when it bites, not before.
- Two extraction paths (`extract_elements` vs `_flat`) still diverge in edge cases.
- `browser/mod.rs` ≈1000 lines again after the split; next feature there should
  extract navigation/history into its own submodule first.
- html5ever `parse_fragment` not shared by innerHTML path (two parser behaviors).
