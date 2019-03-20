use crate::event::KeyEvent;
use crate::karabiner::KBProfile;
use crate::state::{ComplexManipulator, ModifierState, SimpleManipulator};
use evdev_rs::InputEvent;

#[derive(Debug)]
pub struct StateManager {
    modifier_state: ModifierState,
    simple_manipulators: Vec<SimpleManipulator>,
    complex_manipulators: Vec<ComplexManipulator>,
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
        }
    }

    // https://pqrs.org/osx/karabiner/document.html#event-modification-chaining
    pub fn get_mapped_events(&mut self, mut key_event: KeyEvent) -> Vec<InputEvent> {
        // Perform simple remapping of keys first.
        self.apply_simple_modifications(&mut key_event);

        // Process our complex manipulators, and get the transformed events.
        let events = self.apply_complex_modifications(&mut key_event);

        // Update our modifier state.
        self.modifier_state.update(&key_event);

        // Return the transformed events.
        events
    }

    fn apply_simple_modifications(&self, key_event: &mut KeyEvent) {
        for sm in &self.simple_manipulators {
            if key_event.key == sm.from {
                key_event.key = sm.to.clone();
            }
        }
    }

    fn apply_complex_modifications(&mut self, key_event: &mut KeyEvent) -> Vec<InputEvent> {
        let mut output_queue = vec![];

        let mut applied_manipulator = false;
        for cm in self.complex_manipulators.iter_mut() {
            if cm.matches(&self.modifier_state, key_event) {
                cm.apply(&self.modifier_state, key_event, &mut output_queue);

                // Only apply the first complex manipulator that matches.
                applied_manipulator = true;
                break;
            }
        }

        // If no complex manipulators were applied, then just return the event.
        if !applied_manipulator {
            output_queue.push(key_event.create_event());
        }

        output_queue
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
