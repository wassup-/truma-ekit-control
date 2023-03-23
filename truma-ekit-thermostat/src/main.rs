mod caching;
mod ekit;
mod input;
mod output;
mod peripherals;
mod thermostat;
mod wifi;

use esp_idf_hal::adc::{AdcConfig, AdcDriver, Atten0dB};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _;
use output::Output;
use peripherals::SystemPeripherals;
use std::time::Duration;
use thermostat::Thermostat;
use truma_ekit_core::{
    adc::AdcInputPin, ekit::EKit, throttle::Throttle, types::Temperature, util::celsius,
};
use wifi::WifiClient;

/// The hostname of the e-kit controller.
const EKIT_HOSTNAME: &str = "http://192.168.71.1";
/// The default requested temperature.
const DEFAULT_REQUESTED_TEMPERATURE: Temperature = celsius(20.5);
/// The step size to use when rotating the encoder.
const INPUT_STEP_SIZE: Temperature = celsius(0.5);

esp_idf_sys::esp_app_desc!();

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = SystemPeripherals::take();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs_default_partition = EspDefaultNvsPartition::take()?;

    let mut wifi = WifiClient::new(peripherals.modem, sysloop, nvs_default_partition)?;
    wifi.start()?;

    let mut ekit = ekit::EKitHttp::new(EKIT_HOSTNAME, wifi);
    let mut thermostat = Thermostat::new(DEFAULT_REQUESTED_TEMPERATURE);

    let mut read_requested_temperature_adjustment = input::temperature_adjustment(
        peripherals.rot.pin_a,
        peripherals.rot.pin_b,
        INPUT_STEP_SIZE,
    );
    let mut read_actual_temperature = input::ambient_temperature(
        AdcInputPin::pin::<_, _, Atten0dB<_>>(
            peripherals.temperature.voltage,
            AdcDriver::new(
                peripherals.temperature.adc,
                &AdcConfig::default().calibration(true),
            )?,
        ),
        peripherals.temperature.vcc,
    );

    let mut display = output::display(
        peripherals.i2c.i2c,
        peripherals.i2c.sda,
        peripherals.i2c.scl,
    );

    let mut actual_temperature = caching::CachedTemperature::new(None);

    let mut display_throttler = Throttle::max_runs_per_sec(10);
    let mut request_throttler = Throttle::one_run_per(Duration::from_secs(2));

    loop {
        // adjust the requested temperature using the rotary encoder
        if let Some(adjustment) = read_requested_temperature_adjustment() {
            let requested_temperature = thermostat.requested_temperature() + adjustment;
            thermostat.set_requested_temperature(requested_temperature);
            // continue reading input as long as changes are requested
            continue;
        }

        // update the actual temperature
        actual_temperature.update(read_actual_temperature());

        display_throttler.throttle(|| {
            let output = Output {
                requested_temperature: thermostat.requested_temperature(),
                actual_temperature: actual_temperature.last_known_temperature(),
                wifi_connected: ekit.is_connected(),
            };
            display(output);
        });

        // run the e-kit based on the *last known* actual temperature
        let actual_temperature = match actual_temperature.last_known_temperature() {
            Some(temperature) => temperature,
            _ => continue,
        };

        request_throttler.throttle(|| {
            let run_mode = thermostat.suggested_ekit_run_mode(actual_temperature);
            ekit.request_user_run_mode(run_mode);
        })
    }
}
