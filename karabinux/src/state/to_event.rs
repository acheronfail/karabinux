use crate::karabiner::{FromKBKeyCode, KBToDefinition, Modifier};
use crate::key_state::KeyState;
use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::{InputEvent, TimeVal};

#[derive(Debug)]
pub struct ToEvent {
    pub key: Option<EV_KEY>,
    pub modifiers: Vec<Modifier>,
    pub shell_command: Option<String>,
    pub repeat: bool,
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
                .map(|kc| Modifier::from_kb_key_code(kc).unwrap())
                .collect()
        } else {
            vec![]
        };

        let shell_command = kb_to.shell_command.clone();
        let repeat = kb_to.repeat.unwrap_or(true);

        ToEvent {
            key,
            modifiers,
            shell_command,
            repeat,
        }
    }

    pub fn key_event(&self, time: &TimeVal, key_state: KeyState) -> Option<InputEvent> {
        if let Some(key) = &self.key {
            let ev_code = EventCode::EV_KEY(key.clone());
            Some(InputEvent::new(&time, &ev_code, key_state.into()))
        } else {
            None
        }
    }
}
