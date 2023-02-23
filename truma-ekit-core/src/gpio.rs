use esp_idf_hal::gpio::{Level, Output, Pin, PinDriver};

pub struct DigitalOutputPin<'a> {
    output: Box<dyn DigitalOutput + 'a>,
}

impl<'a> DigitalOutputPin<'a> {
    pub fn pin<P: Pin>(driver: PinDriver<'a, P, Output>) -> Self {
        let output = DigitalPinOutput { driver };
        DigitalOutputPin {
            output: Box::new(output),
        }
    }

    #[cfg(test)]
    pub fn test(is_high: bool) -> Self {
        DigitalOutputPin {
            output: Box::new(TestDigitalOutput { is_high }),
        }
    }

    #[cfg(test)]
    /// Returns `true` if the pin output is set high.
    pub fn is_set_high(&self) -> bool {
        self.output.is_set_high()
    }

    /// Sets the pin output high.
    pub fn set_high(&mut self) -> anyhow::Result<()> {
        self.output.set_high()
    }

    /// Sets the pin output low.
    pub fn set_low(&mut self) -> anyhow::Result<()> {
        self.output.set_low()
    }
}

trait DigitalOutput {
    #[cfg(test)]
    fn is_set_high(&self) -> bool;
    fn set_high(&mut self) -> anyhow::Result<()>;
    fn set_low(&mut self) -> anyhow::Result<()>;
}

struct DigitalPinOutput<'a, P: Pin> {
    driver: PinDriver<'a, P, Output>,
}

impl<'a, P: Pin> DigitalOutput for DigitalPinOutput<'a, P> {
    #[cfg(test)]
    fn is_set_high(&self) -> bool {
        self.driver.is_set_high()
    }

    fn set_high(&mut self) -> anyhow::Result<()> {
        self.driver.set_level(Level::High)?;
        Ok(())
    }

    fn set_low(&mut self) -> anyhow::Result<()> {
        self.driver.set_level(Level::Low)?;
        Ok(())
    }
}
struct TestDigitalOutput {
    is_high: bool,
}

impl DigitalOutput for TestDigitalOutput {
    #[cfg(test)]
    fn is_set_high(&self) -> bool {
        self.is_high
    }

    fn set_high(&mut self) -> anyhow::Result<()> {
        self.is_high = true;
        Ok(())
    }

    fn set_low(&mut self) -> anyhow::Result<()> {
        self.is_high = false;
        Ok(())
    }
}
