mod ekit;
mod heating;
mod peripherals;

use ekit::{EKit, EKitLocal};
use esp_idf_hal::{
    adc::{AdcConfig, AdcDriver, Atten11dB},
    gpio::PinDriver,
};
use esp_idf_sys as _;
use heating::HeatingCoil;
use peripherals::SystemPeripherals;
use std::time::Duration;
use truma_ekit_core::{
    adc::AdcInputPin,
    ekit::EKitRunMode,
    peripherals::{fan::Fan, relay::Relay, tmp36::TMP36},
};

const SLEEP_DURATION: Duration = Duration::from_secs(1);

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = SystemPeripherals::take();

    let ekit = EKitLocal::new(
        Fan::new(Relay::connected_to(PinDriver::output(
            peripherals.fan.power,
        )?)),
        HeatingCoil::new(Relay::connected_to(PinDriver::output(
            peripherals.coil1.power,
        )?)),
        HeatingCoil::new(Relay::connected_to(PinDriver::output(
            peripherals.coil2.power,
        )?)),
    );

    let mut runner = EKitRunner::new(
        ekit,
        TMP36::connected_to(AdcInputPin::pin::<_, _, Atten11dB<_>>(
            peripherals.thermometer.voltage,
            AdcDriver::new(
                peripherals.thermometer.adc,
                &AdcConfig::new().calibration(true),
            )?,
        )),
    );

    loop {
        runner.run()?;
        std::thread::sleep(SLEEP_DURATION);
    }
}

struct EKitRunner<'a, E: EKit> {
    ekit: E,
    output: TMP36<'a>,
    run_mode: EKitRunMode,
}

impl<'a, E: EKit> EKitRunner<'a, E> {
    pub fn new(ekit: E, output: TMP36<'a>) -> Self {
        EKitRunner {
            ekit,
            output,
            run_mode: EKitRunMode::Off,
        }
    }

    /// Run the e-kit.
    pub fn run(&mut self) -> anyhow::Result<()> {
        let output_temperature = self.output.measure_temperature().ok();
        self.ekit.set_output_temperature(output_temperature);
        // TODO: update run mode
        self.ekit.request_run_mode(self.run_mode);
        Ok(())
    }
}
