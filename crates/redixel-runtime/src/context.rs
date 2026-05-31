use redixel_core::RedixelError;
use redixel_core::game::GameContext;

/// Concrete engine context passed to [`Game`] callbacks each frame.
///
/// Implements [`GameContext`] so it can be passed as `&mut dyn GameContext`
/// across the `redixel-core` boundary without creating a circular dependency.
#[derive(Debug, Default)]
pub struct Context {
    should_exit: bool,
    error: Option<RedixelError>,
    delta_time: f64,
    fps: f64,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates per-frame timing values. Called by the runtime at frame start.
    pub(crate) fn update_timing(&mut self, delta_time: f64, fps: f64) {
        self.delta_time = delta_time;
        self.fps = fps;
    }

    /// Resets transient per-frame state. Called by the runtime after callbacks.
    pub(crate) fn reset_frame(&mut self) {
        self.should_exit = false;
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

    fn take_error(&mut self) -> Option<RedixelError> {
        self.error.take()
    }
}
