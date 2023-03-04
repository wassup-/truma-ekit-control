#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EKitRunMode {
    Off,
    Cool,
    Half,
    Full,
}

pub trait EKit {
    /// Request the e-kit run mode.
    fn request_run_mode(&mut self, run_mode: EKitRunMode);
}
