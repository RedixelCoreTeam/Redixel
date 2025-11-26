use crate::engine::plataform::Core;
use winit::{error::EventLoopError, event_loop::EventLoop as WinitEventLoop};

#[derive(Debug)]
pub struct EventLoop {
    inner: WinitEventLoop,
}

impl EventLoop {
    pub fn new() -> Self {
        let inner: WinitEventLoop = WinitEventLoop::new().unwrap_or_else(|err: EventLoopError| {
            eprintln!("Failed to initialize event loop: {err:?}");
            std::process::exit(1);
        });

        Self { inner }
    }

    pub fn run_app(self, core: Core) {
        if let Err(err) = self.inner.run_app(core) {
            eprintln!("Failed to run the event loop: {err:?}");
            std::process::exit(1);
        }
    }
}
