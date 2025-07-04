use crate::cmd::Runnable;
use crate::error::{Error, Result};
use crate::util::{audit, fs};
use bytes::Bytes;
use clap::Args;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming as IncomingBody;
use hyper::http::StatusCode;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, private_key};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Address to bind, e.g. 127.0.0.1:8080
    #[arg(long, default_value = "127.0.0.1:8080")]
    pub addr: String,

    /// TLS certificate path
    #[arg(long)]
    pub tls_cert: String,

    /// TLS private key path
    #[arg(long)]
    pub tls_key: String,

    /// Bearer token for authentication
    #[arg(long)]
    pub token: String,
}

#[derive(Deserialize)]
struct CertRequest {
    cn: String,
    days: Option<u32>,
}

impl Runnable for ServeArgs {
    fn run(self, json: bool) -> Result<()> {
        let addr: SocketAddr = self
            .addr
            .parse::<SocketAddr>()
            .map_err(|e| Error::Other(e.to_string()))?;
        info!("starting API on {}", addr);
      
        let certs = load_certs(&self.tls_cert)?;
        let key = load_private_key(&self.tls_key)?;
        let tls_cfg = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| Error::Other(e.to_string()))?;
        let tls_cfg = Arc::new(tls_cfg);

        let token = Arc::new(self.token);
        fs::ensure_dirs()?;
        let rt = tokio::runtime::Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        rt.block_on(async move {
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| Error::Other(e.to_string()))?;
            loop {
                let (stream, _) = listener
                    .accept()
                    .await
                    .map_err(|e| Error::Other(e.to_string()))?;
                let acceptor = tokio_rustls::TlsAcceptor::from(tls_cfg.clone());
                let service = service_fn({
                    let token = token.clone();
                    move |req| handle(req, token.clone())
                });
                tokio::spawn(async move {
                    match acceptor.accept(stream).await {
                        Ok(tls) => {
                            let io = TokioIo::new(tls);
                            if let Err(e) =
                                http1::Builder::new().serve_connection(io, service).await
                            {
                                error!("server error: {}", e);
                            }
                        }
                        Err(e) => error!("tls error: {}", e),
                    }
                });
            }
            #[allow(unreachable_code)]
            Ok::<(), Error>(())
        })?;
        audit::emit("serve", &self.addr, json)?;
        Ok(())
    }
}

async fn handle(
    req: Request<IncomingBody>,
    token: Arc<String>,
) -> std::result::Result<Response<Full<Bytes>>, hyper::Error> {
    if req.method() == Method::POST && req.uri().path() == "/sign" {
        match req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok())
        {
            Some(h) if h == format!("Bearer {}", token.as_str()) => {}
            _ => {
                let mut r = Response::new(Full::new(Bytes::from("unauthorized")));
                *r.status_mut() = StatusCode::UNAUTHORIZED;
                return Ok(r);
            }
        }

        let body = req.collect().await?.to_bytes();
        let data: CertRequest = match serde_json::from_slice(&body) {
            Ok(d) => d,
            Err(_) => {
                let mut resp = Response::new(Full::new(Bytes::from("bad request")));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(resp);
            }
        };
        let days = match data.days {
            Some(d) => d,
            None => {
                let mut resp = Response::new(Full::new(Bytes::from("missing days")));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(resp);
            }
        };
        let args = crate::cmd::sign_cert::SignCertArgs {
            cn: data.cn,
            days,
            san: vec![],
        };
        if let Err(e) = args.run(false) {
            error!("cert signing failed: {}", e);
            let mut resp = Response::new(Full::new(Bytes::from("error")));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(resp);
        }
        Ok(Response::new(Full::new(Bytes::from("ok"))))
    } else {
        let mut resp = Response::new(Full::new(Bytes::from("not found")));
        *resp.status_mut() = StatusCode::NOT_FOUND;
        Ok(resp)
    }
}

fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>> {
    let file = File::open(path).map_err(Error::from)?;
    let mut reader = BufReader::new(file);
    let mut out = Vec::new();
    for c in certs(&mut reader) {
        out.push(c.map_err(|_| Error::Other("bad cert".into()))?);
    }
    Ok(out)
}

fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>> {
    let file = File::open(path).map_err(Error::from)?;
    let mut reader = BufReader::new(file);
    match private_key(&mut reader).map_err(|_| Error::Other("bad key".into()))? {
        Some(k) => Ok(k),
        None => Err(Error::Other("no key found".into())),
    }
}
