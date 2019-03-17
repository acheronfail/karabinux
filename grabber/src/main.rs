use evdev_rs::{Device, GrabMode, ReadFlag};
use grabber::Args;
use karabinux::pipe::write_struct;
use std::fs::File;
use std::time::Duration;
use std::{io, process, thread};
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    // Create a device from the given path.
    let file = File::open(&args.device).expect("failed to open device");
    let mut device = Device::new_from_fd(file).expect("failed to attach to file descriptor");

    // Pause while the output device is being setup (in the emitter process).
    thread::sleep(Duration::from_secs(1));

    // Grab events (request exclusive access).
    if args.grab {
        match device.grab(GrabMode::Grab) {
            Ok(_) => {}
            Err(e) => panic!("failed to grab device: {:?}", e),
        }
    }

    #[cfg(debug)]
    let mut debug_last_ev_code = 0;

    #[cfg(feature = "viewer")]
    let tx: Option<std::sync::mpsc::Sender<_>> = {
        use grabber::viewer::create_gtk_application;
        use std::thread;

        let (tx, rx) = std::sync::mpsc::channel();
        if args.viewer {
            thread::spawn(move || create_gtk_application(rx));
            Some(tx)
        } else {
            None
        }
    };

    // Write any received events from the device straight to stdout.
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    loop {
        let (_, ev) = device
            .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
            .expect("failed to read event");

        // Exit if `backspace` is pressed twice in a row in debug mode.
        #[cfg(debug)]
        {
            use evdev_rs::util::event_code_to_int;

            let (ev_type, ev_code) = event_code_to_int(&ev.event_code);
            // ev_key && pressed
            if ev_type == 1 && ev.value == 1 {
                // backspace
                if ev_code == 14 && debug_last_ev_code == 14 {
                    process::exit(2);
                }

                debug_last_ev_code = ev_code;
            }
        }

        // Send events through to the viewer if enabled.
        #[cfg(feature = "viewer")]
        {
            if args.viewer {
                if let Some(tx) = tx.as_ref() {
                    tx.send(ev.clone()).expect("failed to send event to viewer");
                }
            }
        }

        // Write struct to stdout.
        match write_struct::<libc::input_event>(&mut stdout_handle, &ev.as_raw()) {
            Ok(_) => continue,
            Err(e) => panic!("failed to write event to stdout: {:?}", e),
        }
    }
}
