use crate::types::{Percent, Temperature};
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::i2c::I2c;
use std::time::Duration;

pub struct Measurements {
    pub temperature: Temperature,
    pub relative_humidity: Percent,
}

pub struct BME280<I2C: I2c, P: OutputPin> {
    i2c: I2C,
    power: P,
}

impl<I2C: I2c, P: OutputPin> BME280<I2C, P> {
    pub fn new(i2c: I2C, power: P) -> Self {
        BME280 { i2c, power }
    }

    pub fn measure(&mut self) -> anyhow::Result<Measurements> {
        self.power
            .set_high()
            .unwrap_or_else(|_| panic!("failed to power on BME280"));
        std::thread::sleep(Duration::from_millis(10));
        self.power
            .set_low()
            .unwrap_or_else(|_| panic!("failed to power off BME280"));
        todo!()
    }
}
