use evdev_rs::enums::{EventCode, EventType};
use evdev_rs::ReadFlag;
use evdev_rs::{InputEvent, TimeVal};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, WindowPosition};
use karabinux::key_state::KeyState;
use karabinux::util::find_karabinux_uinput_device;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

const GRID_BLOCK: u32 = 16;
const APPLICATION_ID: &'static str = "com.acheronfail.karabinux";
const APPLICATION_NAME: &'static str = "karabinux event viewer";

struct Column {
    index: i32,
    title: String,
    kind: gtk::Type,
}

impl Column {
    fn new(index: i32, title: &str, kind: gtk::Type) -> Column {
        Column {
            index,
            title: title.to_string(),
            kind,
        }
    }
}

fn format_event_code(code: &EventCode) -> String {
    match code {
        EventCode::EV_KEY(x) => format!("{:?}", x),
        EventCode::EV_MSC(x) => format!("{:?}", x),
        EventCode::EV_SYN(x) => format!("{:?}", x),
        _ => format!("{:?}", code),
    }
}

fn format_timeval(timeval: &TimeVal) -> String {
    use chrono::offset::TimeZone;

    let datetime = chrono::Local.timestamp(timeval.tv_sec, timeval.tv_usec as u32);
    datetime.to_rfc2822()
}

fn add_event_to_list_store(list_store: &gtk::ListStore, ev: &InputEvent) {
    match ev.event_type {
        EventType::EV_KEY => {
            list_store.set(
                &list_store.prepend(),
                &[0, 1, 2, 3],
                &[
                    &format!("{:?}", KeyState::from(ev.value)),
                    &format!("{:?}", ev.event_type),
                    &format_event_code(&ev.event_code),
                    &format_timeval(&ev.time),
                ],
            );
        }
        // TODO: support logging other events as well (checkboxes?)
        _ => {}
    }
}

fn build_table(title: &str, parent_box: &gtk::Box) -> gtk::ListStore {
    // The columns of the table.
    let columns: [Column; 4] = [
        Column::new(0, "Key State", gtk::Type::String),
        Column::new(1, "Event Type", gtk::Type::String),
        Column::new(2, "Event Code", gtk::Type::String),
        Column::new(3, "Time", gtk::Type::String),
    ];

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, GRID_BLOCK as i32);
    parent_box.add(&vbox);

    // Add label to vertical container.
    let label = Label::new(Some(title));
    vbox.add(&label);

    // Ensure the scroller is always scrolled to the top.
    let scroller_vadjustment = gtk::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    scroller_vadjustment.connect_changed(|adj| adj.set_value(0.0));

    // Add a scrollable region to the vertical container.
    let scroller = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, &scroller_vadjustment);
    scroller.set_shadow_type(gtk::ShadowType::EtchedIn);
    scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    vbox.add(&scroller);

    // Create table to store original events.
    let column_types = columns.iter().map(|c| c.kind).collect::<Vec<_>>();
    let list_store = gtk::ListStore::new(&column_types[..]);
    let treeview = gtk::TreeView::new_with_model(&list_store);
    treeview.set_hexpand(true);
    treeview.set_vexpand(true);
    scroller.add(&treeview);

    // Setup the TreeView's columns.
    for col in columns.iter() {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title(&col.title);
        column.add_attribute(&renderer, "text", col.index);
        treeview.append_column(&column);
    }

    list_store
}

fn build_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title(APPLICATION_NAME);
    window.set_border_width(GRID_BLOCK);
    window.set_default_size(640, 480);
    window.set_position(WindowPosition::Center);
    window.connect_key_press_event(|_, _| gtk::Inhibit(true));
    window
}

fn attach_store_receiver(rx: glib::Receiver<InputEvent>, list_store: gtk::ListStore) {
    rx.attach(
        None,
        move |ev| {
            add_event_to_list_store(&list_store, &ev);
            glib::Continue(true)
        },
    );
}

pub fn create_gtk_application(event_receiver: Receiver<InputEvent>) {
    // In order to send the Receiver into GTK's main loop (an `Fn` closure) we
    // need to wrap it in a thread safe container, and allow it to be sent
    // and accessed across threads safely.
    let event_receiver = Arc::new(Mutex::new(Some(event_receiver)));

    gtk::init().expect("failed to initialise gtk");

    let app = Application::new(APPLICATION_ID, Default::default())
        .expect("failed to start application");

    app.connect_activate(move |app| {
        // Create main window.
        let window = build_window(&app);

        // Create a vertical container in window.
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, GRID_BLOCK as i32);
        window.add(&vbox);

        // Create a horizontal container for the tables.
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, GRID_BLOCK as i32);
        vbox.add(&hbox);

        // Attach a device to the original keyboard.
        let ev_store = build_table("original", &hbox);
        {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            thread::spawn(clone!(event_receiver => move || {
                let event_receiver = event_receiver.lock().unwrap();
                let event_receiver = event_receiver.as_ref().unwrap();
                loop {
                    let ev = event_receiver.recv().expect("failed to read event");
                    tx.send(ev).expect("failed to send event");
                }
            }));

            attach_store_receiver(rx, ev_store.clone());
        }

        // Attach a device to the virtual uinput device.
        let kb_store = build_table("karabinux", &hbox);
        {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            thread::spawn(move || {
                let device =
                    find_karabinux_uinput_device().expect("failed to find karabinux device");
                loop {
                    let (_, ev) = device
                        .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
                        .expect("failed to read event");
                    tx.send(ev).expect("failed to send event");
                }
            });

            attach_store_receiver(rx, kb_store.clone());
        }

        // Add a button to clear the log.
        let clear_button = gtk::Button::new_with_label("clear");
        clear_button.connect_clicked(clone!(ev_store, kb_store => move |_| {
            ev_store.clear();
            kb_store.clear();
        }));
        vbox.add(&clear_button);

        // Show the window.
        window.show_all();
    });

    // Run the GTK application.
    app.run(&[]);

    // Close the entire program when this is closed.
    std::process::exit(0);
}
