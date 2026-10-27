use super::workspace::contained;
use anyhow::{Result, ensure};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub max_files: usize,
    pub max_total_bytes: u64,
}

pub fn read(root: &Path) -> Result<Limits> {
    let name = "workflows/snapshot-limits.toml";
    let path = contained(root, name)?;
    if !path.exists() {
        return Ok(Limits {
            max_files: 10_000,
            max_total_bytes: 64 * 1024 * 1024,
        });
    }
    ensure!(
        fs::metadata(&path)?.len() <= 4096,
        "snapshot budget exceeds 4096 bytes"
    );
    let limits: Limits = toml::from_str(&fs::read_to_string(path)?)?;
    ensure!(
        (1..=100_000).contains(&limits.max_files)
            && (1..=1024 * 1024 * 1024).contains(&limits.max_total_bytes),
        "snapshot budget must be within 100000 files and 1 GiB"
    );
    Ok(limits)
}
