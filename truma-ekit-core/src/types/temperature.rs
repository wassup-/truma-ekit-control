use crate::measurement::{Dimension, Measurement, Unit, UnitConverter};

#[derive(Clone, PartialEq, Debug)]
pub struct UnitTemperature {
    symbol: &'static str,
    converter: UnitConverter,
}

pub type Temperature = Measurement<UnitTemperature>;

impl UnitTemperature {
    const fn new_with_coeff_constant(symbol: &'static str, coeff: f32, constant: f32) -> Self {
        UnitTemperature {
            symbol,
            converter: UnitConverter::Linear { coeff, constant },
        }
    }

    pub const fn kelvin() -> Self {
        UnitTemperature::new_with_coeff_constant("K", 1.0, 0.0)
    }

    pub const fn celsius() -> Self {
        UnitTemperature::new_with_coeff_constant("°C", 1.0, -273.15)
    }

    pub const fn fahrenheit() -> Self {
        UnitTemperature::new_with_coeff_constant("°F", 1.8, -459.67)
    }
}

impl Unit for UnitTemperature {
    fn symbol(&self) -> &str {
        self.symbol
    }
}

impl Dimension for UnitTemperature {
    fn base_unit() -> Self {
        Self::kelvin()
    }

    fn converter(&self) -> UnitConverter {
        self.converter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn kelvin_to_other() {
        assert_approx_eq!(
            Measurement::new(20.5, UnitTemperature::kelvin())
                .converted_to(UnitTemperature::kelvin())
                .value,
            20.5
        );

        assert_approx_eq!(
            Measurement::new(21.0, UnitTemperature::kelvin())
                .converted_to(UnitTemperature::celsius())
                .value,
            -252.15
        );

        assert_approx_eq!(
            Measurement::new(5.0, UnitTemperature::kelvin())
                .converted_to(UnitTemperature::fahrenheit())
                .value,
            -450.67
        );
    }

    #[test]
    fn other_to_kelvin() {
        assert_approx_eq!(
            Measurement::new(15.0, UnitTemperature::kelvin())
                .converted_to(UnitTemperature::kelvin())
                .value,
            15.0
        );

        assert_approx_eq!(
            Measurement::new(18.5, UnitTemperature::celsius())
                .converted_to(UnitTemperature::kelvin())
                .value,
            291.65
        );

        assert_approx_eq!(
            Measurement::new(210.5, UnitTemperature::fahrenheit())
                .converted_to(UnitTemperature::kelvin())
                .value,
            372.3167
        );
    }
}
