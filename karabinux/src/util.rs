use crate::key_state::KeyState;
use evdev_rs::enums::{EventCode, EV_KEY, EV_SYN};
use evdev_rs::util::event_code_to_int;
use evdev_rs::{InputEvent, TimeVal};
use std::time::{SystemTime, UNIX_EPOCH};

pub const ALL_MODIFIERS: [EV_KEY; 8] = [
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

pub fn event_time_now() -> TimeVal {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("unexpected clock drift");

    TimeVal::new(now.as_secs() as i64, now.subsec_micros() as i64)
}

pub fn sync_event_now() -> libc::input_event {
    let ev_code = EventCode::EV_SYN(EV_SYN::SYN_REPORT);
    let sync_ev = InputEvent::new(&event_time_now(), &ev_code, 0);

    sync_ev.as_raw()
}

pub fn log_event(ev: &InputEvent, log_all_events: bool) {
    match ev.event_code {
        EventCode::EV_KEY(ref key) => {
            eprintln!(
                "\r  {:<16}{:?}({:?})",
                format!("{:?}", key),
                KeyState::from(ev.value),
                event_code_to_int(&ev.event_code).1
            );
        }
        _ => {
            if log_all_events {
                eprintln!("\r  {:?}\t{:?}", ev.event_type, ev.value);
            }
        }
    }
}
