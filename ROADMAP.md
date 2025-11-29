# **RedPixel Engine Roadmap**

## **Phase 1 — Foundation & Lifecycle**
**Goal:** Establish the OS communication and main loop.
- [x]  Project structure (Workspace/Modules).
- [x]  Window creation.
- [ ]  Event Loop (Polling, Control Flow).
- [ ]  Time Management (Delta Time, FPS Accumulator).
- [ ]  Strict Error Handling & Logging.
- [ ]  Continuous Performance Benchmarking (CI-based FPS Regression Checks). *(Optional)*

## **Phase 2 — The Graphics Core**
**Goal:** Move from "Hardcoded Triangle" to a usable, data-driven Rendering API.
- [ ]  **Vertex Buffers:** Implement the logic to pass custom geometry (Vertices/Indices) from CPU to GPU.
- [ ]  **Math Library:** Implement a custom Linear Algebra module (Vec2, Vec3, Mat4, Orthographic Projection).
- [ ]  **Shaders & Uniforms:** Pass global engine data (Time, Resolution, Camera View) to shaders via Uniform Buffers.
- [ ]  **Texture Support:** Implement raw image parsing (header reading) and texture upload to GPU.
- [ ]  **Camera System:** Implement World-to-Screen coordinate transformation.

## **Phase 3 — The 2D Renderer (Batching)**
**Goal:** Efficiently draw thousands of sprites (The "Draw Call" problem).
- [ ]  **Sprite Struct:** Define the data structure for visual objects (Pos, Size, Rotation, UVs).
- [ ]  **Batch Renderer:** Implement a dynamic Vertex Buffer that groups multiple sprites into a single draw call to minimize GPU overhead.
- [ ]  **Z-Ordering:** Implement CPU-side depth sorting (Painter's Algorithm) or GPU-side depth buffering.
- [ ]  **Primitive Rendering:** Implement logic to draw debug shapes (lines, wireframe rectangles) for physics visualization.

## **Phase 4 — Input & Camera Control**
**Goal:** Decouple OS events from Game Logic.
- [ ]  **Input Abstraction:** Create an "Action Mapping" system (bind "Jump" to "Space" or "A button").
- [ ]  **Coordinate Conversion:** Implement math to convert Screen Coordinates (Pixels) to World Coordinates (Game Units).
- [ ]  **Input State Machine:** Track "Just Pressed," "Held," and "Just Released" states manually.

## **Phase 5 — ECS (Entity Component System)**
**Goal:** Define how game objects are structured and updated.
- [ ]  **Component Storage:** Implement a custom storage architecture (e.g., Sparse Sets or Archetypes) for high-performance data access.
- [ ]  **Entity Management:** ID generation and recycling logic.
- [ ]  **System Dispatcher:** Logic to iterate over components and run update loops.

## **Phase 6 — Core Systems & Resources**
**Goal:** Managing memory and assets efficiently.
- [ ]  **Asset Manager:** Implement a caching system to load resources once and reference them by ID/Handle.
- [ ]  **Scene Management:** Define a custom file format for saving/loading level data.
- [ ]  **Audio Engine:** Implement a basic audio mixer (handling buffers and mixing raw PCM data).

## **Phase 7 — Physics (2D)**
**Goal:** Movement and Collision logic.
- [ ]  **AABB Collision:** Implement Axis-Aligned Bounding Box intersection math.
- [ ]  **Collision Resolution:** Implement separating axis logic to prevent object overlap.
- [ ]  **Spatial Partitioning:** Implement a Quadtree or Grid to optimize collision checks.

## **Phase 8 — Developer Tools (UI)**
**Goal:** Runtime inspection and debugging.
- [ ]  **Debug UI:** Implement a simple immediate-mode text/button renderer for changing variables at runtime.
- [ ]  **Profiler:** Implement internal timers to measure function execution speed and memory usage.