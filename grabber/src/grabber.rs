use evdev_rs::{Device, GrabMode, InputEvent, ReadFlag};
use std::fs::File;
use std::path::Path;

pub struct Grabber {
    // The `Device` struct requires that this file descriptor hang around.
    _file: File,
    is_grabbed: bool,
    device: Device,
}

impl Grabber {
    pub fn new(file: File) -> Grabber {
        let device = Device::new_from_fd(&file).expect("failed to attach to file descriptor");
        Grabber {
            _file: file,
            is_grabbed: false,
            device,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Grabber {
        let file = File::open(path).expect("failed to open device");
        Grabber::new(file)
    }

    pub fn grab(&mut self) {
        match self.device.grab(GrabMode::Grab) {
            Ok(_) => self.is_grabbed = true,
            Err(e) => eprintln!("{:?}", e),
        }
    }

    pub fn ungrab(&mut self) {
        match self.device.grab(GrabMode::Ungrab) {
            Ok(_) => self.is_grabbed = false,
            Err(e) => eprintln!("{:?}", e),
        }
    }

    pub fn next_event(&mut self, flags: ReadFlag) -> InputEvent {
        match self.device.next_event(flags) {
            Ok((_status, event)) => event,
            Err(errno) => panic!("Failed to read event: {:?}", errno),
        }
    }
}

impl Drop for Grabber {
    fn drop(&mut self) {
        self.ungrab();
    }
}
