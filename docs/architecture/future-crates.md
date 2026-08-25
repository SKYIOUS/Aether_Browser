# Future Crate Plan — Vayu Browser

> Crate evaluation ledger: what was adopted, what's still a candidate, what is
> banned and why. Adoption sequencing lives in `PLAN.md`; this file only
> evaluates crates.
>
> **JS engine note:** the "which VM do we embed" question was answered by
> [ADR-0003](../adr/0003-native-javascript-engine.md): we are *building*
> `crates/aether-js` (plan: [js-engine-plan.md](js-engine-plan.md)). The avoid
> rows below remain accurate for the embedding question they were written for.
>
> Last audited against `Cargo.toml`: 2026-08-24.

## Adopted (do not re-evaluate)

| Component | Crate | Note |
|---|---|---|
| UI framework | iced 0.13 | canvas/image/tiny-skia/wgpu/tokio features |
| Async runtime | tokio 1 | rt + macros |
| JS runtime (interim) | rquickjs 0.12 | until aether-js swap gate |
| HTTP client | reqwest 0.12 | blocking, native-tls; rustls switch planned (PLAN C) |
| HTML parsing | html5ever 0.39 | custom TreeSink → aether-dom |
| CSS tokenizer | cssparser 0.37 | inside Stratus |
| Selector matching | selectors 0.40 | Servo |
| Layout | taffy 0.12 | block/inline/flex/grid |
| Text shaping | cosmic-text 0.12 | pinned by iced 0.13's internal version — do not bump independently |
| SVG | resvg/usvg 0.48 | |
| Images | image 0.25 | |
| URL | url 2 | promoted from transitive |
| Errors | thiserror 1 / anyhow 1 | |
| Serialization | serde + serde_json 1 | |
| Caches | lru 0.13 | CSS 100 entries, images 50 |
| Enums | strum 0.26 | |

## Remaining candidates

Adopt only when a PLAN phase names them; each needs a rationale here first.

| Crate | Purpose | When |
|---|---|---|
| libfuzzer-sys | Coverage-guided fuzzing harness for parser/network entry points | Phase C4 - `fuzz/` dev-crate only, never a workspace dependency; targets run in Linux CI on a pinned nightly |
| arbitrary | Structured input generation alongside libfuzzer-sys in the same fuzz dev-crate | Phase C4, same constraints as libfuzzer-sys |
| criterion | Benchmark harness for pipeline measurement (`benches/`, dev-dependency) | Phase D0 |
| lyon | Path tessellation for border-radius/clip-path GPU draws | Phase D+ |
| clipper2 | Polygon boolean ops for non-trivial `clip-path` | Phase D+ |
| swash | Low-level font ops for `@font-face` subsetting (cosmic-text bundles it transitively) | Phase D+ |
| moka | Concurrent cache if global-static contention ever measures | Only on evidence |
| palette | CSS Color Level 4 color spaces | With CSS color work |
| imageproc | CSS filter primitives (blur/contrast/…) | Phase D+ |
| zune-jpeg | Faster JPEG decode | Only on profiling evidence |
| vello / naga / kurbo | GPU renderer stack | Post-Phase-D, large surface |
| ryu, num-bigint | aether-js internals (R4): number formatting, BigInt | Per js-engine-plan rungs |

## Avoid (decided; reopening requires an ADR)

| Crate | Reason |
|---|---|
| boa_engine | As an *embedded* engine: incomplete conformance vs effort. Moot as competitor — see ADR-0003 |
| deno_core | Far too heavy for an embedded-browser runtime |
| quickjs-rs / other QuickJS bindings | rquickjs chosen; one binding only |
| markup5ever_rcdom | Test-only DOM; we sink into aether-dom |
| kuchiki / scraper | Wrappers that fight the flat-DOM model |
| ureq / isahc | Lack browser-grade redirect/cookie/TLS control; reqwest owns this role |
| chrono | Prefer `time` if datetime needed |
| log/env_logger | plog! replacement should be `tracing`, not `log` |
| mockall | Hand-written mocks suffice |
| lru (as concurrency fix) | Fine for current single-threaded caches; moka is the answer *if* contention appears |
