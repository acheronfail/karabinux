use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Args {
    /// Activate debug mode.
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    /// An input device from /dev/input/*/*.
    #[structopt(short = "v", long = "device", parse(from_os_str))]
    pub device: PathBuf,
}
