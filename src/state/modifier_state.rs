use input_linux::{InputEvent, Key, KeyState};

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
            _ => false,
        };

        match Key::from_code(ev.code).unwrap() {
            Key::KeyLeftCtrl => self.left_control = on,
            Key::KeyLeftShift => self.left_shift = on,
            Key::KeyLeftAlt => self.left_alt = on,
            Key::KeyLeftMeta => self.left_meta = on,
            Key::KeyRightCtrl => self.right_control = on,
            Key::KeyRightShift => self.right_shift = on,
            Key::KeyRightAlt => self.right_alt = on,
            Key::KeyRightMeta => self.right_meta = on,
            _ => {}
        }
    }

    // pub fn is_key_pressed(&self, key: &Key) -> bool {
    //     match key {
    //         Key::KeyLeftCtrl => self.left_control,
    //         Key::KeyLeftShift => self.left_shift,
    //         Key::KeyLeftAlt => self.left_alt,
    //         Key::KeyLeftMeta => self.left_meta,
    //         Key::KeyRightCtrl => self.right_control,
    //         Key::KeyRightShift => self.right_shift,
    //         Key::KeyRightAlt => self.right_alt,
    //         Key::KeyRightMeta => self.right_meta,
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
