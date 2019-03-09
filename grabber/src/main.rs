mod args;
mod grabber;

use args::Args;
use evdev_rs::{BLOCKING, NORMAL};
use grabber::Grabber;
use karabinux::pipe::write_struct;
use std::io;
use std::process;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    // Create a device from the given path.
    let mut grabber = Grabber::from_path(args.device);

    // Grab events (request exclusive access).
    if args.grab {
        grabber.grab();
    }

    #[cfg(debug)]
    let mut last_ev_code = 0;

    // Write any received events from the device straight to stdout.
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    loop {
        let ev = grabber.next_event(NORMAL | BLOCKING);

        // Exit if ESC is pressed twice in debug mode.
        #[cfg(debug)]
        {
            use evdev_rs::util::event_code_to_int;

            let (ev_type, ev_code) = event_code_to_int(&ev.event_code);
            if ev_type == 1 && ev.value == 1 {
                // ev_key && pressed
                if ev_code == 14 && last_ev_code == 14 {
                    // backspace
                    process::exit(2);
                }

                last_ev_code = ev_code;
            }
        }

        // Write struct to stdout.
        match write_struct::<libc::input_event>(&mut stdout_handle, &ev.as_raw()) {
            Ok(_) => continue,
            Err(_) => process::exit(1),
        }
    }
}
