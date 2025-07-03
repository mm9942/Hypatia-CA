use crate::error::{Error, Result};
use std::fs;
use std::path::Path;
use tracing::{debug, error};

const ROOT_DIR: &str = "/opt/hypatia-ca/data/root";
const SIG_DIR: &str = "/opt/hypatia-ca/data/signatures";
const CRL_FILE: &str = "/opt/hypatia-ca/data/revoked.txt";
const CERT_DIR: &str = "/opt/hypatia-ca/data/certs";

pub fn ensure_dirs() -> Result<()> {
    fs::create_dir_all(ROOT_DIR).map_err(Error::from)?;
    fs::create_dir_all(SIG_DIR).map_err(Error::from)?;
    fs::create_dir_all(CERT_DIR).map_err(Error::from)?;
    Ok(())
}

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

pub fn read_root_ca() -> Result<(String, String)> {
    let cert_path = Path::new(ROOT_DIR).join("cert.pem");
    let key_path = Path::new(ROOT_DIR).join("key.pem");
    debug!("loading root certificate from {:?}", cert_path);
    let cert = fs::read_to_string(cert_path).map_err(Error::from)?;
    let key = fs::read_to_string(key_path).map_err(Error::from)?;
    Ok((cert, key))
}

pub fn write_cert(name: &str, cert_pem: &str, key_pem: &str) -> Result<()> {
    fs::create_dir_all(CERT_DIR).map_err(Error::from)?;
    let cert_path = Path::new(CERT_DIR).join(format!("{name}.pem"));
    let key_path = Path::new(CERT_DIR).join(format!("{name}.key"));
    debug!("writing certificate to {:?}", cert_path);
    fs::write(cert_path, cert_pem).map_err(Error::from)?;
    fs::write(key_path, key_pem).map_err(Error::from)
}

pub fn write_signature(sig: &[u8], pk: &[u8]) -> Result<()> {
    fs::create_dir_all(SIG_DIR).map_err(Error::from)?;
    let sig_path = Path::new(SIG_DIR).join("sig.bin");
    let pk_path = Path::new(SIG_DIR).join("pk.bin");
    fs::write(sig_path, sig).map_err(Error::from)?;
    fs::write(pk_path, pk).map_err(Error::from)
}

pub fn read_signature() -> Result<(Vec<u8>, Vec<u8>)> {
    let sig_path = Path::new(SIG_DIR).join("sig.bin");
    let pk_path = Path::new(SIG_DIR).join("pk.bin");
    let sig = fs::read(sig_path).map_err(Error::from)?;
    let pk = fs::read(pk_path).map_err(Error::from)?;
    Ok((sig, pk))
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
