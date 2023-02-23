use crate::{adc::AdcInputPin, types::Temperature, util::celsius};

pub struct TMP36<'a> {
    input: AdcInputPin<'a>,
}

impl<'a> TMP36<'a> {
    pub fn connected_to(input: AdcInputPin<'a>) -> Self {
        TMP36 { input }
    }

    fn adc_to_temperature(adc: u16) -> Temperature {
        // There are very subtle differences when coding a temperature sensor on the ESP32 board as opposed to the Arduino.
        // For the ESP32 board, we do not need to multiply the raw sensor reading by the voltage, it simply needs to be normalized by the 2^10 bits.
        let voltage = f32::from(adc) / 1024.0;
        let degrees = (voltage - 0.5) * 100.0;
        celsius(degrees)
    }

    pub fn temperature(&mut self) -> anyhow::Result<Temperature> {
        let val = self.input.read()?;
        Ok(Self::adc_to_temperature(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp36_convert_to_temperature() {
        let mut tmp36 = TMP36::connected_to(AdcInputPin::test(768));
        assert_eq!(tmp36.temperature().unwrap(), celsius(25.0));
    }
}
