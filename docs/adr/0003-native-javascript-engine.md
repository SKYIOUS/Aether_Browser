# ADR-0003: Native JavaScript Engine (`aether-js`)

- **Status:** Accepted
- **Date:** 2026-08-24
- **Decides:** Vayu will build its own ECMAScript engine as a workspace crate instead of embedding an external JS VM.
- **Note:** ADR-0001 (product independence doctrine) and ADR-0002 (Caelum disposition) are cited from `CONTEXT.md`; no separate files exist for them yet.

## Context

Vayu currently executes page scripts through rquickjs (QuickJS) behind the `JsBridge` seam (`src/engine/js/js_bridge.rs`). This contradicts nothing in ADR-0001 — QuickJS sits behind a seam we own — but it leaves the scripting layer, the largest and most security-sensitive subsystem of a browser, as external code we do not control.

`docs/architecture/future-crates.md` previously recorded "avoid Boa / deno_core; rquickjs is the right weight" (its avoid-list, retained). That guidance answered *"which VM should we embed?"* This ADR supersedes it by answering a different question: *"should we embed at all?"*

## Decision

Build a native ECMAScript engine in Rust, workspace member `crates/aether-js`.

The five load-bearing choices, settled during design review:

1. **Motivation: full stack ownership.** No external JS VM at all. This extends ADR-0001 from "external components behind seams we own" to "the JS VM is ours."
2. **Acceptance bar: test262-driven ladder.** Conformance is not declared, it is measured. The engine climbs editions in phases ("rungs"); each rung exits only when its target test262 feature set passes at ≥95%. No rung may be skipped by hand-waving.
3. **Language: Rust.** Workspace-native, no FFI, and memory safety exactly where browser engines get exploited (cyclic GC'd object graphs).
4. **Execution model: bytecode compiler + VM** (QuickJS/Kiesel family). Generators, async/await, and exceptions-across-frames punish tree-walkers; a bytecode VM also leaves JIT attachable per-function later without an AST rewrite.
5. **Heap: arena + typed handles + mark-sweep.** All heap objects live in arenas; `Value`s reference them via typed indices; one tracing mark-sweep collection pass. Safe Rust throughout.

## Interim state

rquickjs stays the live engine until `aether-js` passes its integration gate (see plan: R2/R3 parity + real-page smoke suite). `JsBridge` remains the single swap point — that is precisely why the seam exists.

## Consequences

- Multi-month-to-multi-year effort; each rung still yields a usable, testable engine.
- ECMA-402 (`Intl`) is a non-goal for the foreseeable future.
- Proper tail calls are a non-goal (spec-optional).
- SharedArrayBuffer/Atomics are non-goals while realms are single-threaded.
- External leaf libraries (e.g. `ryu` for shortest-round-trip number formatting) remain permitted under ADR-0001's "external Rust components behind seams we own" rule; they are value libraries, not platforms.

## Rejection notes

Re-embedding debates (Boa, deno_core, Hermes, another QuickJS binding) are closed by this ADR unless a future explorer can show the ladder stalls permanently — reopen then, with evidence, as ADR-0004+.
