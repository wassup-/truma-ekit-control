use truma_ekit_core::types::Temperature;

pub struct CachedTemperature {
    most_recent_temperature: Option<Temperature>,
    last_known_temperature: Option<Temperature>,
}

impl CachedTemperature {
    pub fn new(temperature: Option<Temperature>) -> Self {
        CachedTemperature {
            most_recent_temperature: temperature,
            last_known_temperature: temperature,
        }
    }

    /// Returns the most recent temperature.
    pub fn most_recent_temperature(&self) -> Option<Temperature> {
        self.most_recent_temperature
    }

    /// Returns the last known temperature. This may or may not be the most recent temperature.
    pub fn last_known_temperature(&self) -> Option<Temperature> {
        self.most_recent_temperature.or(self.last_known_temperature)
    }

    /// Update the actual temperature.
    pub fn update(&mut self, temperature: Option<Temperature>) {
        if let Some(temperature) = temperature {
            self.most_recent_temperature = Some(temperature);
            self.last_known_temperature = Some(temperature);
        } else {
            self.most_recent_temperature = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use truma_ekit_core::util::celsius;

    #[test]
    fn initial_values() {
        let sub = CachedTemperature::new(None);
        assert_eq!(sub.most_recent_temperature(), None);
        assert_eq!(sub.last_known_temperature(), None);

        let sub = CachedTemperature::new(Some(celsius(10.3)));
        assert_eq!(sub.most_recent_temperature(), Some(celsius(10.3)));
        assert_eq!(sub.last_known_temperature(), Some(celsius(10.3)));
    }

    #[test]
    fn updated_values() {
        let mut sub = CachedTemperature::new(None);
        assert_eq!(sub.most_recent_temperature(), None);
        assert_eq!(sub.last_known_temperature(), None);

        sub.update(None);
        assert_eq!(sub.most_recent_temperature(), None);
        assert_eq!(sub.last_known_temperature(), None);

        sub.update(Some(celsius(13.5)));
        assert_eq!(sub.most_recent_temperature(), Some(celsius(13.5)));
        assert_eq!(sub.last_known_temperature(), Some(celsius(13.5)));

        sub.update(None);
        assert_eq!(sub.most_recent_temperature(), None);
        assert_eq!(sub.last_known_temperature(), Some(celsius(13.5)));

        sub.update(Some(celsius(15.0)));
        assert_eq!(sub.most_recent_temperature(), Some(celsius(15.0)));
        assert_eq!(sub.last_known_temperature(), Some(celsius(15.0)));

        sub.update(Some(celsius(0.0)));
        assert_eq!(sub.most_recent_temperature(), Some(celsius(0.0)));
        assert_eq!(sub.last_known_temperature(), Some(celsius(0.0)));

        sub.update(None);
        assert_eq!(sub.most_recent_temperature(), None);
        assert_eq!(sub.last_known_temperature(), Some(celsius(0.0)));

        sub.update(None);
        assert_eq!(sub.most_recent_temperature(), None);
        assert_eq!(sub.last_known_temperature(), Some(celsius(0.0)));
    }
}
