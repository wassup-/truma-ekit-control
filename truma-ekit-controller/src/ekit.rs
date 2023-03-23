use crate::{heating::HeatingCoil, overtemperature_protection::OvertemperatureProtection};
use embedded_hal::digital::v2::OutputPin;
use truma_ekit_core::{
    ekit::{EKit as EKitCore, EKitSystemRunMode, EKitUserRunMode},
    measurement::Formatter,
    peripherals::fan::Fan,
    types::Temperature,
};

pub trait EKit: EKitCore + Send {
    fn set_output_temperature(&mut self, output_temperature: Option<Temperature>);
}

pub struct EKitLocal<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    run_mode: EKitSystemRunMode,
    fan: Fan<F>,
    heating_coil1: HeatingCoil<C1>,
    heating_coil2: HeatingCoil<C2>,
    overtemperature_protection: OvertemperatureProtection,
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
            run_mode: EKitSystemRunMode::Off,
            fan,
            heating_coil1,
            heating_coil2,
            overtemperature_protection: OvertemperatureProtection::inactive(),
        };
        ekit.enter_run_mode(EKitSystemRunMode::Off);
        ekit
    }

    /// Returns `true` if the e-kit is currently turned on.
    #[allow(dead_code)]
    pub fn is_on(&self) -> bool {
        !matches!(self.run_mode, EKitSystemRunMode::Off)
    }

    /// Set the output temperature of the e-kit.
    pub fn set_output_temperature(&mut self, output_temperature: Option<Temperature>) {
        if let Some(output_temperature) = output_temperature {
            let formatter = Formatter::with_precision(2);
            log::info!(
                "output temperature: {}",
                formatter.format(&output_temperature)
            );
        }

        self.overtemperature_protection
            .output_temperature_changed(output_temperature);

        self.update_run_mode(None);
    }

    /// Request the e-kit run mode.
    fn request_run_mode(&mut self, run_mode: EKitSystemRunMode) {
        log::info!("request system run mode {:?}", run_mode);

        // disallow changing the run mode when cooldown is active
        if matches!(self.run_mode, EKitSystemRunMode::Cooldown) {
            log::info!("cooldown active, request denied");
            return;
        }

        if matches!(run_mode, EKitSystemRunMode::Off) {
            self.overtemperature_protection.enter();
        }

        self.update_run_mode(Some(run_mode));
    }

    /// Update the e-kit run mode.
    ///
    /// Overtempetature protection forced run mode takes priority over `requested_run_mode`.
    fn update_run_mode(&mut self, requested_run_mode: Option<EKitSystemRunMode>) {
        if let Some(forced_mode) = self.overtemperature_protection.forced_run_mode() {
            log::info!("forcing run mode {:?}", forced_mode);
            self.enter_run_mode(forced_mode);
        } else if let Some(requested_mode) = requested_run_mode {
            self.enter_run_mode(requested_mode);
        };
    }

    /// Enter the e-kit run mode.
    fn enter_run_mode(&mut self, run_mode: EKitSystemRunMode) {
        log::info!("entering run mode {:?}", run_mode);

        match run_mode {
            EKitSystemRunMode::Off => {
                self.heating_coil1.turn_off();
                self.heating_coil2.turn_off();
                // turn off the fan after turning off the heating coils (in case the latter fails)
                self.fan.turn_off();
            }
            EKitSystemRunMode::Cooldown | EKitSystemRunMode::Cool => {
                self.fan.turn_on();
                self.heating_coil1.turn_off();
                self.heating_coil2.turn_off();
            }
            EKitSystemRunMode::Half => {
                // turn on the fan before turning on the heating coils (in case the former fails)
                self.fan.turn_on();
                self.heating_coil1.turn_on();
                self.heating_coil2.turn_off();
            }
            EKitSystemRunMode::Full => {
                // turn on the fan before turning on the heating coils (in case the former fails)
                self.fan.turn_on();
                self.heating_coil1.turn_on();
                self.heating_coil2.turn_on();
            }
        }

        self.run_mode = run_mode;
    }
}

impl<F, C1, C2> EKitCore for EKitLocal<F, C1, C2>
where
    F: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
{
    fn request_user_run_mode(&mut self, run_mode: EKitUserRunMode) {
        log::info!("request user run mode {:?}", run_mode);

        self.request_run_mode(match run_mode {
            EKitUserRunMode::Off => EKitSystemRunMode::Off,
            EKitUserRunMode::Cool => EKitSystemRunMode::Cool,
            EKitUserRunMode::Half => EKitSystemRunMode::Half,
            EKitUserRunMode::Full => EKitSystemRunMode::Full,
        })
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
    use embedded_hal::digital::v2::StatefulOutputPin;
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
        assert!(!EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(true))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(true))),
        )
        .is_on());
    }

    #[test]
    fn update_run_mode_prioritizes_overtemperature_protection() {
        let mut ekit = EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        );

        ekit.update_run_mode(None);
        assert_eq!(ekit.run_mode, EKitSystemRunMode::Off);
        ekit.update_run_mode(Some(EKitSystemRunMode::Full));
        assert_eq!(ekit.run_mode, EKitSystemRunMode::Full);

        ekit.overtemperature_protection.enter();
        ekit.update_run_mode(None);
        assert_eq!(ekit.run_mode, EKitSystemRunMode::Cooldown);
        ekit.update_run_mode(Some(EKitSystemRunMode::Full));
        assert_eq!(ekit.run_mode, EKitSystemRunMode::Cooldown);
    }

    #[test]
    fn overtemperature_protection() {
        let mut ekit = EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        );

        ekit.run_mode = EKitSystemRunMode::Cool;

        ekit.request_run_mode(EKitSystemRunMode::Off);
        assert_eq!(
            ekit.run_mode,
            EKitSystemRunMode::Cooldown,
            "failed to enter overtemperature protection"
        );

        ekit.run_mode = EKitSystemRunMode::Cool;

        ekit.overtemperature_protection.enter();
        ekit.update_run_mode(None);

        assert_eq!(
            ekit.run_mode,
            EKitSystemRunMode::Cooldown,
            "failed to enter overtemperature protection"
        );

        ekit.set_output_temperature(None);
        assert_eq!(
            ekit.run_mode,
            EKitSystemRunMode::Cooldown,
            "failed to stay in overtemperature protection"
        );

        ekit.overtemperature_protection.exit();
        ekit.update_run_mode(None);
        assert_eq!(
            ekit.run_mode,
            EKitSystemRunMode::Off,
            "failed to turn exit overtemperature protection"
        );
    }

    #[test]
    fn turns_peripherals_on_and_off() {
        let mut ekit = EKitLocal::new(
            Fan::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
            HeatingCoil::new(Relay::connected_to(TestPin(false))),
        );

        ekit.enter_run_mode(EKitSystemRunMode::Cool);
        assert!(
            ekit.fan.is_turned_on()
                && !ekit.heating_coil1.is_turned_on()
                && !ekit.heating_coil2.is_turned_on()
        );

        ekit.enter_run_mode(EKitSystemRunMode::Off);
        assert!(
            !ekit.fan.is_turned_on()
                && !ekit.heating_coil1.is_turned_on()
                && !ekit.heating_coil2.is_turned_on()
        );

        ekit.enter_run_mode(EKitSystemRunMode::Half);
        assert!(
            ekit.fan.is_turned_on()
                && ekit.heating_coil1.is_turned_on()
                && !ekit.heating_coil2.is_turned_on()
        );

        ekit.enter_run_mode(EKitSystemRunMode::Off);
        assert!(
            !ekit.fan.is_turned_on()
                && !ekit.heating_coil1.is_turned_on()
                && !ekit.heating_coil2.is_turned_on()
        );

        ekit.enter_run_mode(EKitSystemRunMode::Full);
        assert!(
            ekit.fan.is_turned_on()
                && ekit.heating_coil1.is_turned_on()
                && ekit.heating_coil2.is_turned_on()
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

    impl StatefulOutputPin for TestPin {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.0)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.0)
        }
    }
}
