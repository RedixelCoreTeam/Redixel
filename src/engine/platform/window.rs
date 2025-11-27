use winit::error::RequestError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

#[derive(Debug, Default)]
pub struct WindowManager {
    window: Option<Box<dyn Window>>,
}

impl WindowManager {
    pub fn create_window(&mut self, event_loop: &dyn ActiveEventLoop) -> Result<(), RequestError> {
        if self.window.is_some() {
            return Ok(());
        }

        let attributes: WindowAttributes =
            WindowAttributes::default().with_title("RedPixel Engine");

        let window: Box<dyn Window> = event_loop.create_window(attributes)?;
        self.window = Some(window);
        Ok(())
    }

    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    pub fn event_handler(&mut self, event_loop: &dyn ActiveEventLoop, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => event_loop.exit(),
            WindowEvent::Focused(_is_focused) => {}
            WindowEvent::SurfaceResized(_size) => {}
            _ => {}
        }
    }
}
