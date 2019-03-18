use evdev_rs::enums::EV_KEY;

pub const KARABINUX_DEVICE_NAME: &'static str = "KarabinuxDevice: ";

pub const ALL_MODIFIER_KEYS: [EV_KEY; 8] = [
    // EV_KEY::KEY_CAPSLOCK, // TODO: add this as a modifier???
    EV_KEY::KEY_LEFTALT,
    EV_KEY::KEY_RIGHTALT,
    EV_KEY::KEY_LEFTMETA,
    EV_KEY::KEY_RIGHTMETA,
    EV_KEY::KEY_LEFTCTRL,
    EV_KEY::KEY_RIGHTCTRL,
    EV_KEY::KEY_LEFTSHIFT,
    EV_KEY::KEY_RIGHTSHIFT,
];
