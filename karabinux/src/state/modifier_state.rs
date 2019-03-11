use crate::key_state::KeyState;
use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::InputEvent;

#[derive(Debug)]
pub struct ModifierState {
    pub left_control: bool,
    pub left_shift: bool,
    pub left_alt: bool,
    pub left_meta: bool,
    pub right_control: bool,
    pub right_shift: bool,
    pub right_alt: bool,
    pub right_meta: bool,
}

impl ModifierState {
    pub fn new() -> ModifierState {
        ModifierState {
            left_control: false,
            left_shift: false,
            left_alt: false,
            left_meta: false,
            right_control: false,
            right_shift: false,
            right_alt: false,
            right_meta: false,
        }
    }

    pub fn update(&mut self, ev: &InputEvent) {
        let on = match KeyState::from(ev.value) {
            KeyState::Pressed => true,
            KeyState::Released => false,
            KeyState::Autorepeat => true,
            KeyState::Unknown(_) => false,
        };

        match ev.event_code {
            EventCode::EV_KEY(EV_KEY::KEY_LEFTCTRL) => self.left_control = on,
            EventCode::EV_KEY(EV_KEY::KEY_LEFTSHIFT) => self.left_shift = on,
            EventCode::EV_KEY(EV_KEY::KEY_LEFTALT) => self.left_alt = on,
            EventCode::EV_KEY(EV_KEY::KEY_LEFTMETA) => self.left_meta = on,
            EventCode::EV_KEY(EV_KEY::KEY_RIGHTCTRL) => self.right_control = on,
            EventCode::EV_KEY(EV_KEY::KEY_RIGHTSHIFT) => self.right_shift = on,
            EventCode::EV_KEY(EV_KEY::KEY_RIGHTALT) => self.right_alt = on,
            EventCode::EV_KEY(EV_KEY::KEY_RIGHTMETA) => self.right_meta = on,
            _ => {}
        }
    }

    // pub fn is_key_pressed(&self, key: &EV_KEY) -> bool {
    //     match key {
    //         EV_KEY::KEY_LEFTCTRL => self.left_control,
    //         EV_KEY::KEY_LEFTSHIFT => self.left_shift,
    //         EV_KEY::KEY_LEFTALT => self.left_alt,
    //         EV_KEY::KEY_LEFTMETA => self.left_meta,
    //         EV_KEY::KEY_RIGHTCTRL => self.right_control,
    //         EV_KEY::KEY_RIGHTSHIFT => self.right_shift,
    //         EV_KEY::KEY_RIGHTALT => self.right_alt,
    //         EV_KEY::KEY_RIGHTMETA => self.right_meta,
    //         _ => false
    //     }
    // }

    // pub fn control(&self) -> bool {
    //     self.left_control || self.right_control
    // }

    // pub fn shift(&self) -> bool {
    //     self.left_shift || self.right_shift
    // }

    // pub fn alt(&self) -> bool {
    //     self.left_alt || self.right_alt
    // }

    // pub fn meta(&self) -> bool {
    //     self.left_meta || self.right_meta
    // }

    // pub fn any(&self) -> bool {
    //     self.control() || self.shift() || self.alt() || self.meta()
    // }

    // pub fn none(&self) -> bool {
    //     !self.any()
    // }
}
