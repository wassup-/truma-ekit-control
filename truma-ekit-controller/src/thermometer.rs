use truma_ekit_core::{peripherals::tmp36::TMP36, types::Temperature};

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

pub struct MemoizeTemperature<'a> {
    tmp36: TMP36<'a>,
    temperature: Option<Temperature>,
}

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

impl<'a> MemoizeTemperature<'a> {
    pub fn new(tmp36: TMP36<'a>) -> Self {
        MemoizeTemperature {
            tmp36,
            temperature: None,
        }
    }
}

impl<'a> Thermometer for MemoizeTemperature<'a> {
    fn measure(&mut self) -> anyhow::Result<()> {
        match self.tmp36.temperature() {
            Ok(temperature) => {
                self.temperature = Some(temperature);
                Ok(())
            }
            Err(err) => {
                self.temperature = None;
                Err(err)
            }
        }
    }

    fn temperature(&self) -> Option<Temperature> {
        self.temperature.clone()
    }
}
