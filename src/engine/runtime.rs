use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::WindowId;

use super::graphics::renderer::Renderer;
use super::platform::input::InputManager;
use super::platform::window::WindowManager;

#[derive(Debug)]
pub enum AppEvent {
    RendererCreated(Renderer),
}

#[derive(Debug)]
enum AppState {
    Initializing,
    // Window created, waiting for GPU (WASM specific mostly)
    WaitingForRenderer {
        window_manager: WindowManager,
        input_manager: InputManager,
    },
    // Fully running
    Running {
        renderer: Renderer,
        window_manager: WindowManager,
        input_manager: InputManager,
    },
    Error,
}

pub struct Runtime {
    app_state: AppState,
    proxy: EventLoopProxy<AppEvent>,
}

impl Runtime {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            app_state: AppState::Initializing,
            proxy,
        }
    }

    fn init_renderer(&self, window: Arc<winit::window::Window>) {
        let proxy: EventLoopProxy<AppEvent> = self.proxy.clone();

        // WASM: Non-blocking promise
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Initializing Renderer (Async)...");
                let renderer = (Renderer::new(window).await).expect("Failed to create renderer");
                if let Err(_) = proxy.send_event(AppEvent::RendererCreated(renderer)) {
                    log::error!("Failed to send renderer event - EventLoop closed?");
                }
            });
        }

        // Blocking
        #[cfg(not(target_arch = "wasm32"))]
        {
            let renderer = pollster::block_on(Renderer::new(window)).expect("Failed to create renderer");
            let _ = proxy.send_event(AppEvent::RendererCreated(renderer));
        }
    }
}

impl ApplicationHandler<AppEvent> for Runtime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if matches!(self.app_state, AppState::Initializing) {
            match WindowManager::new(event_loop) {
                Ok(window_manager) => {
                    log::info!("Window created successfully.");

                    // Start Async Renderer Creation
                    self.init_renderer(window_manager.get_window());

                    // Move state to Waiting
                    // This waits for renderer creation
                    self.app_state = AppState::WaitingForRenderer {
                        window_manager,
                        input_manager: InputManager,
                    };
                }
                Err(e) => {
                    log::error!("Failed to create window: {}", e);
                    self.app_state = AppState::Error;
                    event_loop.exit();
                }
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::RendererCreated(renderer) => {
                log::info!("Renderer created. Transitioning to Running.");

                // If rendering creation fail self.app_state keeps AppState::Error value
                let old_state: AppState = std::mem::replace(&mut self.app_state, AppState::Error);

                match old_state {
                    AppState::WaitingForRenderer {
                        window_manager,
                        input_manager,
                    } => {
                        // Request first draw manually, i think there is a more automatic way, but, for now this is it
                        window_manager.request_redraw();

                        // Transition to Running
                        self.app_state = AppState::Running {
                            renderer,
                            window_manager,
                            input_manager,
                        };
                    }
                    state => {
                        log::error!("RendererCreated not in Waiting state: {:?}", state);
                        self.app_state = state;
                    }
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if let AppState::Running {
            renderer,
            window_manager,
            input_manager,
        } = &mut self.app_state
        {
            input_manager.handle_input_event(&event);
            window_manager.handle_window_event(&event);

            match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => event_loop.exit(),

                WindowEvent::RedrawRequested => {
                    let size: winit::dpi::PhysicalSize<u32> = window_manager.get_window().inner_size();
                    if size.width > 0 && size.height > 0 {
                        renderer.resize(size);
                    }

                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(window_manager.get_window().inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => log::error!("Render error: {:?}", e),
                    }

                    window_manager.request_redraw();
                }
                _ => {}
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        match self.app_state {
            AppState::Initializing => {}
            AppState::WaitingForRenderer { .. } => {}
            AppState::Running { .. } => {
                self.app_state = AppState::Initializing;
            }
            AppState::Error => {}
        }
    }
}
