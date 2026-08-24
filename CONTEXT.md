# CONTEXT.md — Vayu Browser domain glossary

Living vocabulary. Keep names exact; architecture discussions use these terms.

## Doctrine
- **Product independence**: no Blink/Gecko/WebKit code anywhere. External Rust components are allowed; they must sit behind seams we own. (ADR-0001)
- **Seam**: a module boundary we own where an external component could be swapped without touching callers (e.g. layout facade).

## Engine stack (canonical, live)
- **Parser**: html5ever + custom TreeSink (`src/engine/parser.rs`) → produces DOM.
- **DOM**: `aether-dom` node tree shared with QuickJS via the flat bridge.
- **Stratus**: CSS front-end (`src/engine/stratus.rs`): tokenizes/parses with Servo `cssparser`, values/resolution live in `aether-css`.
- **Selector matching**: Servo `selectors` crate (`src/engine/js/selector_engine.rs`).
- **Layout seam**: `compute_layout(elements, viewport)` — planned deep module hiding taffy + cosmic-text measurement + inline flow (today: `apply_taffy_layout`).
- **Text**: cosmic-text measurement/shaping (`src/engine/text.rs`); LRU-cached.
- **JsBridge / Bridge façade**: rquickjs (QuickJS) bindings; being decomposed into capability modules (dom/timers/events/net/storage/console) behind one binding policy choke point.
- **Pipeline stages**: Fetch → Parse → Style (Stratus+selectors) → Script (QuickJS) → Layout (seam) → Paint (PageCanvas).
- **PageCanvas**: iced canvas program that paints positioned boxes and hit-tests clicks.
- **Korlang**: embedded UI DSL VM (korlang/) driving sidebar/chrome; NOT a page scripting language. Page-reachable access must go through binding policy.

## Parked / disposed
- **Caelum** (`crates/aether-caelum`): former custom layout engine (flex/grid/block/floats). Not compiled; disposition pending ADR-0002.
- **aether-html** (`crates/aether-html`): tokenizer experiment superseded by html5ever.
