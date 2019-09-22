use crate::karabiner::{KBComplexModifications, KBManipulator, KBManipulatorKind};
use crate::key_state::KeyState;
use crate::state::{FromEvent, FromModifier, ModifierState, ToEvent};
use crate::util::event_time_now;
use evdev_rs::enums::EventCode;
use evdev_rs::{InputEvent, TimeVal};
use std::collections::HashSet;

#[derive(Debug)]
pub struct ComplexManipulator {
    pub description: Option<String>,
    pub from_event: FromEvent,
    pub to_events: Vec<ToEvent>,
}

impl ComplexManipulator {
    pub fn new(manipulator: &KBManipulator) -> ComplexManipulator {
        match manipulator.kind {
            KBManipulatorKind::Basic => {
                let description = manipulator.description.clone();
                let from_event = FromEvent::from_config(&manipulator.from);
                let mut to_events = vec![];
                if let Some(to_config_list) = &manipulator.to {
                    for to_event_config in to_config_list {
                        to_events.push(ToEvent::from_config(to_event_config));
                    }
                }

                ComplexManipulator {
                    description,
                    from_event,
                    to_events,
                }
            }
        }
    }

    pub fn manipulators_from_config(
        kb_complex_modifications: &KBComplexModifications,
    ) -> Vec<ComplexManipulator> {
        kb_complex_modifications
            .rules
            .iter()
            .flat_map(|rule| rule.manipulators.iter().map(|m| ComplexManipulator::new(m)))
            .collect()
    }

    pub fn matches(&self, mod_state: &ModifierState, event: &InputEvent) -> bool {
        if !mod_state.matches(&self.from_event.modifiers) {
            return false;
        }

        if let Some(ref from_key) = self.from_event.key {
            if let EventCode::EV_KEY(ref ev_key) = event.event_code {
                if from_key == ev_key {
                    return true;
                }
            }
        }

        false
    }

    pub fn apply(
        &self,
        mod_state: &ModifierState,
        event: &InputEvent,
        output_queue: &mut Vec<InputEvent>,
    ) {
        let now = event_time_now();
        let key_state = KeyState::from(event.value);

        for to_event in &self.to_events {
            match key_state {
                KeyState::Pressed => {
                    // Clear current mandatory modifiers.
                    output_queue.extend(self.cancel_mandatory_from_modifiers(&now, mod_state));

                    // Wrap the emitted key event in modifiers from the "to_event" definition.
                    output_queue.extend(self.get_to_event_modifiers(
                        &now,
                        &to_event,
                        KeyState::Pressed,
                    ));

                    // If there's a "to" key event, send the release modifiers with it.
                    if let Some(event) = to_event.key_event(&now, key_state) {
                        output_queue.push(event);
                        output_queue.extend(self.get_to_event_modifiers(
                            &now,
                            &to_event,
                            KeyState::Released,
                        ));
                    }
                }
                KeyState::Released => {
                    // If there's a "to" key event, just release it (already manipulated).
                    // Otherwise, send the release modifiers.
                    if let Some(event) = to_event.key_event(&now, key_state) {
                        output_queue.push(event);
                    } else {
                        output_queue.extend(self.get_to_event_modifiers(
                            &now,
                            &to_event,
                            KeyState::Released,
                        ));
                    }
                }
                KeyState::Autorepeat => {
                    if let Some(event) = to_event.key_event(&now, key_state) {
                        if to_event.repeat {
                            output_queue.push(event);
                        }
                    }
                }
                _ => {}
            }

            // Call shell command if one is defined.
            if let Some(shell_cmd) = &to_event.shell_command {
                run_shell_command(&shell_cmd);
            }
        }
    }

    fn cancel_mandatory_from_modifiers(
        &self,
        now: &TimeVal,
        mod_state: &ModifierState,
    ) -> Vec<InputEvent> {
        let mut events = vec![];
        let mut emitted_modifiers = HashSet::new();

        for (from_modifier, condition) in &self.from_event.modifiers {
            if *condition != FromModifier::Mandatory {
                continue;
            }

            if emitted_modifiers.contains(&from_modifier) {
                continue;
            }

            if mod_state.is_active(*from_modifier) {
                for key in mod_state.keys_for_modifier(*from_modifier) {
                    let code = EventCode::EV_KEY(key);
                    let event = InputEvent::new(&now, &code, KeyState::Released.into());
                    events.push(event);
                    emitted_modifiers.insert(from_modifier);
                }
            }
        }

        events
    }

    fn get_to_event_modifiers(
        &self,
        now: &TimeVal,
        to_event: &ToEvent,
        key_state: KeyState,
    ) -> Vec<InputEvent> {
        to_event
            .modifiers
            .iter()
            .filter_map(|modifier| {
                modifier
                    .as_key()
                    .map(|key| InputEvent::new(&now, &EventCode::EV_KEY(key), key_state.into()))
            })
            .collect()
    }
}

fn run_shell_command(shell_cmd: &str) {
    use std::process::Command;
    use std::thread;

    let shell_cmd = shell_cmd.to_string();
    thread::spawn(move || match Command::new(&shell_cmd).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!(
                    r#"shell_command: "{:?}" exited with code: {:?}"#,
                    shell_cmd,
                    status.code()
                );
            }
        }
        Err(e) => eprintln!("{:?}", e),
    });
}
