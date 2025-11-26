use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

#[derive(Debug, Default)]
pub struct WindowManager {
    window: Option<Box<dyn Window>>,
}

impl WindowManager {
    pub fn create(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes: WindowAttributes = WindowAttributes::default();

        match event_loop.create_window(window_attributes) {
            Ok(window) => {
                self.window = Some(window);
            }
            Err(err) => {
                eprintln!("Failed to create window: {err:?}");
                event_loop.exit();
            }
        }
    }

    pub fn handle_event(&mut self, event_loop: &dyn ActiveEventLoop, event: &WindowEvent) {
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
