use redixel_math::Color;
use redixel_math::Vec2;

use crate::RedixelError;
use crate::input::InputAction;
use crate::input::KeyCode;

/// The entry point for user game logic.
///
/// The associated type `Action` is your game's input action enum. The engine
/// is generic over it — `GameContext` exposes a typed input API with zero
/// overhead.
///
/// ```rust,ignore
/// #[derive(Clone, PartialEq, Eq, Hash)]
/// enum MyAction { MoveUp, Fire }
///
/// struct MyGame;
///
/// impl Game for MyGame {
///     type Action = MyAction;
///
///     fn on_start(&mut self, ctx: &mut dyn GameContext<MyAction>) {
///         ctx.input_mut().bind(MyAction::MoveUp, KeyCode::KeyW);
///         ctx.input_mut().bind(MyAction::Fire,   KeyCode::Space);
///     }
///
///     fn on_update(&mut self, ctx: &mut dyn GameContext<MyAction>) {
///         if ctx.input().held(MyAction::MoveUp) { /* ... */ }
///     }
///
///     fn on_render(&mut self, ctx: &mut dyn GameContext<MyAction>) {
///         ctx.draw_rect(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0), Color::WHITE);
///     }
/// }
/// ```
pub trait Game: 'static {
    /// The action enum that maps to keybinds for this game.
    ///
    /// Use `type Action = ()` if you don't need input.
    type Action: InputAction;

    /// Called once after the GPU context is ready. Bind keys and load assets here.
    fn on_start(&mut self, ctx: &mut dyn GameContext<Self::Action>);

    /// Called every frame before rendering. Update game state here.
    fn on_update(&mut self, ctx: &mut dyn GameContext<Self::Action>);

    /// Called every frame after `on_update`. Issue draw calls here.
    fn on_render(&mut self, ctx: &mut dyn GameContext<Self::Action>);
}

/// The interface through which [`Game`] methods talk to the engine each frame.
///
/// Generic over `A: InputAction` so that `input()` returns a typed view
/// without any dynamic dispatch or downcasting.
pub trait GameContext<A: InputAction> {
    /// Requests a clean engine shutdown after the current frame.
    fn exit(&mut self);

    /// Returns `true` if [`exit`] was called this frame.
    fn should_exit(&self) -> bool;

    /// Seconds elapsed between the two most recent frames (delta time).
    fn delta_time(&self) -> f64;

    /// Current FPS measurement.
    fn fps(&self) -> f64;

    /// Width of the rendering surface in pixels.
    fn surface_width(&self) -> u32;

    /// Height of the rendering surface in pixels.
    fn surface_height(&self) -> u32;

    /// Returns a read-only view of the current input state.
    ///
    /// Use this to query `just_pressed`, `held`, and `just_released`.
    fn input(&self) -> &dyn InputQuery<A>;

    /// Returns a mutable handle to bind actions to keys.
    ///
    /// Call this in `on_start` to register your keybinds.
    fn input_mut(&mut self) -> &mut dyn InputBind<A>;

    /// Sets the background clear colour for this frame.
    fn clear_color(&mut self, color: Color);

    /// Draws a filled triangle.
    ///
    /// - `p1`, `p2`, `p3` — The three vertices of the triangle in world coordinates
    /// - `color`          — fill colour
    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color);

    /// Draws a filled, axis-aligned rectangle.
    ///
    /// - `position` — top-left corner in world/screen coordinates (y-down)
    /// - `size`     — width × height in pixels
    /// - `color`    — fill colour
    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color);

    /// Extracts any pending engine error out of the context.
    fn take_error(&mut self) -> Option<RedixelError>;
}

/// Read-only input queries for the current frame.
pub trait InputQuery<A: InputAction> {
    /// Returns `true` on the exact frame the action's key went down.
    fn just_pressed(&self, action: A) -> bool;

    /// Returns `true` every frame the action's key is held down.
    fn held(&self, action: A) -> bool;

    /// Returns `true` on the exact frame the action's key came up.
    fn just_released(&self, action: A) -> bool;

    /// Returns `true` if the action is down in any capacity.
    fn is_down(&self, action: A) -> bool {
        self.just_pressed(action.clone()) || self.held(action)
    }

    /// Returns `true` if a raw `KeyCode` is currently held, bypassing bindings.
    ///
    /// Useful for debug keys or engine-level shortcuts.
    fn key_held(&self, key: KeyCode) -> bool;

    /// Returns `true` if a raw `KeyCode` was just pressed this frame.
    fn key_just_pressed(&self, key: KeyCode) -> bool;
}

/// Mutable binding configuration — call only in `on_start`.
pub trait InputBind<A: InputAction> {
    /// Binds an action to a key. Multiple keys can share the same action.
    fn bind(&mut self, action: A, key: KeyCode);

    /// Removes all bindings for `action`.
    fn unbind(&mut self, action: A);

    /// Removes all bindings entirely.
    fn clear_bindings(&mut self);
}
