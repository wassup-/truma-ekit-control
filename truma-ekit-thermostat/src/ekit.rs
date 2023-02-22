pub enum EKitRunMode {
    Off,
    Half,
    Full,
}

pub struct EKit;

impl EKit {
    pub fn new() -> Self {
        EKit
    }

    pub fn set_run_mode(&mut self, run_mode: EKitRunMode) -> anyhow::Result<()> {
        todo!("set run mode")
    }
}
