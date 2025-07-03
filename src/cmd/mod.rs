pub mod init_root;
pub mod revoke;
pub mod sign;

use crate::error::Result;

pub trait Runnable {
    fn run(&self, cli: &crate::Cli) -> Result<()>;
}
