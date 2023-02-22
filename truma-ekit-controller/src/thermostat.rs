use crate::thermometer::Thermometer;
use truma_ekit_core::types::Temperature;

pub struct Thermostat {
    requested_temperature: Temperature,
    thermometer: Box<dyn Thermometer>,
}

impl Thermostat {
    pub fn new(requested_temperature: Temperature, thermometer: Box<dyn Thermometer>) -> Self {
        Thermostat {
            requested_temperature,
            thermometer,
        }
    }

    /// Set the requested temperature.
    pub fn set_requested_temperature(&mut self, requested_temperature: Temperature) {
        self.requested_temperature = requested_temperature
    }

    /// Returns the requested temperature.
    pub fn requested_temperature(&self) -> Temperature {
        self.requested_temperature.clone()
    }

    /// Returns the actual temperature.
    pub fn actual_temperature(&mut self) -> Option<Temperature> {
        self.thermometer.measure().ok()?;
        self.thermometer.temperature()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        thermometer::{FakeTemperature, NoTemperature},
        util::celsius,
    };

    #[test]
    fn returns_actual_temperature() {
        assert_eq!(
            Thermostat::new(celsius(21.0), Box::new(FakeTemperature(celsius(21.3))))
                .actual_temperature()
                .unwrap(),
            celsius(21.3)
        );
        assert!(Thermostat::new(celsius(21.0), Box::new(NoTemperature))
            .actual_temperature()
            .is_none());
    }
}
