use esp_idf_hal::{
    gpio::{Gpio13, Gpio5, Gpio6},
    i2c::I2C0,
    modem::Modem,
    prelude::Peripherals,
};

pub struct SystemPeripherals<I2C, SDA, SCL, P> {
    pub bme: BmePeripherals<I2C, SDA, SCL, P>,
    pub modem: Modem,
}

impl SystemPeripherals<I2C0, Gpio5, Gpio6, Gpio13> {
    pub fn take() -> Self {
        let peripherals = Peripherals::take().unwrap();
        SystemPeripherals {
            bme: BmePeripherals {
                i2c: peripherals.i2c0,
                sda: peripherals.pins.gpio5,
                scl: peripherals.pins.gpio6,
                power: peripherals.pins.gpio13,
            },
            modem: peripherals.modem,
        }
    }
}

pub struct BmePeripherals<I2C, SDA, SCL, P> {
    pub i2c: I2C,
    pub sda: SDA,
    pub scl: SCL,
    pub power: P,
}
