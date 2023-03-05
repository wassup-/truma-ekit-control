use embedded_svc::{
    http::{client::Client as HttpClient, Status},
    io::Write,
};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use truma_ekit_core::ekit::{EKit as EKitCore, EKitRunMode};

pub struct EKitHttp {
    client: HttpClient<EspHttpConnection>,
    hostname: &'static str,
}

impl EKitHttp {
    pub fn new(hostname: &'static str) -> Self {
        let conn = EspHttpConnection::new(&Configuration::default()).unwrap();
        EKitHttp {
            client: HttpClient::wrap(conn),
            hostname,
        }
    }

    fn post(&mut self, payload: &[u8]) -> anyhow::Result<()> {
        log::info!("POST {:?}", payload);

        let content_length_header = format!("{}", payload.len());
        let headers = [
            ("accept", "text/plain"),
            ("content-type", "text/plain"),
            ("connection", "close"),
            ("content-length", &*content_length_header),
        ];

        let mut req = self.client.post(self.hostname, &headers)?;
        req.write_all(payload)?;
        req.flush()?;

        let res = req.submit()?;
        let status = res.status();

        return if status == 200 {
            Ok(())
        } else {
            anyhow::bail!(
                "{} returned unexpected status code {}",
                self.hostname,
                status
            )
        };
    }
}

impl EKitCore for EKitHttp {
    fn request_run_mode(&mut self, _run_mode: EKitRunMode) {
        let payload = b"\0";
        match self.post(payload) {
            Ok(_) => log::info!("e-kit run mode requested"),
            Err(e) => log::error!("failed to request e-kit run mode ({})", e),
        }
    }
}
