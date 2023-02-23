use crate::peripherals::relay::Relay;

pub struct Fan<'a> {
    relay: Relay<'a>,
}

impl<'a> Fan<'a> {
    pub fn new(relay: Relay<'a>) -> Self {
        Fan { relay }
    }

    /// Returns `true` if the fan is currently turned on.
    #[cfg(test)]
    pub fn is_turned_on(&self) -> bool {
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
    use crate::gpio::DigitalOutputPin;

    #[test]
    fn is_on() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.open();
        assert!(
            !Fan::new(relay).is_turned_on(),
            "fan incorrectly reports being turned on"
        );

        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.close();
        assert!(
            Fan::new(relay).is_turned_on(),
            "fan incorrectly reports being turned off"
        );
    }

    #[test]
    fn turn_on_closes_relay() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.open();
        let mut fan = Fan::new(relay);
        fan.turn_on();
        assert!(fan.is_turned_on(), "fan is not turned on");
        assert!(fan.relay.is_closed(), "fan did not close relay");
    }

    #[test]
    fn turn_off_opens_relay() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.close();
        let mut fan = Fan::new(relay);
        fan.turn_off();
        assert!(!fan.is_turned_on(), "fan is not turned off");
        assert!(!fan.relay.is_closed(), "fan did not open relay");
    }
}
