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

fn setup_logging() -> Result<(), RedixelError> {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info)?;
    }

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("redixel")).init();

    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init() -> Result<(), RedixelError> {
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
