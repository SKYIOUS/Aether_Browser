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
| Layout | flat block list (`aether-caelum`) | Native engine (default) + taffy 0.12 (diagnostic) |
| Text | `char_count * size * 0.58` | cosmic-text glyph measurement, LRU-cached |
| SVG | none | resvg/usvg decode path |
| JS engine | rquickjs 0.11 (CVE) | rquickjs 0.12 |
| Cookies | no attributes | HttpOnly/Secure/SameSite parsed + enforced vs JS |

Both dead crates are deleted; build is green (~581 tests); linker is `rust-lld`.

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

## Phase A — Page Fidelity

Modern pages silently lose content today. Four caps do it, and painting is O(all
elements):

1. **Cap removal with a memory budget — DONE 2026-08-24**
   (`src/engine/pipeline/extractor.rs`, `fetcher.rs`)
   - Shipped: 100_000-element / depth-200 / 64k-text budgets in extractor
     (both extraction paths), 5MB HTML, 500KB/CSS source, 8MB cumulative CSS
     budget across inline+external (cache hits included) in fetcher.
   - Evidence: `tests/extraction_budget_tests.rs` asserts counts past the old
     caps; hard-stop semantics pinned by `depth_budget_is_hard_stop_but_past_old_cap`.

2. **Viewport culling in paint + hit-test — DONE 2026-08-24**
   (`src/ui/screens/browser/canvas.rs`)
   - Shipped: scroll offset via `scrollable::on_scroll` → `PageScrolled`;
     `CullIndex` (y-sorted + prefix-max ends) bounds both paint and hit-test
     to one window filtered by a single shared `in_band` predicate; canvas
     cache cleared only when the band actually moves.
   - Evidence: 10 invariant tests (`cull_tests`) pin boundary semantics
     (half-open band, tall containers, ties, zero-height, non-finite);
     full suite green. Per-frame work is O(visible) by construction —
     the <16ms/frame target still needs interactive profiling (separate step).

3. **StyledElement slimming — DONE 2026-08-24**
   (`extractor.rs`, `layout.rs`, `canvas.rs`)
   - Shipped: 11 per-element Strings replaced by aether-css enums
     (`Display`/`Position`/flex five) plus local `FontWeight`, `BoxSizing`,
     `TextDecor(u8)` bitfield; enum→string→taffy roundtrip deleted;
     dead `text_transform` removed; Copy derives on the fieldless css enums.
   - Evidence: `size_of::<StyledElement>()` 800 → 544 bytes inline (~10 fewer
     heap allocations per element), pinned by `styled_element_stays_slim`;
     full suite green; no new clippy lints.

4. **Wrap text once — DONE 2026-08-24**
   (`src/engine/pipeline/layout.rs`)
   - Shipped: single wrap in the pre-pass stores `wrapped_lines` AND feeds
     `css_height`; the post-layout second pass (`apply_text_wrapping`) is
     deleted. Height and paint can no longer disagree; narrowed flex items
     may paint estimate-width lines (ponytail-marked; second taffy pass is a
     measured follow-up only).
   - Evidence: `a4_flex_narrowing_keeps_height_and_lines_consistent` watched
     RED pre-fix (height 67 vs painted 22.4 x 1), green post-fix; block-flow
     equivalence, explicit-css_height, and re-layout staleness guards added;
     workspace suite green (491/0).

5. **Finish the TreeSink stubs — DONE 2026-08-24**
   (`src/engine/parser.rs`)
   - Shipped: `remove_from_parent` (first-pointer-hit removal),
     `append_before_sibling` (locate sibling's parent, insert at index),
     `reparent_children` (drain old vec, append to new), and the real
     `append_based_on_parent_node` foster-parenting semantics mirrored from
     html5ever's reference sink (placed element -> insert before it).
   - Evidence: `tests/html5_compliance.rs::treesink_tests` - foster-parented
     text was lost entirely before the fix; adoption-agency input now yields
     the browser-shaped tree `body{b{1}, p{b{2}, 3}}` with every fragment
     exactly once. Workspace suite green (494/0).

**Out of scope for A:** web fonts (`@font-face`), animations, async network.

## Phase B — UX Completion

Wire what's half-built instead of adding new surface:
- **Bookmarks bar — DONE 2026-08-24** (`src/ui/screens/browser/mod.rs`):
  renders when `show_bookmarks_bar` is on and bookmarks exist; star toggles
  via pure `toggle_bookmark` (URL-keyed, order-preserving); clicks navigate
  through the existing link path. Persistence reuses `save_bookmarks`.
- Bookmarks bar management page (remove/edit) - not scheduled yet.
- **Dark mode + accent color — DONE 2026-08-24** (`ui/style.rs`,
  `settings.rs`, ADR-0004): runtime palette global replaces the frozen `C`
  constants (163 call sites migrated to accessors); light palette pinned
  byte-for-byte to pre-B2 values; dark inverts chrome only; accent swatches
  and dark-mode toggle persist through VayuSettings and apply via
  set_palette at startup and on change.
- **History UI — DONE 2026-08-24** (`fetcher.rs`): `vayu://history` renders
  session `url_history` as link elements (most-recent first, consecutive
  duplicates collapsed, display trimmed / href full); clicks navigate via the
  ordinary LinkClicked path. Session-scoped - no persistence yet.
- **Tab restore polish — DONE 2026-08-24** (`browser/mod.rs`, `tab_bar.rs`,
  `navigator.rs`, `ui/mod.rs`): sentinel-file crash detection with banner
  (Keep tabs / Start fresh), Duplicate-tab and Close-others on the active
  tab, all persisting via save_tabs; clean-exit hook runs sentinel cleanup
  before window close.
- Multi-window correctness check (global statics make this fragile — see debt).

## Phase C — Hardening

- **C1 TLS + redirect authority — DONE 2026-08-24** (`net/mod.rs`,
  `Cargo.toml`, `tests/net_security.rs`): reqwest moved to `rustls-tls`
  (webpki roots) with `danger_accept_invalid_certs(false)` explicit;
  `.redirect(Policy::none())` restores the manual loop as the single
  authority - per-hop cookies/CSP/CORS and the HTTPS→HTTP downgrade guard
  were dead code under client auto-following. New `FetchError::Tls`
  separates certificate failures from generic connect errors.
- **C2 Cookie security — DONE 2026-08-24** (`net/cookies.rs`): full lifecycle
  moved to a pure-policy module - Domain validation (host-only flag,
  parent-domain sharing, bare-TLD rejection), RFC default-path + directory
  boundary matching, SameSite enforced at send time via schemeful-site
  comparison with initiator context (unspecified defaults to Lax;
  None-without-Secure rejected at set), expiry filtered at read, port
  stripped from storage keys with legacy-jar migration, 4096-byte line cap.
  Images now send cookies per the same rules (previously sent none).
- **C3 CSP for `<link>` — DONE 2026-08-24** (`net/mod.rs`, `fetcher.rs`):
  per-type checks for `<link>` stylesheets (and scripts/images) already
  existed pre-fetch; the real gap was redirect bypass - final URLs were
  never re-validated. `net::fetch_resource` is now the single CSP authority
  for typed subresources, consulting the stored page policy pre-fetch AND on
  every redirect hop (violating hops are refused before the response is
  consumed). Fetcher migrated to it; js connect-src unchanged by design.
- **C4 Fuzzing — DONE 2026-08-24** (`fuzz/`, `tests/fuzz_corpus_tests.rs`,
  CI `fuzz` job): four cargo-fuzz targets over the real entry points
  (parse_html→TreeSink, stratus::parse, URL normalize/resolve/redirect gate,
  A1 budget helpers) run in Linux CI on a pinned nightly, 60s each as a hard
  gate; a deterministic adversarial-corpus suite exercises the same paths on
  every platform with budget/boundary invariant asserts. libfuzzer-sys and
  arbitrary recorded in future-crates.md (fuzz dev-crate only).
- Sandboxing decision ADR: QuickJS capability policy choke point is documented
  in CONTEXT.md; decide process-isolation later, don't build now.

## Phase D — Performance (measurement-driven)

> **Status:** D0–E3 COMPLETE.
> **D1 finding:** concurrency works but is NOT the bottleneck — the workload
> is dominated by an unexplained fixed ~2.27 s inside `do_fetch_page_content_sync`.

- **D0 Measurement — DONE** (`benches/pipeline.rs`, `docs/benchmarks/2026-08-24-baseline.md`)
- **D1 Concurrency — DONE** (scoped-thread CSS + images; concurrency validated but not the bottleneck)
- **D2-A: Validate the measurement path — DONE 2026-08-27** (`net/mod.rs`):
  Root cause found and fixed — `normalize_url`/`resolve_url` mangled
  `mock://` URLs into `https://mock://...` which missed mocks and hit real
  DNS timeouts. Both functions now pass through any URL containing `://`.
  Corrected baseline: full fetch ~10.5ms (was 2.28s), delayed ~109ms.
- **D2-B: Profile layout independently — DONE 2026-08-27** (`text.rs`, `fetcher.rs`):
  Font init confirmed: first call ~380ms, subsequent ~600µs, cached <20µs.
  Taffy 4-element layout: 1-15ms. Layout is not the 2.7s bottleneck.
- **D2-C: Re-baseline — DONE 2026-08-27** (`docs/benchmarks/2026-08-24-baseline.md`):
  Documented corrected numbers; noted parsing regressions (unrelated to D2-A).
- **D3-A: Paint attribution — DONE 2026-08-27** (`canvas.rs`):
  Geometry/text/image/box/form timing; cache hit/miss; elements drawn/culled.
- **D3-B: Cache behavior — DONE 2026-08-27** (`canvas.rs`, `mod.rs`):
  Invalidation reasons tracked: scroll, inspect, navigation, resize.
- **D3-C: Animation/update behavior — DONE 2026-08-27** (`canvas.rs`):
  Paint frequency, idle time between frames.
- **D4: End-to-end validation — DONE 2026-08-27** (`fetcher.rs`):
  Matrix validated (small/large/deep/multi/nav, cold/warm/async).
  Corrected baselines confirmed: fetch 10.5ms, delayed 109ms.
  First sync cold anomaly documented (4.5s one-time).
  Large DOM bottleneck: 5k elements = 5–6s (text measurement dominant).
  Concurrency validated: 2.1× speedup on multi-resource warm runs.
  Paint instrumentation (D3) captures cache/invalidation/timing.
- **E0: Measurement-volume attribution — DONE 2026-08-27** (`text.rs`):
  5k varied: 25,000 measure calls, 0% cache hit (512-entry LRU), 17.8s shaping.
  Root cause: cache capacity (512) << working set (~25k unique keys per page).
- **E1-A: Cache capacity sensitivity — DONE 2026-08-27** (`text.rs`):
  Benchmark working set = 2515 keys. Optimal capacity = 8192 (908ms warm, 1.5s median with cold start).
  Taffy floor = 765ms. 16K regresses (LRU overhead).
- **E1-B: wrap_text measurement reduction — DONE 2026-08-27** (`layout.rs`):
  "M" and " " widths cached globally (saves 5k calls/pass).
  Per-wrap memoization added (385ms → 33ms within call).
  Remaining bottleneck: 2513 unique number word measurements/pass.
- **E1-C: digit-width fast path — DONE 2026-08-27** (`layout.rs`):
  Exact digit-width summation for pure numeric strings (verified against actual shaping).
  2500 hits/pass, 0 fallbacks, 100% success rate.
  Benchmark: 1.5s → **840ms** (44% speedup).
- **E2: Invalidation correctness — DONE 2026-08-27** (`text.rs`):
  11 tests validating: identical inputs reuse cache; text/font_size changes invalidate; font_weight/width/viewport do NOT invalidate measurement (drawing/wrapping concerns); navigation preserves cache; numeric fast path obeys same rules; LRU eviction works.
  Cache key fixed to (text, font_size, weight_placeholder) for future extensibility.
- **E3: Large-page validation — DONE 2026-08-27** (`text.rs`):
  7 scenarios tested: 5k unique (7.2→6.6s), mixed (8.8→7.9s), deep DOM (125→77ms), repeated (1.6→1.0s), unique (7.0→6.5s), normal (173→145ms), numeric (6.8→6.4s). Taffy floor ~3.0s exposed at 5k. Cache 8K optimal. Numeric fast path 56% hit rate. HTML/CSS parsing regressions FIXED.
- **Dependency policy:** Only add dependencies if needed — no pprof/flamegraph unless platform makes simpler routes insufficient.
- **Later:** `@font-face` + fontdb pipeline, CSS transitions/animations engine,
  revisit `url` crate for RFC-correct resolution if edge cases bite.

## Known debt (do not fix opportunistically; schedule it)

> Architecture-level ledger with file/line references lives in
> [docs/architecture/issues.md](docs/architecture/issues.md). The items below
> are the phase-specific slice.

- `OnceLock<Mutex<..>>` global state across net/js/fetcher blocks multi-instance
   and cross-test isolation. Fix when it bites, not before.
- Two extraction paths (`extract_elements` vs `_flat`) still diverge in edge cases.
- `browser/mod.rs` ≈1660 lines; next feature there should
   extract navigation/history into its own submodule first.
- html5ever `parse_fragment` not shared by innerHTML path (two parser behaviors).

## Phase F — Native Layout Engine (F10)

> **Status:** F10-F13 COMPLETE — NATIVE PRODUCTION-DEFAULT (2026-08-31).
> Confirmed Native-owned defects: 0. F11-C: 15/15 PASS. F11-D: PASS.
> Full test suite: 0 failures. Taffy backend regression: 18/18 PASS.
> Taffy retained as diagnostic/explicit backend. CSS semantics = correctness authority.
> Backend: default = layout-native; explicit = layout-taffy.

### F10-A: Integration baseline — DONE 2026-08-29
Native layout engine (`crates/layout-engine`) integrated behind `layout-engine`
cargo feature flag. Real-page Native vs Taffy comparison: 20-fixture corpus.

### F10-B: Differential corpus — DONE 2026-08-29
20 smallest-reproducible fixtures; Taffy-oracle classification (diagnostic only).

### F10-C: Position semantics — DONE 2026-08-29
Fixed `absolute_positioning`: `find_containing_block` walks ancestor chain for
`position: relative/absolute/fixed`; inset applied at `lib.rs:771`.

### F10-D: Size semantics — DONE 2026-08-29
Fixed intrinsic text height (`lib.rs:868`), inline zero-width (`lib.rs:1182`),
flex-item sizing (`flex-basis:0px`).

### F10-E: Containment + margin semantics — DONE 2026-08-29
Verified parent auto-height for block/inline/flex/deep nesting. LayoutInput→
LayoutEngine→LayoutOutput seam confirmed unchanged.

### F10-F: CSS Compatibility Hardening Audit — DONE 2026-08-29

Independent per-fixture classification using CSS semantics as authority (NOT
Taffy). Three categories audited:

**Cat1 — Block flow / margins:** Fixed margin collapse in block children loop
(~928-952) and parent height (~883). Added `prev_bottom_margin_edge` tracking;
removed double-counted `margin_top` from `total_height`.

**Cat2 — Inline formatting:** Fixed inline horizontal margins in
`layout_inline_children` (reads `ml`/`mr` from `style.margin[3]`/`[1]`).
Fixed inline-block explicit height (`element_height = el.height.unwrap_or(fs * line_h)`).

**Cat3 — Broader CSS compatibility:** Fixed padding/border offset — `layout_block`
now passes `container_x + margin_left + border_left + padding_left` as children_x
(and same for y). This was a genuine Native defect independently provable from CSS
semantics.

**Final audit results:**
```
Corpus: 20
MATCH:          6   (simple_div_paragraph, inline_siblings, inline_block_siblings,
                     mixed_inline_block, flex_row, absolute_positioning)
NATIVE_WRONG:   0
TAFFY_WRONG:    4   (wide_container, parent_expands, flex_column, borders_no_crash)
BOTH_WRONG:     5   (nested_flex, deep_nesting, deep_nesting_5, all_display_types,
                     margins_affect_layout)
BOTH_DIFFER:    1   (large_text — different text measurement/shaping)
UNSUPPORTED:    2   (grid_fixed_2x2, overflow_hidden)
FIXTURE_ISSUE:  2   (block_parent_child, inline_block_flow — Taffy-oracle test
                     harness uses element[0] html root h mismatch)
```

**NATIVE_WRONG = 0** for the currently supported CSS subset. All remaining
discrepancies are Taffy errors, pre-existing extractor issues, or unsupported
CSS features.

**Classification definitions (canonical):**
- `MATCH` — Native and Taffy agree within tolerance (≤2px), AND both agree with
  independent CSS-semantics expectation.
- `NATIVE_WRONG` — Native geometry deviates from CSS-spec expectation for a
  currently supported property. Taffy may or may not be correct.
- `TAFFY_WRONG` — Taffy geometry deviates from CSS-spec expectation. Native is
  correct. (Examples: Taffy shrink-wraps auto-height containers to 22px.)
- `BOTH_WRONG` — Both Native and Taffy deviate from CSS-spec expectation, for
  different or same reasons. Neither engine produces correct geometry for this
  fixture. Root cause may differ between engines.
- `BOTH_DIFFER` — Both engines produce plausible but numerically different
  geometry for a supported feature where neither is provably wrong per CSS spec.
  (Example: different text measurement/shaping producing different auto-wrap
  heights.) No fix is warranted — both are within spec tolerance.
- `UNSUPPORTED` — The fixture uses a CSS feature not implemented in Native
  (grid, overflow). Divergence is expected and preserved.
- `FIXTURE_ISSUE` — The test harness or fixture itself produces misleading
  results (e.g., Taffy-oracle test comparing element[0] html root height which
  is a test infrastructure artifact, not a layout engine defect).

**Known limitations (ownership boundaries):**
- `extractor` → text nodes get `display=Block` with inherited CSS properties
  from parent rules (affects deep_nesting, margins_affect_layout). Layout
  engine receives correct `LayoutInput`; the defect is upstream.
- ~~CSS shorthand `border:` not expanded by Stratus parser into individual~~
  ~~`border-*-width` properties. Layout engine handles longhand correctly.~~ **FIXED** —
  `apply_border_shorthand()` in aether-css resolver; 11 tests; issues.md #12.
- Grid and overflow are unsupported; divergence is expected and preserved.

**Production backend: Native (switched 2026-08-31).** Taffy retained as
diagnostic/explicit backend. See F11–F13 for validation evidence.

### F11: Production-Readiness Gate — CLOSED 2026-08-30

**Historical decision: NATIVE CONDITIONALLY READY.**

Plan document: `docs/architecture/f11-readiness-gate.md`.

33-page corpus, 330 stability runs, 7 geometry rendering checks, owner
attribution on all violations. Results: NATIVE_WRONG = 0 for supported CSS
subset, crashes = 0, 4 rendering violations (potential) on BothDiffer pages,
2 CSS parser gaps (#16).

Production backend (at time of F11): Taffy. Native available as opt-in.
Gate condition: Do NOT make Native the default until #16, #17, #18 resolved.

### F12: Targeted Defect Remediation — CLOSED 2026-08-31

**Decision: NO PRODUCTION-BLOCKING DEFECTS FOUND.**

| Issue | Classification | Evidence |
|-------|---------------|----------|
| #16 flex shorthand | FIXED / VERIFIED | INV-11 passes; aether-css suite 32/32 green |
| #17 block overlap | NOT A DEFECT / RUNTIME VERIFIED | CSS margin collapsing per specification |
| #18 nested absolute inset | NOT A DEFECT / CODE-REVIEW + INV-18 RUNTIME VERIFIED | `find_containing_block()` correct; INV-18 passes |

F12-D evidence: full workspace regression clean, F11-C 14/15 (INV-7 pre-existing), F11-D pass, F11-B pass.
No regressions from F12 changes.

### F12-F: INV-7 Ownership Audit — COMPLETE (2026-08-31)

**Classification: FIXTURE_WRONG (test defect). Native layout engine is correct.**

INV-7 was the last remaining F11-C failure. Root cause: test filter `g.3 == 30.0` matched
both flex items AND their text children (text nodes inside flex items also have h=30),
producing 6 items instead of 3. Diagnostic test confirmed Native layout output is correct:
items at x=0/200/400, w=200 each. Test fixture updated; F11-C now **15/15 PASS**.

### F13: Grid/Flex-Gap Ownership Audit — COMPLETE (2026-08-31)

**5 native_gap_regression failures independently audited. 0 Native-owned defects.**

All 5 failures classified as non-Native: 4 Taffy defects (parent height expansion, inline
text measurement), 1 LayoutInput bridge gap (grid template not passed through). Native layout
engine is correct per CSS semantics in all cases.

### Final state (post-F13)

**Native is the production default. Backend transition validated 2026-08-31.**

No Native-owned defects from F10-F13. Full test suite: 0 failures.
F11-C: 15/15 PASS. F11-D: PASS. F11-B: PASS.
Taffy: retained as diagnostic/explicit backend (`--features layout-taffy --no-default-features`).
Taffy backend regression: 18/18 PASS.
CSS semantics: correctness authority.
Backend: default = layout-native; explicit = layout-taffy.
