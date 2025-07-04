use crate::cmd::Runnable;
use crate::error::{Error, Result};
use crate::util::{audit, fs};
use clap::Args;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, body::to_bytes};
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::{error, info};

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Address to bind, e.g. 127.0.0.1:8080
    #[arg(long, default_value = "127.0.0.1:8080")]
    pub addr: String,
}

#[derive(Deserialize)]
struct CertRequest {
    cn: String,
    days: Option<u32>,
}

impl Runnable for ServeArgs {
    fn run(&self, cli: &crate::Cli) -> Result<()> {
        let addr: SocketAddr = self
            .addr
            .parse::<SocketAddr>()
            .map_err(|e| Error::Other(e.to_string()))?;
        info!("starting API on {}", addr);
        let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle)) });
        let server = Server::bind(&addr).serve(make_svc);
        fs::ensure_dirs()?;
        let rt = tokio::runtime::Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        rt.block_on(async {
            if let Err(e) = server.await {
                error!("server error: {}", e);
            }
        });
        audit::emit("serve", &self.addr, cli.json)?;
        Ok(())
    }
}

async fn handle(req: Request<Body>) -> std::result::Result<Response<Body>, hyper::Error> {
    if req.method() == Method::POST && req.uri().path() == "/sign" {
        let body = to_bytes(req.into_body()).await?;
        let data: CertRequest = match serde_json::from_slice(&body) {
            Ok(d) => d,
            Err(_) => {
                return Ok(Response::builder()
                    .status(400)
                    .body(Body::from("bad request"))
                    .unwrap());
            }
        };
        let args = crate::cmd::sign_cert::SignCertArgs {
            cn: data.cn,
            days: data.days.unwrap_or(365),
        };
        if let Err(e) = args.run(&crate::Cli {
            json: false,
            command: crate::Commands::SignCert(args.clone()),
        }) {
            error!("cert signing failed: {}", e);
            return Ok(Response::builder()
                .status(500)
                .body(Body::from("error"))
                .unwrap());
        }
        Ok(Response::new(Body::from("ok")))
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("not found"))
            .unwrap())
    }
}
