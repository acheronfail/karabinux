use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Args {
    /// Activate debug mode.
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    /// Path to a Karabiner config file.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config: PathBuf,
}
