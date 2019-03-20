use crate::event::KeyEvent;
use crate::karabiner::{KBComplexModifications, KBManipulator, KBManipulatorKind, Modifier};
use crate::key_state::KeyState;
use crate::state::{FromEvent, FromModifier, ModifierState, ToEvent};
use crate::util::new_key_event;
use evdev_rs::InputEvent;
use linked_hash_set::LinkedHashSet;

#[derive(Debug)]
pub struct ComplexManipulator {
    pub description: Option<String>,
    pub from_event: FromEvent,
    pub to_events: Vec<ToEvent>,

    cancelled_from_mandatory_modifiers: LinkedHashSet<Modifier>,
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

                    cancelled_from_mandatory_modifiers: LinkedHashSet::new(),
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

    pub fn matches(&self, mod_state: &ModifierState, key_event: &KeyEvent) -> bool {
        if !mod_state.matches(
            &self.from_event.modifiers,
            Modifier::from_key(&key_event.key),
        ) {
            return false;
        }

        if let Some(from_key) = &self.from_event.key {
            if from_key == &key_event.key {
                return true;
            }
        }

        false
    }

    pub fn apply(
        &mut self,
        mod_state: &ModifierState,
        key_event: &mut KeyEvent,
        output_queue: &mut Vec<InputEvent>,
    ) {
        match key_event.key_state {
            KeyState::Pressed => {
                self.cancel_from_mandatory_modifiers(output_queue, mod_state, key_event);

                self.key_pressed_event(output_queue, key_event);

                if !self.is_last_to_event_modifier_key_event() {
                    self.restore_from_mandatory_modifiers(output_queue, mod_state, key_event);
                }
            }
            KeyState::Released => {
                if !key_event.key_up_posted {
                    key_event.key_up_posted = true;

                    self.key_released_event(output_queue, key_event);
                    self.restore_from_mandatory_modifiers(output_queue, mod_state, key_event);
                }
            }
            KeyState::Autorepeat => {
                self.handle_autorepeat_event(output_queue, key_event);
            }
            _ => {}
        }
    }

    // TODO: review exceptional cases in src/share/manipulator/manipulators/basic/event_sender.hpp
    fn cancel_from_mandatory_modifiers(
        &mut self,
        output_queue: &mut Vec<InputEvent>,
        mod_state: &ModifierState,
        key_event: &KeyEvent,
    ) {
        for (from_modifier, condition) in &self.from_event.modifiers {
            if *condition != FromModifier::Mandatory {
                continue;
            }

            if self
                .cancelled_from_mandatory_modifiers
                .contains(&from_modifier)
            {
                continue;
            }

            if mod_state.is_active(*from_modifier) {
                for key in mod_state.keys_for_modifier(*from_modifier) {
                    output_queue.push(new_key_event(&key_event.time, &key, KeyState::Pressed));
                }

                self.cancelled_from_mandatory_modifiers
                    .insert(*from_modifier);
            }
        }
    }

    fn restore_from_mandatory_modifiers(
        &mut self,
        output_queue: &mut Vec<InputEvent>,
        mod_state: &ModifierState,
        key_event: &KeyEvent,
    ) {
        for (from_modifier, condition) in &self.from_event.modifiers {
            if *condition != FromModifier::Mandatory {
                continue;
            }

            if !self
                .cancelled_from_mandatory_modifiers
                .contains(&from_modifier)
            {
                continue;
            }

            if mod_state.is_active(*from_modifier) {
                for key in mod_state.keys_for_modifier(*from_modifier) {
                    output_queue.push(new_key_event(&key_event.time, &key, KeyState::Pressed));
                }

                self.cancelled_from_mandatory_modifiers
                    .remove(from_modifier);
            }
        }
    }

    fn key_pressed_event(&self, output_queue: &mut Vec<InputEvent>, key_event: &mut KeyEvent) {
        for (i, to_event) in self.to_events.iter().enumerate() {
            let is_modifier_key_event = {
                if let Some(key) = &to_event.key {
                    Modifier::is_modifier(&key)
                } else {
                    false
                }
            };

            // Modifier key down events.
            for modifier in &to_event.modifiers {
                if let Some(key) = modifier.as_key() {
                    output_queue.push(new_key_event(&key_event.time, &key, KeyState::Pressed));
                }
            }

            // Key down event.
            if let Some(key) = &to_event.key {
                output_queue.push(new_key_event(&key_event.time, &key, KeyState::Pressed));
            }

            // Key up event.
            if let Some(key) = &to_event.key {
                let event = new_key_event(&key_event.time, &key, KeyState::Released);
                if i != self.to_events.len() && !to_event.repeat {
                    output_queue.push(event);
                } else {
                    key_event.events_at_key_up.push(event);
                }
            }

            // Modifier key up events.
            for modifier in &to_event.modifiers {
                if let Some(key) = modifier.as_key() {
                    let event = new_key_event(&key_event.time, &key, KeyState::Released);
                    if i == self.to_events.len() && is_modifier_key_event {
                        key_event.events_at_key_up.push(event);
                    } else {
                        output_queue.push(event);
                    }
                }
            }
        }
    }

    fn key_released_event(&self, output_queue: &mut Vec<InputEvent>, key_event: &mut KeyEvent) {
        for event in key_event.events_at_key_up.drain(..) {
            output_queue.push(event);
        }
    }

    fn handle_autorepeat_event(&self, _output_queue: &mut Vec<InputEvent>, _key_event: &KeyEvent) {
        // TODO: unimplemented
    }

    fn is_last_to_event_modifier_key_event(&self) -> bool {
        if let Some(to_event) = self.to_events.last() {
            if let Some(key) = &to_event.key {
                return Modifier::is_modifier(&key);
            }
        }

        false
    }
}

// TODO: handle `shell_command` values in `ToEvent`s
#[allow(dead_code)]
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
