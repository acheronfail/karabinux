use input_linux::InputEvent;
use input_linux::Key;
use crate::kb_key::FromKBKeyCode;
use crate::config::{
    KBProfile,
    KBSimpleModification
};

#[derive(Debug)]
pub struct State {
    simple_manipulators: Vec<SimpleManipulator>
}

impl State {
    pub fn from_profile(kb_profile: &KBProfile) -> State {
        let simple_manipulators = kb_profile.simple_modifications.iter()
            .map(|sm| SimpleManipulator::from_config(sm))
            .collect();

        State  {
            simple_manipulators
        }
    }

    pub fn apply_simple_modifications(&self, ev: &mut InputEvent) {
        let key = Key::from_code(ev.code).unwrap();
        for sm in self.simple_manipulators.iter() {
            if key == sm.from {
                ev.code = sm.to as u16;
            }
        }
    }
}

/// A `SimpleManipulator` is just a simple remapping between two keys.
#[derive(Debug)]
struct SimpleManipulator {
    from: Key,
    to: Key
}

impl SimpleManipulator {
    fn from_config(kb_simple_modification: &KBSimpleModification) -> SimpleManipulator {
        SimpleManipulator {
            from: Key::from_kb_key_code(&kb_simple_modification.from.key_code)
                .expect("failed to decode key_code"),
            to: Key::from_kb_key_code(&kb_simple_modification.to.key_code)
                .expect("failed to decode key_code")
        }
    }
}
