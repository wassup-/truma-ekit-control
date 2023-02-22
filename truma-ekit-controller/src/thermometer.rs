use truma_ekit_core::types::Temperature;

pub trait Thermometer {
    /// Measure the current temperature.
    fn measure(&mut self) -> anyhow::Result<()>;

    /// Returns the actual temperature.
    fn temperature(&self) -> Option<Temperature>;
}

#[cfg(test)]
pub struct FakeTemperature(pub Temperature);

#[cfg(test)]
pub struct NoTemperature;

#[cfg(test)]
impl Thermometer for FakeTemperature {
    fn measure(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn temperature(&self) -> Option<Temperature> {
        Some(self.0.clone())
    }
}

#[cfg(test)]
impl Thermometer for NoTemperature {
    fn measure(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn temperature(&self) -> Option<Temperature> {
        None
    }
}
