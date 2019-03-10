use crate::karabiner::{FromKBKeyCode, KBSimpleModification};
use evdev_rs::enums::EV_KEY;

/// A `SimpleManipulator` is just a simple remapping between two keys.
#[derive(Debug)]
pub struct SimpleManipulator {
    pub from: EV_KEY,
    pub to: EV_KEY,
}

impl SimpleManipulator {
    pub fn from_config(kb_simple_modification: &KBSimpleModification) -> SimpleManipulator {
        SimpleManipulator {
            from: EV_KEY::from_kb_key_code(&kb_simple_modification.from.key_code).unwrap(),
            to: EV_KEY::from_kb_key_code(&kb_simple_modification.to.key_code).unwrap(),
        }
    }
}
