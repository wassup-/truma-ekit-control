use esp_idf_hal::{
    adc::ADC1,
    gpio::{AnyIOPin, AnyInputPin, AnyOutputPin, Gpio2},
    i2c::I2C0,
    modem::Modem,
    prelude::Peripherals,
};

pub struct SystemPeripherals<I2C, ADC, GP> {
    pub i2c: I2cPeripherals<I2C>,
    pub rot: RotaryPeripherals,
    pub temperature: ThermometerPeripherals<ADC, GP>,
    pub modem: Modem,
}

impl SystemPeripherals<I2C0, ADC1, Gpio2> {
    pub fn take() -> Self {
        let peripherals = Peripherals::take().unwrap();
        SystemPeripherals {
            i2c: I2cPeripherals {
                i2c: peripherals.i2c0,
                sda: peripherals.pins.gpio5.into(),
                scl: peripherals.pins.gpio6.into(),
            },
            rot: RotaryPeripherals {
                pin_a: peripherals.pins.gpio10.into(),
                pin_b: peripherals.pins.gpio11.into(),
            },
            temperature: ThermometerPeripherals {
                adc: peripherals.adc1,
                voltage: peripherals.pins.gpio2,
                vcc: peripherals.pins.gpio7.into(),
            },
            modem: peripherals.modem,
        }
    }
}

pub struct I2cPeripherals<I2C> {
    pub i2c: I2C,
    pub sda: AnyIOPin,
    pub scl: AnyIOPin,
}

pub struct RotaryPeripherals {
    pub pin_a: AnyInputPin,
    pub pin_b: AnyInputPin,
}

pub struct ThermometerPeripherals<ADC, GP> {
    pub adc: ADC,
    pub voltage: GP,
    pub vcc: AnyOutputPin,
}
