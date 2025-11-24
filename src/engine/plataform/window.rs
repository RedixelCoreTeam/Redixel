use winit::application::ApplicationHandler;
use winit::error::RequestError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default, Debug)]
pub struct WindowHandler {
    window: Option<Box<dyn Window>>,
}

impl WindowHandler {
    fn create(&mut self, event_loop: &dyn ActiveEventLoop) -> Result<(), RequestError> {
        let window_attributes: WindowAttributes = WindowAttributes::default();
        self.window = Some(event_loop.create_window(window_attributes)?);
        Ok(())
    }

    fn handle_event(&mut self, event_loop: &dyn ActiveEventLoop, event: &WindowEvent) {
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

#[derive(Default, Debug)]
pub struct InputHandler;
impl InputHandler {
    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                println!("KeyboardInput Event");
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                println!("MouseWheel Event");
            }
            WindowEvent::PointerMoved {
                device_id,
                position,
                primary,
                source,
            } => {
                println!("PointerMoved Event");
            }
            _ => (),
        }
    }
}

#[derive(Default, Debug)]
pub struct WindowManager {
    input_handler: InputHandler,
    window_handler: WindowHandler,
}

impl ApplicationHandler for WindowManager {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.window_handler = Default::default();
        self.input_handler = Default::default();

        if let Err(err) = self.window_handler.create(event_loop) {
            eprintln!("Failed to create window: {err}");
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _i: WindowId, event: WindowEvent) {
        self.window_handler.handle_event(event_loop, &event);
        self.input_handler.handle_event(&event);
    }
}
