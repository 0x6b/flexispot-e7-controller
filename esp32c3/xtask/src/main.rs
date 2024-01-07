use std::{
    collections::HashMap, env::var, error::Error, fs::read_to_string, path::PathBuf,
    process::Command,
};

use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Parser)]
#[clap(about, override_usage = "cargo <x|xtask> [OPTIONS] <COMMAND>")]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommand,

    #[clap(long, short, default_value = "xtask-config.toml")]
    config: PathBuf,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Build and flash the program to the board
    Run {
        #[clap(long)]
        release: bool,
    },

    /// Build the program
    Build {
        #[clap(long)]
        release: bool,
    },

    /// Clean the build directory
    Clean,

    /// Check the hardware with `hardware_check.rs`
    HardwareCheck,

    /// Open a serial console
    SerialConsole {
        #[clap(long, default_value = "espflash")]
        espflash_path: String,
    },
}

#[derive(Debug, Deserialize)]
struct Config {
    pub env: Option<HashMap<String, String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { sub_command, config } = Args::parse();
    let config: Config = toml::from_str(&read_to_string(config)?)?;
    println!("{:#?}", config);

    let mut command = match sub_command {
        SubCommand::SerialConsole { ref espflash_path } => Command::new(espflash_path),
        _ => Command::new(var("CARGO").unwrap_or_else(|_| "cargo".to_string())),
    };

    if let Some(env) = config.env {
        env.iter().for_each(|(key, value)| {
            command.env(key, value);
        });
    }

    match sub_command {
        SubCommand::Run { release } => {
            command.arg("run");
            if release {
                command.arg("--release");
            }
        }
        SubCommand::Build { release } => {
            command.arg("build");
            if release {
                command.arg("--release");
            }
        }
        SubCommand::HardwareCheck => {
            command.arg("run").arg("--bin").arg("hardware_check");
        }
        SubCommand::SerialConsole { .. } => {
            command.arg("monitor");
        }
        SubCommand::Clean => {
            command.arg("clean");
        }
    }

    command.status()?;
    Ok(())
}
