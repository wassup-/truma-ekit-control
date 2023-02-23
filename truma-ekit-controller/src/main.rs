mod ekit;
mod heating;
mod thermometer;

use ekit::EKitRunMode;
use esp_idf_hal::{
    adc::{config::Config, AdcDriver},
    gpio::PinDriver,
    prelude::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use heating::HeatingCoil;
use std::time::Duration;
use truma_ekit_core::{
    adc::AdcInputPin,
    gpio::DigitalOutputPin,
    peripherals::{fan::Fan, relay::Relay, tmp36::TMP36},
    util::{celsius, format_temperature},
};

const SLEEP_DURATION: Duration = Duration::from_secs(1);

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut requested_temperature = celsius(20.5);
    let mut actual_temperature = celsius(20.5);

    let adc1 = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;
    let gpio2 = peripherals.pins.gpio2;
    let gpio7 = PinDriver::output(peripherals.pins.gpio7)?;
    let gpio8 = PinDriver::output(peripherals.pins.gpio8)?;
    let gpio9 = PinDriver::output(peripherals.pins.gpio9)?;

    let mut ekit = ekit::EKit::new(
        Fan::new(Relay::connected_to(DigitalOutputPin::pin(gpio7))),
        HeatingCoil::new(Relay::connected_to(DigitalOutputPin::pin(gpio8))),
        HeatingCoil::new(Relay::connected_to(DigitalOutputPin::pin(gpio9))),
        Box::new(TMP36::connected_to(AdcInputPin::pin(gpio2, adc1))),
    );

    loop {
        // TODO: update the requested temperature
        // TODO: update the actual temperature

        log::info!(
            "requested temperature: {}",
            format_temperature(&requested_temperature)
        );
        log::info!(
            "actual temperature: {}",
            format_temperature(&actual_temperature)
        );

        if actual_temperature >= requested_temperature {
            // the current temperature is equal to or higher than the requested temperature, turn off the heating
            ekit.set_run_mode(EKitRunMode::Off);
        } else {
            // the current temperature is less than the requested temperature, turn on the heating
            let temp_diff = requested_temperature.clone() - actual_temperature.clone();
            if temp_diff < celsius(1.5) {
                // run the heating at half capacity
                ekit.set_run_mode(EKitRunMode::Half);
            } else {
                // run the heating at full capacity
                ekit.set_run_mode(EKitRunMode::Full);
            }
        }

        std::thread::sleep(SLEEP_DURATION);
    }
}
