use winit::event::WindowEvent;

#[derive(Debug, Default)]
pub struct InputManager;
impl InputManager {
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                println!("KeyboardInput Event");
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                println!("MouseWheel Event");
            }
            WindowEvent::PointerMoved {
                device_id,
                position,
                primary,
                source,
            } => {
                println!("PointerMoved Event");
            }
            _ => (),
        }
    }
}
