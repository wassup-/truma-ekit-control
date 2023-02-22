use crate::gpio::DigitalOutputPin;

pub struct Relay<'a> {
    pin: DigitalOutputPin<'a>,
}

impl<'a> Relay<'a> {
    /// Returns a new relay connected to the given pin.
    pub fn connected_to(pin: DigitalOutputPin<'a>) -> Self {
        Relay { pin }
    }

    /// Returns `true` if the relay is currently closed, i.e. current is currently flowing.
    #[allow(dead_code)]
    pub fn is_closed(&self) -> bool {
        self.pin.is_high()
    }

    /// Close the relay, i.e. let current flow.
    pub fn close(&mut self) {
        self.pin.set_high().expect("failed to close relay")
    }

    /// Open the relay, i.e. stops the current flow.
    pub fn open(&mut self) {
        self.pin.set_low().expect("failed to open relay")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relay_is_closed() {
        let mut pin = DigitalOutputPin::test(true);
        assert!(
            Relay::connected_to(pin).is_closed(),
            "relay incorrectly reports being open"
        );

        let mut pin = DigitalOutputPin::test(false);
        assert!(
            !Relay::connected_to(pin).is_closed(),
            "relay incorrectly reports being closed"
        );
    }

    #[test]
    fn relay_close_sets_pin_high() {
        let mut pin = DigitalOutputPin::test(false);
        let mut relay = Relay::connected_to(pin);
        relay.close();
        assert!(relay.pin.is_high(), "relay did not set pin high");
    }

    #[test]
    fn relay_open_sets_pin_low() {
        let mut pin = DigitalOutputPin::test(true);
        let mut relay = Relay::connected_to(pin);
        relay.open();
        assert!(!relay.pin.is_high(), "relay did not set pin low");
    }
}
