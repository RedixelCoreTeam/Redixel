use std::hash::Hash;

pub use winit::event::MouseButton;
pub use winit::keyboard::KeyCode;

/// Trait bound for user-defined action enums.
///
/// Implement this on an enum to use it as an action type with the engine's
/// input system. The derive macros below cover all requirements:
///
/// ```rust
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum Action {
///     MoveUp,
///     MoveDown,
///     Fire,
/// }
/// ```
///
/// The engine never touches your action values — it only uses them as
/// hash-map keys. Zero runtime overhead versus string-based systems.
pub trait InputAction: Hash + Eq + Clone + 'static {}

impl<T: Hash + Eq + Clone + 'static> InputAction for T {}

/// Unifies Keyboard and Mouse inputs into a single enum for the binding system.
///
/// This allows actions to be triggered seamlessly by either a key press
/// or a mouse click, without the game logic needing to differentiate between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSource {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl From<KeyCode> for InputSource {
    fn from(key: KeyCode) -> Self {
        InputSource::Key(key)
    }
}

impl From<MouseButton> for InputSource {
    fn from(button: MouseButton) -> Self {
        InputSource::Mouse(button)
    }
}

/// The logical state of a key, button, or action at a given frame boundary.
///
/// Transitions each frame:
/// ```text
/// (input down event)  → JustPressed
/// (next frame)        → Held
/// (input up event)    → JustReleased
/// (next frame)        → Up
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    JustPressed,
    Held,
    JustReleased,
    Up,
}

impl KeyState {
    /// Returns `true` if the input is currently active (`JustPressed` or `Held`).
    #[inline]
    pub fn is_down(self) -> bool {
        matches!(self, KeyState::JustPressed | KeyState::Held)
    }
}
