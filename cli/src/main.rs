use clap::Parser;
use command::Mode;

mod command;
mod local;
mod remote;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub mode: Mode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { mode } = Args::parse();

    match mode {
        #[cfg(all(target_os = "linux", target_arch = "arm"))]
        Mode::Local { command, device, pin20 } => local::execute(command, device, pin20)?,
        Mode::Remote { command, address, port, secret } => {
            remote::execute(command, address, port, secret)?
        }
    }
    Ok(())
}
