//! Repository setup runs before mutable workflow stores can be opened.
use super::{bootstrap_fs::Root, digest, private_dir, process};
use crate::domain::Policy;
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};

const RESOURCES: &[(&str, &str)] = &[
    (
        "workflows/delivery.toml",
        include_str!("../../workflows/delivery.toml"),
    ),
    (
        "workflows/methods/plan.md",
        include_str!("../../workflows/methods/plan.md"),
    ),
    (
        "workflows/methods/build.md",
        include_str!("../../workflows/methods/build.md"),
    ),
    (
        "docs/reference/allstack.md",
        include_str!("../../templates/allstack.md"),
    ),
    (
        ".agents/skills/allstack-delivery/SKILL.md",
        include_str!("../../templates/SKILL.md"),
    ),
    (
        ".claude/skills/allstack-delivery/SKILL.md",
        include_str!("../../templates/SKILL.md"),
    ),
];
const RUST_POLICY: &str = include_str!("../../templates/verification-rust.toml");
const IGNORE: &[&str] = &["/.allstack/", "/.chronis/", "/.prime/"];
const INDEX: &str = "- [Allstack CLI and recovery](reference/allstack.md)";

pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new() -> Result<Self> {
        let path = std::env::temp_dir().join(format!("allstack-setup-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&path)?;
        private_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Serialize)]
struct Change {
    path: String,
    status: String,
    digest: String,
    #[serde(skip)]
    before: Option<Vec<u8>>,
    #[serde(skip)]
    bytes: Vec<u8>,
}

fn managed(root: &Root, path: &str, bytes: &[u8], preserve: bool) -> Result<Change> {
    let before = root.read(path)?;
    let (status, bytes) = match &before {
        None => ("created", bytes.to_vec()),
        Some(old) if preserve || old == bytes => ("unchanged", old.clone()),
        Some(_) => ("conflict", bytes.to_vec()),
    };
    Ok(Change {
        path: path.into(),
        status: status.into(),
        digest: digest(&bytes),
        before,
        bytes,
    })
}

fn append(root: &Root, path: &str, entries: &[&str], prefix: &str) -> Result<Change> {
    let before = root.read(path)?;
    let mut bytes = before.clone().unwrap_or_else(|| prefix.as_bytes().to_vec());
    let text = std::str::from_utf8(&bytes).context("index/ignore must be UTF-8")?;
    let missing: Vec<_> = entries
        .iter()
        .filter(|entry| !text.lines().any(|line| line == **entry))
        .collect();
    if !missing.is_empty() {
        if !bytes.is_empty() && !bytes.ends_with(b"\n") {
            bytes.push(b'\n');
        }
        for entry in missing {
            bytes.extend_from_slice(entry.as_bytes());
            bytes.push(b'\n');
        }
    }
    let status = if before.as_ref() == Some(&bytes) {
        "unchanged"
    } else if before.is_some() {
        "appended"
    } else {
        "created"
    };
    Ok(Change {
        path: path.into(),
        status: status.into(),
        digest: digest(&bytes),
        before,
        bytes,
    })
}

fn plan(root: &Root, profile: Option<&str>, policy: &str) -> Result<(Vec<Change>, Vec<String>)> {
    ensure!(
        profile.is_none_or(|p| p == "rust"),
        "unsupported profile; use rust or an existing policy"
    );
    let mut files = RESOURCES
        .iter()
        .map(|(path, text)| managed(root, path, text.as_bytes(), false))
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        !files.iter().any(|f| f.path == policy)
            && ![".gitignore", "docs/README.md", "Cargo.toml", "Cargo.lock"].contains(&policy)
            && !Path::new(policy)
                .components()
                .any(|c| c.as_os_str().to_str().is_some_and(|s| s.starts_with('.'))),
        "policy path overlaps a managed or source input"
    );
    let mut missing = Vec::new();
    if let Some(bytes) = root.read(policy)? {
        let parsed: Policy = toml::from_str(std::str::from_utf8(&bytes)?)?;
        parsed.validate().map_err(anyhow::Error::msg)?;
        files.push(managed(root, policy, &bytes, true)?);
    } else if profile == Some("rust") {
        for path in ["Cargo.toml", "Cargo.lock"] {
            if root.read(path)?.is_none() {
                missing.push(format!("Rust profile requires {path}"));
            }
        }
        files.push(managed(root, policy, RUST_POLICY.as_bytes(), false)?);
    } else {
        missing.push(format!(
            "provide --profile rust or an existing valid policy at {policy}"
        ));
    }
    files.push(append(root, ".gitignore", IGNORE, "")?);
    files.push(append(
        root,
        "docs/README.md",
        &[INDEX],
        "# Documentation\n\n",
    )?);
    for path in [".allstack", ".prime", ".chronis"] {
        root.directory(path)?;
    }
    Ok((files, missing))
}

fn call(scratch: &Scratch, repo: &Path, argv: &[&str]) -> Result<String> {
    process::checked(&scratch.0, repo, argv)
}

fn repair_overridden_ignores(scratch: &Scratch, repo: &Path, files: &mut [Change]) -> Result<()> {
    let outcome = process::run(process::Spec {
        argv: [
            "git",
            "check-ignore",
            "--no-index",
            "--",
            ".allstack/bootstrap-probe",
            ".chronis/bootstrap-probe",
            ".prime/bootstrap-probe",
        ]
        .map(String::from)
        .to_vec(),
        cwd: repo.into(),
        seconds: 30,
        environment: Default::default(),
        input: None,
        directory: scratch.0.join("process"),
    })?;
    ensure!(
        !outcome.timed_out && !outcome.owner_lost && matches!(outcome.code, Some(0 | 1)),
        "cannot inspect private-store ignore state: {}",
        outcome.stderr
    );
    let change = files
        .iter_mut()
        .find(|f| f.path == ".gitignore")
        .context("missing ignore plan")?;
    for (rule, path) in IGNORE.iter().zip([
        ".allstack/bootstrap-probe",
        ".chronis/bootstrap-probe",
        ".prime/bootstrap-probe",
    ]) {
        let pending = &change.bytes[change.before.as_ref().map_or(0, Vec::len)..];
        if !outcome.stdout.lines().any(|line| line == path)
            && !std::str::from_utf8(pending)?
                .lines()
                .any(|line| line == *rule)
        {
            if !change.bytes.ends_with(b"\n") {
                change.bytes.push(b'\n');
            }
            change.bytes.extend_from_slice(rule.as_bytes());
            change.bytes.push(b'\n');
            change.status = if change.before.is_some() {
                "appended"
            } else {
                "created"
            }
            .into();
            change.digest = digest(&change.bytes);
        }
    }
    Ok(())
}

pub fn initialize(repo: &Path, check: bool, profile: Option<&str>, policy: &str) -> Result<Value> {
    let root = Root::new(repo)?;
    let scratch = Scratch::new()?;
    let (mut files, mut missing) = plan(&root, profile, policy)?;
    repair_overridden_ignores(&scratch, repo, &mut files)?;
    if call(&scratch, repo, &["cn", "--version", "--toon"]).is_err() {
        missing.push("Chronis cn executable unavailable".into());
    }
    let tracked = call(
        &scratch,
        repo,
        &["git", "ls-files", "--", ".allstack", ".chronis", ".prime"],
    )?;
    if !tracked.is_empty() {
        missing.push("private store files are tracked; resolve before initialization".into());
    }
    let chronis_exists = root.directory(".chronis")?;
    if !check && chronis_exists && call(&scratch, repo, &["cn", "list", "--all", "--toon"]).is_err()
    {
        missing.push("existing Chronis store unreadable through cn".into());
    }
    let conflicts: Vec<_> = files
        .iter()
        .filter(|f| f.status == "conflict")
        .map(|f| &f.path)
        .collect();
    let ready = missing.is_empty() && conflicts.is_empty();
    let mut report = json!({"template_version":env!("CARGO_PKG_VERSION"), "repository":repo,
        "check":check,"ready":ready,"applied":false,"files":files,"missing":missing,"conflicts":conflicts,
        "chronis":if chronis_exists && check { "existing; engine readback deferred until apply" } else if chronis_exists { "existing" } else { "initialize" }});
    if check {
        return Ok(report);
    }
    ensure!(ready, "bootstrap preflight failed: {report}");
    let mut completed = Vec::new();
    for change in &files {
        let result = match change.status.as_str() {
            "created" => root.create(&change.path, &change.bytes),
            "appended" => root.append(
                &change.path,
                change.before.as_deref().unwrap_or_default(),
                &change.bytes[change.before.as_ref().map_or(0, Vec::len)..],
            ),
            _ => root.read(&change.path).and_then(|actual| {
                ensure!(
                    actual == change.before,
                    "destination changed after preflight: {}; completed: {completed:?}",
                    change.path
                );
                Ok(())
            }),
        };
        result.with_context(|| {
            format!(
                "bootstrap failed at {}; completed: {completed:?}; rerun after inspection",
                change.path
            )
        })?;
        completed.push(change.path.clone());
    }
    if !chronis_exists {
        ensure!(
            !root.directory(".chronis")?,
            "Chronis store appeared after preflight; inspect and rerun; completed: {completed:?}"
        );
        call(&scratch, repo, &["cn", "init", "--toon"]).with_context(|| {
            format!("Chronis initialization uncertain; completed: {completed:?}; inspect and rerun")
        })?;
    }
    ensure!(
        root.directory(".chronis")?,
        "cn did not initialize the target repository; completed: {completed:?}"
    );
    call(&scratch, repo, &["cn", "list", "--all", "--toon"]).with_context(|| {
        format!("Chronis readback failed; completed: {completed:?}; inspect and rerun")
    })?;
    check_ignored(&scratch, repo).with_context(|| {
        format!("private-store ignore readback failed; completed: {completed:?}; inspect and rerun")
    })?;
    for change in &files {
        ensure!(
            root.read(&change.path)
                .with_context(|| format!(
                    "readback failed; completed: {completed:?}; inspect and rerun"
                ))?
                .as_deref()
                == Some(change.bytes.as_slice()),
            "destination readback changed: {}; completed: {completed:?}; inspect and rerun",
            change.path
        );
    }
    report["applied"] = json!(true);
    report["chronis"] = json!(if chronis_exists {
        "existing"
    } else {
        "initialized"
    });
    Ok(report)
}

fn check_ignored(scratch: &Scratch, repo: &Path) -> Result<()> {
    for path in [
        ".allstack/bootstrap-probe",
        ".chronis/bootstrap-probe",
        ".prime/bootstrap-probe",
    ] {
        let actual = call(
            scratch,
            repo,
            &["git", "check-ignore", "--no-index", "--", path],
        )?;
        ensure!(
            actual.trim() == path,
            "private store is not ignored: {path}"
        );
    }
    Ok(())
}

pub fn doctor(repo: &Path, policy: &str) -> Result<Value> {
    let root = Root::new(repo)?;
    let scratch = Scratch::new()?;
    let (files, mut missing) = plan(&root, None, policy)?;
    for f in &files {
        if f.status != "unchanged" {
            missing.push(format!("{}: {}", f.path, f.status));
        }
    }
    let chronis = if root.directory(".chronis")? {
        match call(&scratch, repo, &["cn", "list", "--all", "--toon"]) {
            Ok(_) => "readable",
            Err(_) => {
                missing.push("Chronis store unreadable through cn".into());
                "unreadable"
            }
        }
    } else {
        missing.push("Chronis project store absent".into());
        "absent"
    };
    if check_ignored(&scratch, repo).is_err() {
        missing.push("private stores are not all ignored".into());
    }
    let tracked = call(
        &scratch,
        repo,
        &["git", "ls-files", "--", ".allstack", ".chronis", ".prime"],
    )?;
    if !tracked.is_empty() {
        missing.push("private store files are tracked".into());
    }
    let tools: Vec<_> = ["codex", "claude", "cn", "git", "gh", "cargo", "rustc"].iter().map(|name| {
        let args = if *name == "cn" { vec![*name,"--version","--toon"] } else { vec![*name,"--version"] };
        let result = call(&scratch, repo, &args);
        json!({"tool":name,"available":result.is_ok(),"version":result.ok().map(|s| s.lines().next().unwrap_or("").to_string())})
    }).collect();
    let available = tools.iter().all(|t| t["available"] == true);
    Ok(
        json!({"repository":repo,"store":repo.join(".allstack"),"setup_ready":missing.is_empty(),
        "executables_ready":available,"authentication":"not-probed","missing":missing,"files":files,"chronis":chronis,
        "stores_opened":false,"allsource":"embedded; fsync-per-event when run storage opens",
        "prime":"embedded graph; lexical recall; local-only; not opened by doctor",
        "coordinator":"exclusive-kernel-lock-per-run-command","host_scope":"sequential-same-machine","tools":tools}),
    )
}
