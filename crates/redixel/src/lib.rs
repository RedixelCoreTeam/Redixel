use std::sync::RwLockReadGuard;

use winit::event_loop::{ControlFlow, EventLoop};

use redixel_core::{Game, RedixelError};
use redixel_platform::window::WindowConfig;
use redixel_renderer::RendererConfig;
use redixel_runtime::{EngineSettings, RawBackend, RawPresentMode, Runtime, RuntimeConfig};

pub mod prelude {
    pub use redixel_core::{Game, GameContext, InputSource, KeyCode, KeyState, MouseButton, RedixelError};
    pub use redixel_math::{Color, Mat4, Vec2};
}

fn build_config() -> RuntimeConfig {
    if let Err(e) = EngineSettings::global_write().load("config/config.json") {
        log::warn!("Failed to read config/config.json, using defaults. Error: {e}");
    }

    let settings: RwLockReadGuard<EngineSettings> = EngineSettings::global_read();

    RuntimeConfig {
        target_fps: settings.get_path("window.target_fps", 60.0),
        window: WindowConfig {
            width: settings.get_path("window.width", 1280),
            height: settings.get_path("window.height", 720),
            fullscreen: settings.get_path("window.fullscreen", false),
            title: settings.get_path("app.name", String::from("Redixel")),
        },
        renderer: RendererConfig {
            backends: settings.get_path("renderer.backend", RawBackend(0)).into(),
            present_mode: settings.get_path("renderer.present_mode", RawPresentMode(0)).into(),
        },
    }
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
pub fn run_desktop<G: Game>(game: G) -> Result<(), RedixelError> {
    use winit::event_loop::run_on_demand::EventLoopExtRunOnDemand;

    let mut event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut runtime: Runtime<G> = Runtime::new(game, build_config());
    event_loop.run_app_on_demand(&mut runtime)?;

    if let Some(e) = runtime.take_error() {
        return Err(e);
    }

    Ok(())
}

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
pub fn run_android<G: Game>(game: G, app: AndroidApp) -> Result<(), RedixelError> {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let event_loop: EventLoop = EventLoop::builder().with_android_app(app).build()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let runtime_ptr: *mut Runtime<G> = Box::into_raw(Box::new(Runtime::new(game, build_config())));
    let runtime_ref: &'static mut Runtime<G> = unsafe { &mut *runtime_ptr };
    event_loop.run_app(runtime_ref)?;

    let mut owned_runtime: Box<Runtime<G>> = unsafe { Box::from_raw(runtime_ptr) };
    if let Some(e) = owned_runtime.take_error() {
        return Err(e);
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn run_wasm<G: Game + 'static>(game: G) -> Result<(), RedixelError> {
    let event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let runtime: Runtime<G> = Runtime::new(game, build_config());
    event_loop.run_app(runtime)?;

    Ok(())
}
