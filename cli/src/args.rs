use std::path::PathBuf;

use clap::Parser;

use crate::command::Command;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(long, default_value = "/dev/ttyS0")]
    pub device: PathBuf,
}
