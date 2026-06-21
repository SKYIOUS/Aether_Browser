# Aether Browser

Aether is a modern, high-performance web browser featuring a 100% original engine written in Rust. It emphasizes "Spatial Minimalism" and modularity, using a custom-built rendering pipeline and a dedicated UI programming language.

## 🚀 Key Features

- **Original Engine**: Built from the ground up with no dependency on Blink, WebKit, or Gecko.
- **Stratus CSSOM**: Advanced CSS3 parser and resolver supporting Grid, Flexbox, and complex styling.
- **Caelum Layout**: High-fidelity layout engine optimized for modern web standards.
- **Korlang UI**: A proprietary, lightweight UI language used to script the entire browser interface with reactive data binding and seamless Rust interop.
- **Modern JS Runtime**: Powered by QuickJS with a robust DOM shim supporting Promises, Fetch, and async site logic.
- **Media Engine**: Native audio/video playback support using `symphonia`.

## 📁 Repository Structure

- `src/engine/`: Core browser engine (DOM, CSS, Layout, JS Bridge).
- `korlang/`: The Korlang UI language compiler and virtual machine.
- `src/ui/`: Browser shell and Korlang-to-Iced renderer.
- `docs/`: Technical documentation for all engine modules.

## 🛠 Getting Started

Please see [BUILD.md](BUILD.md) for installation and build instructions.
For details on the UI language, refer to [KORLANG.md](KORLANG.md).

## 📄 License

Proprietary. All rights reserved.
