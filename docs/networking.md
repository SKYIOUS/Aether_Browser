# Networking Module

The networking module provides functionality to fetch web resources, such as HTML content from URLs.

## Usage

```rust
use aether_browser::engine::net;

let html = net::fetch("https://example.com");
println!("{}", html);
```
