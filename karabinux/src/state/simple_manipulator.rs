use crate::karabiner::{FromKBKeyCode, KBSimpleModification};
use input_linux::Key;

/// A `SimpleManipulator` is just a simple remapping between two keys.
#[derive(Debug)]
pub struct SimpleManipulator {
    pub from: Key,
    pub to: Key,
}

impl SimpleManipulator {
    pub fn from_config(kb_simple_modification: &KBSimpleModification) -> SimpleManipulator {
        SimpleManipulator {
            from: Key::from_kb_key_code(&kb_simple_modification.from.key_code).unwrap(),
            to: Key::from_kb_key_code(&kb_simple_modification.to.key_code).unwrap(),
        }
    }
}
