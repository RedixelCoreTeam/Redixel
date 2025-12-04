use std::error::Error;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use super::graphics::renderer::Renderer;
use super::platform::input::InputManager;
use super::platform::window::WindowManager;

#[derive(Debug)]
enum AppState {
    Initializing,
    Loading,
    Error,
    Running {
        renderer: Renderer,
        input_manager: InputManager,
        window_manager: WindowManager,
    },
}

type InitResult = Result<(Renderer, WindowManager), String>;

#[derive(Debug)]
pub struct Runtime {
    app_state: AppState,
    init_tx: Sender<InitResult>,
    init_rx: Receiver<InitResult>,
}

impl Runtime {
    pub fn new() -> Self {
        let (tx, rx): (Sender<InitResult>, Receiver<InitResult>) = channel();

        Self {
            init_rx: rx,
            init_tx: tx,
            app_state: AppState::Initializing,
        }
    }

    fn start_initialization(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_manager: WindowManager = match WindowManager::new(event_loop) {
            Ok(window_manager) => window_manager,
            Err(e) => {
                eprintln!("Failed to initialize Window Manager: {e}");
                self.app_state = AppState::Error;
                return;
            }
        };

        let window: Arc<dyn Window> = window_manager.get_window();
        let proxy: EventLoopProxy = event_loop.create_proxy();
        let sender: Sender<InitResult> = self.init_tx.clone();

        let init_future = async move {
            let result: Result<Renderer, Box<dyn Error>> = Renderer::new(window).await;

            match result {
                Ok(renderer) => {
                    let _ = sender.send(Ok((renderer, window_manager)));
                }
                Err(e) => {
                    let _ = sender.send(Err(format!("Failed to initialize Renderer: {e}")));
                }
            }

            proxy.wake_up();
        };

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(init_future);

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(init_future));

        self.app_state = AppState::Loading;
    }

    fn complete_initialization(&mut self) {
        if let Ok(result) = self.init_rx.try_recv() {
            match result {
                Ok((renderer, window_manager)) => {
                    // Request first draw manually, important to force open the window.
                    // There could be a more automatic way, but, for now this is it.
                    window_manager.request_redraw();

                    self.app_state = AppState::Running {
                        renderer,
                        window_manager,
                        input_manager: InputManager::new(),
                    };

                    println!("Redixel initialized successfully!");
                }
                Err(e) => {
                    eprintln!("Redixel Runtime Initialization failed: {e}");
                    self.app_state = AppState::Error;
                }
            }
        }
    }
}

impl ApplicationHandler for Runtime {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        if matches!(self.app_state, AppState::Initializing) {
            self.start_initialization(event_loop);
        }
    }

    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        if matches!(self.app_state, AppState::Loading) {
            self.complete_initialization();

            if let AppState::Error = self.app_state {
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match &mut self.app_state {
            AppState::Loading => {}
            AppState::Initializing => {}
            AppState::Error => event_loop.exit(),
            AppState::Running {
                input_manager,
                window_manager,
                renderer,
            } => match event {
                WindowEvent::RedrawRequested => {
                    match renderer.render() {
                        Ok(_) => {}
                        // A timeout is usually transient (e.g., frame took too long). 
                        //Just silently skip the frame and it is ok.
                        Err(wgpu::SurfaceError::Timeout) => {},
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => renderer.resize(window_manager.get_window().surface_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => log::error!("Render error: {:?}", e),
                    }

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
