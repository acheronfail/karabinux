use crate::karabiner::{FromKBKeyCode, KBFromDefinition};
use crate::state::FromModifiers;
use evdev_rs::enums::EV_KEY;

#[derive(Debug)]
pub struct FromEvent {
    pub key: Option<EV_KEY>,
    pub modifiers: FromModifiers,
}

impl FromEvent {
    pub fn from_config(kb_from: &KBFromDefinition) -> FromEvent {
        let key = if let Some(key_code) = &kb_from.key_code {
            Some(EV_KEY::from_kb_key_code(&key_code).unwrap())
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
