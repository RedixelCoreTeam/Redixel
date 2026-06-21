use redixel_core::{
    RedixelError,
    game::{GameContext, InputBind, InputQuery},
    input::InputAction,
};
use redixel_math::{Color, Vec2};
use redixel_platform::InputManager;

/// A draw command buffered during `on_render` and flushed by the runtime.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    ClearColor(Color),
    Rect { position: Vec2, size: Vec2, color: Color },
    Triangle { p1: Vec2, p2: Vec2, p3: Vec2, color: Color },
}

/// Concrete engine context passed to [`Game`] callbacks each frame.
///
/// Implements [`GameContext`] and is passed as `&mut dyn GameContext` to keep
/// `redixel-core` free of any dependency on `redixel-runtime`.
///
/// Timing fields are updated by the runtime before each `on_update`.
/// Draw commands are accumulated during `on_render` and drained after.
pub struct Context<A: InputAction> {
    should_exit: bool,
    error: Option<RedixelError>,
    delta_time: f64,
    fps: f64,
    surface_width: u32,
    surface_height: u32,
    pub(crate) input: InputManager<A>,
    pub(crate) commands: Vec<DrawCommand>,
}

impl<A: InputAction> Context<A> {
    pub fn new() -> Self {
        Self {
            should_exit: false,
            error: None,
            delta_time: 0.0,
            fps: 0.0,
            surface_width: 0,
            surface_height: 0,
            input: InputManager::new(),
            commands: Vec::with_capacity(1024),
        }
    }

    /// Updates per-frame timing values. Called before `on_update`.
    pub(crate) fn update_timing(&mut self, delta_time: f64, fps: f64) {
        self.delta_time = delta_time;
        self.fps = fps;
    }

    /// Updates the surface dimensions. Called on resize and after init.
    pub(crate) fn update_state(&mut self, width: u32, height: u32) {
        self.surface_width = width;
        self.surface_height = height;
    }

    /// Advances input state machine. Called at the start of every frame,
    /// before OS events are processed.
    pub(crate) fn tick_input(&mut self) {
        self.input.tick();
    }

    /// Feeds a raw OS event into the input manager.
    pub(crate) fn process_input_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.input.process_event(event)
    }

    /// Drains queued draw commands for the renderer to consume.
    pub(crate) fn drain_commands(&mut self) -> impl Iterator<Item = DrawCommand> + '_ {
        self.commands.drain(..)
    }

    /// Resets transient per-frame flags. Called after the renderer flushes.
    pub(crate) fn reset_frame(&mut self) {
        self.should_exit = false;
        self.commands.clear();
    }
}

impl<A: InputAction> Default for Context<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: InputAction> GameContext<A> for Context<A> {
    fn exit(&mut self) {
        self.should_exit = true;
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn delta_time(&self) -> f64 {
        self.delta_time
    }

    fn fps(&self) -> f64 {
        self.fps
    }

    fn surface_width(&self) -> u32 {
        self.surface_width
    }

    fn surface_height(&self) -> u32 {
        self.surface_height
    }

    fn input(&self) -> &dyn InputQuery<A> {
        &self.input
    }

    fn input_mut(&mut self) -> &mut dyn InputBind<A> {
        &mut self.input
    }

    fn clear_color(&mut self, color: Color) {
        self.commands
            .retain(|c: &DrawCommand| !matches!(c, DrawCommand::ClearColor(..)));
        self.commands.push(DrawCommand::ClearColor(color));
    }

    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        self.commands.push(DrawCommand::Triangle { p1, p2, p3, color });
    }

    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.commands.push(DrawCommand::Rect { position, size, color });
    }

    fn take_error(&mut self) -> Option<RedixelError> {
        self.error.take()
    }
}
