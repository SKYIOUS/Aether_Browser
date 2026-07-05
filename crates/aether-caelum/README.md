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

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `content_size` | yes | Tracks content overflow bounds per node in `LayoutOutput` |
| `debug_layout` | no | Enables `eprintln!`-based debug logging for layout computation |

## Known Limitations

- **Safe alignment** not implemented — content may overflow with non-start alignment values in flexbox/grid
- **Baseline alignment** partially implemented; last baseline and vertical text baselines not handled
- **Float layout** has known edge cases: second float at same Y pushes subsequent content down
- **Vertical writing modes** not supported
- **`visibility: collapse`** not implemented
- **Auto margins for absolute-positioned root** not supported
- **Scrollbar gutter** side always right/bottom regardless of `direction`
- **Grid track sizing** re-runs all tracks instead of only affected ones (performance)
- Table layout not implemented; `display: inline` treated as `display: block`

## Cargo.toml

```toml
[dependencies]
aether-caelum = "0.2"
```

## License

MIT
