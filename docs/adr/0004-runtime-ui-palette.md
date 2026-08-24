# ADR-0004: Runtime UI Palette (Global Theme State)

- **Status:** Accepted
- **Date:** 2026-08-24
- **Decides:** Dark mode and accent color are implemented as a process-level palette snapshot behind a global lock, rather than threading theme state through view/style signatures.

## Context

Every screen styles itself through `ui/style.rs`: a unit struct `C` whose
associated items were compile-time `Color` constants, consumed at ~163 call
sites - including inside iced style closures (`move |_| container::Style { .. }`).
Iced style closures receive no application state and cannot borrow from the
screen that created them, so a palette owned by `BrowserScreen` or `VayuApp`
cannot be read where colors are actually applied without re-plumbing every
closure signature in the codebase.

Settings already persist `dark_mode: bool` and `accent_color: AccentColor{r,g,b}`,
but nothing applied them: the picker wrote a local index, and the palette was
frozen at compile time.

## Decision

`ui/style.rs` owns one process-global palette:

```rust
static PALETTE: OnceLock<RwLock<Palette>>
```

- **`Palette`** is a small `Copy` struct with the fifteen color slots the
  constants used to cover (`TRANSPARENT` stays a true constant). Constructors:
  `Palette::light(accent)` reproduces the pre-B2 values exactly;
  `Palette::dark(accent)` inverts chrome neutrals while keeping web-content
  (`PAGE_*`) and accent roles identical.
- **`set_palette(dark, accent)` is the only mutation path.** It runs once at
  startup (before first render, from settings already loaded there) and again
  from explicit user actions in the settings screen, each paired with
  `VayuSettings::save()` so disk and memory move together.
- **Consumers read through short-lived accessors** (`C::bg()`, `C::accent()`,
  ...) that copy single `Color` values under a brief read lock; no rendering
  operation holds the lock.
- A poisoned lock recovers via `into_inner()` (same policy as the JS bridge
  lock): a panicked writer leaves valid data behind, and a failed read must
  never silently reset the palette to defaults.

## Trade-offs

- **Why global state at all:** iced style closures cannot capture application
  state. The alternatives - per-widget theme objects or an `Arc<Palette>`
  cloned into every closure - would touch every styling site with plumbing
  that exists only to work around the framework constraint. One documented
  global is the smaller system.
- **One active window:** like the net client / cookie / storage statics
  already in service, this assumes a single window. Multi-window support
  (PLAN known-debt) must revisit it: per-window palettes are impossible with
  a process-wide snapshot.
- **Testability:** the palette itself is pure data with deterministic
  constructors; only the two-line global accessor sits above it.

## Consequences

- Dark mode and accent changes apply instantly to every screen after
  `set_palette`, including already-styled surfaces, because style closures
  re-read on every frame.
- New colors must be added to `Palette` (not as fresh constants), keeping the
  theming surface single-sourced.
- If multi-window arrives, this ADR is reopened by that decision per the docs
  map rules.
