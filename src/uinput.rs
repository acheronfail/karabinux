use crate::args::Args;
use crate::device_config::{device_from_config, DeviceConfig};
use evdev_rs::{Device, InputEvent, UInputDevice};
use karabinux::util::sync_event_now;
use std::fs::File;
use std::sync::mpsc::Receiver;
use std::thread;

pub fn init_event_emitter(o_rx: Receiver<InputEvent>, args: Args) {
    thread::spawn(move || event_emitter(o_rx, args));
}

// Writer thread: receives structs from a Receiver, and writes them to stdout.
fn event_emitter(o_rx: Receiver<InputEvent>, args: Args) {
    // First, create a libevdev device.
    let file = File::open(&args.device).expect("failed to open file");
    let device = Device::new_from_fd(file).expect("failed to create device");

    // Then, extract its configuration and create a virtual device from it.
    let config = DeviceConfig::from_device(&device);
    let device = device_from_config(&config);
    let uinput = UInputDevice::create_from_device(&device).expect("failed to create uinput");

    loop {
        match o_rx.recv() {
            Ok(ev) => {
                uinput.write_event(&ev).unwrap();
                uinput.write_event(&sync_event_now()).unwrap();
            }
            Err(e) => panic!("failed to write event to stdout: {:?}", e),
        }
    }
}
