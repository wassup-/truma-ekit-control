use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::delay::FreeRtos;
use std::ops::{Deref, DerefMut};

/// The time it takes for a device to settle after being powered up.
const SETTLE_DURATION_MS: u32 = 50;

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
    pub fn power_up<'b>(&'b mut self) -> PoweredUp<'b, D, VCC> {
        PoweredUp(self.0)
    }
}

impl<'a, D, VCC: OutputPin> PoweredUp<'a, D, VCC> {
    /// Powers down the device.
    pub fn power_down<'b>(&'b mut self) -> PoweredDown<'b, D, VCC> {
        PoweredDown(self.0)
    }
}

impl<'a, D, VCC: OutputPin> Drop for PoweredUp<'a, D, VCC> {
    fn drop(&mut self) {
        self.power_down();
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

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::digital::v2::StatefulOutputPin;

    struct Device;

    struct Pin(/* is_high */ bool);

    #[test]
    fn power_up_down() {
        let mut powered = Powered::new(Device, Pin(true));
        assert!(powered.vcc.is_set_high().unwrap());
        let mut powered_down = powered.power_down();
        assert!(powered_down.0.vcc.is_set_low().unwrap());
        let mut powered_up = powered_down.power_up();
        assert!(powered_up.0.vcc.is_set_high().unwrap());
        let powered_down = powered_up.power_down();
        assert!(powered_down.0.vcc.is_set_low().unwrap());
    }

    #[test]
    fn power_down_on_drop() {
        let mut powered = Powered::new(Device, Pin(true));
        let powered_up = powered.power_up();
        assert!(powered_up.0.vcc.is_set_high().unwrap());
        drop(powered_up);
        assert!(powered.vcc.is_set_low().unwrap());
    }

    impl OutputPin for Pin {
        type Error = std::convert::Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.0 = true;
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.0 = false;
            Ok(())
        }
    }

    impl StatefulOutputPin for Pin {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.0)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.0)
        }
    }
}
