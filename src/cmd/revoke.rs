use crate::error::Result;
use crate::util::{audit, fs};
use clap::Args;
use tracing::{Level, event, info};

#[derive(Args, Debug)]
pub struct RevokeArgs {
    /// Serial number of certificate to revoke
    #[arg(long)]
    pub serial: String,
}

impl crate::cmd::Runnable for RevokeArgs {
    fn run(self, json: bool) -> Result<()> {
        fs::append_revocation(&self.serial)?;
        info!(serial = %self.serial, "certificate revoked");
        audit::emit("revoke", &self.serial, json)?;
        event!(Level::INFO, "revocation written");
        Ok(())
    }
}
