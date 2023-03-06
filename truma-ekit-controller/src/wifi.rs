use embedded_svc::wifi::{AccessPointConfiguration, AuthMethod, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{EspWifi, WifiDriver, WifiWait},
};
use esp_idf_sys::EspError;
use truma_ekit_core::wifi::{WIFI_PASS, WIFI_SSID};

#[derive(thiserror::Error, Debug)]
pub enum WifiApError {
    #[error("ESP error: {0}")]
    EspError(#[from] EspError),
}

pub struct WifiAp<'a> {
    wifi: EspWifi<'a>,
    sysloop: EspSystemEventLoop,
}

impl<'a> WifiAp<'a> {
    pub fn new(
        modem: Modem,
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self, WifiApError> {
        let driver = WifiDriver::new(modem, sysloop.clone(), Some(nvs)).unwrap();
        let mut wifi = EspWifi::wrap(driver)?;

        wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
            ssid: WIFI_SSID.into(),
            password: WIFI_PASS.into(),
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        }))?;

        Ok(WifiAp { wifi, sysloop })
    }

    /// Start the Wifi access point.
    ///
    /// This will block until the access point has started.
    pub fn start(&mut self) -> Result<(), WifiApError> {
        self.wifi.start()?;
        let wait = WifiWait::new(&self.sysloop)?;
        wait.wait(|| self.wifi.is_started().unwrap());
        Ok(())
    }
}
