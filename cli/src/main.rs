use std::error::Error;

use clap::Parser;
use flexispot_e7_controller_lib::FlexispotE7Controller;

use crate::{args::Args, command::Command::Query};

mod args;
mod command;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { device, pin20, command } = Args::parse();

    let mut controller = FlexispotE7Controller::try_new_with(device, pin20)?;
    match command {
        Query => {
            let height = controller.query()?;
            println!("Current height: {height} cm");
        }
        _ => controller.execute(&command.into())?,
    };

    Ok(())
}
