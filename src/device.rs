use crate::args::Args;
use evdev_rs::{Device, GrabMode, ReadFlag};
use karabinux::event::Event;
use std::fs::File;
use std::process;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn init_event_reader(i_tx: Sender<Event>, args: Args) {
    thread::spawn(move || event_reader(i_tx, args));
}

// Reader thread: reads structs from stdin, passes them to a Sender.
fn event_reader(i_tx: Sender<Event>, args: Args) {
    let file = File::open(&args.device).expect("failed to open file");
    let mut device = Device::new_from_fd(file).expect("failed to create device");
    let read_flags = ReadFlag::NORMAL | ReadFlag::BLOCKING;

    // Pause while the output device is being setup.
    thread::sleep(Duration::from_secs(1));

    // Grab events (request exclusive access).
    if args.grab {
        match device.grab(GrabMode::Grab) {
            Ok(_) => {}
            Err(e) => panic!("failed to grab device: {:?}", e),
        }
    }

    // Setup some debug variables.
    #[cfg(debug)]
    let mut debug_last_ev_code = 0;

    // Optionally create a viewer window if enabled.
    #[cfg(feature = "viewer")]
    let tx: Option<std::sync::mpsc::Sender<_>> = {
        use crate::viewer::create_gtk_application;
        use std::thread;

        let (tx, rx) = std::sync::mpsc::channel();
        if args.viewer {
            thread::spawn(move || create_gtk_application(rx));
            Some(tx)
        } else {
            None
        }
    };

    loop {
        match device.next_event(read_flags) {
            Ok((_, ev)) => {
                // Exit if `backspace` is pressed twice in a row in debug mode.
                #[cfg(debug)]
                {
                    use evdev_rs::util::event_code_to_int;

                    let (ev_type, ev_code) = event_code_to_int(&ev.event_code);
                    // type == ev_key && key is pressed
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

                i_tx.send(Event::InputEvent(ev))
                    .expect("failed to send event");
            }
            Err(errno) => match errno {
                e => panic!("failed to read event from stdin: {:?}", e),
            },
        }
    }
}
