use crate::karabiner::{FromKBKeyCode, KBFromModifiers};
use crate::state::ModifierState;
use input_linux::Key;

/// If modifiers is not present: only fire when NO modifiers are pressed
/// If mandatory is present: (swallow modifiers) needs modifiers to be pressed
/// If optional is present: (passes modifiers) event matched independently of modifiers
/// See: https://pqrs.org/osx/karabiner/json.html#from-event-definition-modifiers
#[derive(Debug)]
pub enum FromModifier {
    Absent,
    Optional,
    Mandatory,
}

// TODO: support "shift", "option", "command", "control" and "any"
#[derive(Debug)]
pub struct FromModifiers {
    pub left_control: FromModifier,
    pub left_shift: FromModifier,
    pub left_alt: FromModifier,
    pub left_meta: FromModifier,
    pub right_control: FromModifier,
    pub right_shift: FromModifier,
    pub right_alt: FromModifier,
    pub right_meta: FromModifier,
}

impl FromModifiers {
    pub fn from_config(kb_from_modifiers: &KBFromModifiers) -> FromModifiers {
        let mut from_modifiers = FromModifiers::default();

        if let Some(mandatory) = &kb_from_modifiers.mandatory {
            for m in mandatory {
                let key = Key::from_kb_key_code(&m).unwrap();
                from_modifiers.set(&key, FromModifier::Mandatory);
            }
        }

        if let Some(optional) = &kb_from_modifiers.optional {
            for m in optional {
                let key = Key::from_kb_key_code(&m).unwrap();
                from_modifiers.set(&key, FromModifier::Optional);
            }
        }

        from_modifiers
    }

    pub fn set(&mut self, key: &Key, modifier: FromModifier) {
        match key {
            Key::KeyLeftCtrl => self.left_control = modifier,
            Key::KeyLeftShift => self.left_shift = modifier,
            Key::KeyLeftAlt => self.left_alt = modifier,
            Key::KeyLeftMeta => self.left_meta = modifier,
            Key::KeyRightCtrl => self.right_control = modifier,
            Key::KeyRightShift => self.right_shift = modifier,
            Key::KeyRightAlt => self.right_alt = modifier,
            Key::KeyRightMeta => self.right_meta = modifier,
            _ => {}
        }
    }

    pub fn matches(&self, mod_state: &ModifierState) -> bool {
        let pairs = vec![
            (&self.left_control, mod_state.left_control),
            (&self.left_shift, mod_state.left_shift),
            (&self.left_alt, mod_state.left_alt),
            (&self.left_meta, mod_state.left_meta),
            (&self.right_control, mod_state.right_control),
            (&self.right_shift, mod_state.right_shift),
            (&self.right_alt, mod_state.right_alt),
            (&self.right_meta, mod_state.right_meta),
        ];

        pairs.iter().all(|(cond, state)| match cond {
            FromModifier::Absent => *state == false,
            FromModifier::Optional => true,
            FromModifier::Mandatory => *state == true,
        })
    }
}

impl Default for FromModifiers {
    fn default() -> FromModifiers {
        FromModifiers {
            left_control: FromModifier::Absent,
            left_shift: FromModifier::Absent,
            left_alt: FromModifier::Absent,
            left_meta: FromModifier::Absent,
            right_control: FromModifier::Absent,
            right_shift: FromModifier::Absent,
            right_alt: FromModifier::Absent,
            right_meta: FromModifier::Absent,
        }
    }
}
