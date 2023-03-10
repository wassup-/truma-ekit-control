mod ekit;
mod peripherals;
mod thermostat;
mod wifi;

use bme280_rs::Bme280;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _;
use peripherals::SystemPeripherals;
use thermostat::Thermostat;
use truma_ekit_core::{ekit::EKit, types::Temperature, util::celsius};
use wifi::WifiClient;

const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_secs(1);

/// The hostname of the e-kit controller.
const EKIT_HOSTNAME: &str = "http://192.168.71.1";

esp_idf_sys::esp_app_desc!();

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = SystemPeripherals::take();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs_default_partition = EspDefaultNvsPartition::take()?;

    let mut wifi = WifiClient::new(peripherals.modem, sysloop, nvs_default_partition)?;
    wifi.start()?;

    let mut bme280 = Bme280::new(
        I2cDriver::new(
            peripherals.bme.i2c,
            peripherals.bme.sda,
            peripherals.bme.scl,
            &I2cConfig::default(),
        )?,
        FreeRtos,
    );

    let ekit = ekit::EKitHttp::new(EKIT_HOSTNAME, wifi);
    let thermostat = Thermostat::new(celsius(20.5));

    let mut runner = EKitRunner::new(ekit, thermostat, move || {
        bme280.read_temperature().unwrap_or_default().map(celsius)
    });

    loop {
        runner.run();
        std::thread::sleep(SLEEP_DURATION);
    }
}

struct EKitRunner<E: EKit, F> {
    ekit: E,
    thermostat: Thermostat,
    actual_temperature: F,
}

impl<E, F> EKitRunner<E, F>
where
    E: EKit,
    F: FnMut() -> Option<Temperature>,
{
    pub fn new(ekit: E, thermostat: Thermostat, actual_temperature: F) -> Self {
        EKitRunner {
            ekit,
            thermostat,
            actual_temperature,
        }
    }

    /// Run the e-kit.
    pub fn run(&mut self) {
        // TODO: update requested temperature
        let actual_temperature = (self.actual_temperature)();

        if let Some(actual_temperature) = actual_temperature {
            let run_mode = self.thermostat.suggested_ekit_run_mode(actual_temperature);
            self.ekit.request_run_mode(run_mode);
        }
    }
}
