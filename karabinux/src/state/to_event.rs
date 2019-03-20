use crate::karabiner::{FromKBKeyCode, KBToDefinition, Modifier};
use evdev_rs::enums::EV_KEY;
use evdev_rs::InputEvent;

#[derive(Debug)]
pub struct ToEvent {
    pub key: Option<EV_KEY>,
    pub modifiers: Vec<Modifier>,
    pub shell_command: Option<String>,
    pub repeat: bool,

    events_at_key_up: Vec<InputEvent>,
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
                .map(|kc| match Modifier::from_kb_key_code(kc) {
                    Some(modifier) => modifier,
                    None => panic!("failed to decode modifier: {:?}", kc),
                })
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

            events_at_key_up: vec![],
        }
    }

    pub fn add_event_at_key_up(&mut self, event: InputEvent) {
        self.events_at_key_up.push(event);
    }
}
