use esp_idf_hal::gpio::{AnyInputPin, AnyOutputPin, PinDriver};
use rotary_encoder_hal::{Direction, Rotary};
use std::ops::Neg;
use truma_ekit_core::{
    adc::AdcInputPin, peripherals::tmp36::TMP36, powersaving::Powered, types::Temperature,
};

pub fn temperature_adjustment<'a>(
    pin_a: AnyInputPin,
    pin_b: AnyInputPin,
    step_size: Temperature,
) -> impl FnMut() -> Option<Temperature> + 'a {
    let mut encoder = Rotary::new(
        PinDriver::input(pin_a).unwrap(),
        PinDriver::input(pin_b).unwrap(),
    );
    move || match encoder.update() {
        Ok(Direction::Clockwise) => Some(step_size),
        Ok(Direction::CounterClockwise) => Some(step_size.neg()),
        _ => None,
    }
}

pub fn ambient_temperature<'a>(
    pin: AdcInputPin<'a>,
    vcc: AnyOutputPin,
) -> impl FnMut() -> Option<Temperature> + 'a {
    let tmp36 = TMP36::connected_to(pin);
    let mut tmp36 = Powered::new(tmp36, PinDriver::output(vcc).unwrap());
    move || tmp36.power_up().measure_temperature().ok()
}
