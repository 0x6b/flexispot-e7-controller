#[cfg(all(target_os = "linux", target_arch = "arm"))]
use {
    crate::command::Command,
    flexispot_e7_controller_lib::Controller,
    std::{error::Error, path::PathBuf},
};

#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub fn execute(command: Command, device: PathBuf, pin20: u8) -> Result<(), Box<dyn Error>> {
    let mut controller = Controller::try_new_with(device, pin20)?;
    match command {
        Command::Query => {
            let height = controller.query()?;
            println!("Current height: {height} cm");
        }
        _ => controller.execute(&command.into())?,
    };
}
