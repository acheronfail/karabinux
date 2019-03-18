use std::path::PathBuf;
use structopt::StructOpt;

/// Arguments for Karabinux.
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "karabinux")]
pub struct Args {
    /// Exclusively grab events from the device.
    #[structopt(short = "g", long = "grab")]
    pub grab: bool,

    /// Create a GTK window which views the mapped events.
    #[cfg(feature = "viewer")]
    #[structopt(short = "v", long = "viewer")]
    pub viewer: bool,

    /// An input device from /dev/input/*/*.
    #[structopt(short = "d", long = "device", parse(from_os_str))]
    pub device: PathBuf,

    /// Path to a Karabiner config file.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config: PathBuf,
}
