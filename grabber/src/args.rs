use std::path::PathBuf;
use structopt::StructOpt;

/// Arguments for the grabber process.
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
pub struct Args {
    /// Grab events from the device.
    #[structopt(short = "g", long = "grab")]
    pub grab: bool,

    /// Create a GTK window which views the mapped events.
    #[cfg(feature = "viewer")]
    #[structopt(short = "v", long = "viewer")]
    pub viewer: bool,

    /// An input device from /dev/input/*/*.
    #[structopt(short = "d", long = "device", parse(from_os_str))]
    pub device: PathBuf,
}
