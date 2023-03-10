use crate::celsius;
use truma_ekit_core::{ekit::EKitRunMode, types::Temperature};

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

    /// Set the requested temperature.
    pub fn set_requested_temperature(&mut self, requested_temperature: Temperature) {
        self.requested_temperature = requested_temperature
    }

    /// Get the suggested run mode for the given actual temperature.
    pub fn suggested_ekit_run_mode(&self, actual_temperature: Temperature) -> EKitRunMode {
        if actual_temperature >= self.requested_temperature {
            // the actual temperature is equal to or higher than the requested temperature, turn off the heating
            EKitRunMode::Off
        } else {
            // the actual temperature is less than the requested temperature, turn on the heating
            let temp_diff = self.requested_temperature - actual_temperature;
            if temp_diff < FULL_CAPACITY_TRESHOLD {
                // run the heating at half capacity
                EKitRunMode::Half
            } else {
                // run the heating at full capacity
                EKitRunMode::Full
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
            EKitRunMode::Half
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(20.0)),
            EKitRunMode::Half
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(19.6)),
            EKitRunMode::Half
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(19.5)),
            EKitRunMode::Full
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(0.0)),
            EKitRunMode::Full
        );
        assert_eq!(
            thermostat.suggested_ekit_run_mode(celsius(-10.0)),
            EKitRunMode::Full
        );
    }

    #[test]
    fn actual_temperature_equal_to_requested_temperature() {
        assert_eq!(
            Thermostat::new(celsius(0.0)).suggested_ekit_run_mode(celsius(0.0)),
            EKitRunMode::Off
        );
        assert_eq!(
            Thermostat::new(celsius(21.0)).suggested_ekit_run_mode(celsius(21.0)),
            EKitRunMode::Off
        );
    }

    #[test]
    fn actual_temperature_higher_than_requested_temperature() {
        assert_eq!(
            Thermostat::new(celsius(21.0)).suggested_ekit_run_mode(celsius(21.1)),
            EKitRunMode::Off
        );
        assert_eq!(
            Thermostat::new(celsius(0.0)).suggested_ekit_run_mode(celsius(1.0)),
            EKitRunMode::Off
        );
    }
}
