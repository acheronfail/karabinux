mod test_util;

use evdev_rs::enums::*;
use karabinux::key_state::KeyState;
use test_util::*;

const CONF_FILE_PATH: &'static str = "test_from_modifiers";

#[test]
fn no_modifiers_maps_key_with_no_modifiers() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_A, KeyState::Pressed),
            (EV_KEY::KEY_A, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_B, KeyState::Pressed),
            (EV_KEY::KEY_B, KeyState::Released),
        ],
    );
}

#[test]
fn no_modifiers_does_not_map_key_with_modifiers() {
    for modifier_key in &ALL_MODIFIERS {
        test_complex_modifications(
            CONF_FILE_PATH,
            vec![
                (modifier_key.clone(), KeyState::Pressed),
                (EV_KEY::KEY_A, KeyState::Pressed),
                (EV_KEY::KEY_A, KeyState::Released),
                (modifier_key.clone(), KeyState::Released),
            ],
            vec![
                (modifier_key.clone(), KeyState::Pressed),
                (EV_KEY::KEY_A, KeyState::Pressed),
                (EV_KEY::KEY_A, KeyState::Released),
                (modifier_key.clone(), KeyState::Released),
            ],
        );
    }
}

#[test]
fn mandatory_modifier_any_maps_key_with_no_modifiers() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_C, KeyState::Pressed),
            (EV_KEY::KEY_C, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_C, KeyState::Pressed),
            (EV_KEY::KEY_C, KeyState::Released),
        ],
    );
}

// #[test]
// fn mandatory_modifier_any_maps_key_with_any_modifier_and_cancels_modifier() {
//     for modifier_key in &ALL_MODIFIERS {
//         test_complex_modifications(
//             CONF_FILE_PATH,
//             vec![
//                 (modifier_key.clone(), KeyState::Pressed),
//                 (EV_KEY::KEY_C, KeyState::Pressed),
//                 (EV_KEY::KEY_C, KeyState::Released),
//                 (modifier_key.clone(), KeyState::Released),
//             ],
//             vec![
//                 (modifier_key.clone(), KeyState::Pressed),
//                 (modifier_key.clone(), KeyState::Released),
//                 (EV_KEY::KEY_D, KeyState::Pressed),
//                 (EV_KEY::KEY_D, KeyState::Released),
//             ],
//         );
//     }
// }

#[test]
fn mandatory_modifier_any_maps_key_with_two_modifiers_and_cancels_modifiers() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_LEFTCTRL, KeyState::Pressed),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Pressed),
            (EV_KEY::KEY_C, KeyState::Pressed),
            (EV_KEY::KEY_C, KeyState::Released),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
            (EV_KEY::KEY_LEFTCTRL, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_LEFTCTRL, KeyState::Pressed),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTCTRL, KeyState::Released),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
            (EV_KEY::KEY_D, KeyState::Pressed),
            (EV_KEY::KEY_D, KeyState::Released),
            // TODO: how does this affect other modifiers before these are let go?
            // Meaning:
            //  1 CTRL
            //  1 SHIFT
            //  1 C
            //  0 C
            //     <-- Now, with CTRL and SHIFT held down, what if there's another
            //         modifier that requires CTRL and SHIFT, and we activate that?
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
            (EV_KEY::KEY_LEFTCTRL, KeyState::Released),
        ],
    );
}

#[test]
fn mandatory_modifier_any_maps_key_with_three_modifiers_and_cancels_modifiers() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTMETA, KeyState::Pressed),
            (EV_KEY::KEY_LEFTALT, KeyState::Pressed),
            (EV_KEY::KEY_C, KeyState::Pressed),
            (EV_KEY::KEY_C, KeyState::Released),
            (EV_KEY::KEY_LEFTALT, KeyState::Released),
            (EV_KEY::KEY_LEFTMETA, KeyState::Released),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTMETA, KeyState::Pressed),
            (EV_KEY::KEY_LEFTALT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
            (EV_KEY::KEY_LEFTALT, KeyState::Released),
            (EV_KEY::KEY_LEFTMETA, KeyState::Released),
            (EV_KEY::KEY_D, KeyState::Pressed),
            (EV_KEY::KEY_D, KeyState::Released),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Pressed),
            (EV_KEY::KEY_LEFTMETA, KeyState::Pressed),
            (EV_KEY::KEY_LEFTMETA, KeyState::Released),
            (EV_KEY::KEY_LEFTSHIFT, KeyState::Released),
        ],
    );
}

#[test]
fn optional_modifier_any_maps_key_with_no_modifiers() {
    test_complex_modifications(
        CONF_FILE_PATH,
        vec![
            (EV_KEY::KEY_E, KeyState::Pressed),
            (EV_KEY::KEY_E, KeyState::Released),
        ],
        vec![
            (EV_KEY::KEY_F, KeyState::Pressed),
            (EV_KEY::KEY_F, KeyState::Released),
        ],
    );
}

#[test]
fn optional_modifier_any_maps_key_with_any_modifier() {
    for modifier_key in &ALL_MODIFIERS {
        test_complex_modifications(
            CONF_FILE_PATH,
            vec![
                (modifier_key.clone(), KeyState::Pressed),
                (EV_KEY::KEY_E, KeyState::Pressed),
                (EV_KEY::KEY_E, KeyState::Released),
                (modifier_key.clone(), KeyState::Released),
            ],
            vec![
                (modifier_key.clone(), KeyState::Pressed),
                (EV_KEY::KEY_F, KeyState::Pressed),
                (EV_KEY::KEY_F, KeyState::Released),
                (modifier_key.clone(), KeyState::Released),
            ],
        );
    }
}
