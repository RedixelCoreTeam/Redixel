use winit::event::WindowEvent;

#[derive(Debug, Default)]
pub struct InputManager;

impl InputManager {
    pub fn event_handler(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {}
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {}
            WindowEvent::PointerMoved {
                device_id,
                position,
                primary,
                source,
            } => {}
            _ => {}
        }
    }
}
