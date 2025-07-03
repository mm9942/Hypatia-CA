use crate::error::{Error, Result};
use std::fs;
use std::path::Path;
use tracing::{debug, error};

const ROOT_DIR: &str = "/opt/hypatia-ca/data/root";
const SIG_DIR: &str = "/opt/hypatia-ca/data/signatures";
const CRL_FILE: &str = "/opt/hypatia-ca/data/revoked.txt";

pub fn write_root_ca(cert_pem: &str, key_pem: &str, force: bool) -> Result<()> {
    fs::create_dir_all(ROOT_DIR).map_err(Error::from)?;
    let cert_path = Path::new(ROOT_DIR).join("cert.pem");
    let key_path = Path::new(ROOT_DIR).join("key.pem");

    if !force {
        if cert_path.exists() || key_path.exists() {
            error!("root CA exists and --force not set");
            return Err(Error::Other(
                "root CA already exists; use --force to overwrite".into(),
            ));
        }
    }

    debug!("writing certificate to {:?}", cert_path);
    fs::write(cert_path, cert_pem).map_err(Error::from)?;
    fs::write(key_path, key_pem).map_err(Error::from)?;
    Ok(())
}

pub fn write_signature(sig: &[u8]) -> Result<()> {
    fs::create_dir_all(SIG_DIR).map_err(Error::from)?;
    let path = Path::new(SIG_DIR).join("sig.bin");
    fs::write(path, sig).map_err(Error::from)
}

pub fn append_revocation(serial: &str) -> Result<()> {
    fs::create_dir_all(Path::new(CRL_FILE).parent().unwrap()).map_err(Error::from)?;
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(CRL_FILE)
        .map_err(Error::from)?;
    writeln!(file, "{}", serial).map_err(Error::from)
}
