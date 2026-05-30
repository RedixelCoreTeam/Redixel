use winit::event::WindowEvent;

/// Receives and dispatches raw input events from the OS.
///
/// Currently classifies events and provides hooks for future input-state
/// tracking (pressed keys, cursor position, etc.).
/// Add fields here as the input system grows — never accumulate state
/// inside the runtime loop.
#[derive(Debug, Default)]
pub struct InputManager;

impl InputManager {
    pub fn new() -> Self {
        Self
    }

    /// Returns `true` if `event` is an input event that this manager owns.
    pub fn is_input_event(&self, event: &WindowEvent) -> bool {
        matches!(
            event,
            WindowEvent::KeyboardInput { .. } | WindowEvent::MouseWheel { .. } | WindowEvent::PointerMoved { .. }
        )
    }

    /// Processes an input event. Only called when [`is_input_event`] is `true`.
    pub fn handle(&self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::PointerMoved { .. } => {}
            _ => {}
        }
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
    use winit::keyboard::Key;
    use winit::keyboard::KeyCode;
    use winit::keyboard::KeyLocation;
    use winit::keyboard::PhysicalKey;

    fn device_id() -> Option<DeviceId> {
        Some(DeviceId::from_raw(1))
    }

    #[test]
    fn accepts_keyboard_mouse_scroll() {
        let mgr: InputManager = InputManager::new();

        let key: WindowEvent = WindowEvent::KeyboardInput {
            is_synthetic: false,
            device_id: device_id(),
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

        assert!(mgr.is_input_event(&key));

        let motion: WindowEvent = WindowEvent::PointerMoved {
            device_id: device_id(),
            position: PhysicalPosition::new(0.0, 0.0),
            primary: true,
            source: PointerSource::Mouse,
        };

        assert!(mgr.is_input_event(&motion));

        let scroll: WindowEvent = WindowEvent::MouseWheel {
            device_id: device_id(),
            delta: MouseScrollDelta::LineDelta(1.0, 0.0),
            phase: TouchPhase::Moved,
        };

        assert!(mgr.is_input_event(&scroll));
    }

    #[test]
    fn rejects_non_input_events() {
        let mgr: InputManager = InputManager::new();

        assert!(!mgr.is_input_event(&WindowEvent::Focused(true)));
        assert!(!mgr.is_input_event(&WindowEvent::SurfaceResized(PhysicalSize::new(800, 600))));
        assert!(!mgr.is_input_event(&WindowEvent::RedrawRequested));
    }
}
