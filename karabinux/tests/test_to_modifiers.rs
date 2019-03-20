mod test_util;

use evdev_rs::enums::*;
use karabinux::key_state::KeyState;
use test_util::*;

const CONF_FILE_PATH: &'static str = "test_from_modifiers";

#[test]
fn simply_remaps_one_modifier_to_another() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_RIGHTCTRL, KeyState::Pressed),
            (EV_KEY::KEY_RIGHTCTRL, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_LEFTCTRL, KeyState::Pressed),
            (EV_KEY::KEY_LEFTCTRL, KeyState::Released),
        ],
    );
}

#[test]
fn simply_remaps_one_modifier_to_multiple_modifiers() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_RIGHTALT, KeyState::Pressed),
            (EV_KEY::KEY_RIGHTALT, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_LEFTALT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTCTRL, KeyState::Pressed),
            (EV_KEY::KEY_LEFTMETA, KeyState::Pressed),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTALT, KeyState::Released),
            (EV_KEY::KEY_LEFTCTRL, KeyState::Released),
            (EV_KEY::KEY_LEFTMETA, KeyState::Released),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
        ],
    );
}
