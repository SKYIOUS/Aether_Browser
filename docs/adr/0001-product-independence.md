# ADR-0001: Product independence from best-of-breed Rust components

Date: 2026-08-24 · Status: Accepted

## Context
"Truly independent browser" is ambiguous. Options considered:
- **A — Product independence**: no Blink/Gecko/WebKit code; assemble vetted Rust components behind seams we own.
- **B — Library independence**: additionally replace html5ever/taffy/cosmic-text/rquickjs with in-house engines.
- **C — Hybrid**: A now, designated components growable later.

## Decision
**Doctrine A.** Vayu is an independent engine/product: zero WebKit/Blink/Gecko code. External Rust crates are allowed (html5ever, cssparser/selectors, taffy, cosmic-text, rquickjs/QuickJS, resvg, reqwest) but must sit behind seams we own.

## Consequences
- `crates/aether-caelum` (~10k lines custom layout) and `crates/aether-html` are not seed stock for future swaps; disposition recorded in ADR-0002.
- Engineering effort goes to deepening our seams — layout facade, net facade, JS capability split — not reimplementing leaf components.
- Any component swap must be invisible above its seam; swaps happen only when an external component measurably fails us.
- The scripting layer is the one deliberate exception: [ADR-0003](0003-native-javascript-engine.md) grows `aether-js` behind the same seam discipline (JsBridge is the swap point).
