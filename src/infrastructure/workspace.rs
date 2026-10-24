use super::{Artifacts, digest, git, private_dir};
use crate::domain::{FileEntry, Snapshot};
use anyhow::{Context, Result, ensure};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Component, Path, PathBuf},
};

pub fn contained(root: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    ensure!(
        !path.as_os_str().is_empty()
            && path.components().all(|c| matches!(c, Component::Normal(_))),
        "unsafe relative path"
    );
    ensure!(
        !path
            .components()
            .any(|c| c.as_os_str() == ".git" || c.as_os_str() == ".allstack"),
        "reserved workspace path"
    );
    let mut current = root.to_path_buf();
    for part in path.components() {
        current.push(part);
        if let Ok(meta) = fs::symlink_metadata(&current) {
            ensure!(
                !meta.file_type().is_symlink(),
                "symlink inputs are unsupported: {relative}"
            );
        }
    }
    Ok(current)
}

fn manifest(state: &Path, cwd: &Path) -> Result<BTreeMap<String, (Vec<u8>, bool)>> {
    let tracked = git::call(state, cwd, &["ls-files", "-z"])?;
    let others = git::call(
        state,
        cwd,
        &["ls-files", "--others", "--exclude-standard", "-z"],
    )?;
    ensure!(
        others.is_empty(),
        "untracked files require explicit git add before checkpoint; never snapshot them implicitly"
    );
    let mut result = BTreeMap::new();
    let mut total = 0;
    for name in tracked.split('\0').filter(|s| !s.is_empty()) {
        let lower = name.to_lowercase();
        ensure!(
            !lower.split('/').any(|p| p == ".env"
                || p.starts_with(".env.")
                || p == "credentials.json"
                || p == "id_rsa"
                || p.ends_with(".pem")),
            "sensitive path requires removal from tracked snapshot: {name}"
        );
        let path = contained(cwd, name)?;
        if !path.exists() {
            continue;
        }
        let meta = fs::metadata(&path)?;
        ensure!(
            meta.is_file() && meta.len() <= 4 * 1024 * 1024,
            "unsupported file or file exceeds 4 MiB: {name}"
        );
        total += meta.len();
        ensure!(
            total <= 64 * 1024 * 1024 && result.len() < 10000,
            "snapshot exceeds supported bounds"
        );
        result.insert(
            name.to_string(),
            (fs::read(path)?, meta.permissions().mode() & 0o111 != 0),
        );
    }
    Ok(result)
}

pub fn capture(state: &Path, cwd: &Path, artifacts: &Artifacts) -> Result<String> {
    let base = git::call(state, cwd, &["rev-parse", "HEAD"])?;
    let before = manifest(state, cwd)?;
    let files = before
        .iter()
        .map(|(name, (bytes, executable))| {
            Ok((
                name.clone(),
                FileEntry {
                    digest: artifacts.put(bytes)?,
                    executable: *executable,
                },
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    ensure!(
        manifest(state, cwd)? == before && git::call(state, cwd, &["rev-parse", "HEAD"])? == base,
        "workspace changed during checkpoint"
    );
    artifacts.put(&serde_json::to_vec(&Snapshot { base, files })?)
}

pub fn read_snapshot(artifacts: &Artifacts, key: &str) -> Result<Snapshot> {
    Ok(serde_json::from_slice(&artifacts.get(key)?)?)
}

pub fn materialize(state: &Path, cwd: &Path, artifacts: &Artifacts, key: &str) -> Result<()> {
    let snapshot = read_snapshot(artifacts, key)?;
    let mut bytes = BTreeMap::new();
    for (name, entry) in &snapshot.files {
        contained(cwd, name)?;
        bytes.insert(name, artifacts.get(&entry.digest)?);
    }
    let tracked = git::call(state, cwd, &["ls-files", "-z"])?;
    for name in tracked.split('\0').filter(|s| !s.is_empty()) {
        if !snapshot.files.contains_key(name) {
            let target = contained(cwd, name)?;
            if target.exists() {
                fs::remove_file(target)?;
            }
        }
    }
    for (name, data) in bytes {
        let target = contained(cwd, name)?;
        fs::create_dir_all(target.parent().context("missing file parent")?)?;
        fs::write(&target, data)?;
        fs::set_permissions(
            &target,
            fs::Permissions::from_mode(if snapshot.files[name].executable {
                0o755
            } else {
                0o644
            }),
        )?;
    }
    // Restored files must enter the index so later snapshots include new source.
    let names: Vec<&str> = snapshot.files.keys().map(String::as_str).collect();
    if !names.is_empty() {
        let mut args = vec!["add", "--"];
        args.extend(names);
        git::call(state, cwd, &args)?;
    }
    let check = capture(state, cwd, artifacts)?;
    let actual = read_snapshot(artifacts, &check)?;
    ensure!(actual.files == snapshot.files, "restored snapshot differs");
    Ok(())
}

pub fn restore(
    state: &Path,
    repo: &Path,
    artifacts: &Artifacts,
    key: &str,
    run: &str,
    epoch: u64,
) -> Result<PathBuf> {
    let snapshot = read_snapshot(artifacts, key)?;
    let path = state.join("workspaces").join(format!("{run}-{epoch}"));
    private_dir(path.parent().context("workspace parent missing")?)?;
    if path.exists() {
        let actual = read_snapshot(artifacts, &capture(state, &path, artifacts)?)?;
        ensure!(
            actual.files == snapshot.files && actual.base == snapshot.base,
            "interrupted handoff workspace differs; inspect before recovery"
        );
        return Ok(path);
    }
    git::call(
        state,
        repo,
        &[
            "worktree",
            "add",
            "--detach",
            path.to_str().context("non-UTF8 workspace")?,
            &snapshot.base,
        ],
    )?;
    materialize(state, &path, artifacts, key)?;
    Ok(path)
}

pub fn content_digest(artifacts: &Artifacts, key: &str) -> Result<String> {
    let snapshot = read_snapshot(artifacts, key)?;
    Ok(digest(&serde_json::to_vec(&snapshot.files)?))
}

pub fn write_plan(state: &Path, cwd: &Path, run: &str, text: &str) -> Result<String> {
    ensure!(
        cwd.join("docs/README.md").is_file(),
        "target needs indexed docs/README.md before planning"
    );
    let name = format!("docs/plans/allstack-{run}.md");
    let target = contained(cwd, &name)?;
    fs::create_dir_all(target.parent().context("missing plan parent")?)?;
    let document = text
        .lines()
        .map(|line| if line.trim().is_empty() { "" } else { line })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    fs::write(target, document)?;
    let index = contained(cwd, "docs/README.md")?;
    let mut body = fs::read_to_string(&index)?;
    body.push_str(&format!(
        "\n- [Allstack delivery plan {run}](plans/allstack-{run}.md)\n"
    ));
    fs::write(index, body)?;
    git::call(state, cwd, &["add", "--", &name, "docs/README.md"])?;
    Ok(name)
}
