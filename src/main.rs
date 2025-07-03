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
    /// Sign data using post-quantum algorithms
    Sign(cmd::sign::SignArgs),
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
    event!(Level::DEBUG, command = ?cli.command, "dispatching command");
    match &cli.command {
        Commands::InitRoot(args) => args.run(&cli)?,
        Commands::Sign(args) => args.run(&cli)?,
        Commands::Revoke(args) => args.run(&cli)?,
    }
    Ok(())
}
