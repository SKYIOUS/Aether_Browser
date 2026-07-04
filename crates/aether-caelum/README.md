# aether-caelum

A between-the-lines layout engine for Rust.

Caelum implements **Flexbox**, **Grid**, and **Block** layout algorithms
for building UI frameworks, browsers, and renderers. It is the layout
layer powering the [Aether Browser](https://github.com/SKYIOUS/aether-browser).

## Quick Start

```rust
use aether_caelum::prelude::*;

let mut tree = CaelumTree::<()>::new();
let root = tree.new_leaf(Style::default()).unwrap();

tree.compute_layout(root, Size {
    width: AvailableSpace::Definite(800.0),
    height: AvailableSpace::Definite(600.0),
}).unwrap();

let layout = tree.layout(root).unwrap();
println!("root: {}x{} at ({},{})",
    layout.size.width, layout.size.height,
    layout.location.x, layout.location.y);
```

## Features

- **Flexbox** — Full flex container and item layout
- **Grid** — CSS Grid layout with named lines and areas
- **Block** — Block layout with margin collapsing
- **Standalone** — Zero external layout dependencies (only `slotmap`)

## Cargo.toml

```toml
[dependencies]
aether-caelum = "0.1"
```

## License

MIT
