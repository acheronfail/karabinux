/// An enum that helps to interpret the `value` field of an `InputEvent` struct.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum KeyState {
    /// The key was pressed
    Pressed,
    /// The key was released
    Released,
    /// The key fired an autorepeat event
    Autorepeat,
    /// The key event has an unknown value
    Unknown(i32),
}

impl From<i32> for KeyState {
    fn from(value: i32) -> Self {
        match value {
            0 => KeyState::Released,
            1 => KeyState::Pressed,
            2 => KeyState::Autorepeat,
            value => KeyState::Unknown(value),
        }
    }
}

impl From<KeyState> for i32 {
    fn from(key_state: KeyState) -> Self {
        match key_state {
            KeyState::Released => 0,
            KeyState::Pressed => 1,
            KeyState::Autorepeat => 2,
            KeyState::Unknown(value) => value,
        }
    }
}
