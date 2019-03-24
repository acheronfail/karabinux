use crate::karabiner::FromKBKeyCode;
use crate::state::ModifierKey;

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
pub enum KBModifier {
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

impl KBModifier {
    // A lazy way to iterate over an enum.
    // TODO: find a better approach for this
    pub const VARIANTS: [KBModifier; 14] = [
        KBModifier::Any,
        KBModifier::Alt,
        KBModifier::Meta,
        KBModifier::Shift,
        KBModifier::Control,
        KBModifier::Capslock,
        KBModifier::LeftAlt,
        KBModifier::RightAlt,
        KBModifier::LeftMeta,
        KBModifier::RightMeta,
        KBModifier::LeftShift,
        KBModifier::RightShift,
        KBModifier::LeftControl,
        KBModifier::RightControl,
    ];

    pub fn as_modifiers(self) -> Vec<ModifierKey> {
        match self {
            KBModifier::Any => vec![],
            KBModifier::Alt => vec![ModifierKey::LeftAlt, ModifierKey::RightAlt],
            KBModifier::Meta => vec![ModifierKey::LeftMeta, ModifierKey::RightMeta],
            KBModifier::Shift => vec![ModifierKey::LeftShift, ModifierKey::RightShift],
            KBModifier::Control => vec![ModifierKey::LeftControl, ModifierKey::RightControl],
            KBModifier::Capslock => vec![ModifierKey::Capslock],
            KBModifier::LeftAlt => vec![ModifierKey::LeftAlt],
            KBModifier::RightAlt => vec![ModifierKey::RightAlt],
            KBModifier::LeftMeta => vec![ModifierKey::LeftMeta],
            KBModifier::RightMeta => vec![ModifierKey::RightMeta],
            KBModifier::LeftShift => vec![ModifierKey::LeftShift],
            KBModifier::RightShift => vec![ModifierKey::RightShift],
            KBModifier::LeftControl => vec![ModifierKey::LeftControl],
            KBModifier::RightControl => vec![ModifierKey::RightControl],
        }
    }

    pub fn from_modifier(modifier: ModifierKey) -> KBModifier {
        match modifier {
            ModifierKey::Capslock => KBModifier::Capslock,
            ModifierKey::LeftAlt => KBModifier::LeftAlt,
            ModifierKey::RightAlt => KBModifier::RightAlt,
            ModifierKey::LeftMeta => KBModifier::LeftMeta,
            ModifierKey::RightMeta => KBModifier::RightMeta,
            ModifierKey::LeftShift => KBModifier::LeftShift,
            ModifierKey::RightShift => KBModifier::RightShift,
            ModifierKey::LeftControl => KBModifier::LeftControl,
            ModifierKey::RightControl => KBModifier::RightControl,
        }
    }
}

impl FromKBKeyCode for KBModifier {
    fn from_kb_key_code(key_code: &str) -> Option<KBModifier> {
        match key_code {
            "any" => Some(KBModifier::Any),
            "shift" => Some(KBModifier::Shift),
            "option" => Some(KBModifier::Alt),
            "command" => Some(KBModifier::Meta),
            "control" => Some(KBModifier::Control),

            "caps_lock" => Some(KBModifier::Capslock),

            "left_alt" => Some(KBModifier::LeftAlt),
            "left_option" => Some(KBModifier::LeftAlt),
            "right_alt" => Some(KBModifier::RightAlt),
            "right_option" => Some(KBModifier::RightAlt),

            "left_gui" => Some(KBModifier::LeftMeta),
            "left_command" => Some(KBModifier::LeftMeta),
            "right_gui" => Some(KBModifier::RightMeta),
            "right_command" => Some(KBModifier::RightMeta),

            "left_shift" => Some(KBModifier::LeftShift),
            "right_shift" => Some(KBModifier::RightShift),

            "left_control" => Some(KBModifier::LeftControl),
            "right_control" => Some(KBModifier::RightControl),

            _ => None,
        }
    }
}
