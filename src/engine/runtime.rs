use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use super::graphics::renderer::Renderer;
use super::platform::input::InputManager;
use super::platform::window::WindowManager;

#[derive(Debug, Default)]
pub struct Runtime {
    window_manager: WindowManager,
    input_manager: InputManager,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for Runtime {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        if let Err(e) = self.window_manager.create_window(event_loop) {
            eprintln!("Critical: Failed to create window: {}", e);
            event_loop.exit();
        }

        if let Some(window) = self.window_manager.get_window() {
            // Block on async initialization (simplest for now)
            let renderer: Renderer = pollster::block_on(Renderer::new(window));
            self.renderer = Some(renderer);
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _i: WindowId, event: WindowEvent) {
        self.window_manager.event_handler(event_loop, &event);
        self.input_manager.event_handler(&event);

        if event == WindowEvent::RedrawRequested {
            self.window_manager.request_redraw();
        }

        match event {
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            // Reconfigure if lost
                            // renderer.resize(renderer.size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                self.window_manager.request_redraw();
            }

            WindowEvent::SurfaceResized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
            }

            _ => {}
        }
    }
}
