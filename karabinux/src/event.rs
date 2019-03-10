use evdev_rs::InputEvent;

#[derive(Debug)]
pub enum Event {
    Timeout,
    KeyEvent(InputEvent),
}
