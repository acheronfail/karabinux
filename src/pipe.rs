use crate::util::*;
use crate::Event;
use input_linux::sys;
use std::io::{self, Read, Write};
use std::mem;
use std::slice;
use std::sync::mpsc::{Receiver, Sender};

// Reads a struct directly from `stdin`.
pub fn read_struct<T>(reader: &mut Read) -> io::Result<T> {
    let mut buffer = vec![0; mem::size_of::<T>()];
    match reader.read_exact(&mut buffer) {
        Ok(_) => Ok(unsafe { std::ptr::read(buffer.as_ptr() as *const T) }),
        Err(e) => Err(e),
    }
}

// Writes a struct directly to `stdout`.
pub fn write_struct<T>(writer: &mut Write, s: &T) -> io::Result<()> {
    let num_bytes = mem::size_of::<T>();
    unsafe {
        let buffer = slice::from_raw_parts(s as *const T as *const u8, num_bytes);
        writer.write(buffer).unwrap();
        writer.flush()
    }
}

// Reader thread: reads structs from stdin, passes them to a Sender.
pub fn reader_thread(i_tx: Sender<Event>) {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    loop {
        if let Ok(s) = read_struct::<sys::input_event>(&mut stdin_handle) {
            i_tx.send(Event::KeyEvent(s)).unwrap();
        }
    }
}

// Writer thread: receives structs from a Receiver, and writes them to stdout.
pub fn writer_thread(o_rx: Receiver<sys::input_event>) {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    loop {
        if let Ok(raw_event) = o_rx.recv() {
            write_struct::<sys::input_event>(&mut stdout_handle, &raw_event).unwrap();
            write_struct::<sys::input_event>(&mut stdout_handle, &sync_event_now()).unwrap();

            #[cfg(debug)]
            {
                use input_linux::{InputEvent, Key};

                let ev = InputEvent::from_raw(&raw_event).unwrap();
                match Key::from_code(ev.code).unwrap() {
                    Key::KeyEsc | Key::KeyGrave => {}
                    _ => log_event(&ev, true),
                }
            }
        }
    }
}
