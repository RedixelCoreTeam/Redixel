use std::sync::{
    Arc, mpsc,
    mpsc::{Receiver, Sender},
};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

use wgpu::SurfaceError;

use redixel_core::{Game, RedixelError, game::GameContext};
use redixel_platform::{WindowManager, window::WindowConfig};
use redixel_renderer::{Renderer, RendererConfig};

use crate::{
    context::{Context, DrawCommand},
    time::TimeManager,
};

#[derive(Clone)]
pub struct RuntimeConfig {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
    pub target_fps: f64,
}

type BridgePayload = Result<(Renderer, WindowManager), RedixelError>;

struct RunningState<G: Game> {
    renderer: Renderer,
    window: WindowManager,
    time: TimeManager,
    context: Context<G::Action>,
    game: G,
}

enum AppState<G: Game> {
    Initializing,
    Loading,
    Running(Box<RunningState<G>>),
}

/// Implements [`ApplicationHandler`] and drives the engine from creation to
/// shutdown. Owns the application state and the async initialisation bridge.
pub struct Runtime<G: Game> {
    state: AppState<G>,
    pending_game: Option<G>,
    fatal_error: Option<RedixelError>,
    bridge_tx: Sender<BridgePayload>,
    bridge_rx: Receiver<BridgePayload>,
    config: RuntimeConfig,
}

impl<G: Game> Runtime<G> {
    pub fn new(game: G, config: RuntimeConfig) -> Self {
        let (bridge_tx, bridge_rx): (Sender<BridgePayload>, Receiver<BridgePayload>) = mpsc::channel();
        Self {
            state: AppState::Initializing,
            pending_game: Some(game),
            fatal_error: None,
            bridge_tx,
            bridge_rx,
            config,
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

    fn transition_to_running(&mut self, renderer: Renderer, window: WindowManager) {
        window.request_redraw();

        let mut time: TimeManager = TimeManager::new();
        time.set_target_fps(self.config.target_fps);

        let initial_size: PhysicalSize<u32> = window.surface_size();
        let mut context: Context<G::Action> = Context::new();
        context.update_state(initial_size.width, initial_size.height);

        let mut game: G = self.pending_game.take().expect("pending_game already consumed");
        game.on_start(&mut context);

        self.state = AppState::Running(Box::new(RunningState {
            renderer,
            window,
            time,
            context,
            game,
        }));

        log::info!("[3/3] Redixel is running.");
    }

    async fn init_gpu(
        tx: Sender<BridgePayload>,
        window: Arc<dyn Window>,
        window_mgr: WindowManager,
        proxy: EventLoopProxy,
        config: RendererConfig,
    ) {
        let result: Result<(Renderer, WindowManager), RedixelError> =
            Renderer::new(window, config).await.map(|r: Renderer| (r, window_mgr));

        tx.send(result).ok();
        proxy.wake_up();
    }

    fn spawn_gpu_init(&self, event_loop: &dyn ActiveEventLoop, window_mgr: WindowManager) {
        let tx: Sender<BridgePayload> = self.bridge_tx.clone();
        let window: Arc<dyn Window> = window_mgr.window_arc();
        let proxy: EventLoopProxy = event_loop.create_proxy();
        let config: RendererConfig = self.config.renderer.clone();

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(Self::init_gpu(tx, window, window_mgr, proxy, config));

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(Self::init_gpu(tx, window, window_mgr, proxy, config)));
    }

    fn on_can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        log::info!("[1/3] Creating window…");

        match WindowManager::new(event_loop, &self.config.window) {
            Ok(window) => {
                self.spawn_gpu_init(event_loop, window);
                self.state = AppState::Loading;
            }
            Err(e) => self.abort(event_loop, e),
        }
    }

    fn on_proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        let payload: Result<(Renderer, WindowManager), RedixelError> = match self.bridge_rx.try_recv() {
            Ok(p) => p,
            Err(_) => return,
        };

        match payload {
            Ok((renderer, window)) => self.transition_to_running(renderer, window),
            Err(e) => self.abort(event_loop, e),
        }
    }

    fn run_frame(&mut self, event_loop: &dyn ActiveEventLoop) {
        let AppState::Running(state) = &mut self.state else {
            return;
        };

        state.context.tick_input();
        state.time.begin_frame();

        state.context.update_timing(state.time.delta_time(), state.time.fps());
        state.game.on_update(&mut state.context);

        if state.context.should_exit() {
            event_loop.exit();
            return;
        }

        state.game.on_render(&mut state.context);

        // Flush draw commands from context into renderer
        for cmd in state.context.drain_commands() {
            match cmd {
                DrawCommand::ClearColor(c) => {
                    state.renderer.set_clear_color(c);
                }
                DrawCommand::Rect { position, size, color } => {
                    state.renderer.draw_rect(position, size, color);
                }
                DrawCommand::Triangle { p1, p2, p3, color } => {
                    state.renderer.draw_triangle(p1, p2, p3, color);
                }
            }
        }

        match state.renderer.render() {
            Ok(()) => {}
            // Transient; skip the frame silently.
            Err(RedixelError::Surface(SurfaceError::Timeout)) => {}
            // The swap chain has been lost or is outdated; we must recreate it.
            Err(RedixelError::Surface(SurfaceError::Lost | SurfaceError::Outdated)) => {
                state.renderer.resize(state.window.surface_size());
            }
            Err(e) => {
                self.fatal_error = Some(e);
                event_loop.exit();
                return;
            }
        }

        state.context.reset_frame();
        state.time.end_frame();
        state
            .time
            .every_seconds(1.0, |fps: f64| state.window.set_title_fps(fps));

        state.window.request_redraw();
    }

    fn on_window_event(&mut self, event_loop: &dyn ActiveEventLoop, event: WindowEvent) {
        let AppState::Running(state) = &mut self.state else {
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
                state.renderer.resize(size);
                state.context.update_state(size.width, size.height);
            }

            WindowEvent::RedrawRequested => {
                self.run_frame(event_loop);
            }
            ref e => {
                if state.context.process_input_event(e) {
                    return;
                }

                state.window.process_window_event(e);
            }
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

    use redixel_core::GameContext;
    use redixel_math::{Color, Vec2};

    struct Dummy;
    impl Game for Dummy {
        type Action = ();
        fn on_start(&mut self, _ctx: &mut dyn GameContext<()>) {}
        fn on_update(&mut self, _ctx: &mut dyn GameContext<()>) {}
        fn on_render(&mut self, _ctx: &mut dyn GameContext<()>) {}
    }

    fn mock_config() -> RuntimeConfig {
        RuntimeConfig {
            target_fps: 60.0,
            window: WindowConfig {
                width: 800,
                height: 600,
                fullscreen: false,
                title: String::from("TEST_TITLE"),
            },
            renderer: RendererConfig {
                backends: wgpu::Backends::all(),
                present_mode: wgpu::PresentMode::AutoVsync,
            },
        }
    }

    #[test]
    fn initial_state_is_initializing() {
        let rt: Runtime<Dummy> = Runtime::new(Dummy, mock_config());
        assert!(matches!(rt.state, AppState::Initializing));
        assert!(rt.fatal_error.is_none());
        assert!(rt.pending_game.is_some());
    }

    #[test]
    fn bridge_channel_is_open() {
        let rt: Runtime<Dummy> = Runtime::new(Dummy, mock_config());
        rt.bridge_tx
            .send(Err(RedixelError::Dummy))
            .expect("channel must be open at construction");
        assert!(rt.bridge_rx.try_recv().is_ok());
    }

    #[test]
    fn bridge_delivers_error_correctly() {
        let rt: Runtime<Dummy> = Runtime::new(Dummy, mock_config());
        rt.bridge_tx.send(Err(RedixelError::Dummy)).unwrap();
        let received: Result<BridgePayload, TryRecvError> = rt.bridge_rx.try_recv();
        assert!(matches!(received.unwrap(), Err(RedixelError::Dummy)));
    }

    #[test]
    fn take_error_moves_and_clears() {
        let mut rt: Runtime<Dummy> = Runtime::new(Dummy, mock_config());
        rt.fatal_error = Some(RedixelError::Dummy);
        assert!(matches!(rt.take_error(), Some(RedixelError::Dummy)));
        assert!(rt.fatal_error.is_none());
    }

    #[test]
    fn context_draw_commands_accumulate() {
        let mut ctx: Context<()> = Context::new();
        ctx.draw_rect(Vec2::new(0.0, 0.0), Vec2::new(100.0, 50.0), Color::RED);
        ctx.draw_rect(Vec2::new(10.0, 10.0), Vec2::new(20.0, 20.0), Color::BLUE);
        assert_eq!(ctx.commands.len(), 2);

        let drained: Vec<DrawCommand> = ctx.drain_commands().collect();
        assert_eq!(drained.len(), 2);
        assert!(ctx.commands.is_empty());
    }

    #[test]
    fn context_clear_color_deduplicates() {
        let mut ctx: Context<()> = Context::new();

        ctx.clear_color(Color::RED);
        ctx.clear_color(Color::BLUE);

        let clears: Vec<&DrawCommand> = ctx
            .commands
            .iter()
            .filter(|c: &&DrawCommand| matches!(c, DrawCommand::ClearColor(_)))
            .collect();

        assert_eq!(clears.len(), 1);
    }

    #[test]
    fn context_exit_flag_roundtrip() {
        let mut ctx: Context<()> = Context::new();
        assert!(!ctx.should_exit());
        ctx.exit();
        assert!(ctx.should_exit());
        ctx.reset_frame();
        assert!(!ctx.should_exit());
    }

    #[test]
    fn context_timing_update() {
        let mut ctx: Context<()> = Context::new();
        ctx.update_timing(0.016, 62.5);
        assert!((ctx.delta_time() - 0.016).abs() < 1e-9);
        assert!((ctx.fps() - 62.5).abs() < 1e-9);
    }
}
