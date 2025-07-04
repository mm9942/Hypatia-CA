use crate::error::{Error, Result};
use crate::util::{audit, fs};
use clap::Args;
use rcgen::{CertificateParams, DnType, IsCa, Issuer, KeyPair};
use time::{Duration, OffsetDateTime};
use tracing::{Level, debug, event, info};
use zeroize::Zeroizing;

#[derive(Args, Debug)]
pub struct SignCertArgs {
    /// Common-Name for the new certificate
    #[arg(long)]
    pub cn: String,

    /// Validity period in days
    #[arg(long, default_value = "365")]
    pub days: u32,

    /// Subject Alternative Names
    #[arg(long)]
    pub san: Vec<String>,
}

impl crate::cmd::Runnable for SignCertArgs {
    fn run(self, json: bool) -> Result<()> {
        let (ca_cert, ca_key) = fs::read_root_ca()?;
        let ca_key = KeyPair::from_pem(&ca_key).map_err(Error::from)?;
        let ca = Issuer::from_ca_cert_pem(&ca_cert, ca_key).map_err(Error::from)?;

        let mut params = CertificateParams::new(self.san).map_err(Error::from)?;

        params.is_ca = IsCa::ExplicitNoCa;
        params
            .distinguished_name
            .push(DnType::CommonName, self.cn.to_owned());
        let now = OffsetDateTime::now_utc();
        params.not_after = now + Duration::days(self.days.into());

        debug!("signing certificate");
        let key = KeyPair::generate().map_err(Error::from)?;
        let cert = params.signed_by(&key, &ca).map_err(Error::from)?;
        let cert_pem = cert.pem();
        let key_pem: Zeroizing<String> = Zeroizing::new(key.serialize_pem());

        fs::write_cert(&self.cn, &cert_pem, &key_pem)?;
        audit::emit("sign-cert", &self.cn, json)?;
      
        event!(Level::INFO, cn = %self.cn, "certificate signed");
        info!("certificate created for {}", self.cn);
        Ok(())
    }
}
