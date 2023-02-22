use esp_idf_hal::gpio::{Output, Pin, PinDriver};

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

    /// Returns `true` if the pin is currently set high.
    pub fn is_high(&self) -> bool {
        self.output.is_high()
    }

    /// Sets the pin output high.
    pub fn set_high(&mut self) -> anyhow::Result<()> {
        self.output.set_is_high(true)
    }

    /// Sets the pin output low.
    pub fn set_low(&mut self) -> anyhow::Result<()> {
        self.output.set_is_high(false)
    }
}

trait DigitalOutput {
    fn is_high(&self) -> bool;
    fn set_is_high(&mut self, high: bool) -> anyhow::Result<()>;
}

struct DigitalPinOutput<'a, P: Pin> {
    driver: PinDriver<'a, P, Output>,
}

impl<'a, P: Pin> DigitalOutput for DigitalPinOutput<'a, P> {
    fn is_high(&self) -> bool {
        self.driver.is_set_high()
    }

    fn set_is_high(&mut self, is_high: bool) -> anyhow::Result<()> {
        self.driver.set_level(is_high.into())?;
        Ok(())
    }
}
struct TestDigitalOutput {
    is_high: bool,
}

impl DigitalOutput for TestDigitalOutput {
    fn is_high(&self) -> bool {
        self.is_high
    }

    fn set_is_high(&mut self, is_high: bool) -> anyhow::Result<()> {
        self.is_high = is_high;
        Ok(())
    }
}
