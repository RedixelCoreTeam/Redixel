use std::sync::Arc;

use winit::error::RequestError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowAttributes;

#[derive(Debug)]
pub struct WindowManager {
    pub window: Arc<dyn Window>,
}

impl WindowManager {
    pub fn new(event_loop: &dyn ActiveEventLoop) -> Result<Self, RequestError> {
        let attributes: WindowAttributes = WindowAttributes::default().with_title("RedPixel Engine");
        let window: Box<dyn Window> = event_loop.create_window(attributes)?;

        Ok(Self {
            window: Arc::from(window),
        })
    }

    pub fn get_window(&self) -> Arc<dyn Window> {
        self.window.clone()
    }

    pub fn handle_window_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(..) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            _ => {}
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn is_window_event(&self, event: &WindowEvent) -> bool {
        matches!(event, WindowEvent::Focused { .. } | WindowEvent::ScaleFactorChanged { .. })
    }
}
