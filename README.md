# Redixel

[![Online Playground](https://img.shields.io/badge/Online-Playground-orange?style=for-the-badge)](https://redixel-web.vercel.app/)

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

The engine adheres to a strict **Layered Architecture** divided into isolated crates within a Cargo Workspace. Dependencies flow downwards; circular dependencies between crates are strictly forbidden to ensure modularity.

### 1\. Public API Facade (`redixel`)

The outermost layer. It re-exports the essential types and traits from the internal crates via a unified `prelude`, providing a clean and ergonomic interface for the end-user.

### 2\. Runtime Layer (`redixel-runtime`)

The "Brain" of the engine. It implements the `winit::ApplicationHandler` trait.

- **Responsibility:** Orchestrates the application lifecycle (Initialization, Update Loop, Render Loop, Shutdown) and manages the `TimeManager`.
- **Behavior:** Owns the sub-systems and safely routes OS events to the platform and graphics modules.

### 3\. Graphics Layer (`redixel-renderer`)

Abstracts the GPU hardware via WGPU.

- **Renderer:** Encapsulates the WGPU Instance, Surface, Device, Queue, and Render Pipeline.
- **Capability:** Manages the swapchain, render passes, clear colors, and command encoding.

### 4\. Platform Layer (`redixel-platform`)

Abstracts Operating System specifics, ensuring the core engine remains platform-agnostic.

- **Window Manager:** Handles window creation, lifecycle events, and safe suspension/resumption natively and on WebAssembly.
- **Input Manager:** Decouples raw OS input events from game logic, sanitizing key states and pointer data.

### 5\. Core Layer (`redixel-core`)

The structural foundation of the engine framework.

- **Responsibility:** Defines the main lifecycles (`Game` trait), the shared execution context (`GameContext`), the primitive rendering command queues, and the centralized error system (`RedixelError`).
- **Dependencies:** Relies on `redixel-math` for type definitions, but remains completely agnostic of runtime, windowing, or graphics implementations.

### 6\. Mathematics Layer (`redixel-math`)

The absolute bedrock utility crate of the entire workspace.

- **Responsibility:** Implements pure, zero-dependency linear algebra operations, including vectors (`Vec2`), transformation matrices (`Mat4`), projection calculations, and color representations (`Color`).
- **Behavior:** Completely isolated from engine systems and game logic, ensuring optimal compilation times and maximum portability.

## Directory Structure

```text
redixel/
├── Cargo.toml                  # Workspace Root Configuration
├── crates/
│   ├── redixel-core/           # Base types, errors, no heavy dependencies
│   ├── redixel-math/           # Linear algebra (Vec2, Mat4, Color)
│   ├── redixel-platform/       # Winit: window, input, web-sys DOM injection
│   ├── redixel-renderer/       # Wgpu: GPU device, render pass, commands
│   ├── redixel-runtime/        # Loop, AppState, TimeManager, Settings
│   └── redixel/                # Public facade API (pub use ...)
└── examples/
    ├── pong/                   # Classic 2D game demonstrating input and physics
    ├── triangle/               # Basic 2D rendering example
    └── triangle_3d/            # Basic 3D rendering and camera example
```

## Getting Started

### Prerequisites

- Rust Toolchain (Stable)

### Running Native (Windows/Linux/macOS)

To run the engine and the included Pong game natively on your local machine:

```sh
cargo run --release --bin pong
```

> **Note:** The `--release` flag compiles the engine with maximum optimizations, which is highly recommended to ensure stable framerates. For faster compilation times during development, you can omit this flag.

### Running on Web (WASM)

Redixel uses a pure-Rust pipeline for WebAssembly, requiring no manual HTML or JS files.

1.  **Install the WASM target and server runner:**

    ```sh
    rustup target add wasm32-unknown-unknown
    cargo install wasm-server-runner
    ```

2.  **Run the example:**
    ```sh
    cargo run --target wasm32-unknown-unknown -p pong
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

The Redixel test suite is focused on validating the pure CPU logic (e.g., input handling, runtime state, math vectors, and time calculation).

### Running Unit Tests

To execute all available logic tests:

```sh
cargo test
```

## Roadmap

The project is currently transitioning to Phase 2 (The Graphics Core).
For a detailed breakdown of upcoming features, including Batch Rendering, ECS, and Physics, please refer to the [ROADMAP](./ROADMAP.md).

## Contributing

To maintain architectural integrity, we enforce strict coding standards and a specific development workflow. Please read our [CONTRIBUTING](./CONTRIBUTING.md) guide before opening a Merge Request.

## License

Redixel is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](./LICENSE) for details.

Copyright 2026 Redixel Core Team.
