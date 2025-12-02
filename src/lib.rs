mod engine;

use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use engine::runtime::Runtime;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Native logging (Desktop/Server)
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn setup_logging() {
    env_logger::init();
}

// Web logging (WASM)
#[cfg(target_arch = "wasm32")]
fn setup_logging() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Failed to initialize WASM logger");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init() {
    setup_logging();
    let event_loop: EventLoop = EventLoop::new().expect("Couldn't create EventLoop");
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(Runtime::new()).expect("Couldn't run app EventLoop");
}
