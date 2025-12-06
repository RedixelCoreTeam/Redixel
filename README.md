# Redixel

Redixel is a high-performance, strict 2D game engine written in Rust.

The primary goal of this project is to build a clean, modular, and scalable engine architecture from scratch. It enforces strict software engineering standards—such as unidirectional data flow and strong layer separation—to prevent the technical debt common in growing game engines.

## Technology Stack

Redixel is built on top of the modern Rust ecosystem, prioritizing safety and cross-platform compatibility (Desktop & Web).

| Component        | Technology  | Description                                                            |
| :--------------- | :---------- | :--------------------------------------------------------------------- |
| **Language**     | Rust (2024) | Memory safety and performance without garbage collection.              |
| **Windowing**    | Winit       | Event loop management and low-level platform abstraction.              |
| **Graphics**     | WGPU        | Portable graphics API targeting Vulkan, Metal, DX12, and WebGL/WebGPU. |
| **Build System** | Cargo       | Standard Rust package manager and build tool.                          |

## Architecture

The engine adheres to a strict **Layered Architecture**. Dependencies flow downwards; circular dependencies between layers are strictly forbidden to ensure modularity.

### 1\. Runtime Layer (`runtime.rs`)

The "Brain" of the engine. It implements the `winit::ApplicationHandler` trait.

- **Responsibility:** Orchestrates the entire application lifecycle (Initialization, Update Loop, Render Loop, Shutdown).
- **Behavior:** Owns the sub-systems and manages the flow of data between the Platform and Graphics layers.

### 2\. Platform Layer (`platform/`)

Abstracts Operating System specifics, ensuring the core engine remains platform-agnostic.

- **Window Manager:** Handles window creation, lifecycle events, DPI scaling, and safe suspension/resumption.
- **Input Manager:** Decouples raw Winit events from game logic, sanitizing input data.

### 3\. Graphics Layer (`graphics/`)

Abstracts the GPU hardware via WGPU.

- **Renderer:** Encapsulates the WGPU Instance, Surface, Device, Queue, and Render Pipeline.
- **Capability:** Manages the swapchain, render passes, and command encoding.

## Directory Structure

```text
src/
├── engine/
│   ├── graphics/               # GPU interaction layer
│   │   ├── renderer.rs         # Frame rendering & Command encoding
│   │   ├── renderer_device.rs  # WGPU Device, Queue & Surface initialization
│   │   └── mod.rs              # Graphics module definitions
│   ├── platform/               # OS interaction layer
│   │   ├── input.rs            # Input event sanitization
│   │   ├── window.rs           # Window lifecycle management
│   │   └── mod.rs              # Platform module definitions
│   ├── mod.rs                  # Engine module definitions
│   └── runtime.rs              # Main Application Handler (The Engine Loop)
├── lib.rs                      # Library entry point (init)
└── main.rs                     # Binary entry point (bootstrapping)
```

## Getting Started

### Prerequisites

- Rust Toolchain (Stable)
- Python 3 (Optional, for hosting the Web build)

### Running Native (Windows/Linux/macOS)

To run the engine on your local machine:

```sh
cargo run
```

### Running on Web (WASM)

1.  **Install wasm-pack:**

    ```sh
    cargo install wasm-pack
    ```

2.  **Build for the web target:**

    ```sh
    wasm-pack build --target web
    ```

3.  **Serve the application:**

    ```sh
    python3 -m http.server 8000
    ```

    Open your browser at `http://localhost:8000`.

## Roadmap

The project is currently in **Phase 1 (Foundation & Lifecycle)**.
For a detailed breakdown of upcoming features, including Batch Rendering, ECS, and Physics, please refer to the [ROADMAP](ROADMAP).

## Contributing

To maintain architectural integrity, we enforce strict coding standards and a specific development workflow. Please read our [CONTRIBUTING](CONTRIBUTING) guide before opening a Merge Request.

## License

Redixel is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2025 Redixel Core Team.
