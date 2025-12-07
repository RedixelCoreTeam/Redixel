mod engine;

use std::cell::RefCell;
use std::rc::Rc;

use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use engine::error::RedixelError;
use engine::error::SharedError;
use engine::runtime::Runtime;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Native logging (Desktop/Server)
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn setup_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("red_pixel")).init();
}

// Web logging (WASM)
#[cfg(target_arch = "wasm32")]
fn setup_logging() -> Result<(), RedixelError> {
    console_log::init_with_level(log::Level::Info)?;
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init() -> Result<(), RedixelError> {
    #[cfg(not(target_arch = "wasm32"))]
    setup_logging();

    #[cfg(target_arch = "wasm32")]
    setup_logging()?;

    let error_sink: SharedError = Rc::new(RefCell::new(None));
    let event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(Runtime::new(error_sink.clone()))?;

    match error_sink.borrow_mut().take() {
        Some(e) => Err(e),
        None => Ok(()),
    }
}
