use std::cell::RefCell;
use evdev_rs::enums::EV_KEY;
use crate::event::KeyEvent;
use crate::karabiner::{KBComplexModifications, KBManipulator, KBManipulatorKind, Modifier};
use crate::key_state::KeyState;
use crate::state::{FromEvent, FromModifier, ModifierState, ToEvent};
use crate::util::new_key_event;
use evdev_rs::{InputEvent, TimeVal};
use linked_hash_set::LinkedHashSet;

#[derive(Debug)]
struct ManipulationEvent {
    pub key: EV_KEY,
    pub state: KeyState,
    pub time: TimeVal,
    pub key_up_posted: bool,
    pub events_at_key_up: Vec<InputEvent>,
    pub cancelled_from_mandatory_modifiers: LinkedHashSet<Modifier>,
}

impl ManipulationEvent {
    pub fn new(key_event: &KeyEvent) -> ManipulationEvent {
        ManipulationEvent {
            key: key_event.key.clone(),
            state: key_event.state,
            time: key_event.time.clone(),
            key_up_posted: false,
            events_at_key_up: vec![],
            cancelled_from_mandatory_modifiers: LinkedHashSet::new()
        }
    }
}

#[derive(Debug)]
pub enum ManipulationResult {
    Skipped,
    Applied,
}

#[derive(Debug)]
pub struct ComplexManipulator {
    pub description: Option<String>,
    pub from_event: FromEvent,
    pub to_events: Vec<ToEvent>,

    manipulated_events: RefCell<Vec<ManipulationEvent>>,
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

                    manipulated_events: RefCell::new(vec![]),
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

    pub fn manipulate(
        &mut self,
        mod_state: &ModifierState,
        key_event: &mut KeyEvent,
        output_queue: &mut Vec<InputEvent>,
    ) -> ManipulationResult {
        let current_manipulated_event_index;

        if !self.test_from_event(key_event) {
            return ManipulationResult::Skipped;
        }

        match key_event.state {
            KeyState::Pressed => {
                if !self.test_from_modifiers(mod_state, key_event) {
                    return ManipulationResult::Skipped;
                }

                // TODO: check conditions here

                // TODO: check ALL KEYS IN FROM_EVENT DEFINITION ARE PRESSED HERE
                //  CHECK:
                //      for ev in input_queue:
                //          create ORDERED_KEY_DOWN_EVENTS
                //          create ORDERED_KEY_UP_EVENTS
                //          create FROM_EVENTS vec
                //          use queue to check time/delay for simultaneous events
                //          match key_state:
                //              pressed:
                //                  test against from event definition
                //                      true:
                //                          add to FROM_EVENTS if not already    <-- I believe it's copied here?
                //                          add to ORDERED_KEY_DOWN_EVENTS
                //                      false:
                //                          unless all from events are found:
                //                              bail (SKIPPED)
                //              released:
                //                  check if in FROM_EVENTS:
                //                      yes:
                //                      unless all FROM_EVENTS are found:
                //                          bail (SKIPPED)
                //      if FROM_EVENTS empty
                //          bail (SKIPPED)
                //      test KEY_ORDER for simultaneous events:
                //          otherwise: bail (SKIPPED)
                //      check all FROM_EVENTS are found:
                //          yes:
                //              WAIT if simultaneous event and it hasn't been cancelled yet
                //              if time exceeded:
                //                  bail (SKIPPED)
                //
                {
                    let mut manipulated_events = self.manipulated_events.borrow_mut();

                    manipulated_events.push(ManipulationEvent::new(key_event));
                    current_manipulated_event_index = Some(manipulated_events.len() - 1);
                }
            },
            KeyState::Released => {
                // TODO: we'll want to handle a list of events here in the future
                current_manipulated_event_index = self.manipulated_events
                    .borrow_mut()
                    .iter()
                    .position(|me| me.key == key_event.key && me.state == KeyState::Pressed && !me.key_up_posted);
            },
            _ => {
                // TODO: handle autorepeat events
                return ManipulationResult::Skipped;
            }
        }

        if let Some(index) = current_manipulated_event_index {
            let mut manipulated_events =self.manipulated_events.borrow_mut();
            let manipulated_event = manipulated_events.get_mut(index).unwrap();

            self.perform_manipulation(mod_state, output_queue, key_event.state, manipulated_event);
            ManipulationResult::Applied
        } else {
            ManipulationResult::Skipped
        }
    }

    fn perform_manipulation(
        &self,
        mod_state: &ModifierState,
        output_queue: &mut Vec<InputEvent>,
        key_state: KeyState,
        mut manipulated_event: &mut ManipulationEvent,
    ) {

        match key_state {
            KeyState::Pressed => {
                self.cancel_from_mandatory_modifiers(output_queue, mod_state, manipulated_event);

                self.key_pressed_event(output_queue, &mut manipulated_event);

                if !self.is_last_to_event_modifier_key_event() {
                    dbg!(&output_queue);
                    self.restore_from_mandatory_modifiers(output_queue, mod_state, manipulated_event);
                    dbg!(&output_queue);
                }
            }
            KeyState::Released => {
                if !manipulated_event.key_up_posted {
                    manipulated_event.key_up_posted = true;

                    self.key_released_event(output_queue, &mut manipulated_event);
                    self.restore_from_mandatory_modifiers(output_queue, mod_state, manipulated_event);
                }
            }
            KeyState::Autorepeat => {
                self.handle_autorepeat_event(output_queue, &manipulated_event);
            }
            _ => {}
        }
    }

    fn test_from_event(&self, key_event: &KeyEvent) -> bool {
        if let Some(from_key) = &self.from_event.key {
            if from_key == &key_event.key {
                return true;
            }
        }

        false
    }

    fn test_from_modifiers(&self, mod_state: &ModifierState, key_event: &KeyEvent) -> bool {
        let current_modifier = Modifier::from_key(&key_event.key);
        mod_state.test_modifiers(&self.from_event.modifiers, current_modifier)
    }

    // TODO: review exceptional cases in src/share/manipulator/manipulators/basic/event_sender.hpp
    fn cancel_from_mandatory_modifiers(
        &self,
        output_queue: &mut Vec<InputEvent>,
        mod_state: &ModifierState,
        manipulated_event: &mut ManipulationEvent,
    ) {
        for (from_modifier, condition) in &self.from_event.modifiers {
            if *condition != FromModifier::Mandatory {
                continue;
            }

            if manipulated_event
                .cancelled_from_mandatory_modifiers
                .contains(&from_modifier)
            {
                continue;
            }

            if mod_state.is_active(*from_modifier) {
                for key in mod_state.keys_for_modifier(*from_modifier) {
                    output_queue.push(new_key_event(&manipulated_event.time, &key, KeyState::Released));
                }

                manipulated_event.cancelled_from_mandatory_modifiers
                    .insert(*from_modifier);
            }
        }
    }

    // @@@ behaviour differs,
    //  see test_modifiers in from_event_definition
    fn restore_from_mandatory_modifiers(
        &self,
        output_queue: &mut Vec<InputEvent>,
        mod_state: &ModifierState,
        manipulated_event: &mut ManipulationEvent,
    ) {
        for (from_modifier, condition) in &self.from_event.modifiers {
            if *condition != FromModifier::Mandatory {
                continue;
            }

            if !manipulated_event
                .cancelled_from_mandatory_modifiers
                .contains(&from_modifier)
            {
                continue;
            }

            if mod_state.is_active(*from_modifier) {
                for key in mod_state.keys_for_modifier(*from_modifier) {
                    output_queue.push(new_key_event(&manipulated_event.time, &key, KeyState::Pressed));
                }

                manipulated_event.cancelled_from_mandatory_modifiers
                    .remove(from_modifier);
            }
        }
    }

    fn key_pressed_event(&self, output_queue: &mut Vec<InputEvent>, manipulated_event: &mut ManipulationEvent) {
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
                    output_queue.push(new_key_event(&manipulated_event.time, &key, KeyState::Pressed));
                }
            }

            // Key down event.
            if let Some(key) = &to_event.key {
                output_queue.push(new_key_event(&manipulated_event.time, &key, KeyState::Pressed));
            }

            // Key up event.
            if let Some(key) = &to_event.key {
                let event = new_key_event(&manipulated_event.time, &key, KeyState::Released);
                if i != self.to_events.len() - 1 && !to_event.repeat {
                    output_queue.push(event);
                } else {
                    manipulated_event.events_at_key_up.push(event);
                }
            }

            // Modifier key up events.
            for modifier in &to_event.modifiers {
                if let Some(key) = modifier.as_key() {
                    let event = new_key_event(&manipulated_event.time, &key, KeyState::Released);
                    if i == self.to_events.len() - 1 && is_modifier_key_event {
                        manipulated_event.events_at_key_up.push(event);
                    } else {
                        output_queue.push(event);
                    }
                }
            }
        }
    }

    fn key_released_event(&self, output_queue: &mut Vec<InputEvent>, manipulated_event: &mut ManipulationEvent) {
        for event in manipulated_event.events_at_key_up.drain(..) {
            output_queue.push(event);
        }
    }

    fn handle_autorepeat_event(&self, _output_queue: &mut Vec<InputEvent>, _manipulated_event: &ManipulationEvent) {
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
