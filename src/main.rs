mod engine;
use crate::engine::plataform::{Core, EventLoop};

fn main() {
    let event_loop: EventLoop = EventLoop::new();
    let core: Core = Core::default();
    event_loop.run_app(core);
}
