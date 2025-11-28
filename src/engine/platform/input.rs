use winit::event::WindowEvent;

#[derive(Debug, Default)]
pub struct InputManager;

impl InputManager {
    pub fn event_handler(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event: _,
                is_synthetic: _,
            } => {}
            WindowEvent::MouseWheel {
                device_id: _,
                delta: _,
                phase: _,
            } => {}
            WindowEvent::PointerMoved {
                device_id: _,
                position: _,
                primary: _,
                source: _,
            } => {}
            _ => {}
        }
    }
}
