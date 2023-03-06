mod ekit;
mod heating;
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

    let ekit = Arc::new(Mutex::new(EKitLocal::new(
        Fan::new(Relay::connected_to(PinDriver::output(
            peripherals.fan.power,
        )?)),
        HeatingCoil::new(Relay::connected_to(PinDriver::output(
            peripherals.coil1.power,
        )?)),
        HeatingCoil::new(Relay::connected_to(PinDriver::output(
            peripherals.coil2.power,
        )?)),
    )));

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

    runner.start()?;

    loop {
        runner.run()?;
        std::thread::sleep(SLEEP_DURATION);
    }
}

struct EKitRunner<'a, E: EKit> {
    ekit: Arc<Mutex<E>>,
    output: TMP36<'a>,
    server: EKitHttpServer<E>,
}

impl<'a, E: EKit + 'static> EKitRunner<'a, E> {
    pub fn new(ekit: Arc<Mutex<E>>, output: TMP36<'a>) -> Self {
        EKitRunner {
            ekit: ekit.clone(),
            output,
            server: EKitHttpServer::new(ekit).unwrap(),
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.server.start()?;
        Ok(())
    }

    /// Run the e-kit.
    pub fn run(&mut self) -> anyhow::Result<()> {
        let output_temperature = self.output.measure_temperature().ok();
        let mut ekit = self.ekit.lock().unwrap();
        ekit.set_output_temperature(output_temperature);
        Ok(())
    }
}
