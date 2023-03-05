mod ekit;
mod peripherals;
mod thermostat;

use bme280_rs::Bme280;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
};
use esp_idf_sys as _;
use peripherals::SystemPeripherals;
use thermostat::Thermostat;
use truma_ekit_core::{ekit::EKit, util::celsius};

const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_secs(1);

esp_idf_sys::esp_app_desc!();

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = SystemPeripherals::take();

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

    let mut ekit = ekit::EKitHttp::new("http://192.168.178.1");

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
