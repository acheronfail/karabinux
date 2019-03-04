extern crate serde_derive;
extern crate serde_json;
extern crate input_linux;

mod config;
mod kb_key;
mod pipe;
mod state;
mod util;

use std::fmt;
use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;
use input_linux::{InputEvent, EventKind};
use input_linux::sys::input_event;
use crate::config::KBConfig;
use crate::state::State;

pub enum Event {
    Timeout,
    KeyEvent(input_event),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Event::Timeout => write!(f, "Timeout"),
            &Event::KeyEvent(ev) => {
                let inp_ev = InputEvent::from_raw(&ev).unwrap();
                write!(f, "KeyEvent({:?})", &inp_ev)
            }
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Config file argument required");
        process::exit(1);
    }

    let kb_config = KBConfig::from_path(&args[1]).expect("failed to construct config");
    let selected_profile = kb_config.profiles
        .iter()
        .find(|p| p.selected)
        .expect("failed to find selected profile");
    let state = State::from_profile(&selected_profile);

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

                        // TODO:
                        //  should handle actions on key_down, key_up, etc.
                        //  should be able to block key repeats (in between down and up)
                        EventKind::Key => {
                            // TODO: complex modifications
                            // TODO: simultaneous modifications
                            // TODO: handle mouse actions as well?

                            // TODO: order of operations
                            // https://pqrs.org/osx/karabiner/document.html#event-modification-chaining

                            state.apply_simple_modifications(&mut ev);

                            o_tx.send(*ev.as_raw()).unwrap();
                        },

                        // Ignore anything else.
                        _ => {}
                    }
                },
                Ok(Event::Timeout) => {},
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
