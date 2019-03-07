use input_linux::sys::input_event;
use input_linux::InputEvent;
use std::fmt;

pub enum Event {
    Timeout,
    KeyEvent(input_event),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Event::Timeout => write!(f, "Timeout"),
            &Event::KeyEvent(ev) => {
                let inp_ev = InputEvent::from_raw(&ev).unwrap();
                write!(f, "KeyEvent({:?})", &inp_ev)
            }
        }
    }
}
