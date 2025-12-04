# Redixel (Work in Progress)

Redixel is an experimental 2D game engine written in Rust.
The goal of this project is to build a clean, modular, and scalable engine architecture from scratch, adhering to strict software engineering standards and modern Rust best practices.

## Tech Stack

- **Language:** Rust
- **Windowing:** Winit (Event Loop & Platform Abstraction)
- **Graphics:** WGPU (WebGPU implementation for Vulkan/Metal/DX12)

## Architecture

The engine uses a strict layered architecture to separate concerns. No circular dependencies are allowed between layers.

### 1. Runtime Layer (`runtime.rs`)

The "Brain" of the engine. It implements the Winit `ApplicationHandler` trait.

- **Responsibility:** Orchestrates the lifecycle (Start, Update, Render, Stop).
- **Behavior:** Owns the sub-systems and manages the flow of data between them.

### 2. Platform Layer (`platform/`)

Abstracts the Operating System specifics.

- **Window Manager:** Handles window creation, lifecycle events, and safe suspension/resumption.
- **Input Manager:** Decouples raw Winit events from game logic (sanitizes input).

### 3. Graphics Layer (`graphics/`)

Abstracts the GPU hardware.

- **Renderer:** Encapsulates the WGPU Instance, Surface, Device, Queue, and Render Pipeline.
- **Current Capability:** Basic render pass with hardcoded geometry (Hardware check).

## Directory Structure

The project follows the "Sibling Module" pattern to keep the file tree clean.

```text
src/
├── engine/
│   ├── graphics/           # GPU interaction layer
│   │   └── renderer.rs     # WGPU Context & Render Pipeline
│   ├── platform/           # OS interaction layer
│   │   ├── input.rs        # Input event sanitization
│   │   └── window.rs       # Window lifecycle management
│   ├── graphics.rs         # Module definition for graphics
│   ├── platform.rs         # Module definition for platform
│   └── runtime.rs          # Main Application Handler (The Engine Loop)
├── engine.rs               # Engine library root
├── lib.rs                  # Library entry point (init)
└── main.rs                 # Binary entry point (bootstrapping)
```

This layout will evolve as more engine systems are introduced.

## Project Status

The engine is in a very early prototype stage.
Expect structural changes, experimentation, and reorganization as the architecture matures.

## Run

### Unix and Windows tested

```sh
cargo run
```

### Wasm

Install wasm-pack

```sh
cargo install wasm-pack
```

Build

```sh
wasm-pack build --target web
```

Run

```sh
python3 -m http.server 8000
```

## License

Redixel is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.
