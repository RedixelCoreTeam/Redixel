use redixel_math::{Color, Vec2};

use crate::RedixelError;

/// The entry point for user game logic.
///
/// Implement this trait on your own struct and pass it to [`redixel::run`].
/// The engine calls these methods in the following order each session:
///
/// ```text
/// on_start()  ← called once, after the GPU is ready
///   loop {
///     on_update()   ← game logic, physics, input reads
///     on_render()   ← issue draw calls
///   }
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use redixel::{Game, Context};
///
/// struct MyGame;
///
/// impl Game for MyGame {
///     fn on_start(&mut self, _ctx: &mut Context) {}
///     fn on_update(&mut self, _ctx: &mut Context) {}
///     fn on_render(&mut self, _ctx: &mut Context) {}
/// }
///
/// fn main() {
///     redixel::run(MyGame).unwrap();
/// }
/// ```
pub trait Game: 'static {
    /// Called once, immediately after the GPU context is ready.
    fn on_start(&mut self, ctx: &mut dyn GameContext);

    /// Called every frame before rendering.
    fn on_update(&mut self, ctx: &mut dyn GameContext);

    /// Called every frame after `on_update`.
    fn on_render(&mut self, ctx: &mut dyn GameContext);
}

/// The interface through which `Game` methods interact with the engine.
///
/// Defined as a trait so that `redixel-core` stays free of any dependency on
/// `redixel-runtime`. The concrete implementation lives in `redixel-runtime`
/// and is passed as `&mut dyn GameContext` to keep the boundary clean.
pub trait GameContext {
    /// Signals the engine to shut down cleanly after the current frame.
    fn exit(&mut self);

    /// Returns `true` if [`exit`] has been called this frame.
    fn should_exit(&self) -> bool;

    /// Returns the time in seconds between the last two frames (delta time).
    fn delta_time(&self) -> f64;

    /// Returns the current FPS measurement.
    fn fps(&self) -> f64;

    /// Returns the current dimensions of the rendering surface.
    ///
    /// The tuple contains `(width, height)` in pixels. This value updates automatically
    /// whenever the window is resized, making it essential for responsive rendering,
    /// UI alignment, and camera calculations.
    fn screen_size(&self) -> (f32, f32);

    /// Sets the background clear colour for this frame.
    fn clear_color(&mut self, color: Color);

    /// Draws a filled, axis-aligned rectangle.
    ///
    /// - `position` — top-left corner in world coordinates
    /// - `size`     — width and height in world units
    /// - `color`    — fill colour
    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color);

    /// Draws a filled triangle.
    ///
    /// - `p1`, `p2`, `p3` — The three vertices of the triangle in world coordinates
    /// - `color`          — fill colour
    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color);

    /// Returns the error that caused the engine to stop, if any.
    fn take_error(&mut self) -> Option<RedixelError>;
}
