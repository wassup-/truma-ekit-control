use esp_idf_hal::{
    adc::{Adc, AdcChannelDriver, AdcDriver, Atten11dB, Attenuation},
    gpio::ADCPin,
};

pub struct AdcInputPin<'a> {
    input: Box<dyn AdcInput + 'a>,
}

impl<'a> AdcInputPin<'a> {
    pub fn pin<P, A>(pin: P, driver: AdcDriver<'a, A>) -> Self
    where
        P: ADCPin,
        A: Adc,
        Atten11dB<A>: Attenuation<<P as ADCPin>::Adc>,
    {
        let input = AdcDriverAndChannelDriver {
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

struct AdcDriverAndChannelDriver<'a, ADC: Adc, T: ADCPin>
where
    Atten11dB<ADC>: Attenuation<<T as ADCPin>::Adc>,
{
    driver: AdcDriver<'a, ADC>,
    channel_driver: AdcChannelDriver<'a, T, Atten11dB<ADC>>,
}

impl<'a, ADC: Adc, T: ADCPin> AdcInput for AdcDriverAndChannelDriver<'a, ADC, T>
where
    Atten11dB<ADC>: Attenuation<<T as ADCPin>::Adc>,
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
