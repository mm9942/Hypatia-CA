use crate::error::{Error, Result};
use crate::util::{audit, fs};
use clap::Args;
use crypt_guard::*;
use crypt_guard::KDF::{
    Dilithium2, Dilithium3, Dilithium5, Falcon1024, Falcon512, Detached, Signature,
};
use crypt_guard::KDF::KeyOperations;
use crypt_guard::error::CryptError;
use tracing::{debug, event, info, Level};
use zeroize::Zeroize;

#[derive(Args, Debug)]
pub struct SignArgs {
    /// Message to sign
    #[arg(long)]
    pub message: String,

    /// Algorithm: falcon512, falcon1024, dilithium2, dilithium3, dilithium5
    #[arg(long, default_value = "falcon512")]
    pub algorithm: String,
}

impl crate::cmd::Runnable for SignArgs {
    fn run(&self, cli: &crate::Cli) -> Result<()> {
        let data = self.message.as_bytes().to_vec();
        let sig = match self.algorithm.as_str() {
            "falcon512" => {
                let (_pk, sk) = FalconKeypair!(512);
                Signature!(Falcon, sk.to_owned(), 512, data.to_owned(), Detached)
            }
            "falcon1024" => {
                let (_pk, sk) = FalconKeypair!(1024);
                Signature!(Falcon, sk.to_owned(), 1024, data.to_owned(), Detached)
            }
            "dilithium2" => {
                let (_pk, sk) = DilithiumKeypair!(2);
                Signature!(Dilithium, sk.to_owned(), 2, data.to_owned(), Detached)
            }
            "dilithium3" => {
                let (_pk, sk) = DilithiumKeypair!(3);
                Signature!(Dilithium, sk.to_owned(), 3, data.to_owned(), Detached)
            }
            "dilithium5" => {
                let (_pk, sk) = DilithiumKeypair!(5);
                Signature!(Dilithium, sk.to_owned(), 5, data.to_owned(), Detached)
            }
            _ => return Err(Error::Other("unknown algorithm".into())),
        };
        debug!("signature generated");
        fs::write_signature(&sig)?;
        info!("signature stored");
        audit::emit("sign", &self.algorithm, cli.json)?;
        event!(Level::INFO, "message signed");
        Ok(())
    }
}
