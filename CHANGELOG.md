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

- **Engine module layout** (`engine`, `runtime`, `platform/input`, `platform/window`).
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
