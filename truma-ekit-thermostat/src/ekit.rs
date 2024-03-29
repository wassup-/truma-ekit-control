use crate::wifi::{WifiClient, WifiClientError};
use embedded_svc::{
    http::{
        client::{Client as HttpClient, Connection, Response},
        Status,
    },
    io::{Read, Write},
};
use esp_idf_svc::{
    errors::EspIOError,
    http::client::{Configuration, EspHttpConnection},
};
use esp_idf_sys::EspError;
use truma_ekit_core::ekit::{EKit as EKitCore, EKitUserRunMode, PostEKitRunMode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ESP error: {0}")]
    Esp(#[from] EspError),
    #[error("ESP IO error: {0}")]
    Io(#[from] EspIOError),
    #[error("unexpected status {0}")]
    UnexpectedStatus(u16),
    #[error(transparent)]
    WifiClient(#[from] WifiClientError),
}

pub struct EKitHttp<'a> {
    hostname: &'static str,
    client: HttpClient<EspHttpConnection>,
    wifi: WifiClient<'a>,
}

impl<'a> EKitHttp<'a> {
    pub fn new(hostname: &'static str, wifi: WifiClient<'a>) -> Self {
        // remove trailing slash from hostname
        let hostname = hostname.strip_suffix('/').unwrap_or(hostname);
        let conn = EspHttpConnection::new(&Configuration::default()).unwrap();
        let client = HttpClient::wrap(conn);

        EKitHttp {
            hostname,
            client,
            wifi,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.wifi.is_connected()
    }

    fn post(&mut self, path: &str, payload: &[u8]) -> Result<(), Error> {
        log::info!("POST {} {:?}", path, payload);

        self.wifi.connect()?;

        let content_length_header = format!("{}", payload.len());
        let headers = [
            ("accept", "text/plain"),
            ("content-type", "text/plain"),
            ("connection", "close"),
            ("content-length", &*content_length_header),
        ];

        let url = format!("{}{}", self.hostname, path);
        let mut req = self.client.post(&url, &headers)?;
        req.write_all(payload)?;
        req.flush()?;

        let res = req.submit()?;
        let status = res.status();
        // drain the full response body
        EKitHttp::drain_response(res);

        if status == 200 {
            Ok(())
        } else {
            Err(Error::UnexpectedStatus(status))
        }
    }

    fn drain_response<C>(mut resp: Response<C>)
    where
        C: Connection,
    {
        let (_headers, body) = resp.split();
        let mut buf = [0_u8; 1024];

        loop {
            match body.read(&mut buf) {
                Ok(len) if len > 0 => continue,
                _ => break,
            }
        }
    }
}

impl<'a> EKitCore for EKitHttp<'a> {
    fn request_user_run_mode(&mut self, run_mode: EKitUserRunMode) {
        log::info!("requesting e-kit run mode {:?}...", run_mode);

        let payload = serde_urlencoded::to_string(PostEKitRunMode { run_mode }).unwrap();
        match self.post("/run-mode", payload.as_bytes()) {
            Ok(_) => log::info!("e-kit run mode requested"),
            Err(e) => log::error!("failed to request e-kit run mode ({})", e),
        }
    }
}
