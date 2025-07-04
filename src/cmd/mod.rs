pub mod init_root;
pub mod revoke;
pub mod serve;
pub mod sign_cert;
pub mod signature;

use crate::error::Result;

pub trait Runnable {
    fn run(self, json: bool) -> Result<()>;
}
