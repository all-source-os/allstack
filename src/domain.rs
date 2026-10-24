use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Host {
    Codex,
    ClaudeCode,
}

impl Host {
    pub fn name(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::ClaudeCode => "claude-code",
        }
    }
}

impl std::str::FromStr for Host {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "codex" => Ok(Self::Codex),
            "claude-code" => Ok(Self::ClaudeCode),
            _ => Err("host must be codex or claude-code".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
    pub from_host: Host,
    pub from_epoch: u64,
    pub to_host: Host,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Planning,
    Building,
    Verifying,
    Shipping,
    Shipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FileEntry {
    pub digest: String,
    pub executable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub base: String,
    pub files: BTreeMap<String, FileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gate {
    pub name: String,
    pub argv: Vec<String>,
    pub success_marker: Option<String>,
    #[serde(default)]
    pub test: bool,
    #[serde(default)]
    pub compile: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub gates: Vec<Gate>,
    #[serde(default = "default_deadline")]
    pub command_seconds: u64,
    #[serde(default)]
    pub environment: Vec<String>,
}

fn default_deadline() -> u64 {
    120
}

impl Policy {
    pub fn validate(&self) -> Result<(), String> {
        if self.command_seconds == 0 || self.command_seconds > 300 || self.gates.is_empty() {
            return Err("require gates and command_seconds in 1..=300".into());
        }
        if !self.gates.iter().any(|g| g.test) {
            return Err("at least one executed-test gate is required".into());
        }
        let mut names = std::collections::BTreeSet::new();
        for gate in &self.gates {
            if gate.argv.is_empty() || !names.insert(&gate.name) {
                return Err("gate names must be unique and argv cannot be empty".into());
            }
            if gate.test && gate.success_marker.as_deref().is_none_or(str::is_empty) {
                return Err("test gates require a nonempty execution success marker".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verification {
    pub commit: String,
    pub snapshot: String,
    pub policy: String,
    pub workflow: String,
    pub environment: String,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub commit: String,
    pub remote: String,
    pub branch: String,
    pub pr: String,
    pub task_completed: bool,
    pub memory_indexed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowSelection {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub id: String,
    pub repository: String,
    pub remote: String,
    pub branch: String,
    pub base: String,
    pub trunk: String,
    pub workflow: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection: Option<WorkflowSelection>,
    pub methods: BTreeMap<String, String>,
    pub method_paths: BTreeMap<String, String>,
    pub policy: String,
    pub policy_path: String,
    pub request: String,
    pub plan: Option<String>,
    pub task: Option<String>,
    pub host: Host,
    pub epoch: u64,
    pub stage: Stage,
    pub workspace: String,
    pub checkpoint: String,
    pub pending: Option<String>,
    pub transfer: Option<Transfer>,
    pub memory_pending: bool,
    pub prepared_commit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_snapshot: Option<String>,
    pub verification: Option<Verification>,
    pub publication: Option<Publication>,
    pub allow_commit: bool,
    pub allow_publish: bool,
    pub revision: u64,
}

impl Run {
    pub fn workflow_id(&self) -> &str {
        self.selection
            .as_ref()
            .map_or("delivery", |s| s.id.as_str())
    }

    pub fn workflow_path(&self) -> &str {
        self.selection
            .as_ref()
            .map_or("workflows/delivery.toml", |s| s.path.as_str())
    }

    pub fn check_epoch(&self, epoch: u64) -> Result<(), String> {
        if self.epoch != epoch {
            Err(format!(
                "stale host epoch: expected {}, received {epoch}",
                self.epoch
            ))
        } else if self.stage == Stage::Shipped {
            Err("run already shipped".into())
        } else {
            Ok(())
        }
    }

    pub fn require_resolved(&self) -> Result<(), String> {
        match &self.pending {
            Some(operation) => Err(format!(
                "unresolved operation {operation}; reconcile before execution"
            )),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub schema: u32,
    pub operation: String,
    pub previous: String,
    pub kind: String,
    pub run: Run,
}

pub fn validate_transition(previous: Option<&Run>, event: &Event) -> Result<(), String> {
    if ![
        "started",
        "intent",
        "task_bound",
        "task_claimed",
        "planned",
        "built",
        "ownership_changed",
        "handoff_completed",
        "checkpointed",
        "reconciled",
        "verified",
        "destination_confirmed",
        "task_completed",
        "memory_indexed",
        "shipped",
    ]
    .contains(&event.kind.as_str())
    {
        return Err("unknown workflow event kind".into());
    }
    if event.schema != 1 || event.operation.is_empty() {
        return Err("unsupported event schema or absent operation id".into());
    }
    match previous {
        None if event.kind == "started" && event.run.revision == 1 && event.run.epoch == 1 => {
            Ok(())
        }
        Some(old) if old.id == event.run.id && old.revision + 1 == event.run.revision => {
            if old.repository != event.run.repository
                || old.branch != event.run.branch
                || old.workflow != event.run.workflow
                || old.selection != event.run.selection
                || old.methods != event.run.methods
                || old.method_paths != event.run.method_paths
                || old.policy != event.run.policy
                || old.policy_path != event.run.policy_path
                || old.remote != event.run.remote
                || old.request != event.run.request
                || old.base != event.run.base
                || old.trunk != event.run.trunk
                || old.allow_commit != event.run.allow_commit
                || old.allow_publish != event.run.allow_publish
            {
                return Err("immutable run scope changed".into());
            }
            let same = old.stage == event.run.stage;
            let allowed = matches!(
                (old.stage, event.run.stage),
                (Stage::Planning, Stage::Building)
                    | (Stage::Building, Stage::Verifying)
                    | (Stage::Verifying, Stage::Building)
                    | (Stage::Verifying, Stage::Shipping)
                    | (Stage::Shipping, Stage::Shipped)
            );
            if !same && !allowed {
                return Err("invalid stage transition".into());
            }
            if event.run.epoch < old.epoch || event.run.epoch > old.epoch + 1 {
                return Err("invalid ownership epoch".into());
            }
            if (event.run.epoch != old.epoch || event.run.host != old.host)
                && (event.kind != "ownership_changed"
                    || event.run.epoch != old.epoch + 1
                    || old.transfer.as_ref().is_none_or(|t| {
                        t.from_host != old.host
                            || t.from_epoch != old.epoch
                            || t.to_host != event.run.host
                    }))
            {
                return Err("ownership changed without a bound handoff".into());
            }
            if old.stage == Stage::Shipped {
                return Err("terminal run cannot change".into());
            }
            if matches!(event.run.stage, Stage::Shipping | Stage::Shipped)
                && event.run.verification.is_none()
            {
                return Err("publication requires verification".into());
            }
            if event.run.stage == Stage::Shipped && event.run.publication.is_none() {
                return Err("shipped requires destination receipt".into());
            }
            Ok(())
        }
        _ => Err("missing, duplicated, or invalid event sequence".into()),
    }
}
