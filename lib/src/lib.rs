use std::{error::Error, path::PathBuf};

use rppal::uart::{Parity, Uart};

use crate::command::Command;

mod command;

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

    fn execute(&mut self, command: &Command) -> Result<usize, Box<dyn Error>> {
        Ok(self.uart.write(&command.command())?)
    }

    pub fn up(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Up)?)
    }

    pub fn down(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Down)?)
    }

    pub fn standing(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Standing)?)
    }

    pub fn sitting(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Sitting)?)
    }

    pub fn preset1(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Preset1)?)
    }

    pub fn preset2(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Preset2)?)
    }

    pub fn preset3(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Preset3)?)
    }

    pub fn preset4(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(self.execute(&Command::Preset4)?)
    }
}
