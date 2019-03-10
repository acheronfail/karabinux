use crate::karabiner::{KBComplexModifications, KBManipulator, KBManipulatorKind};
use crate::state::{FromEvent, ModifierState, ToEvent};
use evdev_rs::InputEvent;
use evdev_rs::enums::EventCode;

#[derive(Debug)]
pub struct ComplexManipulator {
    pub description: Option<String>,
    pub from_event: FromEvent,
    pub to_events: Vec<ToEvent>,
    // TODO: conditions
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
        // TODO: handle parameters
        let complex_manipulators = kb_complex_modifications
            .rules
            .iter()
            .flat_map(|rule| rule.manipulators.iter().map(|m| ComplexManipulator::new(m)))
            .collect();

        complex_manipulators
    }

    pub fn matches(&self, mod_state: &ModifierState, ev: &InputEvent) -> bool {
        if !self.from_event.modifiers.matches(mod_state) {
            return false;
        }

        if let Some(ref from_key) = self.from_event.key {
            if let EventCode::EV_KEY(ref ev_key) = ev.event_code {
                if from_key == ev_key {
                    return true;
                }
            }
        }

        false
    }

    pub fn apply(
        &self,
        _mod_state: &ModifierState,
        ev: &InputEvent,
        output_queue: &mut Vec<InputEvent>,
    ) {
        if let Some(ref from_key) = self.from_event.key {
            if let EventCode::EV_KEY(ref ev_key) = ev.event_code {
                if from_key == ev_key {
                    // TODO: call shell command if it exists
                    // TODO: handle repeats in to_events
                    for to_event in &self.to_events {
                        // Emit manipulated event with the correct modifiers.
                        match ev.value {
                            // Pressed
                            1 => {
                                if let Some(v) = to_event.modifiers(ev.value) {
                                    output_queue.extend(v);
                                }
                                if let Some(e) = to_event.key_event(ev.value) {
                                    output_queue.push(e);
                                }
                            }
                            // Released
                            0 => {
                                if let Some(e) = to_event.key_event(ev.value) {
                                    output_queue.push(e);
                                }
                                if let Some(v) = to_event.modifiers(ev.value) {
                                    output_queue.extend(v);
                                }
                            }

                            // TODO: handle other key states.
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
