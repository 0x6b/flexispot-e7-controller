use std::path::PathBuf;

use clap::Parser;

use crate::command::Command;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// Path to serial device
    #[clap(long, default_value = "/dev/ttyS0")]
    pub device: PathBuf,

    /// GPIO (BCM) number of PIN 20
    #[clap(long, default_value = "12")]
    pub pin20: u8,
}
