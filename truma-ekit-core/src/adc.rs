use esp_idf_hal::{
    adc::{Adc, AdcChannelDriver, AdcDriver, Attenuation},
    gpio::ADCPin,
};

pub struct AdcInputPin<'a> {
    input: Box<dyn AdcInput + 'a>,
}

impl<'a> AdcInputPin<'a> {
    pub fn pin<P, ADC, ATTEN>(pin: P, driver: AdcDriver<'a, ADC>) -> Self
    where
        P: ADCPin,
        ADC: Adc,
        ATTEN: Attenuation<<P as ADCPin>::Adc> + 'a,
    {
        let input = AdcDriverAndChannelDriver::<_, _, ATTEN> {
            driver,
            channel_driver: AdcChannelDriver::new(pin).unwrap(),
        };
        AdcInputPin {
            input: Box::new(input),
        }
    }

    #[cfg(test)]
    pub fn test(value: u16) -> Self {
        AdcInputPin {
            input: Box::new(TestAdcInput(value)),
        }
    }

    pub fn read(&mut self) -> anyhow::Result<u16> {
        self.input.read()
    }
}

trait AdcInput {
    fn read(&mut self) -> anyhow::Result<u16>;
}

struct AdcDriverAndChannelDriver<'a, ADC, GP, ATTEN>
where
    ADC: Adc,
    GP: ADCPin,
    ATTEN: Attenuation<<GP as ADCPin>::Adc>,
{
    driver: AdcDriver<'a, ADC>,
    channel_driver: AdcChannelDriver<'a, GP, ATTEN>,
}

impl<'a, ADC, GP, ATTEN> AdcInput for AdcDriverAndChannelDriver<'a, ADC, GP, ATTEN>
where
    ADC: Adc,
    GP: ADCPin,
    ATTEN: Attenuation<<GP as ADCPin>::Adc>,
{
    fn read(&mut self) -> anyhow::Result<u16> {
        let val = self.driver.read(&mut self.channel_driver)?;
        Ok(val)
    }
}

struct TestAdcInput(u16);

impl AdcInput for TestAdcInput {
    fn read(&mut self) -> anyhow::Result<u16> {
        Ok(self.0)
    }
}
