use crate::heating::HeatingCoil;
use embedded_hal::digital::v2::OutputPin;
use truma_ekit_core::{
    ekit::{EKit as EKitCore, EKitRunMode},
    peripherals::fan::Fan,
    types::Temperature,
    util::celsius,
};

/// The temperature treshold for entering overtemperature protection mode.
const OVERTEMPERATURE_LIMIT: Temperature = celsius(90.0);

pub trait EKit: EKitCore + Send {
    fn set_output_temperature(&mut self, output_temperature: Option<Temperature>);
}

pub struct EKitLocal<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    run_mode: EKitRunMode,
    fan: Fan<F>,
    heating_coil1: HeatingCoil<C1>,
    heating_coil2: HeatingCoil<C2>,
    is_overtemperature: bool,
}

impl<F, C1, C2> EKitLocal<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    pub fn new(
        fan: Fan<F>,
        heating_coil1: HeatingCoil<C1>,
        heating_coil2: HeatingCoil<C2>,
    ) -> Self {
        let mut ekit = EKitLocal {
            run_mode: EKitRunMode::Off,
            fan,
            heating_coil1,
            heating_coil2,
            is_overtemperature: false,
        };
        ekit.run_peripherals();
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

    /// Set the output temperature of the e-kit.
    pub fn set_output_temperature(&mut self, output_temperature: Option<Temperature>) {
        self.is_overtemperature = match output_temperature {
            Some(temperature) => temperature >= OVERTEMPERATURE_LIMIT,
            None => true, // if we failed to get the temperature, we force overtemperature protection
        };
        self.run_peripherals()
    }

    /// Run the peripherals of the e-kit.
    fn run_peripherals(&mut self) {
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
                EKitRunMode::Cool => {
                    log::info!("running fan only");
                    self.fan.turn_on();
                    self.heating_coil1.turn_off();
                    self.heating_coil2.turn_off();
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
}

impl<F, C1, C2> EKitCore for EKitLocal<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    fn request_run_mode(&mut self, run_mode: EKitRunMode) {
        self.run_mode = run_mode;
        self.run_peripherals();
    }
}

impl<F, C1, C2> EKit for EKitLocal<F, C1, C2>
where
    F: OutputPin + Send,
    C1: OutputPin + Send,
    C2: OutputPin + Send,
{
    fn set_output_temperature(&mut self, output_temperature: Option<Temperature>) {
        EKitLocal::set_output_temperature(self, output_temperature);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use truma_ekit_core::peripherals::relay::Relay;

    #[test]
    fn is_initially_turned_off() {
        assert!(!EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        )
        .is_on());
        assert!(!EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        )
        .is_on());
        assert!(!EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        )
        .is_on());
        assert!(!EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
        )
        .is_on());
        assert!(!EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
        )
        .is_on());
    }

    #[test]
    fn turns_on_and_off() {
        let mut ekit = EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
        );

        ekit.request_run_mode(EKitRunMode::Half);
        assert!(ekit.is_on(), "failed to turn on");

        ekit.request_run_mode(EKitRunMode::Off);
        assert!(!ekit.is_on(), "failed to turn off");

        ekit.request_run_mode(EKitRunMode::Full);
        assert!(ekit.is_on(), "failed to turn on");
    }

    #[test]
    fn overtemperature_protection() {
        let mut ekit = EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        );

        ekit.set_output_temperature(Some(OVERTEMPERATURE_LIMIT - celsius(0.1)));
        assert!(
            !ekit.is_overtemperature,
            "failed to exit overtemperature protection"
        );

        ekit.set_output_temperature(Some(OVERTEMPERATURE_LIMIT + celsius(0.1)));
        assert!(
            ekit.is_overtemperature,
            "failed to enter overtemperature protection"
        );

        ekit.set_output_temperature(None);
        assert!(
            ekit.is_overtemperature,
            "failed to stay in overtemperature protection"
        );

        ekit.set_output_temperature(Some(celsius(10.0)));
        assert!(
            !ekit.is_overtemperature,
            "failed to exit overtemperature protection"
        );
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
