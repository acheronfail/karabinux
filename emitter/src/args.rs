use std::path::PathBuf;
use structopt::StructOpt;

/// Arguments for the emitter process.
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Args {
    /// An input device from /dev/input/*/*.
    #[structopt(short = "d", long = "device", parse(from_os_str))]
    pub device: PathBuf,
}
