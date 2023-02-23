use crate::i2c::I2c;

pub struct SSD1306<'a> {
    i2c: Box<dyn I2c + 'a>,
}
