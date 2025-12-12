# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0]

### Added

- `CONTRIBUTING.md` guide enforcing strict coding styles, environment setup, and CI workflow.
- Initial project structure for the **Redixel Engine**.
- Engine bootstrap (`main.rs`, `lib.rs`) with `redixel::init()` entry point.
- **Runtime system** implementing `winit::ApplicationHandler`, orchestrating:
  - Event processing
  - Surface creation
  - Redraw requests
- **Platform layer**:
  - `WindowManager` for window creation, lifecycle handling, and redraw requests.
  - `InputManager` for basic input event dispatch (keyboard, mouse wheel, pointer movement).
- **Graphics layer**:
  - Implemented **`RendererDevice`**, handling:
    - WGPU instance creation (`Instance`)
    - Surface creation from a `winit` window
    - Adapter selection with `HighPerformance` preference
    - Device & queue creation via `request_device`
    - Automatic surface format and present-mode selection
    - Surface configuration (`SurfaceConfiguration`) including SRGB format detection
  - Implemented **`Renderer`**, providing:
    - Clear-color rendering pipeline (basic render pass)
    - Swapchain acquisition (`get_current_texture`)
    - Command encoder creation & submission
    - Resize handling that updates surface configuration
    - Presentation of rendered frames
- **Web Assembly (WASM) Support**:
  - Enabled `wasm32-unknown-unknown` target support.
  - Integrated `wasm-bindgen` for JavaScript interoperability.
  - Added `console_error_panic_hook` for mapping Rust panics to the browser console.
  - Enabled `wgpu`'s `webgl` feature flag for broad browser compatibility.
  - Implemented DOM manipulation logic to attach the `winit` window to the HTML Canvas.
- **Engine module layout** (`engine`, `runtime`, `platform/input`, `platform/window`, `graphics/renderer`, `graphics/renderer_device`).
- CI pipeline (`.github/workflows/ci.yml`) including toolchain bootstrap, fmt, and clippy checks.
- Repository configuration files (`rust-toolchain.toml`, `rustfmt.toml`).
- **Error Handling System**:
  - Implemented a centralized `RedixelError` enum using `thiserror` to capture and contextually wrap errors from `winit`, `wgpu`, and `web-sys`.
  - Added robust error propagation across the runtime, enabling graceful shutdown on failure.
  - Integrated `log` crate with `env_logger` (Desktop) and `console_log` (WASM) for structured logging and debugging.
- **Fps Tracker and Limiting**:
  - Implemented `FpsTracker` for precise frame timing, delta-time calculation, and performance monitoring.
  - Added a high-precision **hybrid sleep/spin-lock** mechanism to enforce target framerates with minimal CPU overhead.
- **Configuration System**:
  - Implemented **`EngineSettings`** as a thread-safe global singleton (`OnceLock`, `RwLock`) enabling concurrent access from any thread.
  - Integrated `serde` and `serde_json` for robust parsing of external `config.json` files with automatic error recovery and logging.
  - Added a generic `get_path<T>` utility for querying nested settings using dot-notation strings (e.g., `"renderer.present_mode"`).
  - Implemented logic to map integer configuration values directly to `wgpu` enums (Backend, PresentMode).
  - Added `CONFIG.md` documentation comprehensively detailing the `app`, `window`, and `renderer` schemas and their default behaviors.

### Changed

- Updated `LICENSE` copyright to "Redixel Core Team".
- Updated `README.md` with professional formatting and architecture overview.
- Updated `ROADMAP.md` to reflect the current technical status of Phase 1 and next infrastructure steps.
- Refactored core initialization modules (`WindowManager::new`, `Renderer::new`, `Runtime`) to return `Result<T, RedixelError>`, eliminating fragile `unwrap()` and `expect()` calls in critical paths.
- Updated Application Entry Points:
  - **Desktop (`main`)**: Now returns `Result` and prints formatted fatal errors to `stderr` via the logging system.
  - **WASM (`init`)**: Now implements `From<RedixelError>` for `JsValue`, ensuring Rust errors are correctly mapped and displayed as exceptions in the Browser Console.
