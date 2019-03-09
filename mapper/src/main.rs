mod args;

use args::Args;
use input_linux::{EventKind, InputEvent};
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
                Ok(Event::KeyEvent(raw_event)) => {
                    let mut ev = *InputEvent::from_raw(&raw_event).unwrap();

                    match ev.kind {
                        // These are optional and can be ignored.
                        // https://www.kernel.org/doc/html/v4.17/input/event-codes.html
                        EventKind::Misc => continue,

                        // Ignore all received synchronize events, since we send our own.
                        EventKind::Synchronize => continue,

                        // TODO: should be able to block key repeats (in between down and up)
                        // TODO: simultaneous modifications
                        // TODO: handle mouse actions
                        EventKind::Key => {
                            // https://pqrs.org/osx/karabiner/document.html#event-modification-chaining

                            state.apply_simple_modifications(&mut ev);

                            for emitted_ev in state.apply_complex_modifications(ev) {
                                o_tx.send(*emitted_ev.as_raw()).unwrap();
                            }

                            state.update_modifiers(&ev);
                        }

                        // Ignore anything else.
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
