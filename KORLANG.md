# Korlang UI Programming Language

**Korlang** is a proprietary, lightweight, and modern UI programming language designed specifically for the Aether Browser. It allows for declarative UI definitions with reactive state management and high-performance execution on a custom Virtual Machine.

## ✨ Philosophy

- **High Readability**: Syntax inspired by SwiftUI and QML.
- **Low Learning Curve**: Easy to understand even for non-programmers.
- **Deep Interoperability**: Seamlessly calls Rust and C++ logic via a dedicated Bridge.
- **Speed**: Compiles to custom bytecode for a lightweight VM or can be transpiled directly to Rust.

## 📝 Syntax Overview

A simple component in Korlang:

```korlang
Component BrowserHeader {
    // State management
    state url: String = "https://aether.design"
    state is_loading: Bool = false

    Row {
        padding: 10
        spacing: 12

        Button(text: "←") { on_click: "nav_back" }

        // Two-way data binding with 'bind'
        TextInput(placeholder: "Search...", value: bind url) {
            on_submit: "navigate"
        }

        Button(text: "⟳") { on_click: "reload" }
    }
}
```

## 🏗 Architecture

1.  **Lexer & Parser**: Tokenizes source code and builds an Abstract Syntax Tree (AST).
2.  **Compiler**: Lowers the AST into Korlang Bytecode.
3.  **Virtual Machine (VM)**: Executes the bytecode and maintains the component tree and state heap.
4.  **Renderer**: A Rust-based backend (`src/ui/kor_renderer.rs`) that translates VM objects into visible UI elements.

## 🔗 Rust Interoperability

Korlang connects to Rust logic using the `Bridge` (`korlang/src/interop/`). Rust handlers are registered to respond to Korlang events like `on_click` or `on_submit`.

## 📂 Implementation Location

All Korlang source code resides in the `korlang/` directory of the Aether repository.
