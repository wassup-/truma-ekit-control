#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

    pub fn set_run_mode(&mut self, _run_mode: EKitRunMode) -> anyhow::Result<()> {
        todo!("set run mode")
    }
}
