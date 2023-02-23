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
    #[cfg(test)]
    pub fn is_closed(&self) -> bool {
        self.pin.is_set_high()
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
        assert!(
            Relay::connected_to(DigitalOutputPin::test(true)).is_closed(),
            "relay incorrectly reports being open"
        );
        assert!(
            !Relay::connected_to(DigitalOutputPin::test(false)).is_closed(),
            "relay incorrectly reports being closed"
        );
    }

    #[test]
    fn relay_close_sets_pin_high() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(false));
        relay.close();
        assert!(relay.pin.is_set_high(), "relay did not set pin high");
    }

    #[test]
    fn relay_open_sets_pin_low() {
        let mut relay = Relay::connected_to(DigitalOutputPin::test(true));
        relay.open();
        assert!(!relay.pin.is_set_high(), "relay did not set pin low");
    }
}
