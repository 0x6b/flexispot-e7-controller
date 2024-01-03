use std::{env::var, error::Error, net::IpAddr, time::Duration};

use clap::Parser;
use command::Command;
use flexispot_e7_controller_web::{RequestPayload, ResponsePayload};

mod command;
#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// Path to serial device
    #[clap(long, default_value = "192.168.68.52")]
    pub address: IpAddr,

    /// GPIO (BCM) number of PIN 20
    #[clap(long, default_value = "8000")]
    pub port: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let secret = var("E7_SECRET").expect("Specify secret with E7_SECRET environment variable");
    let Args { command, address, port } = Args::parse();
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
            println!("{height:?}");
        }
        _ => {
            client
                .post(&format!("http://{address}:{port}/"))
                .set("Authorization", &secret)
                .send_json(RequestPayload { command: command.into() })?;
        }
    }
    Ok(())
}
