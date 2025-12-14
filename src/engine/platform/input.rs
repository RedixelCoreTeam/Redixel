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

#[cfg(test)]
mod tests {
    use super::*;
    use winit::dpi::PhysicalPosition;
    use winit::dpi::PhysicalSize;
    use winit::event::DeviceId;
    use winit::event::ElementState;
    use winit::event::KeyEvent;
    use winit::event::MouseScrollDelta;
    use winit::event::PointerSource;
    use winit::event::TouchPhase;
    use winit::event::WindowEvent;
    use winit::keyboard::Key;
    use winit::keyboard::KeyCode;
    use winit::keyboard::KeyLocation;
    use winit::keyboard::PhysicalKey;

    fn dummy_device_id() -> Option<DeviceId> {
        Some(DeviceId::from_raw(1))
    }

    #[test]
    fn test_input_filter_accepts_valid_inputs() {
        let manager: InputManager = InputManager::new();

        let key_event: WindowEvent = WindowEvent::KeyboardInput {
            is_synthetic: false,
            device_id: dummy_device_id(),
            event: KeyEvent {
                state: ElementState::Pressed,
                logical_key: Key::Character("a".into()),
                physical_key: PhysicalKey::Code(KeyCode::KeyA),
                location: KeyLocation::Standard,
                repeat: false,
                text: None,
                text_with_all_modifiers: None,
                key_without_modifiers: Key::Character("a".into()),
            },
        };

        assert!(manager.is_input_event(&key_event), "Should accept keyboard input");

        let mouse_move: WindowEvent = WindowEvent::PointerMoved {
            device_id: dummy_device_id(),
            position: PhysicalPosition::new(0.0, 0.0),
            primary: true,
            source: PointerSource::Mouse,
        };

        assert!(manager.is_input_event(&mouse_move), "Should accept mouse movement");

        let mouse_wheel: WindowEvent = WindowEvent::MouseWheel {
            device_id: dummy_device_id(),
            delta: MouseScrollDelta::LineDelta(1.0, 1.0),
            phase: TouchPhase::Moved,
        };

        assert!(manager.is_input_event(&mouse_wheel), "Should accept mouse scroll");
    }

    #[test]
    fn test_input_filter_rejects_non_inputs() {
        let manager: InputManager = InputManager::new();

        let resize_event: WindowEvent = WindowEvent::SurfaceResized(PhysicalSize::new(800, 600));
        let focus_event: WindowEvent = WindowEvent::Focused(true);
        let redraw_event: WindowEvent = WindowEvent::RedrawRequested;

        assert!(!manager.is_input_event(&focus_event), "Should not accept Focus as input");
        assert!(!manager.is_input_event(&resize_event), "Should not accept Resize as input");
        assert!(!manager.is_input_event(&redraw_event), "Should not accept Redraw as input");
    }
}
