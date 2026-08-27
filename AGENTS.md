# AGENTS.md — Vayu Browser (operating contract)

Rules here bind every agent and human change. Violations = the change is wrong
even if tests pass.

## Docs map (authority order)
| Doc | Role |
|---|---|
| `AGENTS.md` | Operating contract (this file) |
| `PLAN.md` | Single source of truth for what's next; Track 2 = native JS engine |
| `CONTEXT.md` | Domain vocabulary + doctrine. Use its terms exactly |
| `docs/adr/` | Decisions. New decision → new ADR; conflicts reopen via ADR only |
| `docs/architecture/issues.md` | Verified open-debt ledger |
| `docs/architecture/future-crates.md` | Crate evaluation; new dependency needs an entry here first |
| `docs/architecture/js-engine-plan.md` | aether-js engine plan (ADR-0003) |

**Doc discipline:** a doc that contradicts the code is a bug. Any change that
invalidates a statement in these docs updates them in the same commit.

## Non-negotiables
1. **Evidence before claims.** Run `cargo test`; run `cargo fmt --check` +
   `cargo clippy -- -D warnings` when touching code CI covers. Cite output
   stating the exact commands and what they covered — "tests pass" without
   scope is not evidence. `-D warnings` is deliberate: every warning
   (including rustc's) is treated as an error. Never report success from
   intent. An unverified claim is a false claim.
2. **Scope discipline.** Touch only what the task names. No drive-by refactors,
   no comment churn, no reformatting of untouched lines. Exception: deleting
   what your own change made redundant is in scope (Operating protocol §3) —
   anything else outside the named task is drift.
3. **Root cause, not symptom.** Before changing a function's behavior or
   contract, find every caller. Fix at the shared choke point once — not
   per-caller guards.
4. **Files normally stay under 1000 lines.** Crossing means extract a
   submodule — unless extraction would worsen cohesion, which requires a
   documented reason naming why (commit message or ADR).
   (`browser/mod.rs` is the repeat offender.)
5. **No new dependency** without a `future-crates.md` row + rationale.
   *Leaf crate* = may depend on foundational libraries but must not own
   browser-wide orchestration, UI policy, platform integration, or
   cross-subsystem state. Nothing platform-level inside `crates/aether-js`.
6. **Decisions get recorded.** Non-obvious choice → ADR. Naming/concepts →
   `CONTEXT.md`. No second vocabulary.
7. **Comments carry why, not what.** Deliberate ceilings get a
   `ponytail:` marker naming the ceiling and upgrade path.
8. **Security boundaries are explicit.** Page-controlled content must never
   reach browser UI state, Korlang, filesystem APIs, privileged host
   capabilities, or other-origin data except through an explicitly reviewed
   bridge with an enforced policy and negative-path tests.

## Operating protocol (how every task runs)
1. **Skills before action.** Invoke matching skills BEFORE responding,
   exploring, or asking clarifying questions — when available in the
   environment. Process skills first (brainstorming / systematic-debugging /
   grill-\*), implementation skills second. If a prescribed skill is found
   inapplicable or incorrect, stop using it and record the reason in the task
   evidence. No matching skill exists → proceed under this contract and note
   that.
2. **Codebase Memory MCP always, with a fallback.** The graph is the map of
   record:
   - Before editing any symbol → `search_graph` / `get_code_snippet` it.
   - Before changing a function → `trace_path` every caller.
   - Before optimizing anything → `query_graph` complexity/hot-path fields;
     measure, never guess.
   - After structural changes → re-index so the graph stays truthful.
   - **Fallback:** if MCP is unavailable, stale, or out of scope for the
     change, use direct repository tracing (grep/rg, compiler diagnostics,
     `cargo check`) and state that fallback in the final evidence. Never
     block on tooling; never silently skip verification either.
3. **Exhaustive effort is the default, not a mode.**
   **Implementation-changing tasks** end with **prove → prune → re-check**;
   investigation/review/doc-only tasks end with **prove → re-check** (do not
   invent cleanup to satisfy the ritual):
   - **Prove:** evidence attached (command output, test names, file:line refs).
   - **Prune:** same-change deletion of exactly what your work made redundant —
     code, docs, deps — even in files the task didn't name. That deletion is
     not scope drift; anything beyond it is. Leaving dead weight behind is an
     incomplete task.
   - **Re-check:** fresh read of the final diff; adversarial pass over risky
     logic; every touched doc re-checked against reality.
4. **Subagents over solo guessing.** Dispatch them instead of hand-waving:
   - Unfamiliar area → `explore` agent before planning anything.
   - Non-trivial diff → fresh-context reviewer (code-review / adversarial /
     test-gap finder) before claiming done; reviewers get no stake in the
     outcome, so their findings outrank self-assessment.
   - Independent subtasks run as parallel subagent dispatches — each with
     disjoint file scopes stated upfront; overlapping scopes serialize.
     Reviewers are read-only: they report findings, they do not edit.
5. **The loop never stops.** Every **implementation-changing** task runs
   PLAN → DO → CHECK in continuous cycles: plan the slice (todo list),
   implement it, run its tests, feed the results into the next slice's plan.
   Investigation, review, and documentation tasks use the appropriate subset
   of the loop but must still close with evidence. Stopping between loops is
   how half-done work ships; a task is done only when a full loop closes with
   green evidence and prove/prune/re-check complete.

## Test-first mandate
1. **Failing test first** for any testable behavior change or bugfix:
   red → green → refactor. Implementation without a preceding failing test is
   rejected on review.
2. Bug fixes begin with a **reproduction test** that fails on old code and
   passes on new. "Fixed" without a repro is not fixed.
3. Structural-only changes (refactors, extractions, dependency swaps) gate on
   the existing suite staying green — do not manufacture new tests to satisfy
   process. Performance work starts from a measured baseline, not a hunch.
4. Trust boundaries (net input, CSP, cookies, JS↔DOM bridge, aether-js host
   layer) get **negative-path tests**, not just happy paths.
5. aether-js work uses **test262** as its harness; gates per js-engine-plan.md §3.
6. Trivial one-liners are exempt — YAGNI applies to tests too.

## Anti-slop policy (hard bans)
1. **No narration comments.** Comments exist only for non-obvious why.
   `// increment counter` and friends are deleted on sight.
2. **No speculative abstractions.** No single-implementation traits, factories
   for one product, config knobs for constants. Deletion beats addition.
3. **No placeholders.** No stubs "for later"; no TODO/FIXME without a matching
   PLAN.md or issues.md row naming who/when.
4. **Never invent APIs, flags, or dependencies.** Verify against real source
   (codebase memory MCP) or authoritative upstream documentation before first
   use.
5. **Errors are handled, never swallowed.** No silent `.ok()` on data paths,
   no catch-log-continue where state matters.
6. **No scope drift.** Renames, reformats, comment edits outside the named
   task are reverted regardless of quality.
7. **No slop prose.** No emoji, no filler ("In summary…"), no sycophancy in
   commits, docs, PRs, or responses. State facts once.
8. **3am rule.** If you couldn't defend the line during a production incident,
   don't ship it.

## Commands
- Build: `cargo build` · Run: `cargo run`
- Tests: `cargo test` · single: `cargo test <name>`
- Status baseline (verified 2026-08-27, Windows local): 565 tests green
  (raw cargo-test totals double-count the lib block - src/main.rs re-runs it;
  per-phase deltas report focused suites plus this full-workspace figure); CI
  gates fmt/clippy/test on Linux+Windows+macOS plus a bounded Linux fuzz job
  (`fuzz/`, pinned nightly - not runnable locally on Windows-MSVC).
  Performance baseline: docs/benchmarks/2026-08-24-baseline.md (`cargo bench`).
  **D2-A finding:** the original 2.28s `fetch_full_mock` benchmark was invalid — mock:// URLs were mangled by normalize_url/resolve_url into https://mock://... which missed mocks and hit real DNS timeouts. Fixed: both functions pass through non-HTTP schemes unchanged. Corrected baseline: full fetch ~10.5ms, delayed variant ~109ms.
  **D2-B finding:** font init = ~380ms one-time (FontSystem::new), shaping = ~700µs/unique string, cached <20µs; Taffy layout on 4 elements = 1-15ms.
  **D1 finding:** concurrency is functionally validated but provides no meaningful end-to-end speedup because the measured workload was dominated by DNS timeouts (now fixed).
  **D1 finding:** the ~1.8s 5k-element layout benchmark remains the strongest validated performance target.
  **D3 finding:** paint instrumentation deployed — cache hit/miss, per-component timing (geometry/text/image/box/form), culling stats, idle time, invalidation reasons (scroll/inspect/navigation/resize).
  **D4 finding:** end-to-end matrix validated against corrected baselines. First sync cold anomaly: 4.5s one-time initialization (not steady-state). Large DOM (5k) = 5–6s (text measurement dominant). Concurrency: 2.1× speedup on multi-resource warm. Parser regression 2–4× persists (independent).
  **E0 finding:** 5k-element layout = 18s cold (realistic varied text). Breakdown: 25,000 measure calls, **0% cache hit** (512-entry LRU overflow), 25,000 Buffer constructions, **17.8s shaping**. Taffy floor when cached: 293ms–3.8s. Root cause: cache capacity (512) << working set (~25k unique keys per page).
  **E1-A finding:** cache capacity sensitivity measured. Benchmark working set = 2515 keys. Optimal capacity = 8192 → 908ms (2× speedup vs 1.8s). Taffy floor = 765ms. 16K regresses (LRU overhead).
  **Open finding:** HTML/CSS parsing regressed 2–4× vs the 2026-08-24 baseline (parse_html_small 362µs→1.87ms, parse_html_big_5k 169ms→612ms, parse_css_2k_rules 3.63ms→8.48ms); extraction/layout stable. Root cause unknown — requires independent profiling.
  Update this figure in the same commit that adds or removes tests.
- Commits: conventional prefixes (`feat:`/`fix:`/`docs:`/`refactor:`/
  `chore:`), atomic per concern, subject ≤72 chars; docs invalidated by a
  change ship in the same commit (Doc discipline above). Branches: short-lived
  topic branches off master; rebase before merge.

## Architecture
Rust, edition 2021. Entry: `src/main.rs` → `src/lib.rs` → `src/ui/mod.rs`
(`VayuApp`). Workspace: `korlang`, `crates/aether-dom`, `crates/aether-css`,
plus planned `crates/aether-js` (ADR-0003). UI is Iced 0.13 (canvas/image/
tiny-skia/wgpu/tokio), theme runtime-switchable via the global palette
(ADR-0004; `set_palette` at startup + settings actions).

Architecture status legend: **CURRENT** = exists, authoritative ·
**INTERIM** = use only within existing boundaries, replacement planned ·
**TARGET** = planned; do not implement unless PLAN.md authorizes it.

Engine stack:
- HTML (CURRENT): html5ever 0.39 + custom TreeSink (`src/engine/parser.rs`) → `aether_dom::Node`
- CSS (CURRENT): cssparser 0.37 tokenizer in Stratus (`src/engine/stratus.rs`), values/cascade in `aether-css`; selectors crate matching in `src/engine/js/selector_engine.rs`
- Layout seam (CURRENT): `apply_taffy_layout()` (taffy 0.12) in `src/engine/pipeline/layout.rs`, incl. inline formatting context
- Text (CURRENT): cosmic-text measurement/shaping (`src/engine/text.rs`), LRU-cached
- JS (INTERIM): rquickjs 0.12 (`src/engine/js/`, JsBridge flat DOM behind `Arc<Mutex<>>`) until the aether-js swap gate
- JS engine (TARGET): `crates/aether-js` per ADR-0003 + js-engine-plan.md rungs
- Net (INTERIM transport): blocking reqwest confined to `tokio::task::spawn_blocking` workers (`src/engine/pipeline/fetcher.rs`), rustls-tls + webpki roots, cookie jar w/ attributes, HTTP cache, CSP checks (`src/engine/net/mod.rs`); redirects are followed ONLY by the manual loop in `fetch_inner` (client policy is `Policy::none`) - async replacement is PLAN D
- Korlang (CURRENT): embedded UI DSL VM driving sidebar/chrome; never page-reachable except through binding policy (Non-negotiable #8)

Module map:
```
src/engine/{parser,stratus,text,style}.rs  net/  js/{js_bridge,timers,events,fetch,storage,selector*}
src/engine/pipeline/{fetcher,extractor,layout,navigator}.rs
src/ui/screens/browser/{mod,canvas,tab_bar,workspaces,devtools}.rs
src/ui/screens/{settings,palette}.rs       src/ui/style.rs
```

## Rendering pipeline
```
fetch_page_content() (async on Iced thread)
1. fetch HTML (reqwest blocking, max 1MB)          5. scripts → JsBridge vs flat DOM → to_dom()
2. parse_html() → aether_dom tree                  6. extract_elements() → StyledElements (+image decode)
3. collect <style> + <link> CSS (≤50, ≤500KB ea)   7. apply_taffy_layout() → x/y/w/h per element
4. Stratus parses CSS (no truncation)              8. PageCanvas paints; click hit-testing → messages
```

## Conventions & gotchas
- URL helpers: `resolve_url()`/`normalize_url()` (`net/mod.rs`); `normalize_nav_url()` (`navigator.rs`)
- Global statics: `OnceLock<Mutex/RwLock<…>>` for client, caches, cookies, storage — blocks multi-window; reuse where already established, but new global mutable state requires an ADR. Adding fields/routes to existing global statics counts as new global state. Don't spread the pattern
- Runtime data files (gitignored): `vayu_settings.json`, `vayu_tabs.json`, `vayu_local_storage.json`, `vayu_cookies.json`
- Linker is `rust-lld` (`.cargo/config.toml`); don't switch to lld-link unless installed
- Iced 0.13: `Task::perform(future, mapper)`; there is no `iced::Command`
- Extraction budgets (A1, done): safety-stop ceilings, not fidelity targets —
  100_000 elements (`MAX_ELEMENTS`), depth 200, 64k chars/text node
  (`src/engine/pipeline/extractor.rs`), 5MB HTML, 500KB/CSS source,
  8MB cumulative CSS (`src/engine/pipeline/fetcher.rs`)
- Blocking reqwest confined to `tokio::task::spawn_blocking` workers — never on the Iced UI thread; don't expand beyond existing network boundaries (async replacement: PLAN D)
- Browser submodules read `BrowserScreen` private fields directly (child-module access)
- `Tab::new(title, url, workspace_id)` — three args
- CSP: check `net::csp_blocks_scripts()` / `csp_blocks_styles()` before processing
- wgpu 0.19 future-incompat warnings are harmless until iced upgrades

## Testing
- Integration tests in `tests/`, unit tests inline under `#[cfg(test)]` (~444 total)
- Layout tests call `apply_taffy_layout(&mut elements, width, height)` directly — no network
- No mock network layer; `BrowserScreen::navigate()` does real HTTP
- aether-js (future): test262 is its conformance harness — see js-engine-plan.md §3

## Removed (do not reintroduce)
- `crates/aether-html` (→ html5ever), `crates/aether-caelum` (→ taffy)
- `build.rs` + `css-caelum-bridge.json` codegen bridge
- CSS input-length truncation (`MAX_INPUT_LENGTH`/`MAX_ITERATIONS`), `CHAR_W_SCALE` heuristic
