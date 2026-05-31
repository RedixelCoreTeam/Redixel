use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use redixel_platform::window::WindowConfig;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use wgpu::SurfaceError;

use redixel_core::Game;
use redixel_core::RedixelError;
use redixel_core::game::GameContext;
use redixel_platform::InputManager;
use redixel_platform::WindowManager;
use redixel_renderer::Renderer;
use redixel_renderer::RendererConfig;

use crate::context::Context;
use crate::settings::EngineSettings;
use crate::settings::RawBackend;
use crate::settings::RawPresentMode;
use crate::time::TimeManager;

type BridgePayload = Result<(Renderer, WindowManager), RedixelError>;

#[derive(Debug)]
enum AppState<G: Game> {
    Initializing,
    Loading,
    Running {
        renderer: Box<Renderer>,
        input: InputManager,
        window: WindowManager,
        time: TimeManager,
        context: Context,
        game: G,
    },
}

/// Implements [`ApplicationHandler`] and drives the engine from creation to
/// shutdown. Owns the application state and the async initialisation bridge.
pub struct Runtime<G: Game> {
    state: AppState<G>,
    pending_game: Option<G>,
    fatal_error: Option<RedixelError>,
    bridge_tx: Sender<BridgePayload>,
    bridge_rx: Receiver<BridgePayload>,
}

impl<G: Game> Runtime<G> {
    pub fn new(game: G) -> Self {
        let (bridge_tx, bridge_rx) = mpsc::channel();
        Self {
            state: AppState::Initializing,
            pending_game: Some(game),
            fatal_error: None,
            bridge_tx,
            bridge_rx,
        }
    }

    /// Moves the stored fatal error out of the runtime.
    /// Called by `redixel::run()` after `run_app` returns.
    pub fn take_error(&mut self) -> Option<RedixelError> {
        self.fatal_error.take()
    }

    fn abort(&mut self, event_loop: &dyn ActiveEventLoop, error: RedixelError) {
        log::error!("Fatal error: {error}");
        self.fatal_error = Some(error);
        event_loop.exit();
    }

    fn renderer_config() -> RendererConfig {
        let settings: RwLockReadGuard<'_, EngineSettings> = EngineSettings::global_read();
        RendererConfig {
            backends: settings.get_path("renderer.backend", RawBackend(0)).into(),
            present_mode: settings.get_path("renderer.present_mode", RawPresentMode(0)).into(),
        }
    }

    fn spawn_gpu_init(&self, event_loop: &dyn ActiveEventLoop, window_mgr: WindowManager) {
        let tx: Sender<BridgePayload> = self.bridge_tx.clone();
        let window: Arc<dyn Window> = window_mgr.window_arc();
        let proxy: EventLoopProxy = event_loop.create_proxy();
        let config: RendererConfig = Self::renderer_config();

        let task = async move {
            let result: Result<(Renderer, WindowManager), RedixelError> =
                Renderer::new(window, config).await.map(|r: Renderer| (r, window_mgr));

            tx.send(result).ok();
            proxy.wake_up();
        };

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(task);

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(task));
    }

    fn on_can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        log::info!("[1/3] Creating window…");

        let window_config: WindowConfig = {
            let settings: RwLockReadGuard<'_, EngineSettings> = EngineSettings::global_read();

            WindowConfig {
                title: settings.get_path("app.name", String::from("Redixel")),
                width: settings.get_path("window.width", 1280),
                height: settings.get_path("window.height", 720),
                fullscreen: settings.get_path("window.fullscreen", false),
            }
        };

        match WindowManager::new(event_loop, &window_config) {
            Ok(wm) => {
                self.spawn_gpu_init(event_loop, wm);
                self.state = AppState::Loading;
            }
            Err(e) => self.abort(event_loop, e),
        }
    }

    fn on_proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        let payload = match self.bridge_rx.try_recv() {
            Ok(p) => p,
            Err(_) => return,
        };

        match payload {
            Err(e) => self.abort(event_loop, e),
            Ok((renderer, window)) => {
                window.request_redraw();

                let mut context: Context = Context::new();
                let mut time: TimeManager = TimeManager::new();
                time.set_target_fps(EngineSettings::global_read().get_path("window.target_fps", 60.0));

                let mut game: G = self
                    .pending_game
                    .take()
                    .expect("pending_game consumed before GPU was ready");

                game.on_start(&mut context);

                self.state = AppState::Running {
                    renderer: Box::new(renderer),
                    input: InputManager::new(),
                    window,
                    time,
                    context,
                    game,
                };

                log::info!("[3/3] Redixel is running.");
            }
        }
    }

    fn on_window_event(&mut self, event_loop: &dyn ActiveEventLoop, event: WindowEvent) {
        let AppState::Running {
            renderer,
            input,
            window,
            time,
            context,
            game,
        } = &mut self.state
        else {
            if matches!(self.state, AppState::Loading) {
                log::info!("[2/3] Awaiting GPU context…");
            }

            return;
        };

        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                event_loop.exit();
            }

            WindowEvent::SurfaceResized(size) => {
                renderer.resize(size);
            }

            WindowEvent::RedrawRequested => {
                time.begin_frame();
                context.update_timing(time.delta_time(), time.fps());
                game.on_update(context);

                if context.should_exit() {
                    event_loop.exit();
                    return;
                }

                match renderer.render() {
                    Ok(()) => {}

                    // Transient; skip the frame silently.
                    Err(RedixelError::Surface(SurfaceError::Timeout)) => {}

                    // The swap chain has been lost or is outdated; we must recreate it.
                    Err(RedixelError::Surface(SurfaceError::Lost | SurfaceError::Outdated)) => {
                        renderer.resize(window.surface_size());
                    }

                    Err(e) => {
                        self.fatal_error = Some(e);
                        event_loop.exit();
                        return;
                    }
                }

                game.on_render(context);
                context.reset_frame();
                time.end_frame();
                time.every_seconds(1.0, |fps: f64| window.set_title_fps(fps));
                window.request_redraw();
            }

            ref e if input.is_input_event(e) => input.handle(e),
            ref e if window.is_window_event(e) => window.handle_window_event(e),
            _ => {}
        }
    }
}

impl<G: Game> ApplicationHandler for Runtime<G> {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        if matches!(self.state, AppState::Initializing) {
            log::info!("Initializing Redixel Engine.");
            self.on_can_create_surfaces(event_loop);
        }
    }

    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        if matches!(self.state, AppState::Loading) {
            self.on_proxy_wake_up(event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.on_window_event(event_loop, event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpsc::TryRecvError;

    struct Dummy;
    impl Game for Dummy {
        fn on_start(&mut self, _ctx: &mut dyn GameContext) {}
        fn on_update(&mut self, _ctx: &mut dyn GameContext) {}
        fn on_render(&mut self, _ctx: &mut dyn GameContext) {}
    }

    #[test]
    fn initial_state_is_initializing() {
        let rt: Runtime<Dummy> = Runtime::new(Dummy);
        assert!(matches!(rt.state, AppState::Initializing));
        assert!(rt.fatal_error.is_none());
        assert!(rt.pending_game.is_some());
    }

    #[test]
    fn bridge_channel_is_open() {
        let rt: Runtime<Dummy> = Runtime::new(Dummy);
        rt.bridge_tx
            .send(Err(RedixelError::Dummy))
            .expect("channel must be open at construction");
        assert!(rt.bridge_rx.try_recv().is_ok());
    }

    #[test]
    fn bridge_delivers_error_correctly() {
        let rt: Runtime<Dummy> = Runtime::new(Dummy);
        rt.bridge_tx.send(Err(RedixelError::Dummy)).unwrap();
        let received: Result<BridgePayload, TryRecvError> = rt.bridge_rx.try_recv();
        assert!(matches!(received.unwrap(), Err(RedixelError::Dummy)));
    }

    #[test]
    fn take_error_moves_and_clears() {
        let mut rt: Runtime<Dummy> = Runtime::new(Dummy);
        rt.fatal_error = Some(RedixelError::Dummy);
        assert!(matches!(rt.take_error(), Some(RedixelError::Dummy)));
        assert!(rt.fatal_error.is_none());
    }

    #[test]
    fn context_exit_flag_roundtrip() {
        let mut ctx: Context = Context::new();
        assert!(!ctx.should_exit());
        ctx.exit();
        assert!(ctx.should_exit());
        ctx.reset_frame();
        assert!(!ctx.should_exit());
    }

    #[test]
    fn context_timing_update() {
        let mut ctx: Context = Context::new();
        ctx.update_timing(0.016, 62.5);
        assert!((ctx.delta_time() - 0.016).abs() < 1e-9);
        assert!((ctx.fps() - 62.5).abs() < 1e-9);
    }
}
