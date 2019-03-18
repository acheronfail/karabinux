mod args;
mod device;
mod device_config;
mod uinput;
#[cfg(feature = "viewer")]
mod viewer;

use args::Args;
use evdev_rs::enums::EventType;
use karabinux::event::Event;
use karabinux::karabiner::KBConfig;
use karabinux::state::StateManager;
use std::process;
use std::sync::mpsc;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    // Parse the Karabiner configuration file.
    let kb_config = KBConfig::from_path(&args.config).expect("failed to construct config");
    let selected_profile = kb_config
        .profiles
        .iter()
        .find(|p| p.selected)
        .expect("failed to find selected profile");

    let mut state = StateManager::from_profile(&selected_profile);

    // Input channel: reads events from the libevdev device.
    let (i_tx, i_rx) = mpsc::channel();
    device::init_event_reader(i_tx, args.clone());

    // Output channel: writes events to a virtual libevdev uinput device.
    let (o_tx, o_rx) = mpsc::channel();
    uinput::init_event_emitter(o_rx, args.clone());

    // Run karabinux and map events.
    loop {
        match i_rx.recv() {
            Ok(Event::KeyEvent(ev)) => {
                match ev.event_type {
                    // These are optional and can be ignored.
                    // https://www.kernel.org/doc/html/v4.17/input/event-codes.html
                    EventType::EV_MSC => continue,

                    // Ignore all received synchronize events, since we send our own.
                    EventType::EV_SYN => continue,

                    // Handle key events by transforming them via the state.
                    EventType::EV_KEY => {
                        for event in state.get_mapped_events(ev) {
                            o_tx.send(event).unwrap();
                        }
                    }

                    // Ignore anything else.
                    _ => {}
                }
            }
            Ok(Event::Timeout) => {}
            Err(e) => {
                eprintln!("{:?}", e);
                process::exit(1);
            }
        }
    }
}
