mod ekit;
mod heating;
mod peripherals;
mod thermometer;

use ekit::EKitRunMode;
use esp_idf_hal::{
    adc::{AdcConfig, AdcDriver, Atten11dB},
    gpio::PinDriver,
};
use esp_idf_sys as _;
use heating::HeatingCoil;
use peripherals::SystemPeripherals;
use std::time::Duration;
use thermometer::MemoizeTemperature;
use truma_ekit_core::{
    adc::AdcInputPin,
    peripherals::{fan::Fan, relay::Relay, tmp36::TMP36},
};

const SLEEP_DURATION: Duration = Duration::from_secs(1);

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = SystemPeripherals::take();

    let mut ekit = ekit::EKit::new(
        Fan::new(Relay::connected_to(PinDriver::output(
            peripherals.fan.power,
        )?)),
        HeatingCoil::new(Relay::connected_to(PinDriver::output(
            peripherals.coil1.power,
        )?)),
        HeatingCoil::new(Relay::connected_to(PinDriver::output(
            peripherals.coil2.power,
        )?)),
        Box::new(MemoizeTemperature::new(TMP36::connected_to(
            AdcInputPin::pin::<_, _, Atten11dB<_>>(
                peripherals.thermometer.voltage,
                AdcDriver::new(
                    peripherals.thermometer.adc,
                    &AdcConfig::new().calibration(true),
                )?,
            ),
        ))),
    );

    let mut requested_run_mode = EKitRunMode::Off;

    loop {
        // TODO: update requested run mode
        ekit.set_run_mode(requested_run_mode);

        std::thread::sleep(SLEEP_DURATION);
    }
}
