mod ekit;
mod heating;
mod overtemperature_protection;
mod peripherals;
mod server;
mod wifi;

use ekit::{EKit, EKitLocal};
use esp_idf_hal::{
    adc::{AdcConfig, AdcDriver, Atten11dB},
    gpio::PinDriver,
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _;
use heating::HeatingCoil;
use peripherals::SystemPeripherals;
use server::EKitHttpServer;
use std::sync::{Arc, Mutex};
use truma_ekit_core::{
    adc::AdcInputPin,
    peripherals::{fan::Fan, relay::Relay, tmp36::TMP36},
    powersaving::Powered,
    types::Temperature,
};
use wifi::WifiAp;

const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_secs(1);

esp_idf_sys::esp_app_desc!();

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = SystemPeripherals::take();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs_default_partition = EspDefaultNvsPartition::take()?;

    let mut wifi_ap = WifiAp::new(peripherals.modem, sysloop, nvs_default_partition)?;
    wifi_ap.start()?;

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

    let tmp36 = TMP36::connected_to(AdcInputPin::pin::<_, _, Atten11dB<_>>(
        peripherals.thermometer.voltage,
        AdcDriver::new(
            peripherals.thermometer.adc,
            &AdcConfig::new().calibration(true),
        )?,
    ));
    let mut tmp36 = Powered::new(
        tmp36,
        PinDriver::output(peripherals.thermometer.vcc).unwrap(),
    );
    let mut tmp36 = tmp36.power_down();

    let mut runner = EKitRunner::new(ekit, move || tmp36.power_up().measure_temperature().ok());
    runner.start()?;

    loop {
        runner.run();
        std::thread::sleep(SLEEP_DURATION);
    }
}

struct EKitRunner<E: EKit, F> {
    ekit: Arc<Mutex<E>>,
    server: EKitHttpServer<E>,
    output_temperature: F,
}

impl<E, F> EKitRunner<E, F>
where
    E: EKit + 'static,
    F: FnMut() -> Option<Temperature>,
{
    pub fn new(ekit: E, output_temperature: F) -> Self {
        let ekit = Arc::new(Mutex::new(ekit));
        EKitRunner {
            ekit: ekit.clone(),
            server: EKitHttpServer::new(ekit).unwrap(),
            output_temperature,
        }
    }

    /// Start the e-kit runner.
    pub fn start(&mut self) -> anyhow::Result<()> {
        self.server.start()?;
        Ok(())
    }

    /// Run the e-kit.
    pub fn run(&mut self) {
        let output_temperature = (self.output_temperature)();
        let mut ekit = self.ekit.lock().unwrap();
        ekit.set_output_temperature(output_temperature);
    }
}
