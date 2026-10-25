use super::{
    catalog,
    chronis::Tasks,
    digest, forge, git, hosts,
    memory::Memory,
    process::{self, Spec},
    store::Store,
    workspace,
};
use crate::{
    application::ports::{Platform, Start},
    domain::{Policy, Publication, Run, Stage, Verification, WorkflowSelection},
};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub struct Runtime {
    pub repo: PathBuf,
    pub state: PathBuf,
    pub store: Store,
    pub memory: Option<Memory>,
}

impl Runtime {
    pub async fn open(repo: &Path) -> Result<Self> {
        let repo = repo.canonicalize()?;
        let state = repo.join(".allstack");
        let store = Store::open(&state).await?;
        let memory = Memory::open(&state.join("prime")).await.ok();
        Ok(Self {
            repo,
            state,
            store,
            memory,
        })
    }
    fn tasks(&self) -> Tasks<'_> {
        Tasks {
            state: &self.state,
            repo: &self.repo,
        }
    }
    pub async fn shutdown(self) -> Result<()> {
        if let Some(memory) = self.memory {
            memory.shutdown().await?;
        }
        self.store.shutdown().await
    }
    fn policy(&self, run: &Run) -> Result<Policy> {
        Ok(toml::from_str(&String::from_utf8(
            self.object(&run.policy)?,
        )?)?)
    }
    fn snapshot(&self, path: &Path) -> Result<String> {
        workspace::capture(&self.state, path, &self.store.artifacts)
    }
    fn environment(&self, policy: &Policy) -> Result<String> {
        let mut data = BTreeMap::new();
        data.insert("os".into(), std::env::consts::OS.into());
        data.insert("arch".into(), std::env::consts::ARCH.into());
        data.insert(
            "allstack_binary".into(),
            digest(&fs::read(std::env::current_exe()?)?),
        );
        for name in &policy.environment {
            data.insert(name.clone(), std::env::var(name).unwrap_or_default());
        }
        for executable in ["git", "rustc", "cargo", "cn"] {
            let output = process::checked(&self.state, &self.repo, &[executable, "--version"])?;
            data.insert(executable.into(), output);
        }
        Ok(digest(&serde_json::to_vec(&data)?))
    }
}

impl Platform for Runtime {
    async fn load(&self, id: &str) -> Result<Run> {
        let run = self.store.load(id).await?;
        ensure!(
            Path::new(&run.repository) == self.repo,
            "run repository does not match coordinator scope"
        );
        let expected = self
            .state
            .join("workspaces")
            .join(format!("{}-{}", run.id, run.epoch));
        ensure!(
            Path::new(&run.workspace) == expected,
            "run workspace does not match active epoch scope"
        );
        Ok(run)
    }
    async fn save(&self, run: &mut Run, kind: &str) -> Result<()> {
        self.store.save(run, kind).await
    }
    fn object(&self, key: &str) -> Result<Vec<u8>> {
        self.store.artifacts.get(key)
    }
    fn put(&self, bytes: &[u8]) -> Result<String> {
        self.store.artifacts.put(bytes)
    }
    fn initialize(&self, input: &Start) -> Result<Run> {
        if input.native {
            ensure!(
                input.allow_commit,
                "native adoption requires commit authority"
            );
        } else {
            git::clean(&self.state, &self.repo)?;
        }
        ensure!(
            self.repo.join(".chronis").is_dir(),
            "initialize project Chronis with cn init --toon first"
        );
        let branch = git::call(&self.state, &self.repo, &["branch", "--show-current"])?;
        let trunk = git::trunk(&self.state, &self.repo)?;
        ensure!(
            !branch.is_empty() && branch != trunk,
            "start on an existing authorized feature branch"
        );
        let remote = git::call(&self.state, &self.repo, &["remote", "get-url", "origin"])?;
        let name = forge::repository(&remote)?;
        forge::live(&self.state, &self.repo, &name, &branch)?;
        let request = fs::read(workspace::contained(&self.repo, &input.request)?)?;
        ensure!(request.len() <= 64 * 1024, "request exceeds 64 KiB");
        let policy_bytes = fs::read(workspace::contained(&self.repo, &input.policy)?)?;
        let policy: Policy = toml::from_str(std::str::from_utf8(&policy_bytes)?)?;
        policy.validate().map_err(anyhow::Error::msg)?;
        let selected = catalog::select(&self.repo, &input.workflow)?;
        let mut methods = BTreeMap::new();
        let mut method_paths = BTreeMap::new();
        for (method, data) in &selected.methods {
            methods.insert(method.clone(), self.put(&data.bytes)?);
            method_paths.insert(method.clone(), data.path.clone());
        }
        let base = git::call(&self.state, &self.repo, &["rev-parse", "HEAD"])?;
        let id = uuid::Uuid::new_v4().to_string();
        let checkpoint = self.snapshot(&self.repo)?;
        let workspace = workspace::restore(
            &self.state,
            &self.repo,
            &self.store.artifacts,
            &checkpoint,
            &id,
            1,
        )?;
        Ok(Run {
            id,
            repository: self.repo.to_string_lossy().into(),
            remote,
            branch,
            base,
            trunk,
            workflow: self.put(&selected.bytes)?,
            selection: Some(WorkflowSelection {
                id: selected.id,
                path: selected.path,
            }),
            methods,
            method_paths,
            policy: self.put(&policy_bytes)?,
            policy_path: input.policy.clone(),
            request: self.put(&request)?,
            plan: None,
            task: None,
            host: input.host,
            epoch: 1,
            stage: Stage::Planning,
            workspace: workspace.to_string_lossy().into(),
            checkout_snapshot: input.native.then(|| checkpoint.clone()),
            checkpoint,
            pending: None,
            transfer: None,
            memory_pending: false,
            prepared_commit: None,
            verification: None,
            publication: None,
            allow_commit: input.allow_commit,
            allow_publish: input.allow_publish,
            revision: 0,
        })
    }
    fn checkpoint(&self, run: &Run) -> Result<String> {
        self.snapshot(Path::new(&run.workspace))
    }
    fn restore(&self, run: &Run, snapshot: &str, epoch: u64) -> Result<String> {
        Ok(workspace::restore(
            &self.state,
            &self.repo,
            &self.store.artifacts,
            snapshot,
            &run.id,
            epoch,
        )?
        .to_string_lossy()
        .into())
    }
    fn create_task(&self, run: &Run) -> Result<String> {
        self.tasks().create(run)
    }
    fn claim(&self, run: &Run, previous: Option<&Run>) -> Result<()> {
        self.tasks()
            .claim(run, previous.map(Tasks::previous_actor).as_deref())
    }
    fn claim_readback(&self, run: &Run) -> Result<()> {
        self.tasks().ensure_claim(run)
    }
    fn complete_task(&self, run: &Run) -> Result<()> {
        self.tasks().complete(run)
    }
    fn recall(&self, query: &str) -> Value {
        self.memory
            .as_ref()
            .map(|m| m.recall(query))
            .unwrap_or_else(
                || json!({"retrieval":"unavailable","authority":"context-only","nodes":[]}),
            )
    }
    async fn remember(&self, run: &Run, phase: &str, source: &str, summary: &str) -> Result<()> {
        self.memory
            .as_ref()
            .context("Prime unavailable; run state remains readable and indexing remains pending")?
            .record(&run.id, run.workflow_id(), phase, source, summary)
            .await
    }
    fn invoke(&self, run: &Run, prompt: &str, planning: bool) -> Result<String> {
        hosts::invoke(&self.state, run, &self.policy(run)?, prompt, planning)
    }
    fn write_plan(&self, run: &Run, text: &str) -> Result<String> {
        workspace::write_plan(&self.state, Path::new(&run.workspace), &run.id, text)
    }
    fn prepare(&self, run: &Run, message: &str) -> Result<String> {
        let branch = git::call(&self.state, &self.repo, &["branch", "--show-current"])?;
        ensure!(
            branch == run.branch && branch != run.trunk,
            "original checkout branch changed"
        );
        forge::live(
            &self.state,
            &self.repo,
            &forge::repository(&run.remote)?,
            &run.branch,
        )?;
        let head = git::call(&self.state, &self.repo, &["rev-parse", "HEAD"])?;
        let current = self.snapshot(&self.repo)?;
        let target = workspace::read_snapshot(&self.store.artifacts, &run.checkpoint)?;
        let observed = workspace::read_snapshot(&self.store.artifacts, &current)?;
        let identical = observed.files == target.files;
        let owned_dirty = run
            .checkout_snapshot
            .as_ref()
            .map(|key| workspace::read_snapshot(&self.store.artifacts, key))
            .transpose()?
            .is_some_and(|snapshot| snapshot.files == observed.files);
        let marker = format!("Allstack-Run: {}", run.id);
        let managed = git::call(&self.state, &self.repo, &["log", "-1", "--format=%B"])?
            .lines()
            .any(|line| line == marker);
        ensure!(
            head == run.base
                || run.prepared_commit.as_ref() == Some(&head)
                || (managed && identical),
            "original checkout moved outside the run"
        );
        if !identical {
            if !owned_dirty {
                git::clean(&self.state, &self.repo)?;
            }
            workspace::materialize(
                &self.state,
                &self.repo,
                &self.store.artifacts,
                &run.checkpoint,
            )?;
        }
        if head != run.base && identical && managed {
            git::clean(&self.state, &self.repo)?;
            return Ok(head);
        }
        let range = format!("origin/{}..HEAD", run.trunk);
        let authors = git::call(&self.state, &self.repo, &["log", "--format=%ae", &range])?;
        let owner = git::call(&self.state, &self.repo, &["config", "user.email"])?;
        ensure!(
            authors.lines().all(|a| a == owner),
            "branch contains another author's commits"
        );
        let count: usize =
            git::call(&self.state, &self.repo, &["rev-list", "--count", &range])?.parse()?;
        ensure!(
            count <= 1,
            "collapse owned branch explicitly before starting run"
        );
        let changed = git::call(&self.state, &self.repo, &["diff", "--name-only", "-z"])?;
        let paths: Vec<&str> = changed.split('\0').filter(|s| !s.is_empty()).collect();
        for chunk in paths.chunks(100) {
            let mut args = vec!["add", "--"];
            args.extend(chunk);
            git::call(&self.state, &self.repo, &args)?;
        }
        ensure!(
            !git::call(
                &self.state,
                &self.repo,
                &["diff", "--cached", "--name-only", "-z"]
            )?
            .is_empty(),
            "candidate has no changes to commit"
        );
        git::call(&self.state, &self.repo, &["diff", "--cached", "--check"])?;
        let mut args = vec!["commit"];
        if count == 1 {
            args.push("--amend");
        }
        args.extend(["-m", message, "-m", &marker]);
        git::call(&self.state, &self.repo, &args)?;
        git::clean(&self.state, &self.repo)?;
        git::call(&self.state, &self.repo, &["rev-parse", "HEAD"])
    }
    fn candidate(&self, run: &Run) -> Result<Verification> {
        git::clean(&self.state, &self.repo)?;
        ensure!(
            git::call(&self.state, &self.repo, &["branch", "--show-current"])? == run.branch,
            "candidate branch changed"
        );
        let snapshot = self.snapshot(&self.repo)?;
        ensure!(
            workspace::content_digest(&self.store.artifacts, &snapshot)?
                == workspace::content_digest(&self.store.artifacts, &run.checkpoint)?,
            "candidate content differs from accepted checkpoint"
        );
        catalog::verify_scope(&self.repo, run)?;
        ensure!(
            digest(&fs::read(workspace::contained(
                &self.repo,
                &run.policy_path
            )?)?)
                == run.policy,
            "verification policy file changed from accepted scope"
        );
        Ok(Verification {
            commit: git::call(&self.state, &self.repo, &["rev-parse", "HEAD"])?,
            snapshot: workspace::content_digest(&self.store.artifacts, &snapshot)?,
            policy: run.policy.clone(),
            workflow: run.workflow.clone(),
            environment: self.environment(&self.policy(run)?)?,
            logs: vec![],
        })
    }
    fn gates(&self, _run: &Run, policy: &Policy) -> Result<Vec<String>> {
        let mut logs = Vec::new();
        for gate in &policy.gates {
            let _build = if gate.compile {
                Some(super::build_lock::acquire(&self.state, &self.repo)?)
            } else {
                None
            };
            let outcome = process::run(Spec {
                argv: gate.argv.clone(),
                cwd: self.repo.clone(),
                seconds: policy.command_seconds.min(290),
                environment: BTreeMap::new(),
                input: None,
                directory: self.state.join("process"),
            })?;
            let log = self.put(&serde_json::to_vec(&outcome)?)?;
            logs.push(log);
            ensure!(
                outcome.success(),
                "gate {} failed; receipt {}",
                gate.name,
                logs.last().unwrap()
            );
            if let Some(marker) = &gate.success_marker {
                ensure!(
                    outcome.stdout.contains(marker) || outcome.stderr.contains(marker),
                    "gate {} missing execution marker",
                    gate.name
                );
            }
            if gate.test {
                let text = format!("{}\n{}", outcome.stdout, outcome.stderr);
                ensure!(
                    super::gates::executed_tests(&text) > 0,
                    "zero-test or unsupported test-runner output is inconclusive; use Cargo or nextest with retained execution summary"
                );
            }
        }
        Ok(logs)
    }
    fn publish(&self, run: &Run, title: &str) -> Result<Publication> {
        forge::publish(&self.state, run, title)
    }
    async fn cleanup(&self, run: &Run) -> Result<()> {
        let mut workspaces = BTreeMap::new();
        for event in self.store.history(&run.id).await? {
            workspaces.insert(event.run.workspace, event.run.checkpoint);
        }
        for (path, key) in workspaces {
            let path = Path::new(&path);
            if !path.exists() {
                continue;
            }
            let parent = self.state.join("workspaces").canonicalize()?;
            ensure!(
                path.canonicalize()?.parent() == Some(parent.as_path())
                    && path
                        .file_name()
                        .and_then(|p| p.to_str())
                        .is_some_and(|p| p.starts_with(&format!("{}-", run.id))),
                "cleanup path is outside owned run workspaces"
            );
            let actual = workspace::read_snapshot(&self.store.artifacts, &self.snapshot(path)?)?;
            let accepted = workspace::read_snapshot(&self.store.artifacts, &key)?;
            ensure!(
                actual == accepted,
                "workspace has unexpected edits; preserve and inspect before cleanup: {}",
                path.display()
            );
            git::call(
                &self.state,
                &self.repo,
                &[
                    "worktree",
                    "remove",
                    "--force",
                    path.to_str().context("non-UTF8 workspace")?,
                ],
            )?;
        }
        Ok(())
    }
}
