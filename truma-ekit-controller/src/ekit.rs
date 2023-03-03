use crate::{heating::HeatingCoil, thermometer::Thermometer};
use embedded_hal::digital::v2::OutputPin;
use truma_ekit_core::{
    peripherals::fan::Fan,
    types::Temperature,
    util::{celsius, format_temperature},
};

#[derive(Copy, Clone, Debug)]
pub enum EKitRunMode {
    Off,
    Half,
    Full,
}

const OVERTEMPERATURE_LIMIT: Temperature = celsius(90.0);

pub struct EKit<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    run_mode: EKitRunMode,
    is_overtemperature: bool,
    fan: Fan<F>,
    heating_coil1: HeatingCoil<C1>,
    heating_coil2: HeatingCoil<C2>,
    thermometer: Box<dyn Thermometer>,
}

impl<F, C1, C2> EKit<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    pub fn new(
        fan: Fan<F>,
        heating_coil1: HeatingCoil<C1>,
        heating_coil2: HeatingCoil<C2>,
        thermometer: Box<dyn Thermometer>,
    ) -> Self {
        let mut ekit = EKit {
            run_mode: EKitRunMode::Off,
            is_overtemperature: false,
            fan,
            heating_coil1,
            heating_coil2,
            thermometer,
        };
        ekit.run_mode_changed();
        ekit
    }

    /// Returns `true` if the e-kit is currently turned on.
    #[allow(dead_code)]
    pub fn is_on(&self) -> bool {
        if self.is_overtemperature {
            false
        } else {
            matches!(self.run_mode, EKitRunMode::Half | EKitRunMode::Full)
        }
    }

    /// Set the run mode.
    pub fn set_run_mode(&mut self, run_mode: EKitRunMode) {
        self.run_mode = run_mode;
        self.run_mode_changed();
    }

    /// Signal that the run mode has changed.
    fn run_mode_changed(&mut self) {
        self.is_overtemperature = match self.current_temperature() {
            Some(temperature) => temperature >= OVERTEMPERATURE_LIMIT,
            None => true, // if we failed to get the temperature, we force overtemperature protection
        };

        if self.is_overtemperature {
            log::warn!("overtemperature protection active");
            self.fan.turn_on();
            self.heating_coil1.turn_off();
            self.heating_coil2.turn_off();
        } else {
            match self.run_mode {
                EKitRunMode::Off => {
                    log::info!("turning off");
                    // turn off the heating coils before turning off the fan (in case the former fails)
                    self.heating_coil1.turn_off();
                    self.heating_coil2.turn_off();
                    self.fan.turn_off();
                }
                EKitRunMode::Half => {
                    log::info!("running at half capacity");
                    // turn on the fan before turning on the heating coils (in case the former fails)
                    self.fan.turn_on();
                    self.heating_coil1.turn_on();
                    self.heating_coil2.turn_off();
                }
                EKitRunMode::Full => {
                    log::info!("running at full capacity");
                    // turn on the fan before turning on the heating coils (in case the former fails)
                    self.fan.turn_on();
                    self.heating_coil1.turn_on();
                    self.heating_coil2.turn_on();
                }
            }
        }
    }

    fn current_temperature(&mut self) -> Option<Temperature> {
        self.thermometer.measure().ok()?;
        match self.thermometer.temperature() {
            Some(temperature) => {
                log::debug!("current temperature: {}", format_temperature(&temperature));
                Some(temperature)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thermometer::{FakeTemperature, NoTemperature};
    use truma_ekit_core::peripherals::relay::Relay;

    #[test]
    fn is_initially_turned_off() {
        let ekit = EKit::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            Box::new(FakeTemperature(celsius(21.0))),
        );
        assert!(!ekit.is_on())
    }

    #[test]
    fn overtemperature_protection() {
        let ekit = EKit::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            Box::new(FakeTemperature(OVERTEMPERATURE_LIMIT - celsius(0.1))),
        );
        assert!(!ekit.is_overtemperature);

        let ekit = EKit::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            Box::new(FakeTemperature(OVERTEMPERATURE_LIMIT + celsius(0.1))),
        );
        assert!(ekit.is_overtemperature);

        let ekit = EKit::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            Box::new(NoTemperature),
        );
        assert!(ekit.is_overtemperature);
    }

    struct TestPin(bool);

    impl OutputPin for TestPin {
        type Error = std::convert::Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.0 = true;
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.0 = false;
            Ok(())
        }
    }
}
