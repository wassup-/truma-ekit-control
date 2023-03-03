use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
use truma_ekit_core::peripherals::relay::Relay;

pub struct HeatingCoil<P: OutputPin> {
    relay: Relay<P>,
}

impl<P> HeatingCoil<P>
where
    P: OutputPin,
{
    pub fn new(relay: Relay<P>) -> Self {
        HeatingCoil { relay }
    }

    /// Returns `true` if the heating coil is currently turned on.
    pub fn is_turned_on(&self) -> bool
    where
        P: StatefulOutputPin,
    {
        self.relay.is_closed()
    }

    /// Turn on the heating coil.
    pub fn turn_on(&mut self) {
        self.relay.close()
    }

    /// Turn off the heating coil.
    pub fn turn_off(&mut self) {
        self.relay.open()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_on() {
        let mut relay = Relay::connected_to(TestPin(false));
        relay.open();
        assert!(
            !HeatingCoil::new(relay).is_turned_on(),
            "coil incorrectly reports being turned on"
        );

        let mut relay = Relay::connected_to(TestPin(false));
        relay.close();
        assert!(
            HeatingCoil::new(relay).is_turned_on(),
            "coil incorrectly reports being turned off"
        );
    }

    #[test]
    fn turn_on_closes_relay() {
        let mut relay = Relay::connected_to(TestPin(false));
        relay.open();
        let mut coil = HeatingCoil::new(relay);
        coil.turn_on();
        assert!(coil.relay.is_closed(), "coil did not close relay");
    }

    #[test]
    fn turn_off_opens_relay() {
        let mut relay = Relay::connected_to(TestPin(false));
        relay.close();
        let mut coil = HeatingCoil::new(relay);
        coil.turn_off();
        assert!(!coil.relay.is_closed(), "coil did not open relay");
    }

    struct TestPin(bool);

    impl OutputPin for TestPin {
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

    impl StatefulOutputPin for TestPin {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.0)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.0)
        }
    }
}
