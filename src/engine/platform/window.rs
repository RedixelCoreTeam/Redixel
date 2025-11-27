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
        let attributes: WindowAttributes =
            WindowAttributes::default().with_title("RedPixel Engine");

        match event_loop.create_window(attributes) {
            Ok(window) => {
                self.window = Some(window);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn event_handler(&mut self, event_loop: &dyn ActiveEventLoop, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("CloseRequested Event");
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(_) => {
                println!("SurfaceResized Event");
            }
            WindowEvent::RedrawRequested => {
                println!("RedrawRequested Event");
            }
            _ => (),
        }
    }
}
