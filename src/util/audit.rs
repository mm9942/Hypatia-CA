use crate::error::{Error, Result};
use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use tracing::{Level, debug, event};

const LOG_FILE: &str = "/opt/hypatia-ca/audit.log";

pub fn emit(action: &str, details: &str, json: bool) -> Result<()> {
    fs::create_dir_all("/opt/hypatia-ca").map_err(Error::from)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .map_err(Error::from)?;

    if json {
        let entry = serde_json::json!({
            "ts": Utc::now().to_rfc3339(),
            "action": action,
            "details": details,
        });
        writeln!(file, "{}", entry.to_string()).map_err(Error::from)?;
    } else {
        writeln!(file, "{}: {}", action, details).map_err(Error::from)?;
    }
    debug!(%action, "audit log entry written");
    event!(Level::TRACE, "audit committed");
    Ok(())
}
