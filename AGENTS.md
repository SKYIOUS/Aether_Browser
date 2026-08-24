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
   `cargo clippy -- -D warnings` when touching code CI covers. Cite output.
   Never report success from intent. An unverified claim is a false claim.
2. **Scope discipline.** Touch only what the task names. No drive-by refactors,
   no comment churn, no reformatting of untouched lines.
3. **Root cause, not symptom.** Before editing a function, find every caller.
   Fix at the shared choke point once — not per-caller guards.
4. **Files stay under 1000 lines.** Growing one means extract a submodule
   instead (`browser/mod.rs` is the repeat offender).
5. **No new dependency** without a `future-crates.md` row + rationale. Leaf
   crates only; nothing platform-level inside `crates/aether-js`.
6. **Decisions get recorded.** Non-obvious choice → ADR. Naming/concepts →
   `CONTEXT.md`. No second vocabulary.
7. **Comments carry why, not what.** Deliberate ceilings get a
   `ponytail:` marker naming the ceiling and upgrade path.

## Operating protocol (how every task runs)
1. **Skills before action.** Invoke matching skills BEFORE responding,
   exploring, or asking clarifying questions. Process skills first
   (brainstorming / systematic-debugging / grill-\*), implementation skills
   second. If a skill proves wrong mid-task, say so and drop it explicitly.
2. **Codebase Memory MCP always.** The graph is the map of record:
   - Before editing any symbol → `search_graph` / `get_code_snippet` it.
   - Before changing a function → `trace_path` every caller.
   - Before optimizing anything → `query_graph` complexity/hot-path fields;
     measure, never guess.
   - After structural changes → re-index so the graph stays truthful.
3. **Exhaustive effort is the default, not a mode.** Every task ends with
   **prove → prune → re-check**:
   - **Prove:** evidence attached (command output, test names, file:line refs).
   - **Prune:** same-change deletion of everything the work made redundant —
     code, docs, deps. Leaving dead weight behind is an incomplete task.
   - **Re-check:** fresh read of the final diff; adversarial pass over risky
     logic; every touched doc re-checked against reality.
4. **Subagents over solo guessing.** Dispatch them instead of hand-waving:
   - Unfamiliar area → `explore` agent before planning anything.
   - Non-trivial diff → fresh-context reviewer (code-review / adversarial /
     test-gap finder) before claiming done; reviewers get no stake in the
     outcome, so their findings outrank self-assessment.
   - Independent subtasks run as parallel subagent dispatches.
5. **The loop never stops.** Every task runs PLAN → DO → CHECK in continuous
   cycles: plan the slice (todo list), implement it, run its tests, feed the
   results into the next slice's plan. Stopping between loops is how half-done
   work ships; a task is done only when a full loop closes with green
   evidence and prove/prune/re-check complete.

## Test-first mandate
1. **Failing test first** for any feature or bugfix: red → green → refactor.
   Implementation without a preceding failing test is rejected on review.
2. Bug fixes begin with a **reproduction test** that fails on old code and
   passes on new. "Fixed" without a repro is not fixed.
3. Trust boundaries (net input, CSP, cookies, JS↔DOM bridge, aether-js host
   layer) get **negative-path tests**, not just happy paths.
4. aether-js work uses **test262** as its harness; gates per js-engine-plan.md §3.
5. Trivial one-liners are exempt — YAGNI applies to tests too.

## Anti-slop policy (hard bans)
1. **No narration comments.** Comments exist only for non-obvious why.
   `// increment counter` and friends are deleted on sight.
2. **No speculative abstractions.** No single-implementation traits, factories
   for one product, config knobs for constants. Deletion beats addition.
3. **No placeholders.** No stubs "for later"; no TODO/FIXME without a matching
   PLAN.md or issues.md row naming who/when.
4. **Never invent APIs, flags, or dependencies.** Verify against real source
   (codebase memory MCP) or official docs (webfetch) before first use.
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
- Status (verified 2026-08-24): 444 tests green; CI gates fmt/clippy/test on
  Linux+Windows+macOS (`.github/workflows/ci.yml`)

## Architecture
Rust, edition 2021. Entry: `src/main.rs` → `src/lib.rs` → `src/ui/mod.rs`
(`VayuApp`). Workspace: `korlang`, `crates/aether-dom`, `crates/aether-css`,
plus planned `crates/aether-js` (ADR-0003). UI is Iced 0.13 (canvas/image/
tiny-skia/wgpu/tokio), theme forced Light.

Engine stack:
- HTML: html5ever 0.39 + custom TreeSink (`src/engine/parser.rs`) → `aether_dom::Node`
- CSS: cssparser 0.37 tokenizer in Stratus (`src/engine/stratus.rs`), values/cascade in `aether-css`; selectors crate matching in `src/engine/js/selector_engine.rs`
- Layout seam: `apply_taffy_layout()` (taffy 0.12) in `src/engine/pipeline/layout.rs`, incl. inline formatting context
- Text: cosmic-text measurement/shaping (`src/engine/text.rs`), LRU-cached
- JS: rquickjs 0.12 (`src/engine/js/`, JsBridge flat DOM behind `Arc<Mutex<>>`) — interim until aether-js swap gate
- Net: blocking reqwest, cookie jar w/ attributes, HTTP cache, CSP checks (`src/engine/net/mod.rs`)
- Korlang: embedded UI DSL VM driving sidebar/chrome; never page-reachable except through binding policy

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
- Global statics: `OnceLock<Mutex/RwLock<…>>` for client, caches, cookies, storage — blocks multi-window; scheduled debt, don't spread the pattern
- Runtime data files (gitignored): `vayu_settings.json`, `vayu_tabs.json`, `vayu_local_storage.json`, `vayu_cookies.json`
- Linker is `rust-lld` (`.cargo/config.toml`); don't switch to lld-link unless installed
- Iced 0.13: `Task::perform(future, mapper)`; there is no `iced::Command`
- Element caps live: 2000 elements, depth 50, 5000 chars/text node, 1MB HTML (removal planned, PLAN A1)
- Blocking reqwest everywhere via `run_blocking()`
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
