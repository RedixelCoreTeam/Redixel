mod engine;
use engine::runtime::{Runtime, AppEvent};
use winit::event_loop::{EventLoop, ControlFlow};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop: EventLoop<AppEvent> = EventLoop::with_user_event().build().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut runtime: Runtime = Runtime::new(event_loop.create_proxy());
    let _ = event_loop.run_app(&mut runtime);
}