#[derive(Copy, Clone)]
pub struct Formatter {
    precision: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UnitConverter {
    Linear { coeff: f32, constant: f32 },
}

pub trait Dimension: Unit + PartialEq {
    /// Returns this dimension's base unit.
    fn base_unit() -> Self;
    /// Returns this dimension's converter.
    fn converter(&self) -> UnitConverter;
}

pub trait Unit {
    /// Returns this unit's symbol.
    fn symbol(&self) -> &str;
}

#[derive(Copy, Clone, Debug)]
pub struct Measurement<U> {
    pub value: f32,
    pub unit: U,
}

impl<U> Measurement<U> {
    pub const fn new(value: f32, unit: U) -> Self {
        Measurement { value, unit }
    }
}

impl<U: Dimension> Measurement<U> {
    /// Converts to a given unit.
    pub fn converted_to(&self, unit: U) -> Self {
        if self.unit == unit {
            Measurement::new(self.value, unit)
        } else {
            let base_value = self.unit.converter().to_base_unit(self.value);
            if unit == U::base_unit() {
                Measurement::new(base_value, unit)
            } else {
                let unit_value = unit.converter().from_base_unit(base_value);
                Measurement::new(unit_value, unit)
            }
        }
    }
}

impl UnitConverter {
    /// Convert from base unit value.
    pub fn from_base_unit(&self, val: f32) -> f32 {
        match self {
            Self::Linear { coeff, constant } => (val * coeff) + constant,
        }
    }

    /// Convert to base unit value.
    pub fn to_base_unit(&self, val: f32) -> f32 {
        match self {
            Self::Linear { coeff, constant } => (val - constant) / coeff,
        }
    }
}

impl Formatter {
    pub fn with_precision(precision: usize) -> Self {
        Formatter { precision }
    }

    pub fn format<U: Unit>(&self, measurement: &Measurement<U>) -> String {
        format!("{0:.prec$}", measurement.value, prec = self.precision)
    }
}

impl<U: Dimension> std::ops::Add for Measurement<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Measurement::new(self.value + rhs.value, self.unit)
        } else {
            Measurement::new(
                self.unit
                    .converter()
                    .to_base_unit(self.value)
                    .add(rhs.unit.converter().to_base_unit(rhs.value)),
                U::base_unit(),
            )
        }
    }
}

impl<U: Dimension> std::ops::Sub for Measurement<U> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Measurement::new(self.value - rhs.value, self.unit)
        } else {
            Measurement::new(
                self.unit
                    .converter()
                    .to_base_unit(self.value)
                    .sub(rhs.unit.converter().to_base_unit(rhs.value)),
                U::base_unit(),
            )
        }
    }
}

impl<U: Dimension> PartialEq for Measurement<U> {
    fn eq(&self, other: &Self) -> bool {
        self.unit
            .converter()
            .to_base_unit(self.value)
            .eq(&other.unit.converter().to_base_unit(other.value))
    }
}

impl<U: Dimension> PartialOrd for Measurement<U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.unit
            .converter()
            .to_base_unit(self.value)
            .partial_cmp(&other.unit.converter().to_base_unit(other.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use std::cmp::Ordering;

    #[derive(Eq, PartialEq, Debug)]
    enum Test {
        One,
        Two,
    }

    #[test]
    fn conversion() {
        assert_approx_eq!(
            Measurement::new(1.5, Test::One)
                .converted_to(Test::One)
                .value,
            1.5
        );

        assert_approx_eq!(
            Measurement::new(1.5, Test::One)
                .converted_to(Test::Two)
                .value,
            3.0
        );

        assert_approx_eq!(
            Measurement::new(1.5, Test::Two)
                .converted_to(Test::One)
                .value,
            0.75
        );
    }

    #[test]
    fn add() {
        assert_approx_eq!(
            (Measurement::new(1.5, Test::One) + Measurement::new(2.5, Test::Two))
                .converted_to(Test::One)
                .value,
            2.75
        );
    }

    #[test]
    fn sub() {
        assert_approx_eq!(
            (Measurement::new(1.5, Test::One) - Measurement::new(2.5, Test::Two))
                .converted_to(Test::One)
                .value,
            0.25
        );
    }

    #[test]
    fn eq() {
        // same dimension
        assert_eq!(
            Measurement::new(2.0, Test::One),
            Measurement::new(2.0, Test::One)
        );

        // different dimension
        assert_eq!(
            Measurement::new(3.0, Test::One),
            Measurement::new(6.0, Test::Two)
        );
    }

    #[test]
    fn cmp() {
        // same dimension
        assert_eq!(
            Measurement::new(3.0, Test::One).partial_cmp(&Measurement::new(3.0, Test::One)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Measurement::new(3.0, Test::One).partial_cmp(&Measurement::new(4.0, Test::One)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Measurement::new(4.0, Test::One).partial_cmp(&Measurement::new(3.0, Test::One)),
            Some(Ordering::Greater)
        );

        // different dimension
        assert_eq!(
            Measurement::new(3.0, Test::One).partial_cmp(&Measurement::new(6.0, Test::Two)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Measurement::new(1.0, Test::One).partial_cmp(&Measurement::new(3.0, Test::Two)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Measurement::new(3.0, Test::One).partial_cmp(&Measurement::new(4.0, Test::Two)),
            Some(Ordering::Greater)
        );
    }

    impl Unit for Test {
        fn symbol(&self) -> &str {
            match self {
                Self::One => "One",
                Self::Two => "Two",
            }
        }
    }

    impl Dimension for Test {
        fn base_unit() -> Self {
            Test::One
        }

        fn converter(&self) -> UnitConverter {
            match self {
                Test::One => UnitConverter::Linear {
                    coeff: 1.0,
                    constant: 0.0,
                },
                Test::Two => UnitConverter::Linear {
                    coeff: 2.0,
                    constant: 0.0,
                },
            }
        }
    }
}
