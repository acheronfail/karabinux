use crate::key_state::KeyState;
use crate::util::key_from_event_code;
use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::{InputEvent, TimeVal};

#[derive(Debug)]
pub enum Event {
    Timeout,
    InputEvent(InputEvent),
}

// Place these on a queue, when when a "released" event occurs search up through
// the input queue and find the corresponding KeyEvent
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: EV_KEY,
    pub state: KeyState,
    pub time: TimeVal,
}

impl KeyEvent {
    pub fn new(pressed_event: InputEvent) -> KeyEvent {
        let key = key_from_event_code(&pressed_event.event_code).unwrap();
        KeyEvent {
            key,
            state: KeyState::from(pressed_event.value),
            time: pressed_event.time,
        }
    }

    pub fn create_event(&self) -> InputEvent {
        let code = EventCode::EV_KEY(self.key.clone());
        InputEvent::new(&self.time, &code, self.state.into())
    }
}
