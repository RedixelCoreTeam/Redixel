# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0]

### Added

- Initial project structure for the **RedPixel Engine**.
- Engine bootstrap (`main.rs`, `lib.rs`) with `red_pixel::init()` entry point.
- **Runtime system** implementing `winit::ApplicationHandler`, orchestrating:

  - Event processing
  - Surface creation
  - Redraw requests

- **Platform layer**:

  - `WindowManager` for window creation, lifecycle handling, and redraw requests.
  - `InputManager` for basic input event dispatch (keyboard, mouse wheel, pointer movement).

- **Graphics layer**:

* Implemented **`RendererDevice`**, handling:

  - WGPU instance creation (`Instance`)
  - Surface creation from a `winit` window
  - Adapter selection with `HighPerformance` preference
  - Device & queue creation via `request_device`
  - Automatic surface format and present-mode selection
  - Surface configuration (`SurfaceConfiguration`) including SRGB format detection

* Implemented **`Renderer`**, providing:

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
- Basic **window creation** with default attributes and fallback error handling.
- Basic **input processing** pipeline (stub handlers ready for expansion).

- CI pipeline (`.github/workflows/cy.yml`) including:

  - Rust toolchain bootstrap
  - `cargo fmt --check`
  - `cargo clippy -D warnings`

- Repository configuration files:

  - `rust-toolchain.toml` (stable toolchain, clippy, rustfmt)
  - `rustfmt.toml` (custom formatting rules)

- Documentation:

  - `README.md` with architecture overview and directory structure.
  - `ROADMAP.md` describing planned engine features across multiple phases.

- Licensing:

  - Apache License 2.0 (`LICENSE`).
