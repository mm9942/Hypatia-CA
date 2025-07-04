use clap::{Parser, Subcommand};
use tracing::{Level, event, info};
use tracing_subscriber::{EnvFilter, fmt};

mod cmd;
mod error;
mod util;

use crate::error::Result;
use cmd::Runnable;

#[derive(Parser)]
#[command(name = "hypatia-ca")]
#[command(author, version, about)]
pub struct Cli {
    /// Output logs in JSON
    #[arg(long)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate offline root CA
    InitRoot(cmd::init_root::InitRootArgs),
    /// Sign or verify messages
    Signature(cmd::signature::SignatureArgs),
    /// Sign a certificate with the root CA
    SignCert(cmd::sign_cert::SignCertArgs),
    /// Serve an HTTP API for certificate requests
    Serve(cmd::serve::ServeArgs),
    /// Revoke a certificate
    Revoke(cmd::revoke::RevokeArgs),
}

fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .init();
    info!("hypatia-ca started");
    let cli = Cli::parse();
    let json = cli.json;
    event!(Level::DEBUG, command = ?cli.command, "dispatching command");
    match cli.command {
        Commands::InitRoot(args) => args.run(json)?,
        Commands::Signature(args) => args.run(json)?,
        Commands::SignCert(args) => args.run(json)?,
        Commands::Serve(args) => args.run(json)?,
        Commands::Revoke(args) => args.run(json)?,
    }
    Ok(())
}
