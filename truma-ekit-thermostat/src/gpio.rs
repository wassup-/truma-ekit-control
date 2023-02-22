use esp_idf_hal::gpio::{Level, OutputMode, OutputPin, Pin, PinDriver};

trait PinOutput {
    fn set_high(&mut self) -> anyhow::Result<()>;
    fn set_low(&mut self) -> anyhow::Result<()>;
}

pub struct DigitalOutputPin<'a> {
    pin: Box<dyn PinOutput + 'a>,
}

impl<'a> DigitalOutputPin<'a> {
    pub fn pin<P>(pin: P) -> Self
    where
        P: OutputPin,
    {
        let driver = PinDriver::output(pin).unwrap();
        DigitalOutputPin {
            pin: Box::new(driver),
        }
    }

    pub fn set_high(&mut self) -> anyhow::Result<()> {
        self.pin.set_high()
    }

    pub fn set_low(&mut self) -> anyhow::Result<()> {
        self.pin.set_low()
    }
}

impl<'a, T, MODE> PinOutput for PinDriver<'a, T, MODE>
where
    T: Pin,
    MODE: OutputMode,
{
    fn set_high(&mut self) -> anyhow::Result<()> {
        self.set_level(Level::High)?;
        Ok(())
    }

    fn set_low(&mut self) -> anyhow::Result<()> {
        self.set_level(Level::Low)?;
        Ok(())
    }
}
