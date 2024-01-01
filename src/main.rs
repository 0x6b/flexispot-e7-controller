use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use rppal::uart::{Parity, Uart};

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Adjust the desk upwards
    Up,

    /// Adjust the desk downwards
    Down,

    /// The position of the standing height saved
    Standing,

    /// The position of the sitting height saved
    Sitting,

    /// Position 1, first height position saved
    Preset1,

    /// Position 2, second height position saved
    Preset2,

    /// Position 3, alias for "standing" position
    Preset3,

    /// Position 4, alias for "sitting" position
    Preset4,
}

impl Command {
    fn command(&self) -> [u8; 8] {
        use Command::*;
        match *self {
            Up => [0x9b, 0x06, 0x02, 0x01, 0x00, 0xfc, 0xa0, 0x9d],
            Down => [0x9b, 0x06, 0x02, 0x02, 0x00, 0x0c, 0xa0, 0x9d],
            Standing | Preset3 => [0x9b, 0x06, 0x02, 0x10, 0x00, 0xac, 0xac, 0x9d],
            Sitting | Preset4 => [0x9b, 0x06, 0x02, 0x00, 0x01, 0xac, 0x60, 0x9d],
            Preset1 => [0x9b, 0x06, 0x02, 0x04, 0x00, 0xac, 0xa3, 0x9d],
            Preset2 => [0x9b, 0x06, 0x02, 0x08, 0x00, 0xac, 0xa6, 0x9d],
        }
    }
}

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,

    #[clap(long, default_value = "/dev/ttyS0")]
    device: PathBuf,
}

#[derive(Debug)]
pub struct FlexispotE7Controller {
    uart: Uart,
}

impl FlexispotE7Controller {
    pub fn try_new() -> Result<Self, Box<dyn Error>> {
        FlexispotE7Controller::try_new_with_path("/dev/ttyS0")
    }

    pub fn try_new_with_path(path: impl Into<PathBuf>) -> Result<Self, Box<dyn Error>> {
        let uart = Uart::with_path(path.into(), 9600, Parity::None, 8, 1)?;
        Ok(Self { uart })
    }

    pub fn execute(&mut self, command: &Command) -> Result<usize, Box<dyn Error>> {
        Ok(self.uart.write(&command.command())?)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { device, command } = Args::parse();

    let mut controller = FlexispotE7Controller::try_new_with_path(device)?;
    controller.execute(&command)?;

    Ok(())
}
