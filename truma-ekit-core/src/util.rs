use crate::{
    measurement::{Formatter, Measurement},
    types::{Temperature, UnitTemperature},
};

pub const fn celsius(temperature: f32) -> Temperature {
    Measurement::new(temperature, UnitTemperature::celsius())
}

pub fn format_temperature(temperature: &Temperature) -> String {
    Formatter::with_precision(2).format(temperature)
}
