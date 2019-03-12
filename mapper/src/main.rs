mod args;

use args::Args;
use evdev_rs::enums::EventType;
use karabinux::event::Event;
use karabinux::karabiner::KBConfig;
use karabinux::pipe;
use karabinux::state::StateManager;
use std::process;
use std::sync::mpsc;
use std::thread;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    let kb_config = KBConfig::from_path(&args.config).expect("failed to construct config");
    let selected_profile = kb_config
        .profiles
        .iter()
        .find(|p| p.selected)
        .expect("failed to find selected profile");

    let mut state = StateManager::from_profile(&selected_profile);

    // Input channel: reads events from stdin.
    let (i_tx, i_rx) = mpsc::channel();
    // Output channel: writes event to stdout.
    let (o_tx, o_rx) = mpsc::channel();

    {
        let i_tx = i_tx.clone();
        thread::spawn(move || pipe::reader_thread(i_tx));
        thread::spawn(move || pipe::writer_thread(o_rx));
    }

    let logic_handle = thread::spawn(move || {
        loop {
            match i_rx.recv() {
                Ok(Event::KeyEvent(ev)) => {
                    match ev.event_type {
                        // These are optional and can be ignored.
                        // https://www.kernel.org/doc/html/v4.17/input/event-codes.html
                        EventType::EV_MSC => continue,

                        // Ignore all received synchronize events, since we send our own.
                        EventType::EV_SYN => continue,

                        // Handle key events by sending them to the state.
                        EventType::EV_KEY => state
                            .get_mapped_events(ev)
                            .iter()
                            .for_each(|ev| o_tx.send(ev.as_raw()).unwrap()),

                        // Ignore anything else.
                        // TODO: handle mouse actions?
                        _ => {}
                    }
                }
                // TODO: handle timeouts when needed
                Ok(Event::Timeout) => {}
                Err(e) => {
                    eprintln!("{:?}", e);
                    process::exit(1);
                }
            }
        }
    });

    // Wait for the logic thread to terminate (it won't, so this waits forever)
    let result = logic_handle.join();
    eprintln!("{:?}", result);
}
