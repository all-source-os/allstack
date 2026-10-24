use super::process;
use anyhow::{Context, Result, ensure};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{File, OpenOptions},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

pub fn acquire(state: &Path, repo: &Path) -> Result<File> {
    let path =
        std::env::temp_dir().join(format!("allstack-build-{}.lock", unsafe { libc::getuid() }));
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .mode(0o600)
        .open(path)?;
    lock.try_lock()
        .context("another Allstack build owns the machine build slot")?;
    let table = process::checked(state, repo, &["ps", "-axo", "pid=,ppid=,comm="])?;
    let mut rows = BTreeMap::new();
    for line in table.lines() {
        let mut fields = line.split_whitespace();
        if let (Some(pid), Some(ppid), Some(command)) =
            (fields.next(), fields.next(), fields.next())
        {
            rows.insert(
                pid.parse::<u32>()?,
                (ppid.parse::<u32>()?, command.to_string()),
            );
        }
    }
    let mut ancestors = BTreeSet::new();
    let mut current = std::process::id();
    while ancestors.insert(current) {
        match rows.get(&current) {
            Some((parent, _)) if *parent != 0 => current = *parent,
            _ => break,
        }
    }
    for (pid, (_, command)) in rows {
        let name = Path::new(&command)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        ensure!(
            ancestors.contains(&pid)
                || !["cargo", "rustc", "clang", "cc", "gcc", "ld"].contains(&name),
            "external build active ({name}, pid {pid}); wait rather than competing or killing it"
        );
    }
    Ok(lock)
}
