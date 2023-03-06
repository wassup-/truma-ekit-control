use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEKitRunMode {
    pub run_mode: EKitRunMode,
}
