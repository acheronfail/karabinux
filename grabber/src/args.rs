use std::path::PathBuf;
use structopt::StructOpt;

/// Arguments for the grabber process.
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Args {
    /// Grab events from the device.
    #[structopt(short = "g", long = "grab")]
    pub grab: bool,

    /// An input device from /dev/input/*/*.
    #[structopt(short = "d", long = "device", parse(from_os_str))]
    pub device: PathBuf,
}
