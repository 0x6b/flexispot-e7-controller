use std::error::Error;

use clap::Parser;
use flexispot_e7_controller_lib::FlexispotE7Controller;

use crate::{
    args::Args,
    command::Command::{Down, Go, Query, Set, Up},
};

mod args;
mod command;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { device, pin20, command } = Args::parse();

    let mut controller = FlexispotE7Controller::try_new_with(device, pin20)?;
    match command {
        Up { diff } => controller.up(diff)?,
        Down { diff } => controller.down(diff)?,
        Go { preset } => match preset {
            command::Preset::Standing => controller.standing()?,
            command::Preset::Sitting => controller.sitting()?,
            command::Preset::Preset1 => controller.preset1()?,
            command::Preset::Preset2 => controller.preset2()?,
            command::Preset::Preset3 => controller.preset3()?,
            command::Preset::Preset4 => controller.preset4()?,
        },
        Set { height } => controller.set(height)?,
        Query => {
            let height = controller.query()?;
            println!("Current height: {height} cm");
        }
    };

    Ok(())
}
