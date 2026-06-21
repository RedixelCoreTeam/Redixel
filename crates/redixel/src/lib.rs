use std::sync::RwLockReadGuard;

use winit::event_loop::{ControlFlow, EventLoop};

#[cfg(not(target_arch = "wasm32"))]
use winit::event_loop::run_on_demand::EventLoopExtRunOnDemand;

use redixel_core::{Game, RedixelError};
use redixel_platform::window::WindowConfig;
use redixel_renderer::RendererConfig;
use redixel_runtime::{EngineSettings, RawBackend, RawPresentMode, Runtime, RuntimeConfig};

pub mod prelude {
    pub use redixel_core::{Game, GameContext, InputSource, KeyCode, KeyState, MouseButton, RedixelError};
    pub use redixel_math::{Color, Mat4, Vec2};
}

pub fn run<G: Game>(game: G) -> Result<(), RedixelError> {
    if let Err(e) = EngineSettings::global_write().load("config/config.json") {
        log::warn!("Failed to read config/config.json, using defaults. Error: {e}");
    }

    let config: RuntimeConfig = {
        let settings: RwLockReadGuard<EngineSettings> = EngineSettings::global_read();

        let target_fps: f64 = settings.get_path("window.target_fps", 60.0);

        let window: WindowConfig = WindowConfig {
            width: settings.get_path("window.width", 1280),
            height: settings.get_path("window.height", 720),
            fullscreen: settings.get_path("window.fullscreen", false),
            title: settings.get_path("app.name", String::from("Redixel")),
        };

        let renderer: RendererConfig = RendererConfig {
            backends: settings.get_path("renderer.backend", RawBackend(0)).into(),
            present_mode: settings.get_path("renderer.present_mode", RawPresentMode(0)).into(),
        };

        RuntimeConfig {
            window,
            renderer,
            target_fps,
        }
    };

    #[allow(unused_mut)]
    let mut event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    #[allow(unused_mut)]
    let mut runtime: Runtime<G> = Runtime::new(game, config);

    #[cfg(not(target_arch = "wasm32"))]
    {
        event_loop.run_app_on_demand(&mut runtime)?;

        if let Some(e) = runtime.take_error() {
            return Err(e);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        event_loop.run_app(runtime)?;
    }

    Ok(())
}
