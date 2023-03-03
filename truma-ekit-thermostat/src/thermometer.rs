use bme280_rs::Bme280;
use embedded_hal::blocking::{
    delay::DelayMs,
    i2c::{Read, Write, WriteRead},
};
use truma_ekit_core::types::Temperature;

use crate::celsius;

pub trait Thermometer {
    fn measure(&mut self) -> anyhow::Result<Temperature>;
}

impl<I2C, D, E> Thermometer for Bme280<I2C, D>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u32>,
{
    fn measure(&mut self) -> anyhow::Result<Temperature> {
        match self.read_temperature() {
            Ok(Some(temperature)) => Ok(celsius(temperature)),
            _ => anyhow::bail!("failed to measure temperature"),
        }
    }
}
