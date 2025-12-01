mod engine;

use winit::error::EventLoopError;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use engine::runtime::Runtime;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn init() -> Result<(), EventLoopError> {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    let event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(Runtime::new())?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).expect("Failed to initialize wasm logger!");
    if let Err(e) = init() {
        eprintln!("RedPixel Engine initialization error: {e}");
        std::process::exit(1);
    }
}
