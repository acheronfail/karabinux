use std::path::PathBuf;
use structopt::StructOpt;

/// Arguments for the mapper process.
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Args {
    /// Path to a Karabiner config file.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config: PathBuf,
}
