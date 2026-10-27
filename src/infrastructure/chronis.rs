use super::process::{self, Spec};
use crate::domain::Run;
use anyhow::{Context, Result, ensure};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub fn project_root(state: &Path, repo: &Path) -> Result<Option<PathBuf>> {
    ensure!(
        !repo.join(".chronis").is_symlink(),
        "Chronis store cannot be a symlink"
    );
    let common = super::git::call(
        state,
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    let common = PathBuf::from(common).canonicalize()?;
    if common.file_name().is_some_and(|name| name == ".git") {
        let root = common
            .parent()
            .context("Git common directory has no parent")?;
        if root.join(".chronis").is_dir() {
            ensure!(
                !root.join(".chronis").is_symlink(),
                "Chronis store cannot be a symlink"
            );
            if root != repo && repo.join(".chronis").is_dir() {
                for entry in std::fs::read_dir(repo.join(".chronis"))? {
                    let name = entry?.file_name();
                    ensure!(
                        name == "config.toml" || name == ".gitignore" || name == "sync",
                        "worktree has independent Chronis data; reconcile before enrollment"
                    );
                }
            }
            return Ok(Some(root.to_path_buf()));
        }
    }
    if repo.join(".chronis").is_dir() {
        ensure!(
            !repo.join(".chronis").is_symlink(),
            "Chronis store cannot be a symlink"
        );
        return Ok(Some(repo.to_path_buf()));
    }
    Ok(None)
}

pub fn tracked_private_state(state: &Path, repo: &Path) -> Result<bool> {
    let tracked = super::git::call(
        state,
        repo,
        &["ls-files", "--", ".allstack", ".chronis", ".prime"],
    )?;
    for path in tracked.lines() {
        if path == ".chronis/sync/events.jsonl" || path == ".chronis/.gitignore" {
            continue;
        }
        if path == ".chronis/config.toml" {
            let root = super::bootstrap_fs::Root::new(repo)?;
            let bytes = root.read(path)?.context("tracked Chronis config absent")?;
            ensure!(
                bytes.len() <= 4096,
                "Chronis config exceeds safe metadata bound"
            );
            let config: toml::Value = toml::from_str(std::str::from_utf8(&bytes)?)?;
            if let Some(table) = config.as_table() {
                let safe = table
                    .keys()
                    .all(|key| key == "mode" || key == "instance_id")
                    && table
                        .get("mode")
                        .is_none_or(|mode| mode.as_str() == Some("embedded"))
                    && table.get("instance_id").is_none_or(|id| {
                        id.as_str()
                            .is_some_and(|id| uuid::Uuid::parse_str(id).is_ok())
                    });
                if safe {
                    continue;
                }
            }
        }
        return Ok(true);
    }
    Ok(false)
}

pub struct Tasks<'a> {
    pub state: &'a Path,
    pub repo: &'a Path,
}

impl Tasks<'_> {
    fn call(&self, args: &[&str], actor: Option<&str>) -> Result<String> {
        let project =
            project_root(self.state, self.repo)?.context("Chronis project store absent")?;
        let mut argv = vec!["cn".into()];
        argv.extend(args.iter().map(|s| s.to_string()));
        argv.push("--toon".into());
        let mut environment = BTreeMap::new();
        if let Some(actor) = actor {
            environment.insert("CN_AGENT_ID".into(), actor.into());
        }
        process::run(Spec {
            argv,
            cwd: project,
            seconds: 30,
            environment,
            input: None,
            directory: self.state.join("process"),
        })?
        .checked()
    }
    fn actor(run: &Run) -> String {
        format!("allstack:{}:{}:{}", run.id, run.host.name(), run.epoch)
    }
    pub fn create(&self, run: &Run) -> Result<String> {
        let title = format!("allstack {}", run.id);
        let list = self.call(&["list", "--all"], None)?;
        let found: Vec<&str> = list
            .lines()
            .filter_map(|line| {
                let fields: Vec<&str> = line.split('|').collect();
                (fields.len() >= 3 && fields[2] == title).then(|| fields[0])
            })
            .collect();
        ensure!(
            found.len() <= 1,
            "duplicate Chronis tasks require reconciliation"
        );
        if let Some(id) = found.first() {
            return Ok(id.to_string());
        }
        let result = self.call(
            &[
                "task",
                "create",
                &title,
                "-d",
                &format!(
                    "Allstack run {}; source-linked workflow {}",
                    run.id,
                    run.workflow_id()
                ),
            ],
            None,
        )?;
        result
            .lines()
            .find_map(|s| s.strip_prefix("created:task:"))
            .map(|s| s.split(':').next().unwrap_or(s).to_string())
            .context("uncertain task create; reconcile by exact title before retry")
    }
    pub fn show(&self, id: &str) -> Result<String> {
        self.call(&["show", id], None)
    }
    pub fn claim(&self, run: &Run, previous: Option<&str>) -> Result<()> {
        let id = run.task.as_deref().context("missing Chronis task")?;
        let actual = self.show(id)?;
        let actor = Self::actor(run);
        let owner = actual
            .lines()
            .find_map(|s| s.strip_prefix("claimed_by:"))
            .unwrap_or("");
        if owner == actor && actual.lines().any(|s| s == "status:in-progress") {
            return Ok(());
        }
        if actual.lines().any(|s| s == "status:in-progress") {
            ensure!(
                previous == Some(owner),
                "Chronis task belongs to another active claimant"
            );
            self.call(
                &["release", id, "--reason", "explicit Allstack handoff"],
                Some(owner),
            )?;
        }
        self.call(&["claim", id], Some(&actor))?;
        self.ensure_claim(run)
    }
    pub fn ensure_claim(&self, run: &Run) -> Result<()> {
        let actual = self.show(run.task.as_deref().context("missing task")?)?;
        ensure!(
            actual
                .lines()
                .any(|s| s == format!("claimed_by:{}", Self::actor(run)))
                && actual.lines().any(|s| s == "status:in-progress"),
            "Chronis claim read-back mismatch"
        );
        Ok(())
    }
    pub fn complete(&self, run: &Run) -> Result<()> {
        let id = run.task.as_deref().context("missing task")?;
        if !self.show(id)?.lines().any(|s| s == "status:done") {
            self.ensure_claim(run)?;
            self.call(
                &["done", id, "--reason", "remote commit and PR head verified"],
                Some(&Self::actor(run)),
            )?;
        }
        ensure!(
            self.show(id)?.lines().any(|s| s == "status:done"),
            "task completion unconfirmed"
        );
        Ok(())
    }
    pub fn previous_actor(run: &Run) -> String {
        Self::actor(run)
    }
}
