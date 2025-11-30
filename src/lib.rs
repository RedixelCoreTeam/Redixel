mod engine;

use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

use engine::runtime::Runtime;

pub fn init() -> Result<(), EventLoopError> {
    env_logger::init();
    let event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(Runtime::new())?;
    Ok(())
}
