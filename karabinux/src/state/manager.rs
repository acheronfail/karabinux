use crate::karabiner::KBProfile;
use crate::state::{ComplexManipulator, ModifierState, SimpleManipulator};
use evdev_rs::enums::{EventCode, EV_KEY};
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
    pub fn get_mapped_events(&mut self, mut ev: InputEvent) -> Vec<InputEvent> {
        // Perform simple remapping of keys first.
        self.apply_simple_modifications(&mut ev);

        // Process our complex manipulators, and get the transformed events.
        let events = self.apply_complex_modifications(&ev);

        // Update our modifier state.
        self.update_modifiers(&ev);

        // Return the transformed events.
        events
    }

    fn update_modifiers(&mut self, ev: &InputEvent) {
        self.modifier_state.update(ev);
    }

    fn apply_simple_modifications(&self, ev: &mut InputEvent) {
        #[cfg(debug)]
        {
            if ev.event_code == EventCode::EV_KEY(EV_KEY::KEY_ESC) {
                for _ in 0..30 {
                    eprintln!(".");
                }
            } else if ev.event_code == EventCode::EV_KEY(EV_KEY::KEY_GRAVE) {
                eprintln!("\n{:#?}\n", self);
            }
        }

        for sm in &self.simple_manipulators {
            if ev.event_code == EventCode::EV_KEY(sm.from.clone()) {
                ev.event_code = EventCode::EV_KEY(sm.to.clone());
            }
        }
    }

    fn apply_complex_modifications(&self, ev: &InputEvent) -> Vec<InputEvent> {
        // TODO: should be able to block key repeats (in between down and up)
        // TODO: condition checks
        // TODO: simultaneous events
        let mut output_queue = vec![];

        let mut applied_manipulator = false;
        for cm in &self.complex_manipulators {
            if cm.matches(&self.modifier_state, &ev) {
                cm.apply(&self.modifier_state, &ev, &mut output_queue);

                // Only apply the first complex manipulator that matches.
                applied_manipulator = true;
                break;
            }
        }

        // If no complex manipulators were applied, then just return the event.
        if !applied_manipulator {
            output_queue.push(ev.clone());
        }

        return output_queue;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
