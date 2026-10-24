use super::process;
use anyhow::{Result, ensure};
use std::path::{Path, PathBuf};

pub fn call(root: &Path, cwd: &Path, args: &[&str]) -> Result<String> {
    let mut argv = vec!["git"];
    argv.extend(args);
    Ok(process::checked(root, cwd, &argv)?
        .trim_end_matches('\n')
        .to_string())
}

pub fn root(path: &Path) -> Result<PathBuf> {
    let mut path = path.canonicalize()?;
    loop {
        if path.join(".git").exists() {
            return Ok(path);
        }
        ensure!(path.pop(), "not a git repository");
    }
}

pub fn clean(state: &Path, cwd: &Path) -> Result<()> {
    ensure!(
        call(
            state,
            cwd,
            &["status", "--porcelain", "--untracked-files=all"]
        )?
        .is_empty(),
        "working tree must be clean"
    );
    Ok(())
}

pub fn trunk(state: &Path, cwd: &Path) -> Result<String> {
    let raw = call(state, cwd, &["ls-remote", "--symref", "origin", "HEAD"])?;
    let branch = raw.lines().find_map(|line| {
        line.strip_prefix("ref: refs/heads/")
            .and_then(|s| s.strip_suffix("\tHEAD"))
    });
    Ok(branch
        .ok_or_else(|| anyhow::anyhow!("remote default branch could not be established"))?
        .to_string())
}

pub fn safe_remote(remote: &str) -> Result<()> {
    ensure!(
        !remote.contains('\n')
            && !remote.contains('\r')
            && !remote.contains('?')
            && !remote.contains('#'),
        "unsafe remote URL"
    );
    ensure!(
        remote.starts_with("https://github.com/") || remote.starts_with("git@github.com:"),
        "first release supports credential-free github.com remotes"
    );
    Ok(())
}
