pub mod bootstrap;
pub mod bootstrap_fs;
pub mod build_lock;
pub mod catalog;
pub mod checks;
pub mod chronis;
pub mod forge;
pub mod gates;
pub mod git;
pub mod hosts;
pub mod knowledge;
pub mod memory;
pub mod process;
pub mod runtime;
pub mod skills;
pub mod store;
pub mod workspace;

use anyhow::{Context, Result, ensure};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn private_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    ensure!(
        !fs::symlink_metadata(path)?.file_type().is_symlink(),
        "private directory is a symlink"
    );
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().context("file has no parent")?;
    private_dir(parent)?;
    let tmp = parent.join(format!(".{}", uuid::Uuid::new_v4()));
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp)?;
    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&tmp, path)?;
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

#[derive(Clone)]
pub struct Artifacts(pub PathBuf);

impl Artifacts {
    pub fn put(&self, bytes: &[u8]) -> Result<String> {
        let key = digest(bytes);
        let target = self.0.join(&key);
        if target.exists() {
            ensure!(self.get(&key)? == bytes, "artifact collision");
        } else {
            atomic_write(&target, bytes)?;
        }
        Ok(key)
    }
    pub fn get(&self, key: &str) -> Result<Vec<u8>> {
        ensure!(
            key.len() == 64 && key.bytes().all(|c| c.is_ascii_hexdigit()),
            "invalid artifact digest"
        );
        let path = self.0.join(key);
        ensure!(
            !fs::symlink_metadata(&path)
                .context("missing artifact; recovery blocked")?
                .file_type()
                .is_symlink(),
            "artifact symlink refused"
        );
        let bytes = fs::read(path).context("missing artifact; recovery blocked")?;
        ensure!(digest(&bytes) == key, "artifact digest mismatch");
        Ok(bytes)
    }
}
