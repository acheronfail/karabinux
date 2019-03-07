use crate::karabiner::KBProfile;
use crate::state::{ComplexManipulator, ModifierState, SimpleManipulator};
use input_linux::{InputEvent, Key};

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

    pub fn update_modifiers(&mut self, ev: &InputEvent) {
        self.modifier_state.update(ev);
    }

    pub fn apply_simple_modifications(&self, ev: &mut InputEvent) {
        let key = Key::from_code(ev.code).unwrap();

        #[cfg(debug)]
        {
            if key == Key::KeyEsc {
                for _ in 0..30 {
                    eprintln!(".");
                }
            } else if key == Key::KeyGrave {
                eprintln!("\n{:#?}\n", self);
            }
        }

        for sm in &self.simple_manipulators {
            if key == sm.from {
                ev.code = sm.to as u16;
            }
        }
    }

    pub fn apply_complex_modifications(&self, ev: InputEvent) -> Vec<InputEvent> {
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
            output_queue.push(ev);
        }

        return output_queue;
    }
}
