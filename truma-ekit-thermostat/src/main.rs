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
use truma_ekit_core::{ekit::EKit, util::celsius};
use wifi::WifiClient;

const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_secs(1);

const EKIT_HOSTNAME: &str = "http://192.168.71.1/";

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

    let mut thermostat = Thermostat::new(celsius(20.5));
    thermostat.set_requested_temperature(celsius(21.0));

    let mut ekit = ekit::EKitHttp::new(EKIT_HOSTNAME, wifi);

    loop {
        // // TODO: update requested temperature

        let actual_temperature: Option<_> = bme280.read_temperature()?.map(celsius);

        if let Some(actual_temperature) = actual_temperature {
            let run_mode = thermostat.suggested_ekit_run_mode(actual_temperature);
            ekit.request_run_mode(run_mode);
        }

        std::thread::sleep(SLEEP_DURATION);
    }
}
