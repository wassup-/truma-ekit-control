use crate::celsius;
use truma_ekit_core::{ekit::EKitUserRunMode, types::Temperature};

/// The threshold for running the controller at full capacity.
/// If the temperature difference is below this value, the controller will be run at half capacity.
const FULL_CAPACITY_TRESHOLD: Temperature = celsius(1.5);

pub struct Thermostat {
    requested_temperature: Temperature,
}

impl Thermostat {
    pub fn new(requested_temperature: Temperature) -> Self {
        Thermostat {
            requested_temperature,
        }
    }

    /// Get the requested temperature.
    pub fn requested_temperature(&self) -> Temperature {
        self.requested_temperature
    }

    /// Set the requested temperature.
    pub fn set_requested_temperature(&mut self, temperature: Temperature) {
        self.requested_temperature = temperature;
    }

    /// Get the suggested run mode for the given actual temperature.
    pub fn suggested_ekit_run_mode(&self, actual_temperature: Temperature) -> EKitUserRunMode {
        if actual_temperature >= self.requested_temperature {
            // the actual temperature is equal to or higher than the requested temperature, turn off the heating
            EKitUserRunMode::Off
        } else {
            // the actual temperature is less than the requested temperature, turn on the heating
            let temp_diff = self.requested_temperature - actual_temperature;
            if temp_diff < FULL_CAPACITY_TRESHOLD {
                // run the heating at half capacity
                EKitUserRunMode::Half
            } else {
                // run the heating at full capacity
                EKitUserRunMode::Full
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use truma_ekit_core::util::celsius;

    #[test]
    fn actual_temperature_lower_than_requested_temperature() {
        let thermostat = Thermostat::new(celsius(21.0));
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(20.9)),
            EKitUserRunMode::Half
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(20.0)),
            EKitUserRunMode::Half
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(19.6)),
            EKitUserRunMode::Half
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(19.5)),
            EKitUserRunMode::Full
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(0.0)),
            EKitUserRunMode::Full
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(-10.0)),
            EKitUserRunMode::Full
        );
    }

    #[test]
    fn actual_temperature_equal_to_requested_temperature() {
        assert_eq!(
            Thermostat::new(celsius(0.0)).suggested_ekit_run_mode(celsius(0.0)),
            EKitUserRunMode::Off
        );
        assert_eq!(
            Thermostat::new(celsius(21.0)).suggested_ekit_run_mode(celsius(21.0)),
            EKitUserRunMode::Off
        );
    }

    #[test]
    fn actual_temperature_higher_than_requested_temperature() {
        assert_eq!(
            Thermostat::new(celsius(21.0)).suggested_ekit_run_mode(celsius(21.1)),
            EKitUserRunMode::Off
        );
        assert_eq!(
            Thermostat::new(celsius(0.0)).suggested_ekit_run_mode(celsius(1.0)),
            EKitUserRunMode::Off
        );
    }
}
