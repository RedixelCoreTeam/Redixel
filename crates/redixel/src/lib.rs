pub use redixel_core::RedixelError;
pub use redixel_platform::{InputManager, WindowManager};
pub use redixel_renderer::{Renderer, RendererConfig};
pub use redixel_runtime::{EngineSettings, Runtime, TimeManager};

use winit::event_loop::{ControlFlow, EventLoop};

#[cfg(not(target_arch = "wasm32"))]
use winit::event_loop::run_on_demand::EventLoopExtRunOnDemand;

pub fn run() -> Result<(), RedixelError> {
    if let Err(e) = EngineSettings::global_write().load("config/config.json") {
        log::warn!("Failed to read config/config.json, using defaults. Error: {e}");
    }

    #[allow(unused_mut)]
    let mut event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    #[allow(unused_mut)]
    let mut runtime: Runtime = Runtime::new();

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
