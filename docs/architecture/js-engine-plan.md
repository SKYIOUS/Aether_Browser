# `aether-js` — Native JavaScript Engine Plan

> Plan for the workspace-native ECMAScript engine decided in [ADR-0003](../adr/0003-native-javascript-engine.md).
> Spec of record: [ECMA-262](https://tc39.es/ecma262/) (current living standard).
> Last updated: 2026-08-24

---

## 0. Summary

Build `crates/aether-js`: a Rust ECMAScript engine, executed as lexer → parser → bytecode compiler → VM over an arena/handle heap with mark-sweep GC. Conformance climbs a **rung ladder**, each rung gated by test262 pass rate on its target features. Vayu keeps QuickJS live until the swap gate.

```
R0 skeleton ──► R1 ES5 core ──► R2 ES2015 ──► R3 ES2016/17 ──► R4 ES2018–20 ──► R5+ annual
     │               │               │               │                │
  eval("1+1")   ≥95% ES5 tests  classes/generators async/await    BigInt, Proxy done,
                                Map/Set, Proxy                      modules, real-page swap
```

Non-goals (standing): ECMA-402 `Intl`, proper tail calls, SharedArrayBuffer/Atomics.

---

## 1. Crate layout

```
crates/aether-js/
├── Cargo.toml            # zero iced/tokio deps; sync API only
└── src/
    ├── lib.rs            # Realm, SourceText::eval/run; whole public surface
    ├── value.rs          # Value tagged enum + typed handles
    ├── lexer.rs          # lexical grammar (clause 11); / vs regex via parser hint
    ├── ast.rs
    ├── parser.rs         # syntactic grammar, ASI, early errors (clause 17), cover grammars
    ├── bytecode.rs       # instruction set + per-function metadata
    ├── compile.rs        # AST → bytecode; finally lowering, generator/async transforms
    ├── vm.rs             # iterative frame loop, completion records, resumable frames
    ├── heap.rs           # arenas, roots, mark-sweep, ephemerons (for Weak* later)
    ├── builtins/         # one module per intrinsic object
    │   ├── mod.rs        # ordinary internal methods [[Get]]/[[Set]]/[[DefineOwnProperty]]… ← R1-critical
    │   ├── object.rs function.rs array.rs string.rs number.rs boolean.rs
    │   ├── symbol.rs error.rs math.rs json.rs date.rs regexp_glue.rs
    │   ├── mapset.rs promise.rs gen.rs proxy_reflect.rs typedarray.rs
    ├── regexp/           # pattern parser + backtracking matcher (spec semantics)
    └── host.rs           # HostOps trait: job pump, host object registration, limits
```

Dependency policy: leaf crates only (`ryu` for shortest-round-trip `Number::toString`, `num-bigint` when R4 lands). No platform crates. The engine is embedder-agnostic; the browser is just one embedder.

## 2. Core design decisions

### 2.1 Value representation (`value.rs`)
```rust
enum Value {
    Undefined, Null, Bool(bool), Int32(i32), Float64(f64),
    String(StrIdx), Symbol(SymIdx), Object(ObjIdx),
}
```
- Plain tagged enum first; NaN-boxing only if profiling demands it.
- Int32/Float64 split mirrors spec int ops (bit ops, array indices) without SMIs.
- Strings interned in a dedicated arena; flat UTF-16-backed (spec strings are UTF-16 code units — slicing/indexing semantics depend on it; rope/slice objects are a later optimization).

### 2.2 Heap (`heap.rs`)
- Arenas: `Vec<JsObject>` etc.; indices are stable handles; deletion = free-list slot reuse after sweep.
- Mark-sweep: roots = global object, running + suspended frames, job queue, registered host handles. One pass, stop-the-world (single-threaded realm, so "the world" is cheap).
- WeakMap/WeakSet/WeakRef ride an **ephemeron table** swept after main marking.
- No `unsafe`. If a future moving/compacting collector needs it, that's a measured decision with tests around it, not a default.

### 2.3 Internal methods first (the R1 rule that prevents rewrites)
Every property read/write/define/delete goes through Ordinary internal methods ([[Get]], [[Set]], [[GetOwnProperty]], [[DefineOwnProperty]], [[Delete]], [[HasProperty]], own-property descriptors as data) from day one — even before Proxy exists. Getters/setters, `Object.defineProperty`, and Proxy all hook the same choke points later. Engines that bolt these on late rewrite every builtin; we won't.

### 2.4 Resumable frames (`vm.rs`) — generators/async for free
The interpreter loop is **iterative over an explicit `Vec<Frame>`** (no native recursion for JS calls). A frame owns pc, operand stack, environment chain, this/new.target, function kind flags ([[Call]]/[[Construct]], normal/arrow/method/generator/async).

Consequences:
- Generator `.next()` = detach top frame into the generator object; resume = push it back. Same machinery powers async functions (await = suspend) once Promises exist.
- JS stack overflow = depth counter → clean `RangeError`, never a smashed native stack.
- Cost: slower than native-call interpreters; accepted until profiling says otherwise.

### 2.5 Completion records → Rust control flow
Spec abrupt completions map to one enum threaded through every abstract-op-shaped builtin:
```rust
enum Ctrl { Normal, Throw(Value), Return(Value), Break(LabelId), Continue(LabelId) }
type Cmp<T> = Result<T, Ctrl>;   // Ok = normal completion
```
Builtins are written as direct transcriptions of spec algorithm steps; `?` propagates abrupt completions exactly where the spec says ReturnIfAbrupt. Reviewing a builtin against its clause stays mechanical.

### 2.6 RegExp (`regexp/`)
Spec regexes need backtracking (backrefs, lookbehind) — finite automata don't apply; the `regex` crate is unusable here. Dedicated recursive-backtracking matcher over a pattern AST, with a step budget per match (ReDoS cap; QuickJS-style). Parse-time early errors per clause 22.2 (duplicate named groups, invalid quantifiers...). Unicode-set flags (`v`) land no earlier than R5.

### 2.7 Host seam (`host.rs`)
```rust
trait HostOps {
    fn enqueue_job(&mut self, job: Job);          // promise/microtask pump
    fn alloc_id(&mut self) -> u64;                // opaque handles for host objects
    fn call_host(&mut self, id: u64, args: &[Value]) -> Cmp<Value>;
    fn deadline_exceeded(&self) -> bool;          // runaway-script kill switch
}
```
Host objects (DOM nodes, console, storage) are ordinary exotic objects whose behavior dispatches through `call_host` — same capability-module choke point (`dom/timers/events/net/storage/console`) `js_bridge.rs` uses today, re-hosted onto this FFI. The DOM stays owned by the browser process; JS sees opaque handles, so cross-tab handle misuse fails closed.

## 3. The rung ladder

Gates are test262 pass rate over the rung's declared feature set (`test262` vendored as a git submodule; runner parses frontmatter: features, negative phase, includes `assert.js`/`sta.js`). Pass-rate report per feature emitted to CI on every rung commit.

| Rung | Contents | Gate |
|---|---|---|
| **R0** Walking skeleton | lexer/parser/VM round-trips literals, arithmetic, closures, calls, exceptions; hand-written suite | engine runs `eval` of a few hundred programs correctly |
| **R1** ES5 core | ToPrimitive/ToNumber/ToString coercions incl. `==` quirks; prototypes & constructors; `this`; strict mode; statements; Object/Array/String/Number/Boolean/Error/Math/JSON; Date basics; RegExp basics; **ordinary internal methods** | ≥95% on non-ES2015+ feature dirs (annex `es5` corpus + `runtime semantics` baselines) |
| **R2** ES2015 | let/const/TDZ, classes, arrows, destructuring, template literals, spread/rest, iterators/for-of, **generators**, Symbols + well-knowns (`iterator`, `species`, `toPrimitive`, `hasInstance`, `toStringTag`), Map/Set (+ ephemeron WeakMap/WeakSet), default/computed params, Proxy + Reflect | ≥95% on `class`, `generators`, `Symbol`, `Proxy`, `Map/Set`, `for-of`, `destructuring` feature dirs |
| **R3** ES2016–17 | `**`, Array.includes, async functions + await (suspend via §2.4 frames), Promise (already started in R2 for async glue), Object.entries/values/values-as-enumerator fixes, string padding, trailing commas | ≥95% on `async-functions`, `Promise`, `exponentiation` |
| **R4** ES2018–20 | async iteration, rest/spread properties, regex named groups + lookbehind + `s`, BigInt (`num-bigint`), optional chaining/nullish coalescing, globalThis, Promise.allSettled/race/any, String.matchAll, dynamic `import()` behind host loader hook | ≥95% on those feature dirs |
| **R5** Modern baseline | static modules (parse/link/evaluate, host loader), WeakRefs/FinalizationRegistry, `Intl` still stubbed-and-thrown explicitly | **swap gate:** R4/R5 pass rates + Vayu real-page smoke suite (top sites list from `tests/`) green on aether-js |

Estimates (solo + AI-assisted, deliberately loose): R0 1–2 wks · R1 1–2 mo · R2 2–3 mo · R3 2–4 wks · R4 ongoing. The ladder exists so every rung ships a working engine; slippage moves gates, never quality.

## 4. Integration into Vayu

1. Add `crates/aether-js` to workspace members. Engine crate depends on nothing from `src/engine/`.
2. New `src/engine/js/aether_host.rs` implements `HostOps`, reusing existing capability modules (timers/events/net/storage/console) unchanged — they already sit behind the binding-policy choke point.
3. Realm-per-tab; single-threaded isolate; job pump driven from the iced update cycle (same place timer callbacks fire today). Blocking fetch stays browser-side; host returns promises resolved by pumped jobs.
4. Behind-the-seam parallel run: a setting routes `<script>` execution to aether-js while rquickjs remains default; flip default at the swap gate; delete JsBridge FFI last (capability modules survive).
5. `future-crates.md` now carries an "adopted / candidates / avoid" ledger with a pointer to ADR-0003 (done 2026-08-24); keep it current as engine internals adopt leaf crates (`ryu`, `num-bigint`).

## 5. Risks

| Risk | Mitigation |
|---|---|
| Scope collapse ("engine too big") | Ladder gates make progress binary and visible; non-goals standing |
| Internal-method debt if R1 shortcuts | §2.3 is a review blocker, not a suggestion; Proxy in R2 depends on it |
| ReDoS on backtracking matcher | Step budget → clean `RangeError`/timeout via `deadline_exceeded` |
| Number formatting divergence | `ryu` guarantees shortest-round-trip required by `Number::toString` |
| Generators/async frame bugs | Frames are plain data → unit-testable suspend/resume without full programs |
| Security (browser + script = attacker) | No unsafe, step budgets, host objects fail closed, realm isolation per tab, fuzz parser+lexer with cargo-fuzz from R1 onward |

## 6. Definition of done (per rung)

1. Target test262 dirs ≥95% (report committed).
2. `cargo test` green across workspace; new engine tests inline per module.
3. No new clippy suppressions; files <1000 lines (split builtins by intrinsic, they will want to sprawl).
4. Realms can be created/dropped repeatedly without leaks (arena drop = heap gone; checked with a leak counter in debug builds).
