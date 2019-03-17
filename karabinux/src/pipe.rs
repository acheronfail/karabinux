use crate::event::Event;
use crate::util::*;
use evdev_rs::InputEvent;
use std::io::{self, Read, Write};
use std::mem;
use std::slice;
use std::process;
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
        match read_struct::<libc::input_event>(&mut stdin_handle) {
            Ok(ev) => {
                let ev = InputEvent::from_raw(&ev);
                let ev = Event::KeyEvent(ev);
                i_tx.send(ev).unwrap();
            },
            Err(e) => match e.kind() {
                io::ErrorKind::UnexpectedEof => process::exit(1),
                error => panic!("failed to read event from stdin: {:?}", error),
            },
        }
    }
}

// Writer thread: receives structs from a Receiver, and writes them to stdout.
pub fn writer_thread(o_rx: Receiver<libc::input_event>) {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    loop {
        match o_rx.recv() {
            Ok(raw_event) => {
                write_struct::<libc::input_event>(&mut stdout_handle, &raw_event).unwrap();
                write_struct::<libc::input_event>(&mut stdout_handle, &sync_event_now()).unwrap();
            },
            Err(e) => panic!("failed to write event to stdout: {:?}", e)
        }
    }
}
