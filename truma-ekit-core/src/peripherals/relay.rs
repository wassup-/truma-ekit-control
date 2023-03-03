use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};

pub struct Relay<P: OutputPin> {
    pin: P,
}

impl<P> Relay<P>
where
    P: OutputPin,
{
    /// Returns a new relay connected to the given pin.
    pub fn connected_to(pin: P) -> Self {
        Relay { pin }
    }

    /// Returns `true` if the relay is currently closed, i.e. current is currently flowing.
    pub fn is_closed(&self) -> bool
    where
        P: StatefulOutputPin,
    {
        self.pin
            .is_set_high()
            .unwrap_or_else(|_| panic!("failed to check if relay is closed"))
    }

    /// Close the relay, i.e. let current flow.
    pub fn close(&mut self) {
        self.pin
            .set_high()
            .unwrap_or_else(|_| panic!("failed to close relay"))
    }

    /// Open the relay, i.e. stops the current flow.
    pub fn open(&mut self) {
        self.pin
            .set_low()
            .unwrap_or_else(|_| panic!("failed to open relay"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relay_is_closed() {
        assert!(
            Relay::connected_to(TestPin(true)).is_closed(),
            "relay incorrectly reports being open"
        );
        assert!(
            !Relay::connected_to(TestPin(false)).is_closed(),
            "relay incorrectly reports being closed"
        );
    }

    #[test]
    fn relay_close_sets_pin_high() {
        let mut relay = Relay::connected_to(TestPin(false));
        relay.close();
        assert!(
            relay.pin.is_set_high().unwrap(),
            "relay did not set pin high"
        );
    }

    #[test]
    fn relay_open_sets_pin_low() {
        let mut relay = Relay::connected_to(TestPin(true));
        relay.open();
        assert!(relay.pin.is_set_low().unwrap(), "relay did not set pin low");
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
