use std::{env::var, error::Error, time::Duration};

use clap::Parser;
use command::{Command, Mode};
use flexispot_e7_controller_lib::Controller;
use flexispot_e7_controller_web::{RequestPayload, ResponsePayload};

use crate::command::Command::Query;

mod command;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub mode: Mode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { mode } = Args::parse();

    match mode {
        command::Mode::Local { command, device, pin20 } => {
            let mut controller = Controller::try_new_with(device, pin20)?;
            match command {
                Query => {
                    let height = controller.query()?;
                    println!("Current height: {height} cm");
                }
                _ => controller.execute(&command.into())?,
            };
        }
        command::Mode::Remote { command, address, port } => {
            let secret =
                var("E7_SECRET").expect("Specify secret with E7_SECRET environment variable");
            let client = ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_secs(1))
                .timeout_read(Duration::from_secs(1))
                .timeout_write(Duration::from_secs(1))
                .build();

            match command {
                Command::Query => {
                    let height = client
                        .get(&format!("http://{address}:{port}/query"))
                        .set("Authorization", &secret)
                        .call()?
                        .into_json::<ResponsePayload>()?;
                    println!("Current height: {height} cm");
                }
                _ => {
                    client
                        .post(&format!("http://{address}:{port}/"))
                        .set("Authorization", &secret)
                        .send_json(RequestPayload { command: command.into() })?;
                }
            }
        }
    }

    Ok(())
}
