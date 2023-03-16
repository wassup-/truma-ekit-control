use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::delay::FreeRtos;
use std::ops::{Deref, DerefMut};

/// The time it takes for a device to settle after being powered up.
const SETTLE_DURATION_MS: u32 = 15;

/// A device that is powered up.
///
/// The device will automatically be powered down when dropped.
pub struct PoweredUp<'a, D, VCC: OutputPin>(&'a mut Powered<D, VCC>);

/// A device that is powered down.
pub struct PoweredDown<'a, D, VCC: OutputPin>(&'a mut Powered<D, VCC>);

/// A device that can be powered up and powered down.
///
/// No assumptions are made about the initial state of the device.
pub struct Powered<D, VCC: OutputPin> {
    device: D,
    vcc: VCC,
}

impl<D, VCC: OutputPin> Powered<D, VCC> {
    /// Creates a new powered device.
    pub fn new(device: D, vcc: VCC) -> Self {
        Powered { device, vcc }
    }

    /// Powers up the device.
    pub fn power_up(&mut self) -> PoweredUp<D, VCC> {
        self.vcc
            .set_high()
            .unwrap_or_else(|_| panic!("failed to power up"));
        FreeRtos::delay_ms(SETTLE_DURATION_MS);
        PoweredUp(self)
    }

    /// Powers down the device.
    pub fn power_down(&mut self) -> PoweredDown<D, VCC> {
        self.vcc
            .set_low()
            .unwrap_or_else(|_| panic!("failed to power down"));
        PoweredDown(self)
    }
}

impl<'a, D, VCC: OutputPin> PoweredDown<'a, D, VCC> {
    /// Powers up the device.
    pub fn power_up(self) -> PoweredUp<'a, D, VCC> {
        PoweredUp(self.0)
    }
}

impl<'a, D, VCC: OutputPin> Drop for PoweredUp<'a, D, VCC> {
    fn drop(&mut self) {
        self.0.power_down();
    }
}

impl<'a, D, VCC: OutputPin> Deref for PoweredUp<'a, D, VCC> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.0.device
    }
}

impl<'a, D, VCC: OutputPin> DerefMut for PoweredUp<'a, D, VCC> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.device
    }
}
