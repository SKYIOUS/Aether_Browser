# Korlang UI Scripting Language

Korlang is a custom, lightweight DSL used for defining Aether's browser interface.

## Language Features (v0.2.0)

- **Components:** Define reusable UI blocks with `Component Name { ... }`.
- **State:** Declare reactive variables with `state name: Type = value`.
- **Functions:** Define local logic with `fn name(args) { ... }`.
- **Expressions:** Full support for arithmetic (`+`, `-`, `*`, `/`), boolean logic (`&&`, `||`, `!`), and comparisons.
- **Lists:** Array literals `[1, 2, 3]` and iteration using `for item in collection { ... }`.
- **Interpolation:** String interpolation via `"Hello $var"`.

## Example

```korlang
Component Counter {
    state value: Int = 0
    fn increment(x) { x + 1 }

    Column(spacing: 10) {
        Text(text: "Value: " + value)
        Button(text: "Increment") {
            // Logic handled via native callbacks or state updates
        }
    }
}
```

## Architecture

Korlang uses a two-phase compiler (lexer/parser) that produces bytecode for a stack-based Virtual Machine. The VM supports native function registration, allowing seamless integration between the UI DSL and Rust backend.
