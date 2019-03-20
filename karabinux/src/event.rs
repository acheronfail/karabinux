use crate::key_state::KeyState;
use crate::util::key_from_event_code;
use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::{InputEvent, TimeVal};

#[derive(Debug)]
pub enum Event {
    Timeout,
    InputEvent(InputEvent),
}

#[derive(Debug)]
pub struct KeyEvent {
    pub key: EV_KEY,
    pub key_state: KeyState,
    pub time: TimeVal,
    pub manipulated: bool,

    pub key_up_posted: bool,
    pub events_at_key_up: Vec<InputEvent>,
}

impl KeyEvent {
    pub fn new(event: InputEvent) -> KeyEvent {
        let key = key_from_event_code(&event.event_code).unwrap();
        KeyEvent {
            key,
            key_state: KeyState::from(event.value),
            time: event.time,
            manipulated: false,
            key_up_posted: false,
            events_at_key_up: Vec::new(),
        }
    }

    pub fn create_event(&self) -> InputEvent {
        let code = EventCode::EV_KEY(self.key.clone());
        InputEvent::new(&self.time, &code, self.key_state.into())
    }
}
