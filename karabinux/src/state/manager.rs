use crate::key_state::KeyState;
use crate::event::KeyEvent;
use crate::karabiner::KBProfile;
use crate::state::{ComplexManipulator, ManipulationResult, ModifierState, SimpleManipulator};
use crate::util::key_from_event_code;
use evdev_rs::InputEvent;

#[derive(Debug)]
pub struct StateManager {
    modifier_state: ModifierState,
    simple_manipulators: Vec<SimpleManipulator>,
    complex_manipulators: Vec<ComplexManipulator>,

    input_queue: Vec<KeyEvent>,
}

impl StateManager {
    pub fn from_profile(kb_profile: &KBProfile) -> StateManager {
        let simple_manipulators = kb_profile
            .simple_modifications
            .iter()
            .map(|sm| SimpleManipulator::from_config(sm))
            .collect();

        let complex_manipulators =
            ComplexManipulator::manipulators_from_config(&kb_profile.complex_modifications);

        StateManager {
            modifier_state: ModifierState::new(),
            simple_manipulators,
            complex_manipulators,
            input_queue: Vec::new(),
        }
    }

    // https://pqrs.org/osx/karabiner/document.html#event-modification-chaining
    pub fn get_mapped_events(&mut self, input_event: InputEvent) -> Vec<InputEvent> {
        let mut output_queue = vec![];

        // TODO: support autorepeat events.
        if let Some(mut key_event) = self.get_key_event(input_event) {
            // Perform simple remapping of keys first.
            self.apply_simple_modifications(&mut key_event);

            // Process our complex manipulators.
            self.apply_complex_modifications(&mut output_queue, &mut key_event);

            // Update our modifier state.
            self.modifier_state.update(&key_event);

            // Place the event on the input queue.
            self.input_queue.push(key_event);
        }

        output_queue
    }

    fn get_key_event(&mut self, input_event: InputEvent) -> Option<KeyEvent> {
        #[cfg(debug)]
        assert!(self.input_queue.len() < 50);

        match KeyState::from(input_event.value) {
            KeyState::Pressed => Some(KeyEvent::new(input_event)),
            KeyState::Released => {
                let key = key_from_event_code(&input_event.event_code).unwrap();
                let index = self.input_queue
                    .iter()
                    .position(|ke| ke.key == key)
                    .expect("failed to find KeyEvent pair in input queue");

                // Update KeyEvent.
                let mut key_event = self.input_queue.remove(index);
                key_event.time = input_event.time;
                key_event.state = KeyState::Released;

                Some(key_event)
            },

            // TODO: support autorepeat events
            _ => None
        }
    }

    fn apply_simple_modifications(&self, key_event: &mut KeyEvent) {
        for sm in &self.simple_manipulators {
            if key_event.key == sm.from {
                key_event.key = sm.to.clone();
            }
        }
    }

    fn apply_complex_modifications(&mut self, mut output_queue: &mut Vec<InputEvent>, key_event: &mut KeyEvent) {
        let mut applied_manipulator = false;
        for cm in self.complex_manipulators.iter_mut() {
            match cm.manipulate(&self.modifier_state, key_event, &mut output_queue) {
                ManipulationResult::Skipped => continue,
                ManipulationResult::Applied => {
                    // Only apply the first complex manipulator that matches.
                    applied_manipulator = true;
                    break;
                }
            }
        }

        // If no complex manipulators were applied, then just return the event.
        if !applied_manipulator {
            output_queue.push(key_event.create_event());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
