use std::{error::Error, path::PathBuf, thread, time::Duration};

pub use command::{Command, Preset};
use command::{
    Command::{Down, Go, Memory, Query, Set, Up, WakeUp},
    CommandSequence,
};
use rppal::{
    gpio::{Gpio, OutputPin},
    uart::{Parity, Uart},
};

mod command;

#[derive(Debug)]
pub struct Controller {
    uart: Uart,
    pin: OutputPin,
}

impl Controller {
    pub fn try_new() -> Result<Self, Box<dyn Error>> {
        Controller::try_new_with("/dev/ttyS0", 12)
    }

    pub fn try_new_with(path: impl Into<PathBuf>, pin20: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            uart: Uart::with_path(path.into(), 9600, Parity::None, 8, 1)?,
            pin: Gpio::new()?.get(pin20)?.into_output(),
        })
    }

    pub fn execute(&mut self, command: &Command) -> Result<(), Box<dyn Error>> {
        let seq = CommandSequence::from(command);
        match command {
            Up { diff } | Down { diff } => {
                for _ in 0..Self::to_loop_count(*diff) {
                    self.uart.write(&seq)?;
                }
            }
            Go { .. } | WakeUp | Memory => {
                self.uart.write(&seq)?;
            }
            Set { height } => {
                let current = self.query()? as f32;
                let target = Self::normalize(*height);
                let diff = target - current;

                if diff > 0f32 {
                    self.execute(&Up { diff: Some(diff) })?;
                } else {
                    self.execute(&Down { diff: Some(diff) })?;
                }
            }
            Query => unreachable!("unreachable"),
        }

        Ok(())
    }

    pub fn query(&mut self) -> Result<i32, Box<dyn Error>> {
        // Wake up controller to return current height. I'm not 100% sure I need this though.
        self.pin.set_high();
        thread::sleep(Duration::from_millis(100));
        self.pin.set_low();

        // WakeUp should work, but my unit/environment won't return current hight reliably
        // self.execute(&WakeUp)?;

        // So use Memory instead
        self.execute(&Memory)?;

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
                #[allow(clippy::collapsible_if)]
                if history[2] == 0x9b {
                    if msg_type == 0x12 && msg_len == 7 {
                        if data[0] == 0 {
                            return Err("height is empty".into());
                        } else {
                            valid = true;
                        }
                    }
                }
                #[allow(clippy::collapsible_if)]
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
                #[allow(clippy::collapsible_if)]
                if history[4] == 0x9b {
                    if valid && msg_len == 7 {
                        return Self::decode(history[1], history[0], data[0]);
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

    fn decode(b0: u8, b1: u8, b2: u8) -> Result<i32, Box<dyn Error>> {
        let (height1, decimal1) = Self::decode_seven_segment(b0);
        let (height2, decimal2) = Self::decode_seven_segment(b1);
        let (height3, decimal3) = Self::decode_seven_segment(b2);

        let height1 = height1 * 100;
        let height2 = height2 * 10;

        if height1 < 0 || height2 < 0 || height3 < 0 {
            return Err("Display empty".into());
        }

        let mut height = height1 + height2 + height3;

        if decimal1 || decimal2 || decimal3 {
            height /= 10;
        }
        Ok(height)
    }

    fn decode_seven_segment(byte: u8) -> (i32, bool) {
        (
            match byte & 0b0111_1111 {
                0b0011_1111 => 0,
                0b0000_0110 => 1,
                0b0101_1011 => 2,
                0b0100_1111 => 3,
                0b0110_0110 => 4,
                0b0110_1101 => 5,
                0b0111_1101 => 6,
                0b0000_0111 => 7,
                0b0111_1111 => 8,
                0b0110_1111 => 9,
                0b0100_0000 => 10,
                _ => -1,
            },
            byte & 0b1000_0000 != 0,
        )
    }

    fn to_loop_count(diff: Option<f32>) -> usize {
        // 29 is determined heuristically, so it may not be accurate for every setup.
        match diff {
            Some(v) => (v.abs() * 29f32).ceil() as usize,
            None => 1,
        }
    }

    fn normalize(v: f32) -> f32 {
        if v < 60.5 {
            return 60.5;
        } else if v > 126.0 {
            return 126.0;
        }
        v
    }
}
