use embedded_svc::http::client::Client as HttpClient;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use truma_ekit_core::ekit::{EKit as EKitCore, EKitRunMode};

pub struct EKitHttp {
    client: HttpClient<EspHttpConnection>,
}

impl EKitHttp {
    pub fn new() -> Self {
        let conn = EspHttpConnection::new(&Configuration::default()).unwrap();
        EKitHttp {
            client: HttpClient::wrap(conn),
        }
    }
}

impl EKitCore for EKitHttp {
    fn request_run_mode(&mut self, _run_mode: EKitRunMode) {
        todo!("request run mode")
    }
}
