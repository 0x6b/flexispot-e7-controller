use std::{error::Error, path::PathBuf, thread, time::Duration};

use rppal::{
    gpio::{Gpio, OutputPin},
    uart::{Parity, Uart},
};

use crate::command::Command;

mod command;

#[derive(Debug)]
pub struct FlexispotE7Controller {
    uart: Uart,
    pin: OutputPin,
}

impl Default for FlexispotE7Controller {
    fn default() -> Self {
        FlexispotE7Controller::try_new_with("/dev/ttyS0", 12).unwrap()
    }
}

impl FlexispotE7Controller {
    pub fn try_new_with(path: impl Into<PathBuf>, pin20: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            uart: Uart::with_path(path.into(), 9600, Parity::None, 8, 1)?,
            pin: Gpio::new()?.get(pin20)?.into_output(),
        })
    }

    fn execute(&mut self, command: &Command) -> Result<(), Box<dyn Error>> {
        match self.uart.write(&command.command()) {
            Ok(_) => Ok(()),
            Err(why) => {
                let error: Box<dyn Error> = why.to_string().into();
                return Err(error);
            }
        }
    }

    pub fn up(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Up)?)
    }

    pub fn down(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Down)?)
    }

    pub fn standing(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Standing)?)
    }

    pub fn sitting(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Sitting)?)
    }

    pub fn preset1(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Preset1)?)
    }

    pub fn preset2(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Preset2)?)
    }

    pub fn preset3(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Preset3)?)
    }

    pub fn preset4(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.execute(&Command::Preset4)?)
    }

    fn decode_seven_segment(byte: u8) -> (i32, bool) {
        let binary_byte = format!("{:08b}", byte);
        let decimal = &binary_byte[0..1] == "1";

        match &binary_byte[1..] {
            "0111111" => (0, decimal),
            "0000110" => (1, decimal),
            "1011011" => (2, decimal),
            "1001111" => (3, decimal),
            "1100110" => (4, decimal),
            "1101101" => (5, decimal),
            "1111101" => (6, decimal),
            "0000111" => (7, decimal),
            "1111111" => (8, decimal),
            "1101111" => (9, decimal),
            "1000000" => (10, decimal),
            _ => (-1, decimal),
        }
    }

    pub fn query(&mut self) -> Result<i32, Box<dyn Error>> {
        self.pin.set_high();
        thread::sleep(Duration::from_millis(300));
        self.pin.set_low();

        // Command::WakeUp should work, but my unit/environment won't return current hight reliably
        // self.execute(&Command::WakeUp)?;

        // So use Command::Memory instead
        self.execute(&Command::Memory)?;

        self.uart.set_read_mode(1, Duration::default())?;
        let mut data = [0u8; 1];
        let mut history = [0u8; 5];
        let mut msg_type = 0;
        let mut msg_len = 0;
        let mut valid = false;

        loop {
            if self.uart.read(&mut data)? > 0 {
                if history[0] == 0x9b {
                    msg_len = data[0];
                }
                if history[1] == 0x9b {
                    msg_type = data[0];
                }
                if history[2] == 0x9b {
                    if msg_type == 0x12 && msg_len == 7 {
                        if data[0] == 0 {
                            return Err("height is empty".into());
                        } else {
                            valid = true;
                        }
                    }
                }
                if history[3] == 0x9b {
                    if valid {
                        history[4] = history[3];
                        history[3] = history[2];
                        history[2] = history[1];
                        history[1] = history[0];
                        history[0] = data[0];
                        continue;
                    }
                }
                if history[4] == 0x9b {
                    if valid && msg_len == 7 {
                        let (height1, decimal1) = Self::decode_seven_segment(history[1]);
                        let height1 = height1 * 100;
                        let (height2, decimal2) = Self::decode_seven_segment(history[0]);
                        let height2 = height2 * 10;
                        let (height3, decimal3) = Self::decode_seven_segment(data[0]);
                        if height1 < 0 || height2 < 0 || height3 < 0 {
                            println!("Display empty");
                        } else {
                            let mut height = height1 + height2 + height3;
                            let decimal = decimal1 || decimal2 || decimal3;
                            if decimal {
                                height = height / 10;
                            }
                            return Ok(height);
                        }
                    }
                }
                history[4] = history[3];
                history[3] = history[2];
                history[2] = history[1];
                history[1] = history[0];
                history[0] = data[0];
            }
        }
    }
}
