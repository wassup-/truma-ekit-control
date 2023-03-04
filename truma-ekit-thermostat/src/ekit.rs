use truma_ekit_core::ekit::{EKit as EKitCore, EKitRunMode};

pub struct EKitHttp;

impl EKitHttp {
    pub fn new() -> Self {
        EKitHttp
    }
}

impl EKitCore for EKitHttp {
    fn request_run_mode(&mut self, _run_mode: EKitRunMode) {
        todo!("request run mode")
    }
}
