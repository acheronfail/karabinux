use crate::constants::KARABINUX_DEVICE_NAME;
use crate::key_state::KeyState;
use evdev_rs::enums::{EventCode, EV_KEY, EV_SYN};
use evdev_rs::util::event_code_to_int;
use evdev_rs::{Device, InputEvent, TimeVal};
use std::fs::{read_dir, File};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn find_karabinux_uinput_device() -> Option<Device> {
    for entry in read_dir("/dev/input").expect("failed to read /dev/input directory") {
        let entry = entry.expect("failed to open entry");
        let path = entry.path();
        if !path.is_dir() {
            let file = File::open(&path).expect("failed to open path");
            match Device::new_from_fd(file) {
                Ok(device) => match device.name() {
                    Some(name) => {
                        if name.starts_with(KARABINUX_DEVICE_NAME) {
                            return Some(device);
                        }
                    }
                    None => continue,
                },
                Err(_) => continue,
            }
        }
    }

    None
}

pub fn key_from_event_code(code: &EventCode) -> Option<EV_KEY> {
    match code {
        EventCode::EV_KEY(key) => Some(key.clone()),
        _ => None,
    }
}

pub fn event_time_now() -> TimeVal {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("unexpected clock drift");

    TimeVal::new(now.as_secs() as i64, i64::from(now.subsec_micros()))
}

pub fn sync_event_now() -> InputEvent {
    let ev_code = EventCode::EV_SYN(EV_SYN::SYN_REPORT);
    InputEvent::new(&event_time_now(), &ev_code, 0)
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
