use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use anyhow::Result;
use which::which;

pub fn is_executable(p: &PathBuf) -> bool {
    // Mapping Result<Metadata, io::Error> into Result<bool, io::Error>
    fs::metadata(p)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn is_external_cmd(cmd: &str) -> Result<bool> {
    if cmd.contains("/") {
        let p = Path::new(cmd);
        if p.is_file() && is_executable(&p.to_path_buf()) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    } else {
        if let Ok(_) = which(cmd) {
            return Ok(true);
        }
    }
    return Ok(false);
}
