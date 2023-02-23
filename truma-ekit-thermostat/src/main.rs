mod ekit;

use ekit::{EKit, EKitRunMode};
use esp_idf_hal::{gpio::PinDriver, i2c::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use truma_ekit_core::{
    gpio::DigitalOutputPin,
    measurement::Measurement,
    peripherals::bme280::BME280,
    types::{Temperature, UnitTemperature},
};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = esp_idf_hal::peripherals::Peripherals::take().unwrap();

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let i2c = I2cDriver::new(i2c, sda, scl, &I2cConfig::default())?;

    let gpio13 = PinDriver::output(peripherals.pins.gpio13)?;
    let mut bme280 = BME280::new(i2c, DigitalOutputPin::pin(gpio13));

    let mut ekit = EKit::new();
    let mut requested_temperature: Temperature = celsius(20.5);

    loop {
        // TODO: update requested temperature

        let measurements = bme280.measure()?;
        let actual_temperature = measurements.temperature;

        let run_mode = if actual_temperature >= requested_temperature.clone() {
            EKitRunMode::Off
        } else {
            let diff = requested_temperature.clone() - actual_temperature;
            if diff < celsius(1.5) {
                EKitRunMode::Half
            } else {
                EKitRunMode::Full
            }
        };

        ekit.set_run_mode(run_mode)?;
    }
}

fn celsius(val: f32) -> Temperature {
    Measurement::new(val, UnitTemperature::celsius())
}
