use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use wgpu::SurfaceError;

use super::error::RedixelError;
use super::error::SharedError;
use super::graphics::renderer::Renderer;
use super::platform::input::InputManager;
use super::platform::window::WindowManager;

#[derive(Debug)]
pub enum AppState {
    Loading,
    Initializing,
    Running {
        renderer: Renderer,
        input_manager: InputManager,
        window_manager: WindowManager,
    },
}

type RuntimeBridgeResult = Result<(Renderer, WindowManager), RedixelError>;

#[derive(Debug)]
pub struct Runtime {
    app_state: AppState,
    error_sink: SharedError,
    async_bridge_tx: Sender<RuntimeBridgeResult>,
    async_bridge_rx: Receiver<RuntimeBridgeResult>,
}

impl Runtime {
    pub fn new(error_sink: SharedError) -> Self {
        let channel: (Sender<RuntimeBridgeResult>, Receiver<RuntimeBridgeResult>) = mpsc::channel();

        Self {
            error_sink,
            async_bridge_tx: channel.0,
            async_bridge_rx: channel.1,
            app_state: AppState::Initializing,
        }
    }

    fn capture_error(error_sink: &SharedError, e: RedixelError) {
        *error_sink.borrow_mut() = Some(e)
    }

    fn start_initialization(&mut self, event_loop: &dyn ActiveEventLoop) {
        println!("Step 1/3: Bootstrapping Window System...");

        let window_manager: WindowManager = match WindowManager::new(event_loop) {
            Ok(window_manager) => window_manager,
            Err(e) => {
                Self::capture_error(&self.error_sink, e);
                event_loop.exit();
                return;
            }
        };

        let sender: Sender<RuntimeBridgeResult> = self.async_bridge_tx.clone();
        let window: Arc<dyn Window> = window_manager.get_window();
        let proxy: EventLoopProxy = event_loop.create_proxy();

        let init_future = async move {
            match Renderer::new(window).await {
                Ok(renderer) => {
                    let _ = sender.send(Ok((renderer, window_manager)));
                }
                Err(e) => {
                    let _ = sender.send(Err(e));
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

    fn complete_initialization(&mut self, event_loop: &dyn ActiveEventLoop) {
        if let Ok(result) = self.async_bridge_rx.try_recv() {
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

                    println!("Step 3/3: RedPixel Engine is Running!");
                }
                Err(e) => {
                    Self::capture_error(&self.error_sink, e);
                    event_loop.exit();
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
            self.complete_initialization(event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match &mut self.app_state {
            AppState::Initializing => {}
            AppState::Loading => println!("Step 2/3: Awaiting Graphics Context (WGPU)..."),
            AppState::Running {
                input_manager,
                window_manager,
                renderer,
            } => match event {
                WindowEvent::RedrawRequested => {
                    match renderer.render() {
                        // Frame submitted successfully; no further control flow needed.
                        Ok(_) => {}

                        // A timeout is usually transient (e.g., frame took too long).
                        // Just silently skip the frame and it is ok.
                        Err(RedixelError::Surface(SurfaceError::Timeout)) => {}

                        // The swap chain has been lost or is outdated; we must recreate it.
                        Err(RedixelError::Surface(SurfaceError::Lost | SurfaceError::Outdated)) => {
                            renderer.resize(window_manager.get_window().surface_size());
                        }

                        Err(e @ RedixelError::Surface(SurfaceError::OutOfMemory)) => {
                            Self::capture_error(&self.error_sink, e);
                            event_loop.exit();
                        }

                        Err(e) => {
                            Self::capture_error(&self.error_sink, e);
                            event_loop.exit();
                        }
                    };

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
