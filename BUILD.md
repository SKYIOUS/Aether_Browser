# Building Aether Browser

This document provides instructions for setting up and building the Aether Browser from source.

## 📋 Prerequisites

To build Aether, you need the following installed on your system:

- **Rust**: The stable toolchain (v1.75+). Install via [rustup.rs](https://rustup.rs).
- **CMake**: Version 3.16 or higher.
- **SDL2 & Development Libraries**: Required for the windowing shell.
- **Native TLS Support**: Standard development headers for your OS (e.g., `libssl-dev` on Linux).

## 🔨 Build Instructions

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/your-repo/aether-browser.git
    cd aether-browser
    ```

2.  **Initialize the Workspace**:
    Aether uses a Cargo workspace including the `korlang` language crate.
    ```bash
    cargo fetch
    ```

3.  **Build the Browser**:
    ```bash
    cargo build --release
    ```

4.  **Run the Browser**:
    ```bash
    cargo run --release
    ```

## 🧪 Testing

Run the full test suite (including the Korlang VM and Engine unit tests):
```bash
cargo test
```

To test specifically the Korlang language:
```bash
cargo test -p korlang
```

## 🔧 Troubleshooting

- **rmp-serde Errors**: If build fails due to `rmp` version mismatch, run:
  ```bash
  cargo update -p rmp --precise 0.8.12
  ```
- **SDL2 Not Found**: Ensure `sdl2-config` is in your PATH.
