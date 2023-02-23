use esp_idf_hal::i2c::I2cDriver;

pub trait I2c {}

impl<'a> I2c for I2cDriver<'a> {}
