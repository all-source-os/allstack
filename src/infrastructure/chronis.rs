use super::process::{self, Spec};
use crate::domain::Run;
use anyhow::{Context, Result, ensure};
use std::{collections::BTreeMap, path::Path};

pub struct Tasks<'a> {
    pub state: &'a Path,
    pub repo: &'a Path,
}

impl Tasks<'_> {
    fn call(&self, args: &[&str], actor: Option<&str>) -> Result<String> {
        let mut argv = vec!["cn".into()];
        argv.extend(args.iter().map(|s| s.to_string()));
        argv.push("--toon".into());
        let mut environment = BTreeMap::new();
        if let Some(actor) = actor {
            environment.insert("CN_AGENT_ID".into(), actor.into());
        }
        process::run(Spec {
            argv,
            cwd: self.repo.into(),
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
