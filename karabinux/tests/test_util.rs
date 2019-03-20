#![allow(dead_code)]

use evdev_rs::enums::{int_to_event_type, EventCode, EV_KEY, EV_SYN};
use evdev_rs::util::event_code_to_int;
use evdev_rs::{InputEvent, TimeVal};
use karabinux::karabiner::KBConfig;
use karabinux::key_state::KeyState;
use karabinux::state::StateManager;
use karabinux::event::KeyEvent;
use pretty_assertions::assert_eq;

pub use karabinux::constants::ALL_MODIFIER_KEYS;

/// Create a state from the config file with the same name as the test.
/// Eg, "from_modifiers.rs" will read from "./config/from_modifiers.json".
pub fn create_state(conf_file_name: &str) -> StateManager {
    let conf_file_path = &format!("tests/config/{}.json", conf_file_name);

    let kb_config = KBConfig::from_path(conf_file_path).expect("failed to construct config");
    StateManager::from_profile(&kb_config.profiles[0])
}

pub fn create_key_event(key: EV_KEY, value: KeyState) -> KeyEvent {
    KeyEvent::new(InputEventBuilder::new()
        .code(EventCode::EV_KEY(key))
        .val(value.into())
        .build())
}

pub fn send_key(state: &mut StateManager, key: EV_KEY, value: KeyState) -> Vec<InputEvent> {
    let key_event = create_key_event(key.clone(), value.into());
    state.get_mapped_events(key_event)
}

pub fn send_key_press(state: &mut StateManager, key: EV_KEY) -> Vec<InputEvent> {
    let mut events = send_key(state, key.clone(), KeyState::Pressed);
    events.extend(send_key(state, key.clone(), KeyState::Released));
    events
}

pub fn map_events_to_key_and_state(events: Vec<InputEvent>) -> Vec<(EV_KEY, KeyState)> {
    events
        .iter()
        .map(|ev| match &ev.event_code {
            EventCode::EV_KEY(key) => (key.clone(), ev.value.into()),
            _ => panic!("Expected event to be a key event, got {:?}", ev),
        })
        .collect()
}

pub fn test_complex_modifications(
    conf_file_name: &str,
    input_events: Vec<(EV_KEY, KeyState)>,
    expected_events: Vec<(EV_KEY, KeyState)>,
) {
    let mut state = create_state(conf_file_name);
    let mut events = vec![];

    for (key, key_state) in input_events {
        events.extend(send_key(&mut state, key.clone(), key_state));
    }

    assert_eq!(map_events_to_key_and_state(events), expected_events);
}

pub struct InputEventBuilder {
    time: TimeVal,
    code: EventCode,
    value: i32,
}

impl InputEventBuilder {
    pub fn new() -> InputEventBuilder {
        InputEventBuilder {
            time: TimeVal::new(0, 0),
            code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            value: 0,
        }
    }

    pub fn time(&mut self, sec: i64, usec: i64) -> &mut Self {
        self.time = TimeVal::new(sec, usec);
        self
    }

    pub fn code(&mut self, code: EventCode) -> &mut Self {
        self.code = code;
        self
    }

    pub fn val(&mut self, value: i32) -> &mut Self {
        self.value = value;
        self
    }

    pub fn build(&self) -> InputEvent {
        let (ev_type, _) = event_code_to_int(&self.code);
        InputEvent {
            time: self.time.clone(),
            event_code: self.code.clone(),
            event_type: int_to_event_type(ev_type).unwrap(),
            value: self.value.clone(),
        }
    }
}
