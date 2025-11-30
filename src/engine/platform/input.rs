use winit::event::WindowEvent;

#[derive(Debug, Default)]
pub struct InputManager;
impl InputManager {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_input_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::PointerMoved { .. } => {}
            WindowEvent::KeyboardInput { .. } => {}
            _ => {}
        }
    }

    pub fn is_input_event(&self, event: &WindowEvent) -> bool {
        matches!(
            event,
            WindowEvent::KeyboardInput { .. } | WindowEvent::MouseWheel { .. } | WindowEvent::PointerMoved { .. }
        )
    }
}
