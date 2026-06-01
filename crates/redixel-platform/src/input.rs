use std::collections::HashMap;
use std::collections::HashSet;

use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;

use redixel_core::game::InputBind;
use redixel_core::game::InputQuery;
use redixel_core::input::InputAction;
use redixel_core::input::KeyState;

/// Owns all input state for the current frame and the action→key bindings.
///
/// # Lifecycle (called by the Runtime each frame)
/// ```text
/// tick()              ← advances JustPressed→Held, JustReleased→Up
/// process_event(e)    ← called for each OS window event
/// [game on_update]    ← game queries just_pressed / held / just_released
/// ```
///
/// `A` is the user's action enum. The engine is generic over it — no string
/// lookups, no dynamic dispatch on the hot path.
#[derive(Debug)]
pub struct InputManager<A: InputAction> {
    key_states: HashMap<KeyCode, KeyState>,
    bindings: HashMap<A, HashSet<KeyCode>>,
}

impl<A: InputAction> InputManager<A> {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
            bindings: HashMap::new(),
        }
    }

    /// Advances key states at the start of a new frame:
    /// - `JustPressed`  → `Held`
    /// - `JustReleased` → removed (treated as `Up`)
    ///
    /// Must be called **before** processing OS events for the frame.
    pub fn tick(&mut self) {
        self.key_states.retain(|_, state: &mut KeyState| match state {
            KeyState::JustPressed => {
                *state = KeyState::Held;
                true
            }
            KeyState::Held => true,
            KeyState::JustReleased => false,
            KeyState::Up => false,
        });
    }

    /// Processes a single OS window event. Returns `true` if the event was
    /// an input event and should not be forwarded to the window manager.
    pub fn process_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if let PhysicalKey::Code(code) = key_event.physical_key {
                    match key_event.state {
                        ElementState::Pressed => {
                            if !key_event.repeat {
                                self.key_states.insert(code, KeyState::JustPressed);
                            }
                        }
                        ElementState::Released => {
                            self.key_states.insert(code, KeyState::JustReleased);
                        }
                    }
                }
                true
            }

            WindowEvent::MouseWheel { .. } | WindowEvent::PointerMoved { .. } => true,

            _ => false,
        }
    }
}

impl<A: InputAction> Default for InputManager<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: InputAction> InputQuery<A> for InputManager<A> {
    fn just_pressed(&self, action: A) -> bool {
        self.keys_for(&action)
            .any(|k: KeyCode| self.key_state(k) == KeyState::JustPressed)
    }

    fn held(&self, action: A) -> bool {
        self.keys_for(&action).any(|k: KeyCode| self.key_state(k).is_down())
    }

    fn just_released(&self, action: A) -> bool {
        self.keys_for(&action)
            .any(|k: KeyCode| self.key_state(k) == KeyState::JustReleased)
    }

    fn key_held(&self, key: KeyCode) -> bool {
        self.key_state(key).is_down()
    }

    fn key_just_pressed(&self, key: KeyCode) -> bool {
        self.key_state(key) == KeyState::JustPressed
    }
}

impl<A: InputAction> InputBind<A> for InputManager<A> {
    fn bind(&mut self, action: A, key: KeyCode) {
        self.bindings.entry(action).or_default().insert(key);
    }

    fn unbind(&mut self, action: A) {
        self.bindings.remove(&action);
    }

    fn clear_bindings(&mut self) {
        self.bindings.clear();
    }
}

impl<A: InputAction> InputManager<A> {
    fn key_state(&self, key: KeyCode) -> KeyState {
        self.key_states.get(&key).copied().unwrap_or(KeyState::Up)
    }

    fn keys_for<'a>(&'a self, action: &A) -> impl Iterator<Item = KeyCode> + 'a {
        self.bindings
            .get(action)
            .into_iter()
            .flat_map(|set: &HashSet<KeyCode>| set.iter().copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum TestAction {
        Jump,
        Fire,
    }

    fn make_key_event(code: KeyCode, state: ElementState, repeat: bool) -> WindowEvent {
        use winit::event::KeyEvent;
        use winit::keyboard::Key;
        use winit::keyboard::KeyLocation;

        WindowEvent::KeyboardInput {
            device_id: None,
            is_synthetic: false,
            event: KeyEvent {
                physical_key: PhysicalKey::Code(code),
                logical_key: Key::Unidentified(winit::keyboard::NativeKey::Unidentified),
                text: None,
                text_with_all_modifiers: None,
                location: KeyLocation::Standard,
                state,
                repeat,
                key_without_modifiers: Key::Unidentified(winit::keyboard::NativeKey::Unidentified),
            },
        }
    }

    #[test]
    fn just_pressed_then_held() {
        let mut mgr: InputManager<TestAction> = InputManager::new();
        mgr.bind(TestAction::Jump, KeyCode::Space);

        mgr.process_event(&make_key_event(KeyCode::Space, ElementState::Pressed, false));

        assert!(mgr.just_pressed(TestAction::Jump));
        assert!(mgr.held(TestAction::Jump));

        mgr.tick();
        assert!(!mgr.just_pressed(TestAction::Jump));
        assert!(mgr.held(TestAction::Jump));

        mgr.tick();
        assert!(mgr.held(TestAction::Jump));
    }

    #[test]
    fn just_released() {
        let mut mgr: InputManager<TestAction> = InputManager::new();
        mgr.bind(TestAction::Jump, KeyCode::Space);

        mgr.process_event(&make_key_event(KeyCode::Space, ElementState::Pressed, false));
        mgr.tick();
        mgr.process_event(&make_key_event(KeyCode::Space, ElementState::Released, false));

        assert!(mgr.just_released(TestAction::Jump));
        assert!(!mgr.held(TestAction::Jump));

        mgr.tick();
        assert!(!mgr.just_released(TestAction::Jump));
    }

    #[test]
    fn key_repeat_ignored() {
        let mut mgr: InputManager<TestAction> = InputManager::new();
        mgr.bind(TestAction::Jump, KeyCode::Space);

        mgr.process_event(&make_key_event(KeyCode::Space, ElementState::Pressed, false));
        mgr.tick();

        mgr.process_event(&make_key_event(KeyCode::Space, ElementState::Pressed, true));
        assert!(!mgr.just_pressed(TestAction::Jump));
        assert!(mgr.held(TestAction::Jump));
    }

    #[test]
    fn multi_key_same_action() {
        let mut mgr: InputManager<TestAction> = InputManager::new();
        mgr.bind(TestAction::Jump, KeyCode::Space);
        mgr.bind(TestAction::Jump, KeyCode::ArrowUp);

        mgr.process_event(&make_key_event(KeyCode::ArrowUp, ElementState::Pressed, false));
        assert!(mgr.just_pressed(TestAction::Jump));
    }

    #[test]
    fn unbind_removes_action() {
        let mut mgr: InputManager<TestAction> = InputManager::new();
        mgr.bind(TestAction::Fire, KeyCode::KeyF);
        mgr.unbind(TestAction::Fire);

        mgr.process_event(&make_key_event(KeyCode::KeyF, ElementState::Pressed, false));
        assert!(!mgr.just_pressed(TestAction::Fire));
    }

    #[test]
    fn raw_key_query() {
        let mut mgr: InputManager<TestAction> = InputManager::new();
        mgr.process_event(&make_key_event(KeyCode::Escape, ElementState::Pressed, false));
        assert!(mgr.key_just_pressed(KeyCode::Escape));
        assert!(mgr.key_held(KeyCode::Escape));
    }
}
