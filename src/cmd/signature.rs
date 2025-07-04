use crate::cmd::Runnable;
use crate::error::{Error, Result};
use crate::util::audit;
use clap::{ArgGroup, Args};
use crypt_guard::KDF::{
    Detached, Dilithium2, Dilithium3, Dilithium5, Falcon512, Falcon1024, KeyOperations, Signature,
};
use crypt_guard::error::CryptError;
use crypt_guard::*;
use std::fs;
use tracing::{error, event, info};
use zeroize::Zeroize;

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("mode").required(true).args(["sign", "verify"])))]
pub struct SignatureArgs {
    /// File to sign or verify
    #[arg(long)]
    pub file: String,

    /// Algorithm: falcon512, falcon1024, dilithium2, dilithium3, dilithium5
    #[arg(long, default_value = "falcon512")]
    pub algorithm: String,

    /// Sign the message
    #[arg(long)]
    pub sign: bool,

    /// Verify the message
    #[arg(long)]
    pub verify: bool,
}

impl Runnable for SignatureArgs {
    fn run(self, json: bool) -> Result<()> {
        let data = fs::read(&self.file).map_err(Error::from)?;
        if self.sign {
            match self.algorithm.as_str() {
                "falcon512" => {
                    let (pk, mut sk) = FalconKeypair!(512);
                    let sig = Signature!(Falcon, sk.to_owned(), 512, data.to_owned(), Detached);
                    sk.zeroize();
                    fs::write(format!("{}.pk", self.file), pk).map_err(Error::from)?;
                    fs::write(format!("{}.sig", self.file), sig).map_err(Error::from)?;
                }
                "falcon1024" => {
                    let (pk, mut sk) = FalconKeypair!(1024);
                    let sig = Signature!(Falcon, sk.to_owned(), 1024, data.to_owned(), Detached);
                    sk.zeroize();
                    fs::write(format!("{}.pk", self.file), pk).map_err(Error::from)?;
                    fs::write(format!("{}.sig", self.file), sig).map_err(Error::from)?;
                }
                "dilithium2" => {
                    let (pk, mut sk) = DilithiumKeypair!(2);
                    let sig = Signature!(Dilithium, sk.to_owned(), 2, data.to_owned(), Detached);
                    sk.zeroize();
                    fs::write(format!("{}.pk", self.file), pk).map_err(Error::from)?;
                    fs::write(format!("{}.sig", self.file), sig).map_err(Error::from)?;
                }
                "dilithium3" => {
                    let (pk, mut sk) = DilithiumKeypair!(3);
                    let sig = Signature!(Dilithium, sk.to_owned(), 3, data.to_owned(), Detached);
                    sk.zeroize();
                    fs::write(format!("{}.pk", self.file), pk).map_err(Error::from)?;
                    fs::write(format!("{}.sig", self.file), sig).map_err(Error::from)?;
                }
                "dilithium5" => {
                    let (pk, mut sk) = DilithiumKeypair!(5);
                    let sig = Signature!(Dilithium, sk.to_owned(), 5, data.to_owned(), Detached);
                    sk.zeroize();
                    fs::write(format!("{}.pk", self.file), pk).map_err(Error::from)?;
                    fs::write(format!("{}.sig", self.file), sig).map_err(Error::from)?;
                }
                _ => return Err(Error::Other("unknown algorithm".into())),
            }
            info!("signature stored");
            audit::emit("signature-sign", &self.algorithm, json)?;
            event!(tracing::Level::INFO, "file signed");
        } else if self.verify {
            let sig = fs::read(format!("{}.sig", self.file)).map_err(Error::from)?;
            let pk = fs::read(format!("{}.pk", self.file)).map_err(Error::from)?;
            let res = match self.algorithm.as_str() {
                "falcon512" => Verify!(
                    Falcon,
                    pk.to_owned(),
                    512,
                    sig.to_owned(),
                    data.to_owned(),
                    Detached
                ),
                "falcon1024" => Verify!(
                    Falcon,
                    pk.to_owned(),
                    1024,
                    sig.to_owned(),
                    data.to_owned(),
                    Detached
                ),
                "dilithium2" => Verify!(
                    Dilithium,
                    pk.to_owned(),
                    2,
                    sig.to_owned(),
                    data.to_owned(),
                    Detached
                ),
                "dilithium3" => Verify!(
                    Dilithium,
                    pk.to_owned(),
                    3,
                    sig.to_owned(),
                    data.to_owned(),
                    Detached
                ),
                "dilithium5" => Verify!(
                    Dilithium,
                    pk.to_owned(),
                    5,
                    sig.to_owned(),
                    data.to_owned(),
                    Detached
                ),
                _ => return Err(Error::Other("unknown algorithm".into())),
            };
            if res {
                info!("signature verified");
            } else {
                error!("signature verification failed");
                return Err(Error::Other("verification failed".into()));
            }
            audit::emit("signature-verify", &self.algorithm, json)?;
            event!(tracing::Level::INFO, "verification complete");
        }
        Ok(())
    }
}
