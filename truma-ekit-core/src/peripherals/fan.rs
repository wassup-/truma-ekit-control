use crate::peripherals::relay::Relay;
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};

pub struct Fan<P: OutputPin> {
    relay: Relay<P>,
}

impl<P> Fan<P>
where
    P: OutputPin,
{
    pub fn new(relay: Relay<P>) -> Self {
        Fan { relay }
    }

    /// Returns `true` if the fan is currently turned on.
    pub fn is_turned_on(&self) -> bool
    where
        P: StatefulOutputPin,
    {
        self.relay.is_closed()
    }

    /// Turn on the fan.
    pub fn turn_on(&mut self) {
        self.relay.close()
    }

    /// Turn off the fan.
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
            !Fan::new(relay).is_turned_on(),
            "fan incorrectly reports being turned on"
        );

        let mut relay = Relay::connected_to(TestPin(false));
        relay.close();
        assert!(
            Fan::new(relay).is_turned_on(),
            "fan incorrectly reports being turned off"
        );
    }

    #[test]
    fn turn_on_closes_relay() {
        let mut relay = Relay::connected_to(TestPin(false));
        relay.open();
        let mut fan = Fan::new(relay);
        fan.turn_on();
        assert!(fan.is_turned_on(), "fan is not turned on");
        assert!(fan.relay.is_closed(), "fan did not close relay");
    }

    #[test]
    fn turn_off_opens_relay() {
        let mut relay = Relay::connected_to(TestPin(false));
        relay.close();
        let mut fan = Fan::new(relay);
        fan.turn_off();
        assert!(!fan.is_turned_on(), "fan is not turned off");
        assert!(!fan.relay.is_closed(), "fan did not open relay");
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
