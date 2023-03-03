use esp_idf_hal::i2c::I2c;

pub struct SSD1306<I2C: I2c> {
    i2c: I2C,
}
