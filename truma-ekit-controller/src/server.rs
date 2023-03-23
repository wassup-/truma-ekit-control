use embedded_svc::http::Method;
use esp_idf_svc::{
    errors::EspIOError,
    http::server::{Configuration, EspHttpServer},
};
use std::sync::{Arc, Mutex};
use truma_ekit_core::ekit::{EKit, PostEKitRunMode};

#[derive(thiserror::Error, Debug)]
pub enum EKitServerError {
    #[error("IO error: {0}")]
    IoError(#[from] EspIOError),
}

pub struct EKitHttpServer<E: EKit> {
    server: EspHttpServer,
    ekit: Arc<Mutex<E>>,
}

impl<E> EKitHttpServer<E>
where
    E: EKit + Send + 'static,
{
    pub fn new(ekit: Arc<Mutex<E>>) -> Result<Self, EKitServerError> {
        let server = EspHttpServer::new(&Configuration::default())?;
        Ok(EKitHttpServer { server, ekit })
    }

    pub fn start(&mut self) -> Result<(), EKitServerError> {
        let ekit = self.ekit.clone();
        self.server
            .fn_handler("/run-mode", Method::Post, move |mut req| {
                let (_, body) = req.split();
                let mut buf = [0_u8; 1024];
                let count = body.read(&mut buf)?;
                let post: PostEKitRunMode = serde_urlencoded::from_bytes(&buf[..count])?;

                log::info!("e-kit run mode {:?} requested", post.run_mode);

                let mut ekit = ekit.lock()?;
                ekit.request_user_run_mode(post.run_mode);

                req.into_ok_response()?;

                Ok(())
            })
            .unwrap();
        Ok(())
    }
}
