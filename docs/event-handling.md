# Event Handling and DOM Interaction

Phase 6 implements an event handling system for the Aether Browser.

## Design
- `engine::events` provides a queue for incoming user input.
- Input events (mouse, keyboard) are dispatched to the DOM tree to find target elements.

## API
- `EventQueue` handles event storage.
- Events include `Click` and `KeyDown`.
