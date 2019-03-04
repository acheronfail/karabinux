use std::time::{SystemTime, UNIX_EPOCH};
use input_linux::sys;
use input_linux::{
    InputEvent,
    EventKind,
    Key,
    KeyState,
    EventTime,
    SynchronizeEvent,
    SynchronizeKind
};

pub fn event_time_now() -> EventTime {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("unexpected clock drift");

    EventTime::new(
        now.as_secs() as i64,
        now.subsec_micros() as i64
    )
}

#[inline]
pub fn sync_event_now() -> sys::input_event {
    let sync_ev = SynchronizeEvent::new(
        event_time_now(),
        SynchronizeKind::Report,
        0
    );

    *sync_ev.as_event().as_raw()
}

pub fn log_event(event: &InputEvent, log_all_events: bool) {
    if event.kind == EventKind::Key {
        eprintln!("\r  {:?} {:?} {:?}", Key::from_code(event.code).unwrap(), KeyState::from(event.value), event.code);
    } else if log_all_events {
        eprintln!("\r  {:?} {:?}", event.kind, event.value);
    }
}
