# Aether Browser Project Context

## Project Overview
Aether Browser is a custom, from-scratch web browser engine built with Rust. It provides a modular, production-ready foundation for web browser development, avoiding dependency on legacy engines.

## Architecture
- **Language:** Rust
- **UI Framework:** [Iced](https://iced.rs/) (v0.13)
- **Engine Modules (`src/engine/`):**
  - `dom/`: Document Object Model.
  - `parser/`: HTML5-compliant tokenizer.
  - `css/`: CSS3 parsing and rule matching.
  - `style/`: Cascading style computation and memoization.
  - `layout/`: Box model calculations with layout caching.
  - `renderer/`: GPU-accelerated rendering hooks.
  - `net/`: HTTP resource fetching.
  - `events/`: Interaction/event handling.
  - `js/`: QuickJS runtime integration.
  - `standards/`: Specification compliance interface.
  - `process/`: Multi-process isolation.

## Build and Run
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test:** `cargo test`

## Development Conventions
- **Modular Design:** Engine components are strictly separated.
- **Documentation:** All modules must have corresponding documentation in `docs/`.
- **Roadmap:** High-level project direction is maintained in `Plans/ENGINE.md`.
- **Testing:** High-coverage unit and integration tests are mandatory for all engine modules.
