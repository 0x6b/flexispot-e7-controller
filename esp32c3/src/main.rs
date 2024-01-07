use anyhow::{bail, Result};
use esp_idf_svc::{
    hal::{
        delay::BLOCK,
        gpio::{Gpio0, Gpio1},
        peripherals::Peripherals,
        uart::{
            config,
            config::{DataBits::DataBits8, StopBits},
            UartDriver,
        },
        units::Hertz,
    },
    log::EspLogger,
    sys,
};
use flexispot_e7_controller_lib::Command;

fn main() -> Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let tx = peripherals.pins.gpio4;
    let rx = peripherals.pins.gpio5;

    let config = config::Config::new().baudrate(Hertz(9600)).queue_size(8);
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<Gpio0>::None,
        Option::<Gpio1>::None,
        &config,
    )?;

    let seq: [u8; 8] = (&Command::Up { diff: None }).into();
    uart.write(&seq)?;
    let height = query(&uart)?;
    log::info!("height: {height}");
    Ok(())
}

fn query(uart: &UartDriver) -> Result<f32> {
    let mut data = [0u8; 1];
    let mut history = [0u8; 5];
    let mut msg_type = 0;
    let mut msg_len = 0;
    let mut valid = false;

    loop {
        if uart.read(&mut data, BLOCK)? > 0 {
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
                        bail!("height is empty");
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
                    return decode(history[1], history[0], data[0]);
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

fn decode(b0: u8, b1: u8, b2: u8) -> Result<f32> {
    let (height1, decimal1) = decode_seven_segment(b0);
    let (height2, decimal2) = decode_seven_segment(b1);
    let (height3, decimal3) = decode_seven_segment(b2);

    let height1 = height1 * 100;
    let height2 = height2 * 10;

    if height1 < 0 || height2 < 0 || height3 < 0 {
        bail!("Display empty")
    }

    let mut height: f32 = height1 as f32 + height2 as f32 + height3 as f32;

    if decimal1 || decimal2 || decimal3 {
        height /= 10f32;
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
