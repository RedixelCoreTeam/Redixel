use redixel_core::RedixelError;
use redixel_core::game::GameContext;
use redixel_math::{Color, Vec2};

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
#[derive(Debug, Default)]
pub struct Context {
    should_exit: bool,
    error: Option<RedixelError>,
    delta_time: f64,
    fps: f64,
    screen_size: (f32, f32),
    pub(crate) commands: Vec<DrawCommand>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn update_state(&mut self, width: u32, height: u32) {
        self.screen_size = (width as f32, height as f32);
    }

    /// Updates per-frame timing. Called by the runtime before `on_update`.
    pub(crate) fn update_timing(&mut self, delta_time: f64, fps: f64) {
        self.delta_time = delta_time;
        self.fps = fps;
    }

    /// Resets transient per-frame state. Called by the runtime after flushing.
    pub(crate) fn reset_frame(&mut self) {
        self.should_exit = false;
        self.commands.clear();
    }

    /// Drains the accumulated draw commands. Called by the runtime to flush
    /// them into the renderer.
    pub(crate) fn drain_commands(&mut self) -> impl Iterator<Item = DrawCommand> + '_ {
        self.commands.drain(..)
    }
}

impl GameContext for Context {
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

    fn screen_size(&self) -> (f32, f32) {
        self.screen_size
    }

    fn clear_color(&mut self, color: Color) {
        self.commands.push(DrawCommand::ClearColor(color));
    }

    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.commands.push(DrawCommand::Rect { position, size, color });
    }

    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        self.commands.push(DrawCommand::Triangle { p1, p2, p3, color });
    }

    fn take_error(&mut self) -> Option<RedixelError> {
        self.error.take()
    }
}
