mod engine;

use engine::runtime::Runtime;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn init() -> Result<(), EventLoopError> {
    env_logger::init();
    let event_loop: EventLoop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let runtime: Runtime = Runtime::default();
    event_loop.run_app(runtime)?;
    Ok(())
}
