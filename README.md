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
redixel/
├── Cargo.toml                  # Workspace Root Configuration
├── crates/
│   ├── redixel-core/           # Base types, errors, no heavy dependencies
│   ├── redixel-platform/       # Winit: window, input, web-sys DOM injection
│   ├── redixel-renderer/       # Wgpu: GPU device, render pass, commands
│   ├── redixel-runtime/        # Loop, AppState, TimeManager, Settings
│   └── redixel/                # Public facade API (pub use ...)
└── examples/
    └── hello_triangle/         # End-user application example
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

Redixel uses a pure-Rust pipeline for WebAssembly, requiring no manual HTML or JS files.

1.  **Install the WASM target and server runner:**

    ```sh
    rustup target add wasm32-unknown-unknown
    cargo install wasm-server-runner
    ```

2.  **Run the example:**
    ```sh
    cargo run --target wasm32-unknown-unknown -p triangle
    ```
    This will automatically compile, generate bindings and start a local server at `http://127.0.0.1:1334`.

### WebGPU on Linux (Chromium-based browsers)

On Linux, **WebGPU is not fully enabled by default** on Chrome, Edge, Chromium, Opera, or Brave.
As documented in the official GPUWeb Implementation Status, Linux support is **behind flags**.

To run Redixel with WebGPU enabled on Linux, launch your browser with:

```sh
microsoft-edge \
  --enable-unsafe-webgpu \
  --ozone-platform=x11 \
  --use-angle=vulkan \
  --enable-features=Vulkan,VulkanFromANGLE
```

Or for Chrome/Chromium:

```sh
chromium \
  --enable-unsafe-webgpu \
  --ozone-platform=x11 \
  --use-angle=vulkan \
  --enable-features=Vulkan,VulkanFromANGLE
```

> Reference: [WebGPU Implementation Status](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status#implementation-status)

## Testing

The Redixel test suite is focused on validating the pure CPU logic (e.g., input handling, runtime state, and time calculation).

### Running Unit Tests

To execute all available logic tests:

```sh
cargo test
```

## Roadmap

The project is currently in **Phase 1 (Foundation & Lifecycle)**.
For a detailed breakdown of upcoming features, including Batch Rendering, ECS, and Physics, please refer to the [ROADMAP](./ROADMAP.md).

## Contributing

To maintain architectural integrity, we enforce strict coding standards and a specific development workflow. Please read our [CONTRIBUTING](./CONTRIBUTING.md) guide before opening a Merge Request.

## License

Redixel is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](./LICENSE) for details.

Copyright 2025 Redixel Core Team.
