use crate::karabiner::FromKBKeyCode;
use evdev_rs::enums::EV_KEY;

/// Karabiner has some special modifiers:
///
/// * "any": any modifier may be pressed
/// * "option": either the left or right alt keys
/// * "command": either the left or right meta keys
/// * "shift": either the left or right shift keys
/// * "control": either the left or right control keys
///
/// https://pqrs.org/osx/karabiner/json.html#from-event-definition-modifiers-list
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    Any,
    Alt,
    Meta,
    Shift,
    Control,

    Capslock,
    LeftAlt,
    RightAlt,
    LeftMeta,
    RightMeta,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
}

impl Modifier {
    pub const VARIANTS: [Modifier; 14] = [
        Modifier::Any,
        Modifier::Alt,
        Modifier::Meta,
        Modifier::Shift,
        Modifier::Control,
        Modifier::Capslock,
        Modifier::LeftAlt,
        Modifier::RightAlt,
        Modifier::LeftMeta,
        Modifier::RightMeta,
        Modifier::LeftShift,
        Modifier::RightShift,
        Modifier::LeftControl,
        Modifier::RightControl,
    ];

    pub fn from_key(key: &EV_KEY) -> Option<Modifier> {
        match key {
            EV_KEY::KEY_CAPSLOCK => Some(Modifier::Capslock),
            EV_KEY::KEY_LEFTALT => Some(Modifier::LeftAlt),
            EV_KEY::KEY_RIGHTALT => Some(Modifier::RightAlt),
            EV_KEY::KEY_LEFTMETA => Some(Modifier::LeftMeta),
            EV_KEY::KEY_RIGHTMETA => Some(Modifier::RightMeta),
            EV_KEY::KEY_LEFTSHIFT => Some(Modifier::LeftShift),
            EV_KEY::KEY_RIGHTSHIFT => Some(Modifier::RightShift),
            EV_KEY::KEY_LEFTCTRL => Some(Modifier::LeftControl),
            EV_KEY::KEY_RIGHTCTRL => Some(Modifier::RightControl),
            _ => None,
        }
    }

    pub fn as_key(self) -> Option<EV_KEY> {
        match self {
            Modifier::Capslock => Some(EV_KEY::KEY_CAPSLOCK),
            Modifier::LeftAlt => Some(EV_KEY::KEY_LEFTALT),
            Modifier::RightAlt => Some(EV_KEY::KEY_RIGHTALT),
            Modifier::LeftMeta => Some(EV_KEY::KEY_LEFTMETA),
            Modifier::RightMeta => Some(EV_KEY::KEY_RIGHTMETA),
            Modifier::LeftShift => Some(EV_KEY::KEY_LEFTSHIFT),
            Modifier::RightShift => Some(EV_KEY::KEY_RIGHTSHIFT),
            Modifier::LeftControl => Some(EV_KEY::KEY_LEFTCTRL),
            Modifier::RightControl => Some(EV_KEY::KEY_RIGHTCTRL),
            _ => None,
        }
    }
}

impl FromKBKeyCode for Modifier {
    fn from_kb_key_code(key_code: &str) -> Option<Modifier> {
        match key_code {
            "any" => Some(Modifier::Any),
            "shift" => Some(Modifier::Shift),
            "option" => Some(Modifier::Alt),
            "command" => Some(Modifier::Meta),
            "control" => Some(Modifier::Control),

            "caps_lock" => Some(Modifier::Capslock),

            "left_alt" => Some(Modifier::LeftAlt),
            "left_option" => Some(Modifier::LeftAlt),
            "right_alt" => Some(Modifier::RightAlt),
            "right_option" => Some(Modifier::RightAlt),

            "left_gui" => Some(Modifier::LeftMeta),
            "left_command" => Some(Modifier::LeftMeta),
            "right_gui" => Some(Modifier::RightMeta),
            "right_command" => Some(Modifier::RightMeta),

            "left_shift" => Some(Modifier::LeftShift),
            "right_shift" => Some(Modifier::RightShift),

            "left_control" => Some(Modifier::LeftControl),
            "right_control" => Some(Modifier::RightControl),

            _ => None,
        }
    }
}
