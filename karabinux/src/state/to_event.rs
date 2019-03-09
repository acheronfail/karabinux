use crate::karabiner::{FromKBKeyCode, KBToDefinition};
use crate::util::event_time_now;
use input_linux::{InputEvent, Key, KeyEvent, KeyState};

#[derive(Debug)]
pub struct ToEvent {
    pub key: Option<Key>,
    pub modifiers: Option<Vec<Key>>, // TODO: enforce modifier keys?
    pub shell_command: Option<String>,
    pub repeat: Option<bool>,
}

impl ToEvent {
    pub fn from_config(kb_to: &KBToDefinition) -> ToEvent {
        let key = if let Some(key_code) = &kb_to.key_code {
            Some(Key::from_kb_key_code(&key_code).unwrap())
        } else {
            None
        };

        let modifiers = if let Some(modifier_key_codes) = &kb_to.modifiers {
            Some(
                modifier_key_codes
                    .iter()
                    .map(|kc| Key::from_kb_key_code(kc).unwrap())
                    .collect(),
            )
        } else {
            None
        };

        let shell_command = kb_to.shell_command.clone();
        let repeat = kb_to.repeat.clone();

        ToEvent {
            key,
            modifiers,
            shell_command,
            repeat,
        }
    }

    pub fn key_event(&self, key_state: KeyState) -> Option<InputEvent> {
        if let Some(key) = self.key {
            let now = event_time_now();
            let key_event = KeyEvent::new(now, key, i32::from(key_state));
            Some(*key_event.as_event())
        } else {
            None
        }
    }

    pub fn modifiers(&self, key_state: KeyState) -> Option<Vec<InputEvent>> {
        if let Some(modifiers) = &self.modifiers {
            let now = event_time_now();

            let mut mods = vec![];
            for key in modifiers {
                let key_event = KeyEvent::new(now, *key, i32::from(key_state));
                mods.push(*key_event.as_event());
            }

            Some(mods)
        } else {
            None
        }
    }
}
