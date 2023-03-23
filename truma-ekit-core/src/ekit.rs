use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EKitSystemRunMode {
    Off,
    Cooldown,
    Cool,
    Half,
    Full,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum EKitUserRunMode {
    Off,
    Cool,
    Half,
    Full,
}

pub trait EKit {
    /// Request the e-kit user run mode.
    fn request_user_run_mode(&mut self, run_mode: EKitUserRunMode);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEKitRunMode {
    pub run_mode: EKitUserRunMode,
}
