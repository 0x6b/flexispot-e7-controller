use anyhow::{bail, Result};
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::prelude::Peripherals};
use log::info;
use rgb_led::{RGB8, WS2812RMT};
use wifi::wifi;

mod rgb_led;
mod wifi;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    info!("Hello, world!");

    // Start the LED off yellow
    let mut led = WS2812RMT::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    // Connect to the Wi-Fi network
    let _wifi = match wifi(env!("ESP32C3_SSID"), env!("ESP32C3_PSK"), peripherals.modem, sysloop) {
        Ok(inner) => inner,
        Err(err) => {
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    loop {
        led.set_pixel(RGB8::new(0, 0, 50))?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        info!("Hello, world!");

        led.set_pixel(RGB8::new(0, 50, 0))?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
