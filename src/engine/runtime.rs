use std::error::Error;

use winit::application::ApplicationHandler;
use winit::error::RequestError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use super::graphics::renderer::Renderer;
use super::platform::input::InputManager;
use super::platform::window::WindowManager;

#[derive(Debug)]
enum AppState {
    Error,
    Initializing,
    Running {
        renderer: Renderer,
        input_manager: InputManager,
        window_manager: WindowManager,
    },
}

#[derive(Debug)]
pub struct Runtime {
    app_state: AppState,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            app_state: AppState::Initializing,
        }
    }

    fn transition_to_running(&mut self, event_loop: &dyn ActiveEventLoop) -> Result<(), String> {
        let window_manager: WindowManager = WindowManager::new(event_loop)
            .map_err(|e: RequestError| format!("Failed to initialize Window Manager: {e}"))?;

        let renderer: Renderer = pollster::block_on(Renderer::new(window_manager.get_window()))
            .map_err(|e: Box<dyn Error + 'static>| format!("Failed to initialize Renderer: {e}"))?;

        self.app_state = AppState::Running {
            renderer,
            window_manager,
            input_manager: InputManager::new(),
        };

        Ok(())
    }
}

impl ApplicationHandler for Runtime {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        if matches!(&self.app_state, AppState::Initializing) {
            match self.transition_to_running(event_loop) {
                Ok(()) => println!("RedPixel Engine initialized successfully!"),
                Err(e) => {
                    eprintln!("RedPixel Runtime Initialization failed: {e}");
                    self.app_state = AppState::Error;
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match &mut self.app_state {
            AppState::Initializing => {}
            AppState::Error => event_loop.exit(),
            AppState::Running {
                input_manager,
                window_manager,
                renderer,
            } => match event {
                WindowEvent::RedrawRequested => {
                    let _ = renderer.render(); // TODO: Handle Surface Errors.
                    window_manager.request_redraw();
                }
                WindowEvent::CloseRequested | WindowEvent::Destroyed => event_loop.exit(),
                WindowEvent::SurfaceResized(size) => renderer.resize(size),
                event if input_manager.is_input_event(&event) => input_manager.handle_input_event(&event),
                event if window_manager.is_window_event(&event) => window_manager.handle_window_event(&event),
                _ => {}
            },
        }
    }
}
