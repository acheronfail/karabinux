use crate::karabiner::FromKBKeyCode;
use evdev_rs::enums::EV_KEY;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ModifierKey {
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

impl ModifierKey {
    pub const VARIANTS: [ModifierKey; 9] = [
        ModifierKey::Capslock,
        ModifierKey::LeftAlt,
        ModifierKey::RightAlt,
        ModifierKey::LeftMeta,
        ModifierKey::RightMeta,
        ModifierKey::LeftShift,
        ModifierKey::RightShift,
        ModifierKey::LeftControl,
        ModifierKey::RightControl,
    ];

    pub fn as_key(self) -> Option<EV_KEY> {
        match self {
            ModifierKey::Capslock => Some(EV_KEY::KEY_CAPSLOCK),
            ModifierKey::LeftAlt => Some(EV_KEY::KEY_LEFTALT),
            ModifierKey::RightAlt => Some(EV_KEY::KEY_RIGHTALT),
            ModifierKey::LeftMeta => Some(EV_KEY::KEY_LEFTMETA),
            ModifierKey::RightMeta => Some(EV_KEY::KEY_RIGHTMETA),
            ModifierKey::LeftShift => Some(EV_KEY::KEY_LEFTSHIFT),
            ModifierKey::RightShift => Some(EV_KEY::KEY_RIGHTSHIFT),
            ModifierKey::LeftControl => Some(EV_KEY::KEY_LEFTCTRL),
            ModifierKey::RightControl => Some(EV_KEY::KEY_RIGHTCTRL),
        }
    }

    pub fn from_key(key: &EV_KEY) -> Option<ModifierKey> {
        match key {
            EV_KEY::KEY_CAPSLOCK => Some(ModifierKey::Capslock),
            EV_KEY::KEY_LEFTALT => Some(ModifierKey::LeftAlt),
            EV_KEY::KEY_RIGHTALT => Some(ModifierKey::RightAlt),
            EV_KEY::KEY_LEFTMETA => Some(ModifierKey::LeftMeta),
            EV_KEY::KEY_RIGHTMETA => Some(ModifierKey::RightMeta),
            EV_KEY::KEY_LEFTSHIFT => Some(ModifierKey::LeftShift),
            EV_KEY::KEY_RIGHTSHIFT => Some(ModifierKey::RightShift),
            EV_KEY::KEY_LEFTCTRL => Some(ModifierKey::LeftControl),
            EV_KEY::KEY_RIGHTCTRL => Some(ModifierKey::RightControl),
            _ => None,
        }
    }

    pub fn is_modifier(key: &EV_KEY) -> bool {
        ModifierKey::from_key(key).is_some()
    }
}

impl FromKBKeyCode for ModifierKey {
    fn from_kb_key_code(key_code: &str) -> Option<ModifierKey> {
        match key_code {
            "caps_lock" => Some(ModifierKey::Capslock),

            "left_alt" => Some(ModifierKey::LeftAlt),
            "left_option" => Some(ModifierKey::LeftAlt),
            "right_alt" => Some(ModifierKey::RightAlt),
            "right_option" => Some(ModifierKey::RightAlt),

            "left_gui" => Some(ModifierKey::LeftMeta),
            "left_command" => Some(ModifierKey::LeftMeta),
            "right_gui" => Some(ModifierKey::RightMeta),
            "right_command" => Some(ModifierKey::RightMeta),

            "left_shift" => Some(ModifierKey::LeftShift),
            "right_shift" => Some(ModifierKey::RightShift),

            "left_control" => Some(ModifierKey::LeftControl),
            "right_control" => Some(ModifierKey::RightControl),

            _ => None,
        }
    }
}
