# **Roadmap**

## Phase 1 — Foundation (Current Stage)

**Goal:** Establish the basic platform layer and engine skeleton.

- Define the initial project structure
- Implement window creation using Winit
- Handle fundamental system events (resize, redraw, close)
- Create a basic abstraction layer for platform-dependent functionality
- Begin documenting architecture decisions

Status: _In progress_

## Phase 2 — Input System

**Goal:** Introduce engine-level input handling independent of the platform layer.

- Design an input abstraction (keyboard, mouse, gamepad in the future)
- Translate raw Winit events into engine-friendly input states
- Implement key state tracking (pressed, held, released)
- Prepare groundwork for user-defined input mappings

Status: _Planned_

## Phase 3 — Rendering Initialization

**Goal:** Set up WGPU as the graphics backend.

- Initialize WGPU (instance, adapter, device, queue)
- Integrate WGPU surfaces with the existing window layer
- Handle surface configuration and resize events
- Implement a basic render loop
- Render a clear color for the first frame

Status: _Planned_

## Phase 4 — 2D Rendering Pipeline

**Goal:** Build a minimal but extensible 2D renderer.

- Create a sprite pipeline (vertex buffer, index buffer, shaders)
- Load and upload textures
- Implement a camera/transform system
- Draw basic textured quads
- Introduce batching to improve performance

Status: _Future_

## Phase 5 — Engine Core

**Goal:** Establish fundamental engine systems.

- Implement a time/clock system
- Build an update loop separate from the render loop
- Define engine lifecycle (init, update, render, shutdown)
- Introduce resource management (textures, shaders, etc.)
- Create a simple ECS or component-style architecture

Status: _Future_

## Phase 6 — Tools & Usability

**Goal:** Improve usability for building actual games.

- Implement logging and debugging utilities
- Asset loading helpers (images, configs, etc.)
- Scene or world loading format
- Optional hot-reload or asset watching
- Engine configuration system

Status: _Future_

## Phase 7 — 2D Advanced Features

**Goal:** Add advanced rendering techniques and polish.

- Lighting using normal maps
- Post-processing steps
- Particle system
- Tilemap renderer
- Text rendering

Status: _Future / Research_

## Phase 8 — Game Layer

**Goal:** Build a demo game to validate the engine.

- Simple 2D example game using RedPixel
- Demonstrate rendering, input, and engine flow
- Document engine usage through the demo

Status: _Future_

## Phase 9 — Stabilization

**Goal:** Prepare the project for long-term evolution.

- Review and refine architecture
- Improve modularity
- Reduce technical debt
- Extend documentation
- Define a versioning strategy and license

Status: _Future_
