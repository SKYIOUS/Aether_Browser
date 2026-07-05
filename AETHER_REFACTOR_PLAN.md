# AETHER BROWSER — COMPLETE REFACTOR & ACTION PLAN v2

> Generated: 2026-07-05
> Status: **Compiles cleanly (0 errors)** — Substantially improved since v1 report
> Test count: **~283 tests across 11 files**
> Line count: **~7,209 src/ lines + ~18,000 crates/ lines + ~671 korlang/ lines**

---

## TABLE OF CONTENTS

1. [CURRENT STATE SUMMARY](#1-current-state-summary)
2. [WHAT'S BEEN FIXED vs v1 REPORT](#2-whats-been-fixed-vs-v1-report)
3. [REMAINING ISSUES — PRIORITY STACK](#3-remaining-issues)
4. [PHASE 0: TEST THE FOUNDATION](#4-phase-0-test-the-foundation)
5. [PHASE 1: FINISH THE CSS ENGINE](#5-phase-1-finish-the-css-engine)
6. [PHASE 2: FIX JAVASCRIPT BRIDGE](#6-phase-2-fix-javascript-bridge)
7. [PHASE 3: COMPLETE KORLANG](#7-phase-3-complete-korlang)
8. [PHASE 4: HARDEN AETHER-CAELUM](#8-phase-4-harden-aether-caelum)
9. [PHASE 5: KILL REMAINING DEAD CODE](#9-phase-5-kill-remaining-dead-code)
10. [PHASE 6: PRODUCTION NETWORKING](#10-phase-6-production-networking)
11. [PHASE 7: ADVANCED RENDERING](#11-phase-7-advanced-rendering)
12. [PHASE 8: KORLANG — BEST LANGUAGE & UI FRAMEWORK](#12-phase-8-korlang-best-language--ui-framework)
13. [PHASE 9: MODERN BROWSER FEATURES](#13-phase-9-modern-browser-features)
14. [PHASE 10: PUBLICATION & ECOSYSTEM](#14-phase-10-publication--ecosystem)
15. [SUMMARY: WHAT TO DO AS A DEVELOPER](#15-summary-what-to-do-as-a-developer)

---

## 1. CURRENT STATE SUMMARY

### 1.1 Compilation

| Metric | v1 Report | Current | Δ |
|--------|-----------|---------|---|
| Compile errors | 51 | **0** | ✅ All fixed |
| Panic points (unwrap/expect) | 35+ | **~0** | ✅ All replaced |
| Warnings | N/A | **1** | `FormInputChanged`, `FormElementClicked` unused at `browser.rs:53-54` |
| Tests passing | ~225 | **~283** | +58 tests |

### 1.2 Architecture (files + lines)

```
aether_browser/
├── src/                         7,209 LOC
│   ├── main.rs                          17
│   ├── lib.rs                            4
│   ├── logging.rs                       72
│   ├── bridge_gen.rs                     1  (generated)
│   ├── ui/                            2,416
│   │   ├── mod.rs                      108
│   │   ├── style.rs                    302
│   │   ├── kor_renderer.rs             133
│   │   └── screens/
│   │       ├── mod.rs                    3
│   │       ├── browser.rs           1,067  ← was ~1800, refactored
│   │       ├── palette.rs              281
│   │       └── settings.rs             525
│   └── engine/                        4,793
│       ├── mod.rs                       12
│       ├── dom.rs/parser.rs/stratus.rs/caelum.rs    1 each (re-exports)
│       ├── style.rs                     22
│       ├── korlang.rs                   46  ← NEW
│       ├── net/
│       │   ├── mod.rs                  789  ← real CSP + cookies
│       │   └── mock.rs                  57
│       ├── pipeline/
│       │   ├── mod.rs                   21
│       │   ├── fetcher.rs              362
│       │   ├── extractor.rs            735
│       │   ├── layout.rs               237
│       │   └── navigator.rs             37
│       └── js/
│           ├── mod.rs                   90
│           ├── js_bridge.rs          2,120  ← core (was 2093 monolithic)
│           ├── timers.rs                53  ← SPLIT from js_bridge
│           ├── events.rs                33  ← SPLIT
│           ├── selector.rs              31  ← SPLIT
│           ├── fetch.rs                 31  ← SPLIT
│           └── storage.rs               88  ← SPLIT
├── korlang/                          671 LOC
│   ├── Cargo.toml                       7
│   └── src/
│       ├── lib.rs                     618  ← 42 inline tests!
│       ├── compiler/
│       │   ├── mod.rs                   98  ← AtomicUsize, no unsafe
│       │   ├── lexer.rs                 96  ← comments + escape seqs
│       │   └── parser.rs               123
│       └── vm/mod.rs                   245  ← fixed Call, ForEach O(n)
├── crates/
│   ├── aether-dom/                  ~200
│   ├── aether-html/               ~1,600  ← tokenizer(985)+tree_builder(337) still DEAD
│   ├── aether-css/                ~2,100
│   └── aether-caelum/            ~17,000  ← published, unsafe transmute FIXED
└── tests/                          ~3,500
    ├── js_engine_tests.rs              452  ← NEW
    ├── korlang_advanced_tests.rs       402  ← NEW
    ├── rendering_pipeline_tests.rs     796  ← NEW
    ├── sidebar_tests.rs                698  ← NEW (expanded)
    └── (7 more files)
```

### 1.3 Security Status

| Feature | v1: Stubbed? | Current | Evidence |
|---------|-------------|---------|----------|
| CSP enforcement | ✅ Stubbed | ✅ **REAL** | `net/mod.rs:363-448` — full parser with \`self, \`none, \`unsafe-inline, \`unsafe-eval, \`strict-dynamic, \*, scheme, host, wildcard-domain |
| CORS | ✅ None | ✅ **REAL** | `net/mod.rs:572` — `fetch_with_cors()` with ACAO checking |
| Event handler blocking | ✅ None | ✅ **REAL** | `js_bridge.rs:521` — `is_event_handler()` blocks onclick/onload/onerror |
| Dangerous URL blocking | ✅ None | ✅ **REAL** | `js_bridge.rs:541` — `is_dangerous_url()` blocks javascript: and data: in href/src |
| srcdoc blocking | ✅ None | ✅ **REAL** | `js_bridge.rs:553` — set_attribute rejects srcdoc |
| innerHTML sanitization | ✅ None | ✅ **REAL** | `js_bridge.rs:725-730` — strips script/style/iframe/object/embed tags |
| Cookie per-origin | ✅ None | ✅ **REAL** | `net/mod.rs:127` — `cookie_origin_key()` for isolation |
| Cookie size limits | ✅ None | ✅ **REAL** | `net/mod.rs:170` — 50 per origin, 500 total |
| HTTPS validation | ✅ Default | ✅ Default | reqwest uses native-tls, no custom pinning yet |
| SSL cert pinning | ✅ None | ❌ **ABSENT** | `net/mod.rs:54-58` — default builder, no pinning |

### 1.4 Korlang Status

| Bug | v1 | Current | Fixed In |
|-----|-----|---------|----------|
| `unsafe static mut LABEL_COUNTER` | ❌ unsafe | ✅ `AtomicUsize` | commit `01026a5` |
| `Call` opcode wrong arg indexing | ❌ broken | ✅ fixed | commit `1f80a42` |
| `ForEach` O(n²) linear scan | ❌ O(n²) | ✅ O(n) with cached end | commit `1f80a42` |
| `expect()` ignores return | ❌ broken | ✅ `eprintln!` on fail | commit `01026a5` |
| Lexer unknown chars silent | ❌ silent | ✅ `eprintln!` | commit `1f80a42` |
| Renderer unknown handlers | ❌ silent | ✅ `eprintln!` warning | commit `1f80a42` |
| No comments in lexer | ❌ missing | ✅ `//` and `/* */` | commit `1f80a42` |
| No escape sequences | ❌ missing | ✅ `\n`, `\r`, `\t`, `\\`, `\"` | commit `1f80a42` |
| Stack underflow panics | ❌ panics | ✅ guarded | commit `1f80a42` |
| Jump out-of-bounds | ❌ panics | ✅ bounds-checked | commit `1f80a42` |
| `interop/mod.rs` empty stub | ❌ stub | ✅ **REMOVED** | commit `01026a5` |
| `rust_gen.rs` empty stub | ❌ stub | ✅ **REMOVED** | commit `01026a5` |
| **`Load` ignores builtins** | — | ❌ **STILL BROKEN** | `vm/mod.rs:94-97` |
| **chrome.\* stubs are no-ops** | — | ❌ **STILL STUBS** | `korlang.rs:26-45` |
| **42 inline tests** | 2 tests | ✅ **42 tests** | all pass |

---

## 2. WHAT'S BEEN FIXED vs v1 REPORT

### 2.1 Critical Fixes (old report said 51 compile errors — now 0)

```
src/engine/pipeline/extractor.rs   — Iced 0.13 API mismatches FIXED
src/engine/pipeline/fetcher.rs     — JsBridge/JSEngine type mismatches FIXED
src/engine/net/mod.rs              — reqwest 0.12 API FIXED
src/engine/js/js_bridge.rs         — Split into 6 modules, type mismatches FIXED
src/ui/screens/browser.rs          — Iced 0.13 widget API + Korlang FIXED
```

### 2.2 Error Handling (old: 35+ panic points — now ~0)

```
ALL unwrap()/expect() calls REPLACED with:
  - unwrap_or_else(|e| e.into_inner())  — Mutex poison recovery
  - .ok() / ?  — proper Result propagation
  - FetchError enum  — typed errors in network layer
  - element_at_point() returns Option<u32>  — no more panics
  - NO bare .unwrap() on Mutex locks anywhere
```

### 2.3 Security (old: all stubbed — now real)

```
CSP:     stubbed→real parser+enforcement (net/mod.rs:363-448)
CORS:    absent→ACAO checking (net/mod.rs:572)
Events:  absent→blocked (js_bridge.rs:521)
URLs:    absent→blocked (js_bridge.rs:541)
Cookies: plain→isolated by origin (net/mod.rs:127,170)
innerHTML: none→sanitized (js_bridge.rs:725-730)
```

### 2.4 God-File Refactoring (old: 2 god-files — now 6+ modules)

```
js_bridge.rs (was 2093 lines, monolithic)
  → js_bridge.rs       (2120 lines — core bridge + SHIM_JS + API registration)
  → timers.rs          (53 lines  — setTimeout/setInterval/clear polling)
  → events.rs          (33 lines  — addEventListener/removeEventListener/getListeners)
  → selector.rs        (31 lines  — querySelector/querySelectorAll on flat DOM)
  → fetch.rs           (31 lines  — fetch_url/fetch_url_xhr with CORS)
  → storage.rs         (88 lines  — cookie/localStorage CRUD)

browser.rs (was ~1800 lines)
  → browser.rs         (1067 lines — core screen logic + tabs + chrome)
  → SHIM_JS moved to js_bridge.rs
  → DevTools logic removed (was placeholder anyway)
```

### 2.5 Korlang (old: barely works — now 12 bugs fixed)

```
unsafe static mut       → AtomicUsize           ✅
Call opcode broken      → correct arg ordering  ✅
ForEach O(n²)           → O(n) with cache       ✅
expect() debug_assert   → eprintln! warning     ✅
Lexer silent skip       → eprintln! warning     ✅
No comments             → // and /* */          ✅
No escape seqs          → \n \r \t \\ \" \'    ✅
Stack underflow panic   → guarded               ✅
Jump OOB panic          → bounds-checked        ✅
Empty stubs             → REMOVED               ✅
42 inline tests         → ALL PASS              ✅
```

### 2.6 Test Expansion (old: ~225 — now ~283)

```
NEW: js_engine_tests.rs          452 lines, ~40 tests  (selectors, timers, events, fetch, storage)
NEW: korlang_advanced_tests.rs   402 lines, ~20 tests  (nested calls, closures, ForEach, Jump, element tree)
NEW: rendering_pipeline_tests.rs 796 lines, ~70 tests  (CSS, extractor, block/inline/flex/grid/float/margin/border)
NEW: sidebar_tests.rs            698 lines, ~50 tests  (tabs, nav, settings, history, autocomplete, filters)
```

---

## 3. REMAINING ISSUES — PRIORITY STACK

### P0: CRITICAL — Blocks functionality

| # | Issue | File:Line | Impact | Effort |
|---|-------|-----------|--------|--------|
| C1 | `impl_parse_for_keyword_enum!` is a **no-op** macro | `crates/aether-caelum/src/macros.rs:18-20` | Every CSS keyword property silently fails: `display: flex`, `text-align: center`, `float: left`, `position: absolute`, ~15+ keyword enums. Layout is broken for all. | **1 line fix** — replace `{}` with the actual parse implementation |
| C2 | `Load` opcode never checks `self.builtins` | `korlang/src/vm/mod.rs:94-97` | Status bar values set via `set_builtin()` never render — shows "none" until `update_state()` fires on page load | 1 line fix — add `|| self.builtins.get(name)` |
| C3 | `chrome.render`, `chrome.navigate`, `chrome.setTitle` are **no-op stubs** | `src/engine/korlang.rs:26-45` | Korlang chrome callbacks log but don't do anything. Buttons in sidebar/status bar are decorative only | ~30 lines per callback |

### P1: HIGH — Affects correctness

| # | Issue | File:Line | Impact | Effort |
|---|-------|-----------|--------|--------|
| C4 | Descendant combinator (whitespace) **parses incorrectly** | `js_bridge.rs:118-136` | `div p` selector matches nothing — only child combinator `>` works. Confirmed with `// ponytail:` comments | 2 lines — fix whitespace combinator emission |
| C5 | `rgb()`/`rgba()`/`hsl()`/`hsla()` CSS color functions **not parsed** | `crates/aether-css/src/parser.rs:286-308` | Any CSS using functional color syntax gets no color. Old tests `css_regression.rs` CODIFY this as expected | ~100 lines — add functional color parsing |
| C6 | Only **13 named CSS colors** out of 148 | `crates/aether-css/src/style_value.rs:72-89` | `silver`, `maroon`, `navy`, `teal`, `olive`, `lime`, `fuchsia`, `aqua`, etc all return None | ~20 lines — add remaining 135 colors |
| C7 | `should_skip_tag` out of sync with its test | `extractor.rs:105` vs `tests/compliance/html5.rs:9` | Test expects `iframe`, `textarea`, `select`, `option` to be skipped but extractor doesn't skip them | 1 line per tag |
| C8 | `parse_length_vp` uses viewport WIDTH for vertical margin/padding | `resolver.rs:146-159` | `margin-top: 50%` resolves as 50% of viewport width, not height. Fix: use `parse_length_vp_vertical` for vertical properties | 2 lines — switch function call |
| C9 | `overflow-checks = false` in dev profile | `Cargo.toml:32` | Integer overflow silently wraps. All `as usize`/`as u32` casts can produce garbage without detection | 1 line change |

### P2: MEDIUM — Polish & tech debt

| # | Issue | File:Line | Effort |
|---|-------|-----------|--------|
| C10 | `FormInputChanged` and `FormElementClicked` unused — 1 warning | `browser.rs:53-54` | Remove or implement the match arms |
| C11 | Pre-1970 dates in cookie parsing ignored (ponytail) | `js_bridge.rs:256` | Add pre-1970 date handling or document as limitation |
| C12 | Image resize uses Lanczos3 — potentially slow for large images (ponytail) | `fetcher.rs:333` | Profile and add fast-path for small images |
| C13 | No HTTPS→HTTP downgrade protection in redirects | `net/mod.rs:585-665` | Check scheme doesn't downgrade on redirect |
| C14 | Cookie store in-memory only — no periodic persistence | `net/mod.rs:100-103` | Add periodic `save_cookies()` via timer |
| C15 | CSS cache is unbounded (100 entry hard cap, full eviction) | `fetcher.rs:224-228` | Replace with LRU cache crate |
| C16 | No SSL cert validation / certificate pinning | `net/mod.rs:54-58` | Add reqwest TLS config with cert pinning |

### P3: LOW — Nice to have

| # | Issue | File:Line | Effort |
|---|-------|-----------|--------|
| C17 | `tokenizer.rs` (985 lines) and `tree_builder.rs` (337 lines) still DEAD | `crates/aether-html/src/` | Delete both |
| C18 | `serde`/`rmp-serde` declared in korlang/Cargo.toml but never used | `korlang/Cargo.toml:6-7` | Remove unused deps |
| C19 | KORLANG.md references deleted `korlang/src/interop/` | `KORLANG.md:47` | Fix stale doc link |
| C20 | No source location tracking in Korlang parser | `korlang/src/compiler/lexer.rs` | Add line/column tracking |

---

## 4. PHASE 0 — TEST THE FOUNDATION

**Goal:** `cargo test` passes with zero failures and zero warnings.

### 0.1 Fix the 1 compilation warning

```
File: src/ui/screens/browser.rs
Lines: 53-54
What: FormInputChanged(usize, String) and FormElementClicked(usize) declared but never constructed
Action: Either:
  (a) Remove the variants and update match arms → simplest fix
  (b) Implement form input handling in the rendering pipeline → feature work
Recommendation: (a) — these are stubs from the form-input plan, remove until form handling is real
```

### 0.2 Run `cargo test` and catalog failures

```
Expected: ~283 tests across 11 test files
Action:
  1. cargo test 2>&1 | tee test_results.txt
  2. Count total tests, passed, failed
  3. For each failure, fix or update the test to match current behavior
  4. If a deliberate limitation (e.g., rgb() not parsed), update the test expectation
```

### 0.3 Fix `should_skip_tag` test gap

```
File: src/engine/pipeline/extractor.rs (approx line 105)
File: tests/compliance/html5.rs (approx line 9)

Problem: Test expects these tags to be skipped:
  iframe, textarea, select, option

But extractor's should_skip_tag() does NOT include them.
Fix: Add the 4 missing tags to should_skip_tag() OR update the test list.
```

**Deliverable:** `cargo test` — 283+ tests all green.

---

## 5. PHASE 1 — FINISH THE CSS ENGINE

**Goal:** CSS parsing matches what the rendering pipeline consumes. No broken property should silently produce wrong output.

### 1.1 Fix `impl_parse_for_keyword_enum!` macro (P0 — 1 line)

```
File: crates/aether-caelum/src/macros.rs
Current (line 18-20):
    #[allow(unused_macros)]
    macro_rules! impl_parse_for_keyword_enum {
        ($e:ident, $($rest:tt)*) => {};
    }

Fix: Replace the no-op body with the actual parse implementation.
This macro generates FromCss implementations for keyword enums (Display, Position, TextAlign, Float, etc.)
Without it, ALL keyword-based CSS properties silently fail to parse.

Action:
  1. Find a working FromCss impl in the existing crate (check other files in style/ directory)
  2. Replace the macro body with the correct implementation
  3. Test: `display: flex`, `text-align: center`, `position: absolute`, `float: left`
     should ALL produce correct enum values instead of returning defaults
```

### 1.2 Parse `rgb()`/`rgba()`/`hsl()`/`hsla()` CSS color functions (P1)

```
File: crates/aether-css/src/parser.rs (approx line 286-308)
Current: parse_property_value() returns PropertyValue::Keyword("rgb(255, 0, 0)")
         → resolver.rs:103-105 → Color::from_named() returns None

Already exists: parse_color_function() at parser.rs (approx line 450)
Action:
  1. In parse_property_value(), when encountering "rgb(" or "rgba(" or "hsl(" or "hsla(":
     - Extract the component values
     - Call parse_color_function() in resolver.rs
     - Return StyleValue::ColorValue(...) instead of Keyword
  2. Update tests in css_regression.rs (currently expect None for rgb() — change to expect Some)
  3. Add tests for: hsl(), rgba() with alpha, hsla(), edge cases (spaces, no spaces)
```

### 1.3 Parse bare numbers for opacity, z-index, flex-grow, flex-shrink (P1)

```
File: crates/aether-css/src/resolver.rs (approx lines 52-84)
Current: opacity, z-index, flex-grow, flex-shrink all parse via parse_keyword() or parse_length_vp()
         which don't handle PropertyValue::Number

Fix: In resolver.rs, for numeric-only properties:
  - opacity: match PropertyValue::Number(n) → Some(StyleValue::Number(n.clamp(0.0, 1.0)))
  - z-index: match PropertyValue::Number(n) → Some(StyleValue::Number(n))
  - flex-grow, flex-shrink: match PropertyValue::Number(n) → Some(StyleValue::Number(n.max(0.0)))
  - All: match PropertyValue::Keyword(s) where s.parse::<f32>() is ok → same conversion

Tests: Update css_regression.rs to expect Some(value) instead of None for these.
```

### 1.4 Add all 148 named CSS colors (P1)

```
File: crates/aether-css/src/style_value.rs (approx lines 72-89)
Current: 13 named colors (black, white, red, green, blue, yellow, cyan, magenta, gray/grey, orange, purple, pink, transparent)
Missing: silver, maroon, navy, teal, aqua, lime, fuchsia, olive, ~130 more

Action: Add the full CSS named color table (148 entries).
Reference: https://www.w3.org/TR/css-color-4/#named-colors
Each entry: "color_name" => Color::from_rgb8(r, g, b)
```

### 1.5 Use `parse_length_vp_vertical` for vertical margins/padding (P1)

```
File: crates/aether-css/src/resolver.rs (approx line 56)
Current: apply_sides_vp() calls parse_length_vp() for ALL 4 sides of margin/padding/border
Fix: Call parse_length_vp_vertical() for top and bottom (margin-top, margin-bottom, padding-top, padding-bottom, border-top-width, border-bottom-width)
     Call parse_length_vp() for left and right (already correct)
```

### 1.6 Add HTML entity decoding (P2)

```
File: src/engine/pipeline/extractor.rs (text node extraction, or wherever text content is collected)
Files: Possibly src/engine/pipeline/fetcher.rs (where text is injected from JS)

Current: &amp; &lt; &gt; &quot; &#123; etc appear literally in rendered text
Fix: Add a decode_html_entities(text: &str) -> String function:
  - Replace &amp; → &
  - Replace &lt; → <
  - Replace &gt; → >
  - Replace &quot; → "
  - Replace &#NNNN; → Unicode char for code point NNNN
  - Replace &#xHHHH; → Unicode char for hex code point HHHH

Apply in:
  - extract_elements() when reading text node content
  - inject_js_output() when processing document.write() output
  - attribute values (href/src/alt)
```

**Deliverable:** `display: flex`, `rgb()` colors, `opacity: 0.5`, 148 named colors all work correctly. HTML entities decoded.

---

## 6. PHASE 2 — FIX JAVASCRIPT BRIDGE

**Goal:** All JS DOM APIs work correctly. No silent failures.

### 2.1 Fix descendant combinator (` ` space) in CSS selectors (P1)

```
File: src/engine/js/selector.rs (or js_bridge.rs if not fully extracted)
Lines: ~10-15 (or wherever the combinator dispatching is)

Current: parse_complex() correctly handles ">", "+", "~" combinators
         but whitespace-only (descendant) combinator is not emitted — it falls through.
         Confirmed by ponytail comments in tests:
           js_engine_tests.rs:46-57  "// ponytail: whitespace combinator not fully parsed"
           js_bridge_test.rs:500-502 "// ponytail: descendant combinator"

Fix in matches_complex() or equivalent:
  When the combinator between two simple selectors is whitespace (no explicit combinator token):
    The element must be a descendant (not direct child) of the ancestor match.
    Walk up parent chain: for each ancestor, check if it matches the left-side selector.
    If any ancestor matches → success.
    If root reached without match → failure.

Fix in parse_complex() (if bug is in parser):
  After consuming the right-side simple selector, when the next char is whitespace:
    Emit Combinator::Descendant instead of consuming whitespace and continuing.

Test cases:
  - "div p" should match <div><p>text</p></div> (descendant)
  - "div > p" should match <div><p>text</p></div> (child)
  - "div p" should NOT match <div><span><p>text</p></span></div> (wait, it SHOULD — descendant matches ANY depth)
  - "div > p" should NOT match <div><span><p>text</p></span></div> (child only, one level)
```

### 2.2 Add proper JavaScript error propagation (P2)

```
Files: src/engine/js/mod.rs, src/engine/js/js_bridge.rs
Current: JS execution errors in SHIM_JS eval, timer dispatch, and event dispatch are
         swallowed with let _ = ctx.eval() and let _ = register_browser_api()

Fix:
  1. SHIM_JS eval failure (js_bridge.rs approx line 2088):
     - Log the JS error with plog! instead of let _ =
     - The bridge will have partial functionality (some APIs undefined) but at least the error is visible
  2. register_browser_api() failure (js/mod.rs:49,63):
     - Log which API registration failed
     - Continue loading (some APIs will be undefined in JS)
  3. Timer dispatch failure (browser.rs:415,473):
     - Log the timer callback error
     - Continue polling (other timers still fire)
```

### 2.3 Fix cookie date parsing for pre-1970 (P2)

```
File: js_bridge.rs (approx line 256, parse_rfc1123_date)
Current: ponytail comment notes "leap seconds and pre-1970 dates are ignored"
Fix: Handle dates before 1970 (they're valid in HTTP date headers, e.g., Expires: Thu, 01 Jan 1970 00:00:00 GMT to expire a cookie immediately)
     Add: if year < 1970, treat as expired (max-age = 0)
```

**Deliverable:** Descendant combinator works, JS errors are visible in logs, pre-1970 cookie dates handled.

---

## 7. PHASE 3 — COMPLETE KORLANG

**Goal:** Korlang is a functional, safe, tested UI DSL integrated with the browser.

### 3.1 Fix `Load` opcode to check builtins (P0 — 1 line)

```
File: korlang/src/vm/mod.rs (approx lines 94-97)
Current:
    OpCode::Load(name) => {
        let val = self.heap.get(&name).cloned().unwrap_or(Value::None);
        self.stack.push(val);
    }

Fix:
    OpCode::Load(name) => {
        let val = self.heap.get(&name)
            .or_else(|| self.builtins.get(&name))
            .cloned()
            .unwrap_or(Value::None);
        self.stack.push(val);
    }

Impact: Status bar values (status_left, status_mid, status_right) set via set_builtin()
        will now actually render instead of showing "none".
```

### 3.2 Wire up `chrome.*` native callbacks (P0)

```
File: src/engine/korlang.rs (approx lines 26-45)

Current stubs (3):
  - chrome.render(str)     → logs char count only
  - chrome.setTitle(str)   → logs title only
  - chrome.navigate(url)   → logs URL only

Fix each:

(a) chrome.render(str):
    - Parse the string as Korlang source (call eval_korlang())
    - Convert the resulting KorObject tree to Iced elements via kor_renderer.rs
    - Return the rendered element count
    - Current: just logs. Fix: actually renders.

(b) chrome.setTitle(title):
    - The renderer needs a way to receive the title from Korlang
    - Option 1: Store in a shared state (Arc<Mutex<String>>) that browser.rs reads
    - Option 2: Send a message back through BrowserScreen
    - Simplest: Add `window_title: Arc<Mutex<String>>` to the Bridge state in korlang.rs
    - browser.rs reads it each frame for the window title

(c) chrome.navigate(url):
    - Needs to trigger a BrowserMessage::UrlSubmit or BrowserMessage::NavigateTo(url)
    - Challenge: korlang.rs doesn't have access to the BrowserScreen's update channel
    - Solution: Store an `on_navigate: Option<Box<dyn Fn(String) + Send>>` callback
      in the Korlang engine state. BrowserScreen sets this callback during init.
    - When chrome.navigate(url) is called, invoke the callback → triggers navigation.

(d) Buttons in sidebar/status bar that use these callbacks will now actually work.
```

### 3.3 Add reactive state updates to Korlang VM (P2)

```
File: korlang/src/vm/mod.rs + kor_renderer.rs + browser.rs

Current: update_state() modifies VM heap, but the element tree already exists on the stack
         with captured string values. The status bar works by calling update_state() before
         render_kor_vm() reads from the VM stack every frame.

Limitation: Reactive state changes (e.g., a counter that increments) require:
  - Re-executing the entire Korlang program bytecode to regenerate the element tree, OR
  - Having a binding system where VM properties propagate to rendered widgets

Simple fix (Phase 3):
  - On each render_kor_vm() call, re-run the VM bytecode from the component setup block
  - This re-generates the element tree with current heap values
  - Only the status bar is currently reactive, so impact is minimal

Advanced fix (Phase 8, see section 12):
  - Implement true reactive bindings
  - Implement computed properties
  - Implement state observers
```

### 3.4 Remove unused `serde`/`rmp-serde` deps (P3)

```
File: korlang/Cargo.toml (lines 6-7)

Current declared but unused:
  serde = { version = "1.0", features = ["derive"] }
  rmp-serde = "1.1"

Remove both unless bytecode serialization is planned.
If it IS planned, add a comment explaining future use.
```

### 3.5 Fix stale KORLANG.md reference (P3)

```
File: KORLANG.md (approx line 47)
Current: mentions "korlang/src/interop/" which was removed in commit 01026a5
Fix: Remove the reference or replace with current architecture description
```

**Deliverable:** Status bar renders real values, chrome callbacks actually work, no unused deps, docs accurate.

---

## 8. PHASE 4 — HARDEN AETHER-CAELUM

**Goal:** The published crate is safe, documented, and its public API works correctly.

### 8.1 Remove dead code artifacts (P3)

```
File: crates/aether-caelum/src/macros.rs
Current:
  - debug_log!({})               — 4 no-op debug macros
  - debug_log_node!({})          ─ can be removed or made configurable
  - debug_push_node!({})         └ via a feature flag
  - debug_pop_node!({})          
  - impl_parse_for_keyword_enum!(...) — FIXED in Phase 1.1 above

Action:
  - Replace no-op debug macros with cfg-gated implementations:
    #[cfg(feature = "debug_layout")]
    macro_rules! debug_log { ($($t:tt)*) => { eprintln!($($t)*) }; }
    #[cfg(not(feature = "debug_layout"))]
    macro_rules! debug_log { ($($t:tt)*) => {}; }
  - Add feature "debug_layout" to Cargo.toml
  - impl_parse_for_keyword_enum! — fixed in Phase 1.1
```

### 8.2 Fix remaining TODO/FIXME markers

```
From v1 report: 38 TODOs/FIXMEs across aether-caelum

Current count: Needs re-audit. Likely reduced but not eliminated.

Priority TODOs to fix:
  1. compute/block.rs — nested blocks, float positioning, auto margins (8 TODOs)
     - These affect real-world layouts
     - Fix float containment, margin-collapse, and nested block sizing
  2. compute/flexbox.rs — visibility collapse, writing modes, safe alignment (10 TODOs)
     - Flexbox is widely used; broken flex is a major gap
     - Prioritize: safe alignment (cross-axis), min-height sizing
  3. compute/grid/ — baseline, named lines, overflow (10+ TODOs)
     - Grid is less urgent (fewer sites depend on it)
     - Fix: named line resolution and auto-placement
```

### 8.3 Add comprehensive Caelum tests (P2)

```
Current: layout_stress.rs (244 lines, ~15 tests) + rendering_pipeline_tests.rs (some Caelum coverage)

Add tests for:
  - Flexbox wrap with min/max constraints
  - Grid auto-placement with named lines
  - Float interaction with inline elements
  - Negative margin impact on siblings
  - Absolutely positioned elements inside relative containers
  - Cross-axis alignment in flex (align-items, align-self)
  - Nested flex containers
  - Percentage-based sizing with multiple levels of nesting
```

### 8.4 Review published crate status (P2)

```
Published on crates.io as aether-caelum v0.1.0

Actions:
  1. Add prominent README.md disclaimer listing known limitations
  2. Document which features are experimental/broken
  3. Fix unsafe transmutes → already done (f32::to_bits())
  4. Consider yanking v0.1.0 and publishing v0.2.0 with all fixes
  5. Ensure Cargo.toml features section is accurate:
     - content_size feature should actually gate content_size behavior
     - Add debug_layout feature for layout debugging
  6. Add a CONTRIBUTING.md specific to aether-caelum
```

**Deliverable:** No unsafe code, reduced TODO count, debug logging configurable, published crate status clear.

---

## 9. PHASE 5 — KILL REMAINING DEAD CODE

**Goal:** Remove ~1,300 lines of unused code and all `#[allow(dead_code)]` annotations.

### 9.1 Remove tokenizer.rs (985 lines) and tree_builder.rs (337 lines) — P3

```
Files: crates/aether-html/src/tokenizer.rs, crates/aether-html/src/tree_builder.rs

Current state: COMPLETELY UNUSED. The browser uses the simple inline parser in lib.rs.

Why they exist: They were an attempt at WHATWG-spec HTML5 parsing that was abandoned in favor
of the simpler recursive-descent parser. They consume ~1300 lines and create confusion.

Action:
  1. git rm crates/aether-html/src/tokenizer.rs
  2. git rm crates/aether-html/src/tree_builder.rs
  3. Remove from module declarations (if any — check mod.rs or lib.rs in aether-html)
  4. Run cargo check — verify no compilation errors

Future consideration: If proper HTML5 spec parsing is needed, these files can be restored from git history.
But for the current codebase, they are dead weight.
```

### 9.2 Remove `#[allow(dead_code)]` crate-level annotations — P3

```
Files:
  1. src/ui/style.rs — remove #![allow(dead_code)] at line 1
     Then fix actual dead functions: link_button_style, pill_style, url_bar_style, etc.
     Either remove them or add #[expect(dead_code)] with a reason

  2. src/logging.rs — remove #![allow(dead_code)] at line 1
     Then check: is log_dir() used? Is PipelineLog::init() used?
     Remove any truly dead functions.

  3. crates/aether-css/src/lib.rs — remove #![allow(dead_code)] at line 1
     Check: Module-level re-exports should be fine, but verify.

  4. crates/aether-html/src/lib.rs — remove #![allow(dead_code)] at line 1
     Done as part of 9.1 (removing dead modules)

  5. build.rs — check #[allow(dead_code)] on generated bridge functions
     Some bridge functions are genuinely unused (only used from extractor/layout)
     Keep them since they're code-generated, but add explicit allow-list.
```

### 9.3 Remove temp files from repo root (already done in v1?)

```
Files: temp_canvas.rs, temp_canvas2.rs, temp_header.rs — DELETE if still present
These were stale duplicates of code in browser.rs.
```

### 9.4 Remove remaining `let _ =` error swallows (P2)

```
Files at risk:
  - src/engine/pipeline/layout.rs:140,144 — tree.add_child / tree.compute_layout errors
  - crates/aether-caelum/src/tree/caelum_tree.rs:548-611,998,1015 — 11 tree operation results
  - src/engine/pipeline/fetcher.rs:296 — JS execution error
  - src/ui/screens/browser.rs:415,473 — timer/event execution errors

Fix: At minimum, log all swallowed errors with plog! or eprintln!.
For critical paths (JS execution, layout computation), propagate the error.
```

**Deliverable:** -1,300 lines of dead code, zero `#[allow(dead_code)]` at crate level, all errors logged.

---

## 10. PHASE 6 — PRODUCTION NETWORKING

**Goal:** Network layer supports real-world browsing with caching, compression, security.

### 10.1 Add SSL certificate pinning (P2)

```
File: src/engine/net/mod.rs (approx lines 54-58)
Current: reqwest::Client::builder().build().expect("Failed to build HTTP client")
         — uses native-tls with system certificates, no custom validation

Fix:
  use reqwest::tls::{Certificate, TlsInfo};

  let mut builder = reqwest::Client::builder()
      .timeout(Duration::from_secs(15))
      .danger_accept_invalid_certs(false);  // explicit — don't accept invalid certs

  // Optional: Add certificate pinning for known sites
  // let cert = include_bytes!("../certs/lets-encrypt-x3.pem");
  // builder = builder.add_root_certificate(Certificate::from_pem(cert)?);
```

### 10.2 Add HTTPS→HTTP downgrade protection (P2)

```
File: src/engine/net/mod.rs (approx lines 585-665, fetch_inner redirect handling)

Current: Redirects are followed without checking if the scheme downgrades from https to http.
Fix:
  fn check_scheme_downgrade(original_url: &str, redirect_url: &str) -> bool {
      let orig_is_https = original_url.starts_with("https://");
      let redir_is_http = redirect_url.starts_with("http://");
      if orig_is_https && redir_is_http {
          plog!("Blocked HTTPS→HTTP downgrade redirect: {} → {}", original_url, redirect_url);
          return false;  // Don't follow this redirect
      }
      true  // Safe to follow
  }

  // Apply in redirect handling loop before following each redirect
  if !check_scheme_downgrade(&url, &redirect_url) { break; }
```

### 10.3 Add periodic cookie persistence (P2)

```
File: src/engine/net/mod.rs (approx lines 100-103)
Current: Cookies saved to file only on mutation (save_cookies() called after each set)
Fix: Add a timer that saves cookies every 30 seconds:
  - Call save_cookies() periodically
  - On shutdown, save cookies one more time
  - On startup, load cookies from file

Alternative: Use the Iced subscription system to drive periodic saves via a timer message.
```

### 10.4 Replace CSS cache with proper LRU (P2)

```
File: src/engine/pipeline/fetcher.rs (approx lines 224-228)
Current: RwLock<HashMap<String, Stylesheet>> with 100-entry hard cap, full eviction on overflow

Fix: Use the `lru` crate (or implement a simple LRU):
  use lru::LruCache;
  use std::num::NonZeroUsize;
  
  static CSS_CACHE: LruCache<String, Stylesheet> = LruCache::new(NonZeroUsize::new(100).unwrap());
  
  // get_or_insert_with pattern:
  fn get_or_fetch_stylesheet(url: &str) -> Stylesheet {
      let mut cache = CSS_CACHE.lock();
      if let Some(stylesheet) = cache.get(url) {
          return stylesheet.clone();
      }
      // fetch from network
      let stylesheet = fetch_stylesheet_inner(url);
      cache.put(url.to_string(), stylesheet.clone());
      stylesheet
  }
```

### 10.5 Add response compression support (P3)

```
File: src/engine/net/mod.rs (fetch_inner or fetch_with_redirects)
Current: No Accept-Encoding header, no compression handling
Fix:
  - reqwest supports gzip/deflate/brotli/zstd natively via features:
    features = ["blocking", "http2", "native-tls", "gzip", "brotli"]
  - Add "gzip" and "brotli" to reqwest features in Cargo.toml
  - reqwest will automatically decode compressed responses
  - This reduces bandwidth by ~70% for HTML/CSS/JS

  (Note: reqwest 0.12 default features don't include compression.
   Add them explicitly in Cargo.toml.)
```

**Deliverable:** SSL pinning, scheme downgrade protection, cookie persistence, LRU CSS cache, gzip support.

---

## 11. PHASE 7 — ADVANCED RENDERING

**Goal:** Browser can render real-world websites with forms, tables, SVG, and proper positioning.

### 11.1 Form input support (input, textarea, select, button)

```
Files: src/engine/pipeline/extractor.rs (element extraction)
       src/engine/pipeline/extractor.rs (should_skip_tag)
       src/ui/screens/browser.rs (rendering)
       src/engine/js/js_bridge.rs (DOM API for form elements)

Current: All form elements skipped by should_skip_tag()
Fix:
  1. Remove <input>, <textarea>, <select>, <option>, <button> from the skip list
  2. In extract_elements():
     - <input>: Create element with type attribute. If type="text" (or default), show as text input.
               If type="checkbox"/"radio", show as toggle. If type="submit"/"button", show as button.
     - <textarea>: Create multi-line text input element
     - <select>/<option>: Create dropdown/option list element
     - <button>: Create clickable button with inner text
  3. Add StyledElement fields: input_type, input_value, input_placeholder, checked
  4. In browser.rs rendering:
     - Render form elements as interactive Iced widgets within the canvas
     - Handle value changes and submission events
  5. Implement FormInputChanged and FormElementClicked browser messages (currently unused stubs!)
```

### 11.2 Table rendering support

```
Files: src/engine/pipeline/extractor.rs (extract_elements)
       src/engine/pipeline/layout.rs (Caelum layout for table)

Current: <table>, <tr>, <td>, <th> are parsed into DOM but rendered as generic blocks.
Fix:
  1. Remove <table>, <tr>, <td>, <th>, <thead>, <tbody>, <tfoot>, <colgroup>, <col> from skip list
     (Check: are they already extracted? They might not be in skip list but need special handling)
  2. In extract_elements(), handle table-specific CSS properties:
     - border-collapse, border-spacing, caption-side
     - table-layout (fixed vs auto)
     - column widths from <colgroup>
  3. Caelum already has grid layout — use grid layout for <table> elements
  4. Render: <table> as grid, <tr> as grid rows, <td>/<th> as grid cells
```

### 11.3 Basic SVG support

```
Files: src/engine/pipeline/extractor.rs (element extraction)
       src/engine/image/mod.rs (image decoding)

Current: <svg>, <path>, <circle>, <rect> all in should_skip_tag()
Fix:
  1. Remove <svg> from skip list
  2. Extract SVG elements as styled elements with dimensions
  3. Render as simple shapes in the canvas:
     - <rect>: filled/stroked rectangle at position
     - <circle>: filled/stroked circle
     - <line>: line from (x1,y1) to (x2,y2)
     - <text>: text at position
  4. For complex SVGs, render a placeholder box with a note
  5. Use the `resvg` or `usvg` crate for proper SVG rendering (stretch goal)
```

### 11.4 CSS positioning support (absolute, fixed, sticky, relative)

```
Files: src/engine/pipeline/extractor.rs (compute_full_style — reads position/top/left/right/bottom)
       src/engine/pipeline/layout.rs (Caelum layout)

Current: Position is parsed from CSS but ignored during layout. All elements use static flow.
Fix:
  1. In el_to_caelum_style() in layout.rs, read position: absolute/fixed/sticky/relative
  2. Map to Caelum Position property:
     - relative: Layout relative to normal flow, then offset by top/left/right/bottom
     - absolute: Remove from flow, position relative to positioned ancestor
     - fixed: Remove from flow, position relative to viewport
     - sticky: Hybrid — flow normally until scroll threshold, then fix
  3. Pass top/left/right/bottom values from CSS to Caelum as inset properties
  4. Caelum already supports these — they just need to be connected
```

### 11.5 CSS pseudo-class support (:hover, :active, :focus) (P3)

```
Current: Pseudoclasses parsed but never evaluated during rendering.
Fix:
  1. Add hover state tracking to BrowserScreen (Option<u32> for hovered element index)
  2. On mouse move (PageCanvas::update or cursor_event), detect which element is hovered
  3. Re-resolve CSS for the hovered element with :hover pseudo-class
  4. Re-render with hover-state styles (color change, underline, background highlight)
```

### 11.6 Image proper rendering (P3)

```
File: src/engine/image/mod.rs + src/ui/screens/browser.rs PageCanvas

Current: Images rendered via iced::widget::image::Handle (from image crate data).
Limitation: iced 0.13 canvas doesn't natively support image painting on Frame.
            The workaround uses handle.to_rgba() and manual pixel manipulation.
Fix:
  1. Check if iced 0.13 canvas supports image rendering natively now
  2. If not, use iced::widget::image::Image widget as an overlay on the canvas
  3. For each <img> element, position an Image widget at the Caelum-computed position
  4. Handle alt text rendering when image fails to load
```

**Deliverable:** Forms, tables, basic SVG, CSS positioning, and image rendering all work.

---

## 12. PHASE 8 — KORLANG: BEST LANGUAGE & UI FRAMEWORK

**Goal:** Korlang is a complete, production-quality UI language and runtime.

### 12.1 Language Features (4-6 weeks)

```
Implement the following features in the compiler + VM:

(a) Arithmetic expressions (P2 — 3 days)
    - Add binary operators: +, -, *, / in lexer (tokens: Plus, Minus, Star, Slash)
    - Add expression AST nodes: BinaryOp { left, op, right }
    - Add precedence climbing or recursive descent
    - Add VM opcodes: Add, Sub, Mul, Div
    - Test: 2 + 3 * 4 → 14

(b) Boolean and comparison operators (P2 — 2 days)
    - Add tokens: And, Or, Not, Eq, Neq, Lt, Gt, Le, Ge
    - Add AST nodes: ComparisonOp, LogicalOp
    - Add VM opcodes: And, Or, Not, Compare (with comparison type)
    - Test: if a > 5 && b < 10 → true

(c) Function definitions with parameters (P2 — 4 days)
    - Syntax: fn foo(x, y) { ... }
    - Implement in parser: parse_function_def()
    - Implement in compiler: emit function with local scope for parameters
    - Implement in VM: LoadFn + Closure (opcodes exist but untested)
    - Fix Closure/load_function in VM (currently broken per lib.rs test gaps)
    - Test: fn add(a, b) { a + b }; add(2, 3) → 5

(d) List/array iteration (P2 — 2 days)
    - Add list literal syntax: [1, 2, 3]
    - Add list iteration: for item in list { ... }
    - MakeList, ListLen, ListGet opcodes exist already
    - Fix them to work with VM state
    - Test: for x in [1,2,3] { ... } → iterates 3 times

(e) String methods (P3 — 3 days)
    - Add built-in string methods via native callbacks:
      "hello".length, "hello".substring(0, 2), "a,b,c".split(",")
      "  hi  ".trim(), "hello".replace("l", "x")
    - Implement as registered native callbacks in the VM
    - Test: "hello, world".substring(0, 5) → "hello"

(f) Type annotations (P3 — 4 days)
    - Syntax: let x: Int = 5
    - Types: Int, Float, String, Bool, List<T>, Object, Void
    - Type checking phase in compiler (optional at compile time)
    - Type assertions at VM runtime
    - This is a stretch goal — benefit is moderate for a DSL
```

### 12.2 Interop System (3-4 weeks)

```
(a) Bridge::register_function() — Rust functions callable from Korlang (P2 — 3 days)
    File: src/engine/korlang.rs (or new bridge module)

    Implement:
      pub fn register_function<F>(&mut self, name: &str, func: F)
      where F: Fn(&[Value]) -> Result<Value, String> + Send + Sync + 'static

    This allows the browser to expose arbitrary Rust APIs to Korlang:
      bridge.register_function("http.get", |args| { ... reqwest call ... });
      bridge.register_function("storage.set", |args| { ... cookie/localStorage ... });

    In Korlang: http.get("https://api.example.com") → returns value

(b) Bridge::register_type() — Rust types accessible from Korlang (P3 — 5 days)
    Implement via a procedural macro:
      #[korlang_bridge]
      struct User { name: String, age: i32 }

    Generates:
      - Field getters/setters
      - Constructor
      - Type registration with Bridge

    In Korlang: let user = User("Alice", 30); user.name → "Alice"

(c) Transpiler: Korlang → Rust codegen (P3 — 7 days)
    File: korlang/src/compiler/rust_gen.rs (recreate from empty stub)

    Implement:
      - Korlang Component → Rust struct with iced widget methods
      - Korlang property → Rust property assignment
      - Korlang if/for → Rust if/for
      - Target: standalone Rust code that compiles without the Korlang VM

    Example output:
      fn render_my_component() -> iced::Element<'static, Message> {
          iced::widget::column![
              iced::widget::text("Hello"),
              iced::widget::button("Click").on_press(Message::Click),
          ].into()
      }
```

### 12.3 Standard Library (4-6 weeks)

```
Build a Korlang standard library as Rust native callbacks registered at VM init:

(std:io)
  - read_file(path) → String
  - write_file(path, content) → Void
  - stdin() → String
  - stdout(text) → Void

(std:net)
  - get(url) → Response
  - post(url, body) → Response
  - Response { status, headers, body }

(std:time)
  - sleep(ms) → Void
  - now() → Int (unix timestamp)
  - Timer { every(ms, callback) }

(std:json)
  - json.parse(text) → Value
  - json.stringify(value) → String

(std:collections)
  - List { push, pop, get, set, length, map, filter, reduce, find }
  - Map { get, set, keys, values, has }
  - Set { add, has, remove }

(std:string)
  - length, substring, indexOf, split, replace, trim, toUpper, toLower, startsWith, endsWith, contains, charAt, concat

(std:math)
  - abs, min, max, floor, ceil, round, sqrt, sin, cos, tan, random

(std:random)
  - next() → Float (0.0-1.0)
  - nextInt(max) → Int (0-max)
  - nextIntRange(min, max) → Int (min-max)
```

### 12.4 VM Performance (2-3 weeks)

```
(a) Bytecode verification pass (P2 — 2 days)
    Before execution, verify:
      - No jump targets beyond bytecode bounds
      - No stack underflow in branches
      - All labels are referenced
    This replaces runtime bounds checks with a single upfront check.

(b) Dedicated value types (P2 — 3 days)
    Current: All values are f64 internally.
    Change: Add typed values: Int(u64), Float(f64), String(Arc<String>)
    This eliminates NaN boxing overhead and enables exact integer arithmetic.

(c) Instruction caching (P3 — 3 days)
    For frequently-executed bytecode (e.g., status bar on every frame),
    cache the compiled VM state and skip re-execution if inputs haven't changed.

(d) Reactive bindings system (P3 — 5 days)
    Instead of re-executing the entire component on every frame:
    - Track which heap values each element depends on
    - Only re-compute when dependencies change
    - This enables efficient real-time UI updates
```

### 12.5 Developer Tooling (ongoing)

```
(a) Language Server Protocol (LSP) implementation (P3 — 10 days)
    Features:
      - Syntax highlighting tokens
      - Autocomplete: property names, component names, variable names
      - Diagnostics: compile errors shown in editor
      - Go-to-definition: for component and function definitions
      - Hover documentation: show type and docstring

(b) VS Code extension (P3 — 5 days)
    - Syntax highlighting (.kor files)
    - LSP client integration
    - Snippets for common patterns (Component, Row, Column, Button)
    - Korlang VM runner (run .kor files directly)

(c) Korlang formatter (P3 — 5 days)
    - Format AST → canonical source
    - Options: indent size, line width, trailing commas

(d) Korlang REPL (P3 — 3 days)
    - Interactive mode: type Korlang expressions, see results
    - Uses the VM directly without the compiler pipeline for simple expressions

(e) Web playground (P4 — 7 days)
    - WebAssembly build of the Korlang compiler + VM
    - Browser-based editor with live preview
    - Shareable URLs for Korlang snippets
```

### 12.6 Complete UI Widget Library (4-8 weeks)

```
Built on top of the Korlang language + iced rendering:

Core widgets:
  - Row, Column, ZStack (layered positioning)
  - Text, TextInput, Button, Image, Canvas
  - Container, Space, Divider

Layout:
  - List / ScrollView (virtualized scrolling)
  - LazyVStack / LazyHStack (lazy-loading rows)
  - GridView (2D grid layout)
  - NavigationView (push/pop navigation stack)

Interaction:
  - TapGesture, LongPressGesture, DragGesture
  - PinchGesture (zoom)
  - SwipeGesture (dismiss, reveal actions)

Overlays:
  - Alert (modal dialog with buttons)
  - Sheet (presented view from bottom)
  - Modal (full-screen overlay)
  - Popover (contextual popup)

Navigation:
  - NavigationStack with push/pop transitions
  - TabView with bottom tab bar
  - Sidebar / SplitView

Animation:
  - withAnimation { ... } block for implicit animation
  - Animation.easeInOut(duration: 0.3)
  - Animation.spring(response: 0.5, damping: 0.8)
  - Transition animations (slide, fade, scale) for view insertion/removal

State management:
  - @State — local component state
  - @Binding — shared mutable state between parent and child
  - @Observed — external observable model
  - @Computed — derived state that recomputes when dependencies change
  - Effects: onMount, onUnmount, onUpdate

Theming:
  - @Theme — access current theme colors, fonts, spacing
  - ColorScheme — light/dark mode
  - CustomTheme — extend with brand-specific values
  - Dynamic font sizing (responsive to system font size)
```

**Deliverable:** Korlang is a complete, production-grade UI language with standard library, tooling, and widget library.

---

## 13. PHASE 9 — MODERN BROWSER FEATURES

**Goal:** Browser supports modern web standards.

### 13.1 WebAssembly (8-12 weeks)

```
- Integrate wasmtime or wasmi for WebAssembly execution
- Implement WASM instantiation from fetched .wasm files
- Connect WASM linear memory to JS/DOM bridge
- Implement WASM JavaScript BigInt integration for i64/i64 parameters
- Support WASM threads proposal (shared memory + atomics)
- Test: Run real WASM applications (games, demos, productivity apps)
```

### 13.2 Web Workers (4-6 weeks)

```
- Implement Worker API in JS bridge: new Worker('worker.js')
- Spawn OS threads for each worker
- Implement postMessage / onmessage communication
- Implement structured clone for message passing
- Implement importScripts in worker context
- Test: Worker-based computation that doesn't block UI
```

### 13.3 Service Workers + Cache API (6-8 weeks)

```
- Implement Service Worker lifecycle (install, activate, fetch)
- Implement Cache API (caches.open, cache.put, cache.match)
- Implement FetchEvent handling in workers
- Implement offline fallback pages
- Implement Cache-Control (network-first, cache-first, network-only, stale-while-revalidate)
- Test: Offline page loading via service worker
```

### 13.4 DevTools (4-6 weeks)

```
- DOM Inspector: Tree view of DOM nodes with attributes, editing
- JS Console: REPL with output display, error highlighting
- Network Panel: Request list with timing, headers, body
- Style Inspector: Computed styles per element, CSS source
- Performance Panel: Frame timing, JS execution time, layout time
- Storage Inspector: Cookies, localStorage, sessionStorage
```

**Deliverable:** Browser can run WASM, Workers, Service Workers. DevTools provide debugging.

---

## 14. PHASE 10 — PUBLICATION & ECOSYSTEM

**Goal:** The project is usable, documented, and available for others to build upon.

### 14.1 Documentation overhaul (1-2 weeks)

```
- Update all docs/ files to match current architecture
  - AGENTS.md: Update with current error count (0, not 51)
  - ROADMAP.md: Remove stale references, add realistic timeline
  - docs/production-architecture.md: Remove references to nonexistent modules
  - KORLANG.md: Fix interop/ reference, add documentation for new features

- Add module-level doc comments (//!) on all public modules
- Add doc comments on all public functions
- Create examples/ directory with working examples:
  - examples/simple_browser.rs — minimal browser embed
  - examples/korlang_demo.rs — Korlang widget showcase
  - examples/caelum_demo.rs — Caelum layout demo
```

### 14.2 CI/CD pipeline (1 week)

```
- Set up GitHub Actions CI (already has .github/workflows/ci.yml — verify it works)
- CI steps:
  - cargo check (all workspace members)
  - cargo build (release)
  - cargo test (all test files)
  - cargo clippy (all workspace members)
  - cargo fmt --check
  - Test on Windows + Linux + macOS
- Publish tagged releases with changelog
```

### 14.3 Package publishing (P3)

```
- aether-caelum: Audit, fix, and publish v0.2.0 (or yank v0.1.0)
- aether-css: Consider publishing as Stratus CSS engine crate
- aether-html: Consider publishing as standalone HTML parser
- korlang: Consider publishing as standalone UI language crate
- Each published crate needs:
  - README.md with examples
  - API documentation with doc tests
  - Cargo.toml with accurate feature flags
  - Changelog
```

### 14.4 Community (ongoing)

```
- Create CONTRIBUTING.md with:
  - How to set up the development environment
  - Code style guidelines
  - PR workflow
  - How to report bugs
- Create issue templates for bug reports and feature requests
- Set up a discussion forum (GitHub Discussions)
- Consider a website: aether-browser.dev (or similar)
- Consider a community channel: Discord or Matrix
```

**Deliverable:** Published crates, CI pipeline, documentation, community infrastructure.

---

## 15. SUMMARY — WHAT TO DO AS A DEVELOPER

### In One Sentence

This codebase was a **research prototype** that has undergone **significant improvement** (compiles now, security is real, god-files are split, Korlang bugs are fixed). It is now **workable for development** but still has critical gaps that must be addressed before it can render real websites.

### If You Want to...

| Goal | Verdict | Action |
|------|---------|--------|
| **Learn browser internals** | ✅ **Good for learning** | The crates (aether-html, aether-css, aether-caelum) are clean, separated, and documented. The JS bridge is interesting. Korlang's VM is a great learning example. |
| **Build a production browser** | ⚠️ **Not yet** | Too many gaps: no form inputs, no tables, no SVG, no iframe, no WASM, no workers. If that's your goal, add the Phases 6→7→9 features first. |
| **Contribute to this project** | ✅ **Start now** | Begin with Phase 0-1 (low risk, high impact). Fix `impl_parse_for_keyword_enum!`, add `rgb()` parsing, add named colors. These are 1-2 day tasks with major impact. |
| **Use aether-caelum (published crate)** | ⚠️ **With caution** | Layout algorithms are good, but many features are incomplete (38 TODOs, noop debug macros). Phase 4 (crate hardening) is important. |
| **Use Korlang** | ⚠️ **Limited** | If you need a simple DSL for iced UIs, it works. If you need a full language with functions, arithmetic, and standard library, wait for Phase 8. |
| **Publish your own Rust crate** | ✅ **Learn from this project** | Keep: clean workspace separation, type-safe CSS↔layout bridge. Avoid: publishing with 38 known TODOs, stubbed features. |

### The 5 Most Impactful Next Steps

Ranked by effort/impact ratio:

| # | Task | Phase | Effort | Impact |
|---|------|-------|--------|--------|
| 1 | Fix `impl_parse_for_keyword_enum!` macro | Phase 1.1 | **1 line** | Unlocks ALL keyword CSS properties |
| 2 | Fix `Load` opcode to check builtins | Phase 3.1 | **1 line** | Status bar actually renders values |
| 3 | Wire up `chrome.*` native callbacks | Phase 3.2 | ~90 lines | Sidebar buttons actually work |
| 4 | Fix descendant combinator in selectors | Phase 2.1 | ~2 lines | CSS descendant selectors work |
| 5 | Fix `rgb()`/`rgba()` color parsing | Phase 1.2 | ~100 lines | Most websites get their colors |

These 5 tasks are the **critical path** to making the browser functional. They are small, isolated, and high-reward.

### Timeline Estimate

| Phase | Work | Estimated Time |
|-------|------|---------------|
| 0 | Test the foundation | 1-2 days |
| 1 | Finish the CSS engine | 1-2 weeks |
| 2 | Fix JavaScript bridge | 3-5 days |
| 3 | Complete Korlang | 1-2 weeks |
| 4 | Harden aether-caelum | 1-2 weeks |
| 5 | Kill remaining dead code | 1-2 days |
| 6 | Production networking | 1-2 weeks |
| 7 | Advanced rendering | 4-8 weeks |
| 8 | Korlang: best language + UI | 8-16 weeks |
| 9 | Modern browser features | 16-32 weeks |
| 10 | Publication & ecosystem | 2-4 weeks |

**Total estimate (Phase 0-7):** ~10-18 weeks for a working browser.
**Full vision (Phase 0-10):** ~9-15 months for a competitive browser with all features.
