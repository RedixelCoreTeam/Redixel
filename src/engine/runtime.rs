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

use crate::engine::settings::EngineSettings;

use super::error::RedixelError;
use super::error::SharedError;
use super::graphics::renderer::Renderer;
use super::platform::input::InputManager;
use super::platform::window::WindowManager;
use super::time::TimeManager;

type BridgePayload = Result<(Renderer, WindowManager), RedixelError>;

#[derive(Debug)]
pub enum AppState {
    Loading,
    Initializing,
    Running {
        renderer: Box<Renderer>,
        input_manager: InputManager,
        window_manager: WindowManager,
        time_manager: TimeManager,
    },
}

pub struct Runtime {
    app_state: AppState,
    error_sink: SharedError,
    async_bridge_tx: Sender<BridgePayload>,
    async_bridge_rx: Receiver<BridgePayload>,
}

impl Runtime {
    pub fn new(error_sink: SharedError) -> Self {
        let channel: (Sender<BridgePayload>, Receiver<BridgePayload>) = mpsc::channel();

        Self {
            error_sink,
            async_bridge_tx: channel.0,
            async_bridge_rx: channel.1,
            app_state: AppState::Initializing,
        }
    }

    fn capture_fatal_error(error_sink: &SharedError, error: RedixelError) {
        log::error!("Fatal Error: {error}");
        *error_sink.borrow_mut() = Some(error);
    }

    fn spawn_renderer_task(&self, event_loop: &dyn ActiveEventLoop, window_manager: WindowManager) {
        let sender: Sender<BridgePayload> = self.async_bridge_tx.clone();
        let window: Arc<dyn Window> = window_manager.get_window();
        let proxy: EventLoopProxy = event_loop.create_proxy();

        let init_task = async move {
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
        wasm_bindgen_futures::spawn_local(init_task);

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(init_task));
    }

    fn start_initialization(&mut self, event_loop: &dyn ActiveEventLoop) -> Result<(), RedixelError> {
        log::info!("Step 1/3: Bootstrapping Window System...");
        let window_manager: WindowManager = WindowManager::new(event_loop)?;
        self.spawn_renderer_task(event_loop, window_manager);
        self.app_state = AppState::Loading;
        Ok(())
    }

    fn complete_initialization(&mut self) -> Result<(), RedixelError> {
        if let Ok(result) = self.async_bridge_rx.try_recv() {
            let (renderer, window_manager): (Renderer, WindowManager) = result?;

            // Request first draw manually, important to force open the window.
            // There could be a more automatic way, but, for now this is it.
            window_manager.request_redraw();

            // Initializing fps tracker
            let mut time_manager: TimeManager = TimeManager::new();
            time_manager.set_target_fps(EngineSettings::global_read().get_path("window.target_fps", 60.0));

            self.app_state = AppState::Running {
                input_manager: InputManager::new(),
                renderer: Box::new(renderer),
                window_manager,
                time_manager,
            };

            log::info!("Step 3/3: Redixel Engine is Running!");
        }

        Ok(())
    }
}

impl ApplicationHandler for Runtime {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        if !matches!(self.app_state, AppState::Initializing) {
            return;
        }

        log::info!("Initializing Redixel Engine.");

        if let Err(e) = self.start_initialization(event_loop) {
            Self::capture_fatal_error(&self.error_sink, e);
            event_loop.exit();
        }
    }

    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        if !matches!(self.app_state, AppState::Loading) {
            return;
        }

        if let Err(e) = self.complete_initialization() {
            Self::capture_fatal_error(&self.error_sink, e);
            event_loop.exit();
        };
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match &mut self.app_state {
            AppState::Initializing => {}
            AppState::Loading => log::info!("Step 2/3: Awaiting Graphics Context..."),
            AppState::Running {
                input_manager,
                window_manager,
                renderer,
                time_manager,
            } => match event {
                WindowEvent::RedrawRequested => {
                    time_manager.begin_frame();

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
                            Self::capture_fatal_error(&self.error_sink, e);
                            event_loop.exit();
                        }

                        Err(e) => {
                            Self::capture_fatal_error(&self.error_sink, e);
                            event_loop.exit();
                        }
                    };

                    time_manager.end_frame();
                    time_manager.on_fps_interval(1.0, |fps: f64| window_manager.set_title_fps(fps));
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

#[cfg(test)]
mod tests {
    use super::*;
    use mpsc::TryRecvError;
    use std::cell::Ref;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn create_test_runtime() -> (Runtime, SharedError) {
        let error_sink: Rc<RefCell<Option<RedixelError>>> = Rc::new(RefCell::new(None));
        let runtime: Runtime = Runtime::new(error_sink.clone());
        (runtime, error_sink)
    }

    #[test]
    fn test_initial_state_is_initializing() {
        let (runtime, error_sink): (Runtime, SharedError) = create_test_runtime();
        assert!(matches!(runtime.app_state, AppState::Initializing));
        assert!(error_sink.borrow().is_none());
    }

    #[test]
    fn test_fatal_error_capture() {
        let (runtime, error_sink): (Runtime, SharedError) = create_test_runtime();

        Runtime::capture_fatal_error(&runtime.error_sink, RedixelError::Dummy);
        let captured: Ref<Option<RedixelError>> = error_sink.borrow();

        assert!(captured.is_some());
        assert!(matches!(captured.as_ref().unwrap(), RedixelError::Dummy));
    }

    #[test]
    fn test_async_bridge_connectivity() {
        let (runtime, _): (Runtime, SharedError) = create_test_runtime();

        runtime
            .async_bridge_tx
            .send(Err(RedixelError::Dummy))
            .expect("The communication channel should be open");

        let received: Result<BridgePayload, TryRecvError> = runtime.async_bridge_rx.try_recv();
        assert!(received.is_ok(), "Runtime should have received the sent message");
    }

    #[test]
    fn test_complete_initialization_handles_bridge_error() {
        let (mut runtime, _): (Runtime, SharedError) = create_test_runtime();
        runtime.app_state = AppState::Loading;

        runtime.async_bridge_tx.send(Err(RedixelError::Dummy)).unwrap();
        let result: Result<(), RedixelError> = runtime.complete_initialization();

        assert!(result.is_err(), "The error from the channel should be propagated");
        assert!(matches!(result.unwrap_err(), RedixelError::Dummy));
        assert!(matches!(runtime.app_state, AppState::Loading));
    }
}
