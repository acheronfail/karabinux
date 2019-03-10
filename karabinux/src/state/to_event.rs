use crate::karabiner::{FromKBKeyCode, KBToDefinition};
use crate::util::event_time_now;
use evdev_rs::InputEvent;
use evdev_rs::enums::{EventCode, EV_KEY};

#[derive(Debug)]
pub struct ToEvent {
    pub key: Option<EV_KEY>,
    pub modifiers: Option<Vec<EV_KEY>>, // TODO: enforce modifier keys?
    pub shell_command: Option<String>,
    pub repeat: Option<bool>,
}

impl ToEvent {
    pub fn from_config(kb_to: &KBToDefinition) -> ToEvent {
        let key = if let Some(key_code) = &kb_to.key_code {
            Some(EV_KEY::from_kb_key_code(&key_code).unwrap())
        } else {
            None
        };

        let modifiers = if let Some(modifier_key_codes) = &kb_to.modifiers {
            Some(
                modifier_key_codes
                    .iter()
                    .map(|kc| EV_KEY::from_kb_key_code(kc).unwrap())
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

    pub fn key_event(&self, key_state: i32) -> Option<InputEvent> {
        if let Some(key) = &self.key {
            let now = event_time_now();
            let ev_code = EventCode::EV_KEY(key.clone());
            Some(InputEvent::new(&now, &ev_code, key_state))
        } else {
            None
        }
    }

    pub fn modifiers(&self, key_state: i32) -> Option<Vec<InputEvent>> {
        if let Some(modifiers) = &self.modifiers {
            let now = event_time_now();

            let mut mods = vec![];
            for key in modifiers {
                let ev_code = EventCode::EV_KEY(key.clone());
                mods.push(InputEvent::new(&now, &ev_code, key_state));
            }

            Some(mods)
        } else {
            None
        }
    }
}
