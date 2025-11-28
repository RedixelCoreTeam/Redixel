use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use super::platform::input::InputManager;
use super::platform::window::WindowManager;

#[derive(Debug, Default)]
pub struct Runtime {
    window_manager: WindowManager,
    input_manager: InputManager,
}

impl ApplicationHandler for Runtime {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        if let Err(e) = self.window_manager.create_window(event_loop) {
            eprintln!("Critical: Failed to create window: {}", e);
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _i: WindowId, event: WindowEvent) {
        self.window_manager.event_handler(event_loop, &event);
        self.input_manager.event_handler(&event);

        if event == WindowEvent::RedrawRequested {
            self.window_manager.request_redraw();
        }
    }
}
