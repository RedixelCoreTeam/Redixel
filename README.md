# RedPixel Engine (Work in Progress)

RedPixel is an experimental 2D game engine written in Rust.
The goal of this project is to build a clean, modular, and scalable engine architecture from scratch, and later use this engine as the foundation for developing games.

This repository is currently in an early exploration stage.

## Overview

RedPixel focuses on understanding and implementing the fundamental layers of a modern game engine.
The project aims to explore:

- platform abstraction
- window management
- input processing (planned)
- rendering architecture prepared for WGPU (planned)
- engine systems that can be reused across multiple games

This is not a finished engine, nor a game. It is a long-term learning and engineering project.

## Current Features

- Initial project structure
- Window creation using Winit
- Basic event handling (resize, redraw, close)

## Architecture Direction

The engine is being structured in layers to ensure separation of responsibilities and clarity.

### Platform Layer

Interacts directly with the operating system.
Responsibilities include:

- window creation
- event loop
- raw event processing
- surface creation for a future renderer

Currently, this is the main implemented layer.

## Directory Structure

Current structure:

```
src/
  engine/
    platform/
        window.rs
        input.rs (planned)
  main.rs
  lib.rs
```

This layout will evolve as more engine systems are introduced.

## Project Status

The engine is in a very early prototype stage.
Expect structural changes, experimentation, and reorganization as the architecture matures.

## License

(To be defined.)
