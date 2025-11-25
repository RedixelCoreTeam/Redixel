mod engine;

use engine::plataform::window::WindowManager;
use std::error::Error;
use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let event_loop: EventLoop = EventLoop::new()?;
    let window: WindowManager = WindowManager::default();
    event_loop.run_app(window);
    Ok(())
}
