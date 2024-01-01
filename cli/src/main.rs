use std::error::Error;

use clap::Parser;
use flexispot_e7_controller_lib::FlexispotE7Controller;

use crate::{
    args::Args,
    command::Command::{Down, Preset1, Preset2, Preset3, Preset4, Query, Sitting, Standing, Up},
};

mod args;
mod command;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { device, pin20, command } = Args::parse();

    let mut controller = FlexispotE7Controller::try_new_with(device, pin20)?;
    match command {
        Up => controller.up()?,
        Down => controller.down()?,
        Standing => controller.standing()?,
        Sitting => controller.sitting()?,
        Preset1 => controller.preset1()?,
        Preset2 => controller.preset2()?,
        Preset3 => controller.preset3()?,
        Preset4 => controller.preset4()?,
        Query => {
            let height = controller.query()?;
            println!("Current height: {height} cm");
        }
    };

    Ok(())
}
