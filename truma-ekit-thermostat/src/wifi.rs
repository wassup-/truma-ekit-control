use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{EspWifi, WifiDriver, WifiWait},
};
use esp_idf_sys::EspError;
use std::time::Duration;
use truma_ekit_core::wifi::{WIFI_PASS, WIFI_SSID};

#[derive(thiserror::Error, Debug)]
pub enum WifiClientError {
    #[error("ESP error: {0}")]
    Esp(#[from] EspError),
    #[error("Disconnected")]
    Disconnected,
}

pub struct WifiClient<'a> {
    wifi: EspWifi<'a>,
    sysloop: EspSystemEventLoop,
}

impl<'a> WifiClient<'a> {
    pub fn new(
        modem: Modem,
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self, WifiClientError> {
        let driver = WifiDriver::new(modem, sysloop.clone(), Some(nvs))?;
        let mut wifi = EspWifi::wrap(driver)?;

        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: WIFI_SSID.into(),
            password: WIFI_PASS.into(),
            ..Default::default()
        }))?;

        Ok(WifiClient { wifi, sysloop })
    }

    /// Start the Wifi client.
    ///
    /// This will block until the client has started.
    pub fn start(&mut self) -> Result<(), WifiClientError> {
        self.wifi.start()?;
        let wait = WifiWait::new(&self.sysloop)?;
        wait.wait(|| self.wifi.is_started().unwrap());
        Ok(())
    }

    /// Connect the Wifi client.
    ///
    /// This will block until the client has connected.
    pub fn connect(&mut self) -> Result<(), WifiClientError> {
        self.wifi.connect()?;
        let wait = WifiWait::new(&self.sysloop)?;
        if !wait.wait_with_timeout(Duration::from_secs(5), || self.wifi.is_connected().unwrap()) {
            return Err(WifiClientError::Disconnected);
        }
        Ok(())
    }

    /// Returns `true` if wifi is connected.
    pub fn is_connected(&self) -> bool {
        self.wifi.is_connected().unwrap_or(false)
    }
}
