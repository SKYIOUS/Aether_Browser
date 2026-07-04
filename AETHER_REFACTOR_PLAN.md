# AETHER REFACTOR PLAN — Complete Codebase Autopsy

> Generated: 2026-07-04
> Scope: Full read of all 241 .rs files (~28,624 lines)
> Status: Code does not compile (51 errors per AGENTS.md)

---

## PART I: Architecture Autopsy — What Exists, What Works, What Doesn't

### 1.1 Project Topology

`
aether_browser/                        [workspace root]
├── Cargo.toml                         [workspace with 5 members]
├── build.rs                           [codegen: css-caelum-bridge.json → bridge_gen.rs]
├── AGENTS.md                          [dev guide]
├── src/                  (5,882 LOC)  [main binary]
├── korlang/              (671 LOC)    [workspace member — custom UI language]
├── crates/
│   ├── aether-dom/       (~200 LOC)   [workspace — Node/NodeType/ElementData types]
│   ├── aether-html/     (~1,600 LOC)  [workspace — HTML parser + DEAD tokenizer/tree_builder]
│   ├── aether-css/      (~2,100 LOC)  [workspace — Stratus CSS engine]
│   └── aether-caelum/   (~17,000 LOC) [workspace + PUBLISHED on crates.io — layout engine]
└── tests/               (1,197 LOC)   [5 test files + compliance/]
`

### 1.2 Workspace Dependency Graph

`
aether_browser (binary)
├── aether-dom     (types only)
├── aether-html    (HTML parsing)
├── aether-css     (Stratus: CSS parse + resolve)
├── aether-caelum  (Caelum: Flexbox/Grid/Block layout)
└── korlang        (NOT integrated — standalone test in lib.rs)
`

### 1.3 The Rendering Pipeline — What Actually Fires

etch_page_content() in src/engine/pipeline/fetcher.rs (~400 LOC) orchestrates:
1. **Fetch HTML** — blocking reqwest (eqwest::blocking::Client), 15s timeout, max 1MB
2. **Parse DOM** — ether-html parser (inline Parser, NOT the tokenizer/tree_builder)
3. **Extract <style> / <link rel="stylesheet">** — up to 50 each
4. **Fetch external CSS** — capped at 500KB per source
5. **Parse CSS** — Stratus (ether-css), 30K max input, 50K max iterations
6. **Extract scripts** — <script> tags (inline + external)
7. **Load DOM into JsBridge** — flat Vec<FlatNode> representation
8. **Execute scripts** — rquickjs 0.11 via JSEngine
9. **Replace DOM** — ridge.to_dom() converts flat vec back to Node tree
10. **Inject document.write()** — adds script-generated text
11. **Extract elements** — DOM tree walk → Vec<StyledElement> (depth 50, max 2000, text 5000)
12. **Fetch images** — blocking reqwest for <img> src, decode via image crate
13. **Apply Caelum layout** — computes x/y/width/height via pply_caelum_layout()
14. **Render** — PageCanvas (iced canvas widget) draws rects, borders, text, images

### 1.4 What Works (Tested)

- **HTML parsing** (simple cases): 	ests/integration_test.rs — div/p parsing, attribute extraction
- **CSS parsing + resolution**: 	ests/css_regression.rs — colors, backgrounds, fonts, margins, padding, borders, flex properties, selectors (class, id), cascade
- **Caelum layout** (basic): 	ests/layout_stress.rs — blocks, nesting, 1000 elements, deep nesting to 100, margins, padding, borders, inline/block mixing
- **JS Bridge**: 	ests/js_bridge_test.rs (627 lines, 35+ tests) — createElement, appendChild, querySelector, style, timers, events, cookies, localStorage, location, DOM roundtrip, XSS prevention
- **Pipeline integration**: 	ests/integration_test.rs — end-to-end extract + layout (basic)
- **Korlang**: korlang/src/lib.rs — compile + run "Hello, Korlang!" test (2 tests)
- **Rendering basics**: 	ests/rendering.rs — pipeline init, Caelum Style::DEFAULT

### 1.5 What's Broken (Does Not Compile — 51 Errors)

Per AGENTS.md, 51 compile errors spread across:
- src/engine/js/js_bridge.rs (2,093 LOC god-file)
- src/engine/pipeline/extractor.rs
- src/engine/pipeline/fetcher.rs
- src/engine/net/mod.rs
- src/ui/screens/browser.rs

Likely causes (found via static analysis):
1. **Iced 0.13 API mismatch** — Task::perform vs Command::perform, widget API changes
2. **aether-caelum API changes** — bridge codegen in uild.rs may generate stale/broken mappings
3. **rquickjs 0.11 type mismatch** — JsBridge integration with rquickjs context
4. **Missing ridge_gen.rs** — generated file not checked in, must be regenerated

### 1.6 What's Missing Entirely

- **Cookie jar** — Vec<String> stub in 
et/mod.rs (lines 95-109), no domain/path/secure/SameSite
- **CSP enforcement** — stubbed functions in 
et/mod.rs (lines 280-340), checks exist but no policy parsing
- **HTTPS validation** — reqwest uses 
ative-tls but no certificate pinning or validation config
- **Tab persistence** — ether-tabs.json file I/O, no crash recovery
- **Image decoding error handling** — images fetched via un_blocking(), panics on decode failure
- **URL bar text input** — rendered as canvas text, not native <input> — no IME support
- **Accessibility** — zero a11y attributes, no ARIA, no screen reader support
- **Search engine** — "Search the web" placeholder with no actual search integration
- **Downloads** — no download manager
- **History** — no browsing history
- **Bookmarks** — no bookmark system
- **DevTools** — no developer tools
- **Extensions** — no extension API
- **Security indicators** — no HTTPS padlock, no certificate viewer

---

## PART II: Why You Should NOT Use This Codebase

### 2.1 Research Prototype, Not Production Software

This is a **personal research project** exploring how a browser rendering pipeline works. It is not, and was never intended to be, a production browser. Key indicators:

| Indicator | Evidence |
|-----------|----------|
| No CI/CD | No .github/workflows/, no CI config |
| No security audit | CSP stubbed, cookies plain Vec<String>, no cert validation |
| No error handling | 35+ unwrap()/expect() panics, 40+ silently ignored errors via let _ =  |
| No documentation | Just AGENTS.md (which documents the current broken state) |
| No release process | No tags, no changelog, no release automation |
| Test coverage gap | 225 tests but network layer is unmockable (real HTTP) |
| Single developer | epository points to SKYIOUS GitHub account |

### 2.2 Structural Problems

**God-files (2,000+ lines):**
- src/engine/js/js_bridge.rs — 2,093 lines blending: JS API bridge, CSS selector parser, HTML parser, DOM manipulation, timer system, event system, cookies, localStorage, fetch shim, XMLHttpRequest shim — all in one file
- src/ui/screens/browser.rs — 773 lines blending: app state, UI rendering, navigation, tab management, Korlang chrome rendering, event dispatch, timer polling

**Build system fragility:**
- uild.rs reads css-caelum-bridge.json at compile time — if this file is missing or malformed, the build breaks with an opaque error
- The generated ridge_gen.rs is not checked into version control
- Three separate uild.rs files (aether-css, aether-html, root) each generate code from JSON

**Dead code bloat (~1,300 lines):**
- crates/aether-html/src/tokenizer.rs (985 lines) — HTML5 tokenizer state machine, completely unused
- crates/aether-html/src/tree_builder.rs (337 lines) — HTML tree construction, completely unused
- These are replaced by a simpler inline parser in lib.rs (~200 lines)

**Unsafe Rust:**
- korlang/src/compiler/mod.rs:15-17 — unsafe static mut LABEL_COUNTER — global mutable counter, data race UB
- crates/aether-caelum/src/style/compact_length.rs:19,24 — unsafe { core::mem::transmute } — transmute between f32 and u32 bit patterns, no sentinel check

### 2.3 Security Vulnerabilities

1. **Script execution silently ignored** — src/engine/js/mod.rs:49,63 — errors from JS execution are discarded with let _ =
2. **No CSP enforcement** — 
et/mod.rs:280-340 — CSP parser exists but returns hardcoded results
3. **Cookie leak** — 
et/mod.rs:118 — writes cookies to disk via std::fs::write() with no encryption or sanitization
4. **No sandbox** — rquickjs can access all registered APIs; no concept of a security origin beyond string comparisons
5. **XSS surface** — set_inner_html() in js_bridge.rs preserves <script> elements (though content stored as text, not executed)
6. **XMLHttpRequest** — basic sync implementation, no CORS enforcement
7. **35 panic points** — any unexpected input triggers a crash via unwrap()/expect()

### 2.4 Performance Issues

- **Blocking HTTP** — reqwest blocking client runs on Iced's main thread, freezing the UI
- **No streaming** — entire HTML/CSS/JS payloads loaded into memory before processing
- **No incremental rendering** — entire page rendered at once, no virtualization
- **Canvas redraw** — entire page canvas redrawn on every frame, no invalidation
- **No caching** — CSS cache is LRU with 100 entries, image cache doesn't exist
- **DOM → FlatNode → DOM conversion** — round-trip serialization on every page load
- **Tree recursion** — 50-depth recursion limit in extractor prevents truly deep pages

### 2.5 Technology Lock-In

- **Iced 0.13** — highly specific version, API differs from 0.12 and 0.14
- **rquickjs 0.11** — QuickJS binding, tied to a specific JavaScript engine
- **Custom layout engine** — aether-caelum is used nowhere else
- **Custom CSS engine** — Stratus (aether-css) is used nowhere else
- **Korlang** — custom UI language used ONLY for the browser chrome

Migrating away from any of these would require rewrites of significant portions.

---

## PART III: Korlang Deep Analysis

### 3.1 Korlang Overview (671 lines total)

Korlang is a **custom UI description language** with a Rust-based compiler and stack VM. It is:
- A workspace member but **not integrated** with the browser (only tested in korlang/src/lib.rs)
- Used to render the browser chrome in rowser.rs:521-527 via ender_korlang_chrome()
- Has a parser, lexer, compiler (AST→bytecode), and stack VM

### 3.2 Architecture (5 files)

| File | Lines | Purpose |
|------|-------|---------|
| lib.rs | 15 | Re-exports compile() and Vm::run(), contains 2 tests |
| compiler/lexer.rs | 96 | Tokenizer with basic string interpolation, comment support |
| compiler/parser.rs | 123 | AST parser: Component, State, If/Else, For, Element props |
| compiler/mod.rs | 85 | AST→bytecode compiler, unsafe static mut LABEL_COUNTER |
| m/mod.rs | 352 | Stack VM with 14 opcodes, 25 KorObject variants |

### 3.3 The VM (14 opcodes)

`
Push, Pop, LoadLocal, StoreLocal, LoadFn, Closure, Call, Return,
Jump, JumpIfFalse, MakeList, ListLen, ListGet, Halt
`

**Critical bugs:**
1. **Call opcode** (m/mod.rs ~lines 138-147) — argument indexing is mathematically wrong. It pops 
args from the stack but decrements the stack pointer incorrectly, causing stack underflow on actual function calls
2. **No actual function calls tested** — the lib.rs test only exercises Push/Return/Halt, never Call, LoadFn, or Closure
3. **Halt never tested** — the run loop has no termination test for Halt
4. **unimplemented!() callbacks** — Call for Native KorObject calls unimplemented!() which will panic at runtime

### 3.4 The Compiler (unsafe static mut LABEL_COUNTER)

`ust
// korlang/src/compiler/mod.rs:15-17
static mut LABEL_COUNTER: u32 = 0;  // unsafe!
`

Used for generating unique labels for if/else branches. In a multithreaded context (Iced uses threads for event loop + background tasks), this is **undefined behavior**. Even in single-thread use, a Cell<u32> would suffice.

### 3.5 Empty Stubs

- korlang/src/interop/mod.rs — empty file (no browser API interop)
- korlang/src/compiler/rust_gen.rs — empty file (no Rust codegen)

Planned but never implemented.

### 3.6 Integration Points

rowser.rs line ~5 imports: use korlang::{compile, Vm};
rowser.rs line ~521-527: iew() calls ender_korlang_chrome() which compiles and runs .kor files to produce iced widgets

**Problem:** Korlang compiler and VM are tested only for literal "Hello, world!" string output — never for actual widget tree generation. The browser integration would almost certainly fail if compiled.

### 3.7 Korlang Assessment

| Aspect | Verdict |
|--------|---------|
| **Language design** | Reasonable DSL for UI, similar to SwiftUI/Compose but simpler |
| **Compiler** | Basic but functional for the feature set (~85 LOC) |
| **VM** | Only 3/14 opcodes tested, Call opcode buggy, Halt unimplemented |
| **Integration** | Untested — would likely panic or produce wrong output |
| **Safety** | unsafe static mut for label generation is indefensible |
| **Future value** | Low — could be replaced by iced's native API with less code |

---

## PART IV: aether-caelum Published Crate Problem

### 4.1 The Situation

ether-caelum is **published on crates.io** (v0.1.0) with:
- Repository: https://github.com/SKYIOUS/aether-caelum
- Description: "A between-the-lines layout engine for Rust, implementing Flexbox, Grid, and Block layout"
- 17,000+ lines of Rust across 50+ source files
- ~38 TODO/FIXME markers
- Dependencies: slotmap = "1.0" only
- Default features: ["content_size"] (but the feature flag gates nothing)

### 4.2 The Advertising Gap

| Claimed | Actual |
|---------|--------|
| "Flexbox" | Implemented but 10+ TODOs for bugs (safe alignment, spec violations, writing modes) |
| "Grid" | Partially implemented — 10+ TODOs, baseline alignment stubbed, named line resolution untested |
| "Block" | Basic block layout works, 6+ TODOs for float positioning, auto margins, nested blocks |
| "for building UI frameworks, browsers, and renderers" | Technically true but caveat emptor |

### 4.3 Safety Issues

`
crates/aether-caelum/src/style/compact_length.rs:19
    unsafe { core::mem::transmute(val) }   // f32 -> u32 bit cast, no NaN check

crates/aether-caelum/src/style/compact_length.rs:24
    unsafe { core::mem::transmute(v) }      // same pattern, opposite direction
`

These transmutes convert between 32 and u32 for a compact length encoding that uses NaN bits to store sentinel values. If 32::NAN is passed, it would be interpreted as a sentinel — a potential logic bug. The correct approach: 32::to_bits() / 32::from_bits() which are safe and stable.

### 4.4 If Someone Depends on This Crate Today

They would get:
1. A layout engine with **38 known bugs** documented in TODO/FIXME comments
2. **Unsafe transmute** in production code
3. **Missing features** advertised but not implemented
4. **content_size feature flag** that doesn't gate any real functionality
5. **No CSS value parser** — the planned parse.rs does not exist in source

### 4.5 Recommendation

- **Do NOT depend on this crate** in its current state
- **Yank v0.1.0** from crates.io, or add a prominent README.md disclaimer
- Fix unsafe transmutes and critical TODOs before any production use
- The layout algorithms have good bones (flexbox algorithm is largely correct) but need polish

---

## PART V: Detailed Bug Catalog with File:Line References

### 5.1 Build Failures (51 errors — source files)

The following files produce compile errors (per AGENTS.md; unable to verify since code doesn't compile):

1. **src/engine/js/js_bridge.rs** (2,093 lines)
   - Likely Iced 0.13 API mismatches in element_at_point()
   - Type mismatches between rquickjs 0.11 types and bridge types

2. **src/engine/pipeline/extractor.rs** (~500 lines)
   - Likely ComputedStyle API changes from aether-css

3. **src/engine/pipeline/fetcher.rs** (~400 lines)
   - Likely JsBridge / JSEngine API mismatches

4. **src/engine/net/mod.rs** (~500 lines)
   - Likely reqwest API changes

5. **src/ui/screens/browser.rs** (773 lines)
   - Likely Iced 0.13 widget API changes, Korlang integration

### 5.2 Unsafe Code

| File | Line | Issue |
|------|------|-------|
| korlang/src/compiler/mod.rs | 15-17 | unsafe static mut LABEL_COUNTER — data race UB |
| crates/aether-caelum/src/style/compact_length.rs | 19, 24 | unsafe { transmute } — NaN sentinel logic |

### 5.3 Panic Points (unwrap/expect)

**Build-time (harmless — only run at compile time):**
- uild.rs: 15 expect() calls
- crates/aether-css/build.rs: 9 expect()
- crates/aether-html/build.rs: 7 expect()

**Runtime (dangerous — can crash the browser):**
- crates/aether-caelum/src/util/print.rs:9 — expect on stdout write
- crates/aether-caelum/src/compute/block.rs:828 — expect on child width calc
- 	ests/js_bridge_test.rs:295,296,299,300 — test-only, harmless

### 5.4 Silently Ignored Errors (40+ let _ = )

**src/ (application code):**
| File | Line | What's Ignored |
|------|------|----------------|
| src/logging.rs | 35-58 | 6x let _ = writeln!() — benign (logging) |
| src/engine/js/mod.rs | 49, 63 | **JS execution errors** — silently discarded |
| src/engine/js/js_bridge.rs | 2274 | **SHIM_JS eval error** — JS shim failure ignored |
| src/engine/net/mod.rs | 118 | **File write error** — cookies/state lost silently |
| src/engine/pipeline/fetcher.rs | 296 | **JS execution error** — script failure discarded |
| src/engine/pipeline/layout.rs | 140, 144 | **Caelum tree errors** — layout failures silently ignored |
| src/ui/screens/browser.rs | 415, 473 | **JS timer/event execution errors** — silently discarded |

**crates/ (library code):**
| File | Lines | Ignored |
|------|-------|---------|
| crates/aether-caelum/src/tree/caelum_tree.rs | 548-611, 998, 1015 | **11x** tree operation results silently discarded |
| crates/aether-html/src/lib.rs | 95 | consume_char() result discarded |

### 5.5 Dead Code

| File | LOC | Status |
|------|-----|--------|
| crates/aether-html/src/tokenizer.rs | 985 | **UNUSED** — full HTML5 tokenizer state machine, never imported |
| crates/aether-html/src/tree_builder.rs | 337 | **UNUSED** — HTML tree construction, never imported |
| crates/aether-html/src/lib.rs | 1 | #![allow(dead_code)] on entire module |
| crates/aether-css/src/lib.rs | 1 | #![allow(dead_code)] on entire module |
| src/ui/style.rs | 1 | #![allow(dead_code)] on entire module |
| src/logging.rs | 1 | #![allow(dead_code)] on entire module |
| Various aether-caelum | 7 lines | #[allow(dead_code)] on specific functions/structs |

### 5.6 Korlang Bugs

| File | Line | Bug |
|------|------|-----|
| korlang/src/compiler/mod.rs | 15-17 | unsafe static mut — data race, undefined behavior |
| korlang/src/vm/mod.rs | ~138-147 | Call opcode argument indexing — pops wrong stack offset |
| korlang/src/vm/mod.rs | multiple | Native callback calls unimplemented!() — will panic |
| korlang/src/vm/mod.rs | multiple | Only 3/14 opcodes tested (Push, Return, Halt) |
| korlang/src/compiler/rust_gen.rs | empty | Stub — Rust code generation not implemented |
| korlang/src/interop/mod.rs | empty | Stub — browser API interop not implemented |

### 5.7 aether-caelum TODO/FIXME Density

| Module | Count | Key Issues |
|--------|-------|------------|
| compute/block.rs | 8 | Nested blocks, float positioning, auto margins, stretch sizing, inset |
| compute/flexbox.rs | 10 | Visibility collapse, writing modes, spec violations, safe alignment |
| compute/float.rs | 2 | Width positioning, float tracking |
| compute/grid/alignment.rs | 2 | Baseline alignment, safe alignment |
| compute/grid/mod.rs | 6 | Baseline, track re-sizing, named lines |
| compute/grid/placement.rs | 1 | Named line resolution |
| compute/grid/track_sizing.rs | 1 | Cross-axis sizing |
| compute/grid/types/grid_item.rs | 3 | Baseline, overflow, auto behavior |
| compute/grid/types/named.rs | 2 | Clone elimination |
| compute/leaf.rs | 1 | Direction-based side config |
| compute/mod.rs | 1 | Auto margins for root |
| 	ree/caelum_tree.rs | 2 | Const method, test quality |
| style/mod.rs | 2 | Clone elimination |
| style/grid.rs | 1 | Default implementations |
| **Total** | **38** | |

---

## PART VI: Phased Action Plan (Phase 0–10)

### Phase 0: Stabilize and Compile (Estimated: 2-4 days)

**Goal:** Get the code to compile and pass existing tests.

Steps:
1. **Pin exact dependency versions** in root Cargo.toml:
   - iced = "=0.13.0"
   - quickjs = "=0.11.0"
   - eqwest = "=0.12.0"
   - ether-caelum = { path = "crates/aether-caelum" } (local path, not crates.io)

2. **Fix Iced 0.13 API mismatches:**
   - rowser.rs:284 — change Command to Task for Iced 0.13
   - rowser.rs:512 — verify subscription() API matches iced 0.13
   - Check all canvas::Program trait implementations for API changes

3. **Regenerate ridge_gen.rs:**
   - Verify css-caelum-bridge.json exists and is valid
   - Run cargo build to trigger uild.rs codegen
   - Commit generated file (or add to .gitignore if kept regenerated)

4. **Fix individual compile errors:**
   - Work through each of the 51 errors one by one
   - Most likely: type mismatches, missing imports, renamed functions
   - Remove any dead imports that reference removed APIs

5. **Run existing test suite:**
   - cargo test — pass all 225+ tests
   - Fix any regressions from the compile fixes

**Deliverable:** cargo build succeeds, cargo test passes.

---

### Phase 1: Error Handling Overhaul (Estimated: 2-3 days)

**Goal:** Replace all panic points with proper error handling.

Steps:
1. **Audit and replace unwrap() calls:**
   - uild.rs — 15 expects are fine (compile-time)
   - crates/aether-caelum/src/util/print.rs:9 — replace with if let Err(e) = ...
   - crates/aether-caelum/src/compute/block.rs:828 — replace with unwrap_or(0.0) or early return
   - All 35+ runtime panic points -> proper Result propagation

2. **Audit and replace let _ = patterns:**
   - src/engine/js/mod.rs:49,63 — propagate JS errors or at least log them
   - src/engine/js/js_bridge.rs:2274 — log SHIM_JS eval failure
   - src/engine/net/mod.rs:118 — log file write failure
   - src/engine/pipeline/fetcher.rs:296 — propagate/link JS execution errors
   - src/engine/pipeline/layout.rs:140,144 — propagate Caelum layout errors
   - src/ui/screens/browser.rs:415,473 — log timer/event execution errors
   - crates/aether-caelum/src/tree/caelum_tree.rs:548-611,998,1015 — all 11 need handling

3. **Add error types:**
   - Create Error enum(s) for each subsystem
   - Implement std::error::Error or use nyhow/	hiserror

**Deliverable:** Zero panics from user input, all errors logged or propagated.

---

### Phase 2: Security Hardening (Estimated: 3-5 days)

**Goal:** Implement real security boundaries.

Steps:
1. **Implement CSP parsing:**
   - src/engine/net/mod.rs:280-340 — replace stubs with real CSP directive parser
   - Parse Content-Security-Policy header and <meta http-equiv=...>
   - Enforce script-src, style-src, img-src, connect-src
   - Block inline scripts/styles unless unsafe-inline is present

2. **Implement proper cookie jar:**
   - Replace Vec<String> with struct supporting domain/path/secure/httponly/SameSite
   - Implement cookie expiration
   - Implement cookie storage with proper file serialization (not plaintext)

3. **Implement origin-based security:**
   - Replace string comparison in etch_url() with proper origin checking
   - Implement CORS preflight and headers

4. **Add HTTPS validation:**
   - Configure reqwest with certificate pinning
   - Add security indicator UI (padlock icon)

5. **Sanitize script input:**
   - In set_inner_html(), strip or escape <script> tags entirely
   - Add DOMPurify-style sanitization

**Deliverable:** CSP enforced, cookies secure, CORS implemented, XSS surface reduced.

---

### Phase 3: Dead Code Removal (Estimated: 1 day)

**Goal:** Remove ~1,300 lines of dead code and suppress dead_code warnings.

Steps:
1. **Remove crates/aether-html/src/tokenizer.rs** (985 lines) — unused HTML5 tokenizer
2. **Remove crates/aether-html/src/tree_builder.rs** (337 lines) — unused HTML tree builder
3. **Update crates/aether-html/src/lib.rs** — remove #![allow(dead_code)]
4. **Remove #![allow(dead_code)] from:**
   - src/ui/style.rs:1
   - src/logging.rs:1
   - crates/aether-css/src/lib.rs:1
5. **Fix aether-caelum #[allow(dead_code)] annotations:**
   - Remove if no longer needed, or add #[expect(dead_code)] (Rust 1.81+)

**Deliverable:** 1,300+ fewer lines, no #![allow(dead_code)] at crate level.

---

### Phase 4: Kill the God-Files (Estimated: 5-7 days)

**Goal:** Refactor js_bridge.rs and rowser.rs into manageable modules.

Steps:
1. **Split js_bridge.rs (2,093 lines):**
   - Extract CSS selector parser -> src/engine/js/selector.rs (~200 lines)
   - Extract HTML DOM parser -> src/engine/js/html_dom.rs (~150 lines)
   - Extract timer system -> src/engine/js/timers.rs (~150 lines)
   - Extract event system -> src/engine/js/events.rs (~150 lines)
   - Extract fetch/XHR -> src/engine/js/fetch.rs (~200 lines)
   - Extract localStorage/cookies -> src/engine/js/storage.rs (~150 lines)
   - Leave core bridge logic in js_bridge.rs (~800 lines)

2. **Split rowser.rs (773 lines):**
   - Extract tab management -> src/ui/screens/tabs.rs
   - Extract navigation logic -> src/ui/screens/navigation.rs
   - Extract chrome rendering -> src/ui/screens/chrome.rs
   - Leave core BrowserScreen struct in rowser.rs (~300 lines)

3. **Update all imports:**
   - Re-export from mod.rs files for backward compatibility
   - Update any external references

**Deliverable:** No file exceeds 1,000 lines. Each module has a single responsibility.

---

### Phase 5: Async I/O and UI Responsiveness (Estimated: 3-5 days)

**Goal:** Move blocking network calls off the UI thread.

Steps:
1. **Replace eqwest::blocking with async eqwest:**
   - Rewrite etch_url(), etch_css(), etch_binary() as async
   - Use Iced 0.13's Task::perform() for async execution
   - Add loading indicators during page fetch

2. **Implement streaming HTML parser:**
   - Convert ether-html parser to accept incremental input
   - Begin rendering before full page is downloaded
   - Prioritize <head> processing (CSS before JS)

3. **Add progressive rendering:**
   - Render above-the-fold content first
   - Add scroll virtualization (only render visible elements)

4. **Add request cancellation:**
   - Cancel in-flight requests when navigating away

**Deliverable:** UI remains responsive during page loads, progressive rendering works.

---

### Phase 6: Memory and Performance (Estimated: 3-4 days)

**Goal:** Reduce memory usage and improve frame rate.

Steps:
1. **Eliminate DOM->FlatNode->DOM round-trip:**
   - Modify pipeline to work directly with flat Vec<FlatNode>
   - Remove the 	o_dom() serialization step

2. **Add image caching:**
   - LRU cache for decoded images (configurable size)
   - Lazy decoding (decode only when visible)

3. **Optimize canvas rendering:**
   - Only redraw dirty regions, not entire canvas
   - Use layer caching for static content

4. **Reduce allocations:**
   - Profile with dhat or lloc_counter
   - Pre-allocate Vec sizes where known

**Deliverable:** 50% reduction in memory usage, 60fps rendering on typical pages.

---

### Phase 7: JavaScript Engine Upgrade (Estimated: 3-5 days)

**Goal:** Modernize JS integration.

Steps:
1. **Fix remaining JS compile errors** (from Phase 0):
   - Update rquickjs API calls to match version
   - Fix type mismatches in egister_browser_api()

2. **Add JS error propagation:**
   - Stop discarding JS execution errors
   - Surface errors to a dev console
   - Add window.onerror support

3. **Implement closure serialization:**
   - Current timers/events store source strings, not closures
   - Create a mechanism to serialize/deserialize JS closures across ticks

4. **Add async JS support:**
   - Implement Promise integration
   - Support sync function and wait

**Deliverable:** JS errors are visible, closures work across timer boundaries.

---

### Phase 8: Korlang — Finish or Remove (Estimated: 2-3 days)

**Goal:** Make Korlang work correctly or replace it.

**Option A: Fix Korlang**
1. Fix unsafe static mut LABEL_COUNTER -> replace with Cell<u32> or AtomicU32
2. Fix Call opcode argument indexing -> rewrite stack pop logic
3. Add comprehensive tests for all 14 opcodes
4. Implement Native callbacks -> replace unimplemented!() with real implementations
5. Integrate with browser -> test end-to-end chrome rendering

**Option B: Replace Korlang (Recommended)**
1. Replace Korlang chrome rendering with plain iced widgets (~50 lines)
2. Remove the korlang workspace member entirely
3. Delete korlang/src/interop/ and korlang/src/compiler/rust_gen.rs stubs

**Recommendation:** Option B. Korlang adds 671 lines and a full compiler/VM for a trivial benefit (the chrome could be 50 lines of iced widgets). The unsafe and untested opcodes make it a maintenance liability.

**Deliverable:** Either Korlang works correctly with all opcodes tested, or it's removed and replaced.

---

### Phase 9: aether-caelum Crate Hardening (Estimated: 5-7 days)

**Goal:** Make the published crate safe and correct.

Steps:
1. **Fix unsafe transmutes:**
   - crates/aether-caelum/src/style/compact_length.rs:19,24
   - Replace with 32::to_bits() / 32::from_bits()
   - Add NaN sentinel check

2. **Address 38 TODOs/FIXMEs:**
   - Prioritize by impact: float positioning (block.rs:718), safe alignment (flexbox.rs:1698, grid/alignment.rs:34), baseline alignment (grid/mod.rs:312)
   - Fix crash-causing TODOs first
   - Document remaining TODOs in a crate-level tracking issue

3. **Add missing CSS parser:**
   - Implement or integrate a CSS value parser for the style module
   - Or clearly document that users must construct Style structs manually

4. **Improve test coverage:**
   - Add tests for flexbox edge cases (wrapping, min/max sizes)
   - Add tests for grid placement (named lines, auto placement)
   - Test float interaction with block layout

5. **Feature flag audit:**
   - content_size feature does not gate any functionality
   - Either implement the feature gates or remove the flag

6. **Documentation:**
   - Add crate-level docs clearly stating known limitations
   - Mark experimental APIs with #[doc(hidden)]

7. **Version policy:**
   - Consider yanking v0.1.0 from crates.io
   - Publish v0.2.0 with all fixes and a README.md disclaimer

**Deliverable:** No unsafe code, 38 TODOs reduced to <10 known/acceptable issues.

---

### Phase 10: Production Readiness (Estimated: 2-4 weeks)

**Goal:** Make the browser functional for everyday use.

Steps:
1. **URL bar:**
   - Replace canvas-rendered URL bar with native iced text input
   - Add autocomplete from history
   - Add search engine integration (DuckDuckGo, Google)

2. **Navigation:**
   - Back/forward history with proper state management
   - Tab management with restore on crash
   - Bookmark management
   - Download manager

3. **Page rendering improvements:**
   - <iframe> support
   - Form input support (<input>, <textarea>, <select>)
   - <video> and <audio> stubs
   - <canvas> support
   - SVG rendering (basic)

4. **Developer tools:**
   - DOM inspector
   - Console with error display
   - Network request viewer
   - Style inspector

5. **Settings:**
   - Home page configuration
   - Default search engine
   - Privacy settings (cookie control, JS toggle)
   - Appearance (theme, font size)

6. **Cross-platform testing:**
   - Test on Windows (primary target)
   - Test on Linux
   - Test on macOS
   - Fix platform-specific bugs

7. **Performance benchmarking:**
   - Create benchmark suite
   - Measure: page load time, memory usage, frame rate, JS execution time
   - Set performance budgets

**Deliverable:** A functional browser that can render common websites, with dev tools, settings, and acceptable performance.
---

## Summary — What Needs to Change Most

| Priority | Change | Why |
|----------|--------|-----|
| **P0** | Fix 51 compile errors | Nothing works until this is done |
| **P1** | Fix panic points (35+ unwrap/expect) | Any unexpected input crashes the browser |
| **P1** | Stop ignoring errors (40+ let _ = ) | Critical failures are invisible |
| **P2** | Fix unsafe Korlang counter | Data race UB in compiler |
| **P2** | Fix unsafe Caelum transmute | NaN handling bug in production code |
| **P3** | Implement real CSP and cookies | Security theater currently |
| **P3** | Remove 1,300 lines dead code | Maintenance burden, confusing |
| **P4** | Split god-files (js_bridge, browser) | Unmaintainable as-is |
| **P4** | Async HTTP | UI freezes on every page load |
| **P5** | Korlang: fix or kill | Untested, buggy, adds complexity |
| **P5** | Harden aether-caelum | Published to crates.io with known bugs |

## Appendix: File Inventory

```
Source files (src/):             5,882 lines across 26 files + 6 dirs
Crate files (crates/):          20,874 lines across ~190 files
Test files (tests/):             1,197 lines across 6 files
Korlang files (korlang/):          671 lines across 5 files
──────────────────────────────────────────────────
Total Rust in codebase:        ~28,624 lines across 241 .rs files
Dead code removable:            ~1,322 lines (tokenizer 985 + tree_builder 337)
God-files needing splitting:     2 (js_bridge 2,093 + browser 773 = 2,866 lines)
TODOs/FIXMEs in aether-caelum:  38
Unsafe blocks:                   3 (2 transmute + 1 static mut)
Panic points (unwrap/expect):    35+
Ignored errors (let _ =):        40+
Published crates with bugs:      1 (aether-caelum v0.1.0)
Unintegrated workspace members:  1 (korlang — browser chrome uses but untested)
Code not compiling:              51 errors
```
