use truma_ekit_core::{ekit::EKitSystemRunMode, types::Temperature, util::celsius};

/// Cooldown will be entered if the output temperature is greater than or equal to this limit.
const COOLDOWN_ENTER: Temperature = celsius(90.0);
/// Cooldown will be exited if the output temperature is less than than or equal to this limit.
const COOLDOWN_EXIT: Temperature = celsius(50.0);

#[derive(Debug)]
pub struct OvertemperatureProtection {
    is_active: bool,
    was_active: bool,
}

impl OvertemperatureProtection {
    /// Returns an inactive overtemperature protection.
    pub fn inactive() -> Self {
        OvertemperatureProtection {
            is_active: false,
            was_active: false,
        }
    }

    /// Enter overtemperature protection.
    pub fn enter(&mut self) {
        self.is_active = true;
    }

    /// Exit overtemperature protection.
    #[cfg(test)]
    pub fn exit(&mut self) {
        self.was_active = self.is_active;
        self.is_active = false;
    }

    /// Signals that the e-kit output temperature has changed.
    pub fn output_temperature_changed(&mut self, output_temperature: Option<Temperature>) {
        self.was_active = self.is_active;
        self.is_active = match output_temperature {
            Some(temperature) => {
                if self.is_active {
                    // exit overtemperature protection once the output temperature is less than or equal to `COOLDOWN_EXIT`
                    !(temperature <= COOLDOWN_EXIT)
                } else {
                    // enter overtemperature protection once the output temperature is greater than or equal to `COOLDOWN_ENTER`
                    temperature >= COOLDOWN_ENTER
                }
            }
            // if we failed to get the temperature, we force overtemperature protection
            None => true,
        }
    }

    /// Returns the forced e-kit system run mode.
    pub fn forced_run_mode(&self) -> Option<EKitSystemRunMode> {
        if self.is_active {
            // cooldown
            Some(EKitSystemRunMode::Cooldown)
        } else if self.was_active {
            // turn off after cooling down
            Some(EKitSystemRunMode::Off)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initially_inactive() {
        let sub = OvertemperatureProtection::inactive();
        assert!(!sub.is_active);
        assert!(!sub.was_active);
    }

    #[test]
    fn enter() {
        let mut sub = OvertemperatureProtection::inactive();
        sub.enter();
        assert!(sub.is_active);
        assert!(!sub.was_active);
    }

    #[test]
    fn exit() {
        let mut sub = OvertemperatureProtection::inactive();
        sub.exit();
        assert!(!sub.was_active);
        assert!(!sub.is_active);

        let mut sub = OvertemperatureProtection::inactive();
        sub.enter();
        sub.exit();
        assert!(sub.was_active);
        assert!(!sub.is_active);
    }

    #[test]
    fn activates_on_overtemperature() {
        let mut sub = OvertemperatureProtection::inactive();
        sub.output_temperature_changed(Some(COOLDOWN_ENTER));
        assert!(sub.is_active);
    }

    #[test]
    fn deactivates_after_cooldown() {
        let mut sub = OvertemperatureProtection::inactive();
        sub.enter();
        sub.output_temperature_changed(Some(COOLDOWN_EXIT));
        assert!(!sub.is_active);
        assert!(sub.was_active);
    }

    #[test]
    fn enters_cooldown_when_active() {
        let mut sub = OvertemperatureProtection::inactive();
        sub.enter();
        assert_eq!(sub.forced_run_mode(), Some(EKitSystemRunMode::Cooldown));
    }

    #[test]
    fn turns_off_after_cooldown() {
        let mut sub = OvertemperatureProtection::inactive();
        sub.output_temperature_changed(Some(COOLDOWN_ENTER));
        sub.output_temperature_changed(Some(COOLDOWN_EXIT));
        assert_eq!(sub.forced_run_mode(), Some(EKitSystemRunMode::Off));
    }
}
