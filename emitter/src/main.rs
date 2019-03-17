mod args;

use args::Args;
use evdev_rs::{Device, InputEvent, UInputDevice};
use karabinux::device_config::{device_from_config, DeviceConfig};
use karabinux::pipe::read_struct;
use std::fs::File;
use std::io;
use std::process;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    // First, create a libevdev device.
    let file = File::open(args.device).expect("failed to open device descriptor");
    let dev = Device::new_from_fd(file).expect("failed to attach to file descriptor");

    // Then, extract its configuration and create a virtual device from it.
    let config = DeviceConfig::from_device(&dev);
    let dev = device_from_config(&config);
    let uinput = UInputDevice::create_from_device(&dev).unwrap();

    // Finally, whenever we receive an event from stdin, send it through to our
    // virtual device.
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    loop {
        let ev = match read_struct::<libc::input_event>(&mut stdin_handle) {
            Ok(ev) => ev,
            Err(e) => match e.kind() {
                io::ErrorKind::UnexpectedEof => process::exit(1),
                error => panic!("failed to read event from stdin: {:?}", error),
            },
        };

        uinput.write_event(&InputEvent::from_raw(&ev)).unwrap();
    }
}
