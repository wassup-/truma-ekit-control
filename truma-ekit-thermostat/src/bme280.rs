use crate::{gpio::DigitalOutputPin, i2c::I2c};
use std::time::Duration;
use truma_ekit_core::types::{Percent, Temperature};

pub struct Measurements {
    pub temperature: Temperature,
    pub relative_humidity: Percent,
}

pub struct BME280<'a> {
    i2c: Box<dyn I2c + 'a>,
    vcc_pin: DigitalOutputPin<'a>,
}

impl<'a> BME280<'a> {
    pub fn new(i2c: impl I2c + 'a, vcc_pin: DigitalOutputPin<'a>) -> Self {
        BME280 {
            i2c: Box::new(i2c),
            vcc_pin,
        }
    }

    pub fn measure(&mut self) -> anyhow::Result<Measurements> {
        self.vcc_pin.set_high()?;
        std::thread::sleep(Duration::from_millis(10));
        self.vcc_pin.set_low()?;
        todo!()
    }
}
