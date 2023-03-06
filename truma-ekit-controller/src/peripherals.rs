use esp_idf_hal::{
    adc::ADC1,
    gpio::{AnyOutputPin, Gpio2},
    modem::Modem,
    prelude::Peripherals,
};

pub struct SystemPeripherals<ADC, GP> {
    pub fan: FanPeripherals,
    pub coil1: CoilPeripherals,
    pub coil2: CoilPeripherals,
    pub thermometer: ThermometerPeripherals<ADC, GP>,
    pub modem: Modem,
}

impl SystemPeripherals<ADC1, Gpio2> {
    pub fn take() -> Self {
        let peripherals = Peripherals::take().unwrap();
        SystemPeripherals {
            fan: FanPeripherals {
                power: peripherals.pins.gpio7.into(),
            },
            coil1: CoilPeripherals {
                power: peripherals.pins.gpio8.into(),
            },
            coil2: CoilPeripherals {
                power: peripherals.pins.gpio9.into(),
            },
            thermometer: ThermometerPeripherals {
                adc: peripherals.adc1,
                voltage: peripherals.pins.gpio2,
            },
            modem: peripherals.modem,
        }
    }
}

pub struct FanPeripherals {
    pub power: AnyOutputPin,
}

pub struct CoilPeripherals {
    pub power: AnyOutputPin,
}

pub struct ThermometerPeripherals<ADC, GP> {
    pub adc: ADC,
    pub voltage: GP,
}
