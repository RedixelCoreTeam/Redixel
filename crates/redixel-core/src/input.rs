use std::hash::Hash;

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

/// The logical state of a key or action at a given frame boundary.
///
/// Transitions each frame:
/// ```text
/// (key down event)  → JustPressed
/// (next frame)      → Held
/// (key up event)    → JustReleased
/// (next frame)      → Up
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    JustPressed,
    Held,
    JustReleased,
    Up,
}

impl KeyState {
    #[inline]
    pub fn is_down(self) -> bool {
        matches!(self, KeyState::JustPressed | KeyState::Held)
    }
}
