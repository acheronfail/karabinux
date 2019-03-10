use evdev_rs::{InputEvent, TimeVal};
use evdev_rs::enums::{EventCode, EV_SYN};
use evdev_rs::util::event_code_to_int;
use std::time::{SystemTime, UNIX_EPOCH};

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
                "\r  {:<16}{}({:?})",
                format!("{:?}", key),
                if ev.value == 1 { "Pressed" } else { "Released" },
                event_code_to_int(&ev.event_code).1
            );
        },
        _ => {
            if log_all_events {
                eprintln!("\r  {:?}\t{:?}", ev.event_type, ev.value);
            }
        }
    }
}
