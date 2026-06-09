## Contributing to Redixel

To ensure your code passes our CI/CD pipeline, you must configure your local environment correctly.

### 1\. Rust Analyzer & Clippy

We strictly enforce **Clippy** lints in our Continuous Integration (CI) pipeline. If your code generates Clippy warnings, **the Pull Request will fail**.

To prevent this, you **must** configure your editor to use `clippy` as the default check command instead of `cargo check`.

**VS Code Configuration (`settings.json`):**

```json
{
  "[rust]": { "editor.defaultFormatter": "rust-lang.rust-analyzer" },
  "rust-analyzer.check.command": "clippy"
}
```

This ensures you see the same errors locally that the CI will report remotely.

## Code Style & Conventions

We enforce a strict coding style that differs slightly from standard Rust defaults, particularly regarding imports and type definitions.

### 1\. Import Ordering & Grouping

Imports must be organized in strict blocks separated by a blank line.

1.  **Module Declarations** (`mod ...`)
2.  **Standard Library** (`std::...`)
3.  **External Crates** (Alphabetical, grouped by crate, separated by newlines)
4.  **Internal/Project Imports** (`super::...`, `crate::...`)

**Example:**

```rust
mod engine;

use std::error::Error;
use std::sync::Arc;

use wgpu::Adapter;
use wgpu::Device;

use winit::event::WindowEvent;
use winit::window::Window;

use redixel_core::RedixelError;
use redixel_renderer::Renderer;
```

### 2\. Type Definitions & Return Types

Avoid fully qualified paths inside function signatures or variable declarations. Always import the type first. This keeps signatures clean and readable.

**Incorrect:**

```rust
// Hard to read, especially with long paths
fn get_adapter() -> wgpu::util::backend::Adapter { ... }
```

**Correct:**

```rust
use wgpu::Instance;

fn create_instance() -> Instance { ... }

fn create_instance() -> wgpu::Instance { ... }
```

### 3\. Explicit Typing

While Rust has type inference, we encourage explicit type annotations for variable declarations when initializing complex structs or engine subsystems. This makes the code self-documenting.

**Example:**

```rust
// Clear intent
let instance: Instance = Instance::new(...);
let surface: Surface = instance.create_surface(...);
```

### 4\. Documentation & Comments

- **Self-Documenting Code:** Prioritize clear naming for variables and functions over comments.
- **Context over Triviality:** Do not comment on trivial logic. Use comments to explain **WHY** a specific approach was taken, especially for complex math, unsafe blocks, or platform-specific hacks.

**Incorrect:**

```rust
// Creates a new instance
let instance = Instance::new();
```

**Correct:**

```rust
// Winit's Windows backend explicitly checks thread identity.
// We use `window_handle_any_thread` to bypass this guard safely.
let surface = unsafe { ... };
```

## Development Workflow

### 1\. Protected Branches

You are **strictly forbidden** from pushing directly to the following branches:

- `main`
- `develop`
- `dev-*` (any branch starting with `dev-`)

All changes must go through a Pull Request (PR).

### 2\. Testing

- **Requirement:** Every new feature or bug fix must include relevant tests.
- **Execution:** Ensure all unit tests pass locally by running `cargo test` before opening an PR.
- **Integration:** If your feature affects the rendering pipeline, ensure it does not break existing examples.

### 3\. Changelog

- **Requirement:** You must update the `CHANGELOG.md` file for every substantive change.
- Add a line describing your change under the section corresponding to the **current relevant version**.

### 4\. Pull Request (PR) Process

1.  Push your feature branch to the repository.
2.  Open a Pull Request targeting the relevant version branch (e.g., `dev-0.1.0`, `dev-0.2.1`).
3.  **CI Pipeline:** Wait for the Continuous Integration pipeline to pass. This checks:
    - Compilation (`cargo check`)
    - Formatting (`cargo fmt --check`)
    - Linter (`cargo clippy` - no warnings allowed)
    - Tests (`cargo test`)
4.  **Code Review:** Request a review from a core team member. You cannot merge your own code.
5.  Once approved and the pipeline is green, the PR can be merged.

## License

By contributing to Redixel, you agree that your contributions will be licensed under the **Apache License 2.0**, as defined in the `LICENSE` file.
