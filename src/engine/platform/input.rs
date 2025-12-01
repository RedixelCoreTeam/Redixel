use winit::event::WindowEvent;

#[derive(Debug, Default)]
pub struct InputManager;
impl InputManager {
    pub fn handle_input_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::KeyboardInput { .. } => {}
            _ => {}
        }
    }
}
