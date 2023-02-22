use crate::relay::Relay;

pub struct HeatingCoil<'a> {
    relay: Relay<'a>,
}

impl<'a> HeatingCoil<'a> {
    pub fn new(relay: Relay<'a>) -> Self {
        HeatingCoil { relay }
    }

    /// Returns `true` if the heating coil is currently turned on.
    #[allow(dead_code)]
    pub fn is_on(&self) -> bool {
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
    use crate::gpio::DigitalOutputPin;

    #[test]
    fn is_on() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.open();
        assert!(
            !HeatingCoil::new(relay).is_on(),
            "coil incorrectly reports being turned on"
        );

        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.close();
        assert!(
            HeatingCoil::new(relay).is_on(),
            "coil incorrectly reports being turned off"
        );
    }

    #[test]
    fn turn_on_closes_relay() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.open();
        let mut coil = HeatingCoil::new(relay);
        coil.turn_on();
        assert!(coil.relay.is_closed(), "coil did not close relay");
    }

    #[test]
    fn turn_off_opens_relay() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.close();
        let mut coil = HeatingCoil::new(relay);
        coil.turn_off();
        assert!(!coil.relay.is_closed(), "coil did not open relay");
    }
}
