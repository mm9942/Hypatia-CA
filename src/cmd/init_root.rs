use crate::error::{Error, Result};
use crate::util::{audit, fs};
use clap::Args;
use crypt_guard::{
    KeyControKyber512, KeyControKyber768, KeyControKyber1024, KyberKeyFunctions, KyberKeypair,
    error::CryptError,
};
use rcgen::{BasicConstraints, CertificateParams, DnType, IsCa, KeyPair};
use tracing::{Level, debug, event, info, trace};
use zeroize::{Zeroize, Zeroizing};

#[derive(Args, Debug)]
pub struct InitRootArgs {
    /// Common-Name for the Root certificate
    #[arg(long, default_value = "Hypatia-Root")]
    pub cn: String,

    /// Not-after (days)
    #[arg(long, default_value = "3650")]
    pub days: u32,

    /// Store key in HSM (slot ID)
    #[arg(long)]
    pub hsm: Option<u32>,

    /// Overwrite existing root
    #[arg(long)]
    pub force: bool,
}

impl crate::cmd::Runnable for InitRootArgs {
    fn run(&self, cli: &crate::Cli) -> Result<()> {
        let mut params = CertificateParams::new(vec![]).map_err(Error::from)?;
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params
            .distinguished_name
            .push(DnType::CommonName, self.cn.to_owned());
        params.not_after = rcgen::date_time_ymd(2035, 1, 1);

        debug!("certificate params ready");

        let key_pair = KeyPair::generate().map_err(Error::from)?;
        let cert = params.self_signed(&key_pair).map_err(Error::from)?;
        let cert_pem = cert.pem();
        let key_pem: Zeroizing<String> = Zeroizing::new(key_pair.serialize_pem());

        let (_pq_pub, mut pq_sec) = KyberKeypair!(512);
        trace!("generated kyber keypair");

        info!("storing root certificate");
        fs::write_root_ca(&cert_pem, &key_pem, self.force)?;
        pq_sec.zeroize();
        audit::emit("init-root", &cert_pem, cli.json)?;
        event!(Level::INFO, "Root CA created");
        Ok(())
    }
}
