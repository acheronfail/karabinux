use crate::karabiner::{FromKBKeyCode, KBFromDefinition};
use crate::state::FromModifiers;
use input_linux::Key;

#[derive(Debug)]
pub struct FromEvent {
    pub key: Option<Key>,
    pub modifiers: FromModifiers,
}

impl FromEvent {
    pub fn from_config(kb_from: &KBFromDefinition) -> FromEvent {
        let key = if let Some(key_code) = &kb_from.key_code {
            Some(Key::from_kb_key_code(&key_code).unwrap())
        } else {
            None
        };

        let modifiers = if let Some(from_modifiers) = &kb_from.modifiers {
            FromModifiers::from_config(&from_modifiers)
        } else {
            FromModifiers::default()
        };

        FromEvent { key, modifiers }
    }
}
