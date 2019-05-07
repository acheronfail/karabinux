use crate::karabiner::{FromKBKeyCode, KBToDefinition};
use crate::state::ModifierKey;
use evdev_rs::enums::EV_KEY;

#[derive(Debug)]
pub struct ToEvent {
    pub key: Option<EV_KEY>,
    pub modifiers: Vec<ModifierKey>,
    pub shell_command: Option<String>,
    pub repeat: bool,
    pub lazy: bool,
}

impl ToEvent {
    pub fn from_config(kb_to: &KBToDefinition) -> ToEvent {
        let key = if let Some(key_code) = &kb_to.key_code {
            Some(EV_KEY::from_kb_key_code(&key_code).unwrap())
        } else {
            None
        };

        let modifiers = if let Some(modifier_key_codes) = &kb_to.modifiers {
            modifier_key_codes
                .iter()
                .map(|kc| match ModifierKey::from_kb_key_code(kc) {
                    Some(kb_modifier) => kb_modifier,
                    None => panic!("failed to decode modifier: {:?}", kc),
                })
                .collect()
        } else {
            vec![]
        };

        let shell_command = kb_to.shell_command.clone();
        let repeat = kb_to.repeat.unwrap_or(true);
        let lazy = kb_to.lazy.unwrap_or(false);

        ToEvent {
            key,
            modifiers,
            shell_command,
            repeat,
            lazy,
        }
    }
}
