use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::engine::plataform::{InputManager, WindowManager};

#[derive(Debug, Default)]
pub struct Core {
    window_manager: WindowManager,
    input_manager: InputManager,
}

impl Core {
    fn new() -> Self {
        let window_manager: WindowManager = WindowManager::default();
        let input_manager: InputManager = InputManager::default();

        Self {
            window_manager,
            input_manager,
        }
    }
}

impl ApplicationHandler for Core {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.window_manager = Default::default();
        self.input_manager = Default::default();
        self.window_manager.create(event_loop)
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _i: WindowId, event: WindowEvent) {
        self.window_manager.handle_event(event_loop, &event);
        self.input_manager.handle_event(&event);
    }
}
