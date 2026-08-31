# Architecture Debt Ledger — Vayu Browser

> Canonical list of **open** architecture debt. Every entry was verified against
> the code on 2026-08-24. If an entry no longer matches the code, fix this file
> in the same change — a stale ledger is worse than none.
>
> Division of labor: `PLAN.md` says *what's next and when*; this file says
> *where the debt lives and why it matters*. Phase-specific work items live in
> PLAN.md only; do not duplicate them here.

## Open items

| # | Debt | Where | Why it matters | Tracked by |
|---|------|-------|----------------|------------|
| 1 | `Arc<Mutex<JsBridge>>` shared across threads while rquickjs is single-thread | `src/engine/js/js_bridge.rs` | Deadlock/data-race surface; limits callback design | Interim: thread confinement; terminal: aether-js realm model (ADR-0003) |
| 2 | HTTP cache is an unbounded `HashMap`; TTL checked on read, expired entries never evicted, no byte/entry cap | `src/engine/net/mod.rs:73-96` | Unbounded growth over long sessions (`lru` dep already available) | Unscheduled |
| 3 | Timer/event callbacks stored as source strings and re-evaluated per tick; no `Event` objects, no bubbling/capture phase | `src/engine/js/events.rs`, `timers.rs` | Interactive pages break; closure state impossible | Design input to aether-js host layer (ADR-0003) |
| 4 | DOM API surface minimal (limited standard properties/methods over `aether-dom`) | `crates/aether-dom`, bridge modules | Page JS fails on missing APIs | Ongoing fidelity work (PLAN Phase A/B) |
| 5 | CSS custom properties (`--var`) and `calc()` unsupported (no matches in `aether-css`) | `crates/aether-css` | Ubiquitous on modern sites | Unscheduled |
| 6 | No float or table layout (taffy covers flex/grid/block+inline only) | `src/engine/pipeline/layout.rs` | Legacy and data-heavy pages misrender | PLAN Phase D territory |
| 7 | Static byte ceilings exist (HTML/CSS/DOM budgets from A1) but no runtime memory-pressure response; CSS/image caches remain entry-count LRUs without byte accounting | `src/engine/pipeline/fetcher.rs`, `src/engine/pipeline/extractor.rs` | OOM risk on heavy sessions persists via cache growth | Partially addressed by PLAN A1; pressure handling unscheduled |
| 8 | Hand-rolled logging (`plog!`), no levels/filtering/structure | `src/logging.rs`, call sites | Poor diagnosability | Unscheduled; candidate: `tracing` |
| 9 | Residual `String`-typed errors in net paths despite thiserror adoption elsewhere | `src/engine/net/mod.rs` | Lost error context/kind matching | Unscheduled |
| 10 | CI clippy gate runs without `--all-targets`; widening fails today: `clippy::needless_range_loop` in `korlang` bytecode compiler | `.github/workflows/ci.yml`, `korlang/src` | Test/bench code escapes the lint gate | Fix lint first, then widen CI command |
| 11 | The fmt/clippy/test CI workflow has never run (`gh run list`: CodeQL only); repo is not rustfmt-clean under any recent stable (~1400 diff hunks) and carries ≥14 pre-existing `clippy -D warnings` failures (korlang needless_range_loop, aether-css match→`?`, 12 in vayu-browser incl. if-same-then-else, collapsible-if, useless format!) | `.github/workflows/ci.yml`, repo-wide | Gates documented as binding have never been enforced; "green" claims were unverified until 2026-08-24 | Dedicated formatting/lint chore + first real CI run; do not fix opportunistically inside feature tasks |
| 12 | CSS shorthand `border:` not expanded by Stratus into individual `border-*-width` properties | `src/engine/stratus.rs` | Layout engine handles longhand `border-left-width` correctly; shorthand produces 0 widths | Unscheduled; Stratus parser scope |
| 13 | Text nodes in extractor get `display=Block` with inherited CSS properties from parent rules | `src/engine/pipeline/extractor.rs` | Affects deep_nesting, margins_affect_layout fixtures (inflated parent heights). Layout engine receives correct LayoutInput; defect is upstream in extractor | Known limitation; extractor ownership |
| 14 | Native layout engine: no grid or overflow support | `crates/layout-engine/src/lib.rs` | Grid/overflow fixtures diverge; expected and preserved as unsupported | Future phase if needed |
| 15 | ~~Native layout engine: `position: relative` offsets (`left`/`top`/`right`/`bottom`) silently dropped in `layout_block`~~ **FIXED** | `crates/layout-engine/src/lib.rs:852-857` | F11-C INV-3: Native x=0 vs expected 30 for `left:30px`. Fixed: `rel_x`/`rel_y` computed from `el.inset[3]`/`el.inset[0]` for `Position::Relative`, applied to output coordinates. Children positioned at normal-flow origin (no double-count). | **FIXED in #15** |
| 16 | ~~CSS shorthand `flex:` not expanded by aether-css resolver~~ **FIXED** | `crates/aether-css/src/resolver.rs:111-125` | Same pattern as #12: longhand `flex-grow`/`flex-shrink`/`flex-basis` handled but `flex:1` shorthand ignored. F11-C INV-7/11: all flex items get full container width | **FIXED in F12-A** — resolver expanded; INV-11 now passes |
| 17 | Native layout: paragraphs with vertical margins overlap by margin amount (P03) | `crates/layout-engine/src/lib.rs` | F11-D: block children with `margin: 8px 0` overlap — next paragraph y is 8px above previous paragraph's end. Likely margin collapsing or margin application difference | **NOT A DEFECT** — CSS margin collapsing per specification (F12-B runtime verified) |
| 18 | Native layout: absolute element's containing block resolves to grandparent instead of parent in nested positioned ancestors (P21) | `crates/layout-engine/src/lib.rs` | F11-D: absolute element at (10,10) instead of expected (30,30) — positioned relative to page root, not nested relative parent at (20,20). Containing block resolution for nested positioned elements is wrong | **NOT A DEFECT** — `find_containing_block()` correctly implements nearest-positioned-ancestor CSS rule (F12-C code-review + F12-D INV-18 runtime verified) |

Already scheduled in PLAN.md — see there, not here: viewport culling (A2),
TreeSink stubs (A5), TLS/cookies-enforcement/CSP-for-`<link>`
(C), async fetch + `@font-face` (D), global-statics/multi-window fragility,
dual extraction paths, `browser/mod.rs` size pressure, `parse_fragment`
divergence (Known Debt).

## Resolved since the last audit (kept for history)

| Old issue | Resolution |
|---|---|
| Hand-written HTML parser | html5ever 0.39 + custom TreeSink |
| Character-heuristic text wrapping, no fallback fonts | cosmic-text glyph measurement + fallback, LRU-cached |
| No SVG | resvg/usvg decode path |
| Caelum flat block-list layout, inline post-processing | taffy 0.12 tree layout + inline formatting context |
| Stratus ad-hoc tokenizer | Servo cssparser 0.37 |
| Selector matching hand-rolled in js_bridge | Servo `selectors` crate |
| Hand-written URL resolution | `url` crate promoted to direct dependency |
| Cookies without attributes | HttpOnly/Secure/SameSite parsed + enforced against JS (`CookieAttrs`, `net/mod.rs:101`) |
| CSP source-expressions only | Nonce/hash sources + report-uri/upgrade-insecure directives parsed; per-destination allow fns |
| Image cache O(n) sweep on access | LRU caches (CSS 100 entries, images 50) |
| build.rs codegen + css-caelum-bridge.json | Deleted with Caelum |

Former analysis docs `rendering-layout-fixes.md` and pre-migration sections of
this file are superseded by the table above plus PLAN.md.
