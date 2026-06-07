# **Redixel Roadmap**

## **Phase 1 — Foundation & Lifecycle**

**Goal:** Establish the OS communication, main loop, user entry point, and platform stability.

- [x] **Project Structure:** Established a strict Cargo Workspace architecture (`redixel-core`, `platform`, `renderer`, `runtime`) enforcing unidirectional dependency flow to support future expansion.
- [x] **Window Creation:** Implemented `WindowManager` with `winit` to handle OS events and cross-platform window initialization (Native/WASM).
- [x] **Renderer Module:** Initialized the `wgpu` context (Instance, Surface, Device) and established the basic Clear Color render pass.
- [x] **Error Handling:** Unified system (`RedixelError`) and graceful propagation (Main/WASM).
- [x] **Time Management:** Delta Time calculation and FPS Counter.
- [x] **Game Loop & User API:** Implement `Game` trait (`on_update`, `on_render`) and `Context`.
- [x] **Basic Input & Window Control:** Handle Exit (ESC/Close), Fullscreen toggle, and Cleanup.
- [x] **Logging:** Standardize logs across the engine (`log` crate).
- [x] **Testing Infrastructure:** Establish unit testing patterns and ensure core logic (Time, Config) is covered by `cargo test`.
- [x] **Engine Configuration:** Startup settings struct (Window title, Resolution, VSync, Backend).

## **Phase 2 — The Graphics Core**

**Goal:** Move from "Hardcoded Triangle" to a usable, data-driven Rendering API.

- [ ] **Vertex Buffers:** Implement the logic to pass custom geometry (Vertices/Indices) from CPU to GPU.
- [x] **Math Library:** Implement a custom Linear Algebra module (Vec2, Vec3, Mat4, Orthographic Projection).
- [ ] **Shaders & Uniforms:** Pass global engine data (Time, Resolution, Camera View) to shaders via Uniform Buffers.
- [ ] **Camera System:** Implement World-to-Screen coordinate transformation.
- [ ] **Render Targets / Framebuffers:** Create intermediate textures to allow offscreen rendering, Pixel-Perfect scaling, and Post-Processing.
- [ ] **Texture Support:** Implement raw image parsing (header reading) and texture upload to GPU.

## **Phase 3 — The 2D Renderer (Batching)**

**Goal:** Efficiently draw thousands of sprites (The "Draw Call" problem).

- [ ] **Sprite Struct:** Define the data structure for visual objects (Pos, Size, Rotation, UVs).
- [ ] **Batch Renderer:** Implement a dynamic Vertex Buffer that groups multiple sprites into a single draw call to minimize GPU overhead.
- [ ] **Text & Font Renderer:** Integrate `ab_glyph` to generate font atlases and draw dynamic text via the Batch Renderer.
- [ ] **Particle System:** Implement a lightweight data structure to process and render thousands of ephemeral quads efficiently.
- [ ] **Z-Ordering:** Implement CPU-side depth sorting (Painter's Algorithm) or GPU-side depth buffering.
- [ ] **Primitive Rendering:** Implement logic to draw debug shapes (lines, wireframe rectangles) for physics visualization.

## **Phase 4 — Input & Camera Control**

**Goal:** Decouple OS events from Game Logic.

- [x] **Input Abstraction:** Create an "Action Mapping" system (bind "Jump" to "Space" or "A button").
- [ ] **Gamepad Integration:** Integrate `gilrs` to support USB/Bluetooth controllers alongside keyboard mappings.
- [ ] **Touchscreen Support:** Map touch events (especially for WASM/Mobile environments) seamlessly into the Input Manager.
- [ ] **Coordinate Conversion:** Implement math to convert Screen Coordinates (Pixels) to World Coordinates (Game Units).
- [x] **Input State Machine:** Track "Just Pressed," "Held," and "Just Released" states manually.

## **Phase 5 — ECS (Entity Component System)**

**Goal:** Define how game objects are structured and updated.

- [ ] **Component Storage:** Implement a custom storage architecture (e.g., Sparse Sets or Archetypes) for high-performance data access.
- [ ] **Entity Management:** ID generation and recycling logic.
- [ ] **System Dispatcher:** Logic to iterate over components and run update loops.
- [ ] **Transform Hierarchy:** Implement local/global matrix calculations to support Parent-Child entity relationships (e.g., a sword attached to a player's hand).
- [ ] **Game State Management:** Create a robust State Machine to handle Scene transitions (Menu -> Gameplay -> Pause -> Game Over).

## **Phase 6 — Core Systems & Resources**

**Goal:** Managing memory and assets efficiently.

- [ ] **Asset Manager:** Implement a caching system to load resources once and reference them by ID/Handle.
- [ ] **Async Asset Loading:** Implement non-blocking resource fetching (Promises/Futures) to prevent WASM thread freezing during heavy I/O.
- [ ] **Scene Management:** Define a custom file format for saving/loading level data.
- [ ] **Audio Engine:** Implement a basic audio mixer (handling buffers and mixing raw PCM data).

## **Phase 7 — Physics (2D)**

**Goal:** Movement and Collision logic.

- [ ] **AABB Collision:** Implement Axis-Aligned Bounding Box intersection math.
- [ ] **Collision Resolution:** Implement separating axis logic to prevent object overlap.
- [ ] **Raycasting & Triggers:** Implement Line-of-Sight checks and ghost/sensor collisions without physical resolution.
- [ ] **Spatial Partitioning:** Implement a Quadtree or Grid to optimize collision checks.

## **Phase 8 — Developer Tools (UI)**

**Goal:** Runtime inspection and debugging.

- [ ] **Debug UI:** Implement a simple immediate-mode text/button renderer for changing variables at runtime.
- [ ] **Hot-Reloading:** Enable updating of Textures, Shaders, and configs on-the-fly without restarting the engine.
- [ ] **Profiler:** Implement internal timers to measure function execution speed and memory usage.

## **Phase 9 — Networking (Multiplayer)**

**Goal:** Enable real-time multiplayer with a simple, data-driven API, supporting both Desktop and Web (WASM).

- [ ] **Transport Layer:** Implement an agnostic network layer. Use UDP (via a crate like `renet` or `laminar`) for fast native desktop networking, and WebRTC for WASM/Browser compatibility.
- [ ] **Headless Mode:** Modify the `Runtime` to allow the engine to initialize without `winit` or `wgpu`. This allows the exact same game code to be compiled and run on a Linux VPS as an authoritative Dedicated Server.
- [ ] **Fixed Update Loop:** Implement a fixed-timestep loop (`on_fixed_update`) in the `Runtime` to ensure physics and network ticks are deterministic and isolated from visual framerate fluctuations.
- [ ] **Network API:** Expose `ctx.network()` via the `GameContext` to allow users to easily poll connection events (Connect/Disconnect) and send byte payloads using Reliable or Unreliable channels.
- [ ] **State Serialization:** Provide basic utility traits or integrate `serde` to help developers easily compress and serialize ECS components for network transmission.
